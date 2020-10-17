//! This module contains functions related to reading saved words
//!
//! Anvil is the name of the format used by Minecraft to store world files.
//! It contains all the block data, entities, and biome info of each chunk.

pub use anvil_region::AnvilChunkProvider;
pub use anvil_region::FolderChunkProvider;
pub use anvil_region::ZipChunkProvider;
use anvil_region::ChunkLoadError;
use nbt::CompoundTag;
use zip::ZipArchive;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::PathBuf;
use std::fs::OpenOptions;
use log::*;
use crate::biome_layers::is_oceanic;
use crate::biome_info::biome_id;
use crate::chunk::Point;
use crate::chunk::Point4;
use crate::seed_info::BiomeId;
use crate::seed_info::MinecraftVersion;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;

/// Read all the existing chunks in a `area_size*area_size` block area around
/// `(block_x, block_z)`.
pub fn read_area_around<A: AnvilChunkProvider>(chunk_provider: &mut A, area_size: u64, Point { x: block_x, z: block_z }: Point) -> Result<Vec<CompoundTag>, ChunkLoadError> {
    let mut r = vec![];
    let start_x = (block_x >> 4) as i32;
    let start_z = (block_z >> 4) as i32;
    let ahc = i32::try_from((area_size >> 4) >> 1).unwrap();
    for chunk_x in -ahc..=ahc {
        for chunk_z in -ahc..=ahc {
            match chunk_provider.load_chunk(start_x + chunk_x, start_z + chunk_z) {
                Ok(c) => r.push(c),
                // Expected errors: region or chunk do not exist
                Err(ChunkLoadError::RegionNotFound { .. }) => {}
                Err(ChunkLoadError::ChunkNotFound { .. }) => {}
                // Unexpected errors:
                Err(e) => return Err(e),
            }
        }
    }

    Ok(r)
}

fn is_common_biome(b: BiomeId) -> bool {
    // These biomes have more than a 90% chance of appearing inside
    // a 2000x2000-block square around (0, 0) for any random seed
    // So they make a bad filter because 90% of all seeds can have this biomes
    match b.0 {
        // Skip plains (1) and forest (4)
        // And also skip rivers (7) because they can break some code that
        // assumes all rivers are used for river_seed_finder...
        1 | 4 | 7 => true,
        _ => false,
    }
}

/// Given a map of chunk_coords to river_biome_count, return the chunk with
/// the most rivers in its 3x3 chunk area.
pub fn best_river_chunk(river_chunks: &HashMap<(i32, i32), u8>) -> Option<(i32, i32)> {
    let mut best: Option<(u16, (i32, i32))> = None;

    for ((x, z), score) in river_chunks {
        let x = *x;
        let z = *z;
        let mut s = *score as u16;

        let start_x = x - 1;
        let start_z = z - 1;

        for x in 0..3 {
            for z in 0..3 {
                if let Some(ss) = river_chunks.get(&(start_x+x, start_z+z)) {
                    s += u16::from(*ss);
                }
            }
        }

        match best {
            None => best = Some((s, (x, z))),
            Some((best_score, _chunk)) if s > best_score => best = Some((s, (x, z))),
            _ => {}
        }
    }

    if let Some((score, chunk)) = best {
        debug!("The best river chunk is {:?} with score {}", chunk, score);
    }

    best.map(|(_score, chunk)| chunk)
}

/// Given a path to "saved_minecraft_world/region", read the region files and
///
/// * Find a 3x3 chunk area with many river blocks
/// * Return a few extra biomes
///
/// This is meant to be used together with river_seed_finder.
pub fn get_rivers_and_some_extra_biomes_folder(input_dir: &PathBuf, center_block_arg: Point) -> (Vec<Point>, Vec<(BiomeId, Point)>) {
    let mut chunk_provider = FolderChunkProvider::new(input_dir.to_str().unwrap());

    get_rivers_and_some_extra_biomes(&mut chunk_provider, center_block_arg)
}

pub fn get_rivers_and_some_extra_biomes_zip(input_zip: &PathBuf, center_block_arg: Point) -> (Vec<Point>, Vec<(BiomeId, Point)>) {
    let mut chunk_provider = ZipChunkProvider::file(input_zip).unwrap();

    get_rivers_and_some_extra_biomes(&mut chunk_provider, center_block_arg)
}

pub fn get_rivers_and_some_extra_biomes_zip_1_15(input_zip: &PathBuf, center_block_arg: Point) -> (Vec<Point4>, Vec<(BiomeId, Point4)>) {
    let mut chunk_provider = ZipChunkProvider::file(input_zip).unwrap();

    get_rivers_and_some_extra_biomes_1_15(&mut chunk_provider, center_block_arg)
}

pub fn get_rivers_and_some_extra_biomes<A: AnvilChunkProvider>(chunk_provider: &mut A, center_block_arg: Point) -> (Vec<Point>, Vec<(BiomeId, Point)>) {
    let blocks_around_center: u32 = 1_000;

    let mut biome_data = HashMap::new();
    let mut rivers = vec![];
    let cheb = spiral::ChebyshevIterator::new(0, 0, u16::max_value());
    for (cheb_i, (cheb_x, cheb_z)) in cheb.enumerate() {
        if cheb_i == 10 {
            warn!("This is taking longer than expected");
            if let Point { x: 0, z: 0 } = center_block_arg {
                warn!("Please feel free to specify some center coordinates to speed up the process.");
            } else {
                warn!("The provided coordinates are probably wrong: {:?}", center_block_arg);
                warn!("Please double check that there are rivers near this block coordinates");
            }
        }
        let center_block = Point { x: center_block_arg.x + i64::from(cheb_x) * i64::from(blocks_around_center), z: center_block_arg.z + i64::from(cheb_z) * i64::from(blocks_around_center) };
        debug!("Trying to find chunks around {:?}", center_block);
        let chunks = read_area_around(chunk_provider, u64::from(blocks_around_center), center_block).unwrap();
        if chunks.is_empty() {
            debug!("Area around {:?} is not present in the saved world", center_block);
            continue;
        }

        for c in chunks {
            let level_compound_tag = c.get_compound_tag("Level").unwrap();
            let chunk_x = level_compound_tag.get_i32("xPos").unwrap();
            let chunk_z = level_compound_tag.get_i32("zPos").unwrap();
            let biomes_array_17;
            let biomes_array = match level_compound_tag.get_i32_vec("Biomes") {
                Ok(x) => x,
                Err(_e) => {
                    match level_compound_tag.get_i8_vec("Biomes") {
                        Ok(x) => {
                            // Important: before 1.13 biomes was a byte array,
                            // i8 is wrong, u8 is correct
                            biomes_array_17 = x.into_iter().map(|byte| i32::from(*byte as u8)).collect();
                            &biomes_array_17
                        }
                        Err(e) => panic!("Unknown format for biomes array: {:?}", e),
                    }
                }
            };
            match biomes_array.len() {
                0 => {}
                256 => {}
                n => panic!("Unexpected biomes_array len: {}", n),
            }

            let mut use_rivers_from_chunk = true;
            let mut chunk_rivers = vec![];
            // Only add at most 1 extra biome per chunk
            let mut extra_biomes_per_chunk = 1;
            for (i_b, b) in biomes_array.into_iter().enumerate() {
                let block_x = i64::from(chunk_x) * 16 + (i_b % 16) as i64;
                let block_z = i64::from(chunk_z) * 16 + (i_b / 16) as i64;
                let b = b.clone();

                match b {
                    127 => {
                        // Ignore void biome (set by WorldDownloader for unknown biomes)
                    }
                    b => {
                        // We want to skip rivers near oceanic biomes
                        use_rivers_from_chunk &= !is_oceanic(b);
                        // In mushroom islands rivers are converted to shore, so skip them
                        use_rivers_from_chunk &= b != biome_id::mushroomIslandShore;
                        // Also skip chunks with frozen rivers, as they may be a problem
                        use_rivers_from_chunk &= b != biome_id::frozenRiver;
                        if use_rivers_from_chunk && b == biome_id::river {
                            // Store all the rivers
                            chunk_rivers.push(Point { x: block_x, z: block_z });
                        }

                        // Do not insert common biomes
                        if extra_biomes_per_chunk > 0 && !is_common_biome(BiomeId(b)) {
                            biome_data.insert(Point { x: block_x, z: block_z }, BiomeId(b));
                            extra_biomes_per_chunk -= 1;
                        }
                    }
                }
            }

            if use_rivers_from_chunk {
                rivers.extend(chunk_rivers);
            }
        }

        if biome_data.len() < 50 {
            debug!("Not enough chunks found around {:?}. Maybe that part of the map is not generated? (found {} biomes)", center_block, biome_data.len());
            continue;
        }

        if rivers.len() < 300 {
            debug!("Not enough rivers found around {:?}. Please try again with different coords. (found {} rivers)", center_block, rivers.len());
            continue;
        }

        debug!("biome_data.len(): {}", biome_data.len());

        let mut extra_biomes = vec![];
        // Hashmap iteration follows a random order, so take some random biomes
        extra_biomes.extend(biome_data.iter().map(|(p, b)| (*b, *p)).take(30));
        debug!("extra_biomes: {:?}", extra_biomes);

        return (rivers, extra_biomes);
    }

    error!("Found zero valid chunks. Is this even a minecraft save?");

    (vec![], vec![])
}

pub fn get_rivers_and_some_extra_biomes_1_15<A: AnvilChunkProvider>(chunk_provider: &mut A, center_block_arg: Point) -> (Vec<Point4>, Vec<(BiomeId, Point4)>) {
    let blocks_around_center: u32 = 1_000;

    let mut biome_data = HashMap::new();
    let mut rivers = vec![];
    let cheb = spiral::ChebyshevIterator::new(0, 0, u16::max_value());
    for (cheb_i, (cheb_x, cheb_z)) in cheb.enumerate() {
        if cheb_i == 10 {
            warn!("This is taking longer than expected");
            if let Point { x: 0, z: 0 } = center_block_arg {
                warn!("Please feel free to specify some center coordinates to speed up the process.");
            } else {
                warn!("The provided coordinates are probably wrong: {:?}", center_block_arg);
                warn!("Please double check that there are rivers near this block coordinates");
            }
        }
        let center_block = Point { x: center_block_arg.x + i64::from(cheb_x) * i64::from(blocks_around_center), z: center_block_arg.z + i64::from(cheb_z) * i64::from(blocks_around_center) };
        debug!("Trying to find chunks around {:?}", center_block);
        let chunks = read_area_around(chunk_provider, u64::from(blocks_around_center), center_block).unwrap();
        if chunks.is_empty() {
            debug!("Area around {:?} is not present in the saved world", center_block);
            continue;
        }

        for c in chunks {
            let level_compound_tag = c.get_compound_tag("Level").unwrap();
            let biomes_array = match level_compound_tag.get_i32_vec("Biomes") {
                Ok(x) => x,
                Err(nbt::CompoundTagError::TagNotFound { .. }) => {
                    // Starting from 1.16, it is possible that the "Biomes" tag is missing from
                    // chunks that are not fully generated yet. We simply ignore these chunks as if
                    // they did not exist
                    continue;
                }
                Err(e) => panic!("Unknown format for biomes array: {:?}", e),
            };
            match biomes_array.len() {
                0 => {}
                1024 => {}
                n => panic!("Unexpected biomes_array len: {}", n),
            }

            info!("biomes_array: {:?}", biomes_array);
            let chunk_x = level_compound_tag.get_i32("xPos").unwrap();
            let chunk_z = level_compound_tag.get_i32("zPos").unwrap();

            let mut use_rivers_from_chunk = true;
            let mut chunk_rivers = vec![];
            // Only add at most 1 extra biome per chunk
            let mut extra_biomes_per_chunk = 1;
            for (i_b, b) in biomes_array.into_iter().enumerate().take(4 * 4) {
                // TODO: this is not tested
                let block_x = i64::from(chunk_x) * 4 + (i_b % 4) as i64;
                let block_z = i64::from(chunk_z) * 4 + ((i_b / 4) % 4) as i64;
                let b = b.clone();

                match b {
                    127 => {
                        // Ignore void biome (set by WorldDownloader for unknown biomes)
                    }
                    b => {
                        // We want to skip rivers near oceanic biomes
                        use_rivers_from_chunk &= !is_oceanic(b);
                        // In mushroom islands rivers are converted to shore, so skip them
                        use_rivers_from_chunk &= b != biome_id::mushroomIslandShore;
                        // Also skip chunks with frozen rivers, as they may be a problem
                        use_rivers_from_chunk &= b != biome_id::frozenRiver;
                        if use_rivers_from_chunk && b == biome_id::river {
                            // Store all the rivers
                            chunk_rivers.push(Point4 { x: block_x, z: block_z });
                        }

                        // Do not insert common biomes
                        if extra_biomes_per_chunk > 0 && !is_common_biome(BiomeId(b)) {
                            biome_data.insert(Point4 { x: block_x, z: block_z }, BiomeId(b));
                            extra_biomes_per_chunk -= 1;
                        }
                    }
                }
            }

            if use_rivers_from_chunk {
                rivers.extend(chunk_rivers);
            }
        }

        if biome_data.len() < 50 {
            debug!("Not enough chunks found around {:?}. Maybe that part of the map is not generated? (found {} biomes)", center_block, biome_data.len());
            continue;
        }

        if rivers.len() < 30 {
            debug!("Not enough rivers found around {:?}. Please try again with different coords. (found {} rivers)", center_block, rivers.len());
            continue;
        }

        debug!("biome_data.len(): {}", biome_data.len());

        let mut extra_biomes = vec![];
        // Hashmap iteration follows a random order, so take some random biomes
        extra_biomes.extend(biome_data.iter().map(|(p, b)| (*b, *p)).take(30));
        debug!("extra_biomes: {:?}", extra_biomes);

        return (rivers, extra_biomes);
    }

    error!("Found zero valid chunks. Is this even a minecraft save?");

    (vec![], vec![])
}

pub fn get_biomes_from_chunk_1_15(chunk: &CompoundTag) -> Result<&Vec<i32>, String> {
    let level_compound_tag = chunk.get_compound_tag("Level").unwrap();
    let biomes_array = match level_compound_tag.get_i32_vec("Biomes") {
        Ok(x) => x,
        Err(e) => panic!("Unknown format for biomes array: {:?}", e),
    };
    match biomes_array.len() {
        0 => {}
        1024 => {}
        n => panic!("Unexpected biomes_array len: {}", n),
    }

    Ok(biomes_array)
}

pub fn get_biomes_from_chunk_1_14(chunk: &CompoundTag) -> Result<Vec<i32>, String> {
    let level_compound_tag = chunk.get_compound_tag("Level").unwrap();
    let biomes_array = match level_compound_tag.get_i32_vec("Biomes") {
        Ok(x) => x.clone(),
        Err(_e) => {
            match level_compound_tag.get_i8_vec("Biomes") {
                Ok(x) => {
                    // Important: before 1.13 biomes was a byte array,
                    // i8 is wrong, u8 is correct
                    x.into_iter().map(|byte| i32::from(*byte as u8)).collect()
                }
                Err(e) => panic!("Unknown format for biomes array: {:?}", e),
            }
        }
    };
    match biomes_array.len() {
        0 => {}
        256 => {}
        n => panic!("Unexpected biomes_array len: {}", n),
    }

    Ok(biomes_array)
}

/// Get all the biomes present in the save. For version >= 1.15
pub fn get_all_biomes_1_15<A: AnvilChunkProvider>(chunk_provider: &mut A) -> Vec<(BiomeId, Point4)> {
    let mut biome_data = HashMap::new();
    let all_chunks = chunk_provider.list_chunks().expect("Error listing chunks");
    for (chunk_x, chunk_z) in all_chunks {
        let c = chunk_provider.load_chunk(chunk_x, chunk_z).expect("Error loading chunk");

        let biomes_array = get_biomes_from_chunk_1_15(&c).unwrap();

        for (i_b, b) in biomes_array.into_iter().enumerate().take(4 * 4) {
            // TODO: this is not tested
            let block_x = i64::from(chunk_x) * 4 + (i_b % 4) as i64;
            let block_z = i64::from(chunk_z) * 4 + ((i_b / 4) % 4) as i64;
            let b = b.clone();

            match b {
                127 => {
                    // Ignore void biome (set by WorldDownloader for unknown biomes)
                }
                b => {
                    biome_data.insert(Point4 { x: block_x, z: block_z }, BiomeId(b));
                }
            }
        }
    }

    debug!("biome_data.len(): {}", biome_data.len());

    let mut extra_biomes = vec![];
    extra_biomes.extend(biome_data.iter().map(|(p, b)| (*b, *p)));
    //debug!("extra_biomes: {:?}", extra_biomes);

    return extra_biomes;
}

/// Get all the biomes present in the save. For version <= 1.14
pub fn get_all_biomes_1_14<A: AnvilChunkProvider>(chunk_provider: &mut A) -> Vec<(BiomeId, Point)> {
    let mut biome_data = HashMap::new();
    let all_chunks = chunk_provider.list_chunks().expect("Error listing chunks");
    for (chunk_x, chunk_z) in all_chunks {
        let c = chunk_provider.load_chunk(chunk_x, chunk_z).expect("Error loading chunk");

        let biomes_array = get_biomes_from_chunk_1_14(&c).unwrap();

        let mut all_water = true;
        let mut temp_biome_data = Vec::with_capacity(16*16);
        for (i_b, b) in biomes_array.into_iter().enumerate() {
            let block_x = i64::from(chunk_x) * 16 + (i_b % 16) as i64;
            let block_z = i64::from(chunk_z) * 16 + (i_b / 16) as i64;
            let b = b.clone();

            match b {
                127 => {
                    // Ignore void biome (set by WorldDownloader for unknown biomes)
                }
                b => {
                    // Check if at least one biome is different from ocean (id 0)
                    // This is because in some minecraft versions, some chunks may not have the
                    // biomes array ready and they temporarily fill it will all zeros.
                    // We want to ignore chunks that are all ocean.
                    // Unfortunately this means that if the generated worlds as a chunk all ocean
                    // biome, and this program generetes a different biome, it will be not detected
                    // in tests.
                    // Alternatives: we could simply remove a 2-wide chunk margin.
                    if b != 0 {
                        all_water = false;
                    }
                    temp_biome_data.push((Point { x: block_x, z: block_z}, BiomeId(b)));
                }
            }
        }

        if !all_water {
            biome_data.extend(temp_biome_data);
        }
    }

    debug!("biome_data.len(): {}", biome_data.len());

    let mut extra_biomes = vec![];
    extra_biomes.extend(biome_data.iter().map(|(p, b)| (*b, *p)));
    //debug!("extra_biomes: {:?}", extra_biomes);

    return extra_biomes;
}

pub fn read_seed_from_level_dat_zip(input_zip: &PathBuf, minecraft_version: Option<MinecraftVersion>) -> Result<i64, String> {
    let reader = OpenOptions::new()
        .write(false)
        .read(true)
        .create(false)
        .open(input_zip)
        .map_err(|e| format!("Failed to open file: {:?}", e))?;

    let mut zip_archive = ZipArchive::new(reader).map_err(|e| format!("Failed to read zip: {:?}", e))?;
    let level_dat_path = find_level_dat(&mut zip_archive)?;
    let mut level_dat = zip_archive.by_name(&level_dat_path).map_err(|e| format!("level.dat path incorrectly set: {:?}", e))?;

    match minecraft_version {
        Some(MinecraftVersion::Java1_7)
        | Some(MinecraftVersion::Java1_13)
        | Some(MinecraftVersion::Java1_14)
        | Some(MinecraftVersion::Java1_15) => read_seed_from_level_dat_1_15(&mut level_dat),
        Some(MinecraftVersion::Java1_16) => read_seed_from_level_dat_1_16(&mut level_dat),
        Some(_) => return Err("Unimplemented".to_string()),
        None => {
            // Try to guess version, starting from the newest one
            // Mutating level_dat advances the reader, so read file into memory first
            let mut buf = Vec::with_capacity(512);
            level_dat.read_to_end(&mut buf).map_err(|e| format!("Failed to read level.dat: {:?}", e))?;
            // Store all the errors
            let mut errs = vec![];
            Result::<i64, String>::Err(Default::default()).or_else(|_| {
                read_seed_from_level_dat_1_16(&mut Cursor::new(&buf))
            }).or_else(|e| {
                errs.push(("1.16", e));
                read_seed_from_level_dat_1_15(&mut Cursor::new(&buf))
            }).map_err(|e| {
                errs.push(("1.15", e));
            }).map_err(|_| {
                // Convert the list of errors into a String because sadly that's our error type
                let mut s = String::new();
                s.push_str(&"Failed to read level.dat: unsupported version or corrupted file. Detailed list of errors:\n");
                for (version, err) in errs {
                    s.push_str(&format!("* {}: {}\n", version, err));
                }

                s
            })
        }
    }
}

/// Read seed from level.dat in version 1.15 and below.
///
/// root["Data"]["RandomSeed"]
pub fn read_seed_from_level_dat_1_15<R: Read>(r: &mut R) -> Result<i64, String> {
    let root_tag = nbt::decode::read_gzip_compound_tag(r).map_err(|e| format!("Failed to read gzip compount tag: {:?}", e))?;
    let data_tag = root_tag.get_compound_tag("Data").map_err(|e| format!("Failed to read {:?} tag: {:?}", "Data", e))?;
    let seed = data_tag.get_i64("RandomSeed").map_err(|e| format!("Failed to read {:?} tag: {:?}", "RandomSeed", e))?;

    Ok(seed)
}

/// Read seed from level.dat in version 1.16 and above.
///
/// root["Data"]["WorldGenSettings"]["seed"]
pub fn read_seed_from_level_dat_1_16<R: Read>(r: &mut R) -> Result<i64, String> {
    let root_tag = nbt::decode::read_gzip_compound_tag(r).map_err(|e| format!("Failed to read gzip compount tag: {:?}", e))?;
    let data_tag = root_tag.get_compound_tag("Data").map_err(|e| format!("Failed to read {:?} tag: {:?}", "Data", e))?;
    let world_gen_settings_tag = data_tag.get_compound_tag("WorldGenSettings").map_err(|e| format!("Failed to read {:?} tag: {:?}", "WorldGenSettings", e))?;
    let seed = world_gen_settings_tag.get_i64("seed").map_err(|e| format!("Failed to read {:?} tag: {:?}", "seed", e))?;

    Ok(seed)
}

// Find the path of the level.dat file inside the zip archive.
// For example: "level.dat", "world/level.dat" or "saves/world/level.dat"
// Returns error if no region folder is found
// Returns error if more than one folder is found
fn find_level_dat<R: Read + Seek>(
    zip_archive: &mut ZipArchive<R>,
) -> Result<String, String> {
    let mut found_path = String::from("/");
    let mut found_file_count = 0;
    for i in 0..zip_archive.len() {
        // This unwrap is safe because we are iterating from 0 to len
        let file = zip_archive.by_index(i).unwrap();
        let full_path = file.sanitized_name();
        // file_name() returns None when the path ends with "/.."
        // we handle that case as an empty string
        let file_name = full_path.file_name().unwrap_or_default();
        if file_name == "level.dat" {
            found_file_count += 1;
            found_path = file.name().to_string();
            // Keep searching after finding the first folder, to make sure
            // there is only one region/ folder
        }
    }
    if found_file_count == 0 {
        return Err("level.dat not found in zip archive".to_string());
    }
    if found_file_count > 1 {
        return Err("More than one level.dat file found in zip archive".to_string());
    }

    Ok(found_path)
}
