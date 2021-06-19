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
use std::collections::HashSet;
use std::convert::TryFrom;
use std::path::Path;
use std::fs::OpenOptions;
use log::*;
use crate::biome_layers::is_oceanic;
use crate::biome_info::biome_id;
use crate::chunk::Point;
use crate::chunk::Point4;
use crate::seed_info::BiomeId;
use crate::seed_info::MinecraftVersion;
use crate::fastanvil_ext::Dimension;
use crate::fastanvil_ext::region_for_each_chunk;
use crate::zip_ext::find_file_in_zip_exactly_once;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::str::FromStr;

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
pub fn get_rivers_and_some_extra_biomes_folder(input_dir: &Path, center_block_arg: Point) -> (Vec<Point>, Vec<(BiomeId, Point)>) {
    let mut chunk_provider = FolderChunkProvider::new(input_dir.to_str().unwrap());

    get_rivers_and_some_extra_biomes(&mut chunk_provider, center_block_arg)
}

pub fn get_rivers_and_some_extra_biomes_zip(input_zip: &Path, center_block_arg: Point) -> (Vec<Point>, Vec<(BiomeId, Point)>) {
    let mut chunk_provider = ZipChunkProvider::file(input_zip).unwrap();

    get_rivers_and_some_extra_biomes(&mut chunk_provider, center_block_arg)
}

pub fn get_rivers_and_some_extra_biomes_zip_1_15(input_zip: &Path, center_block_arg: Point) -> (Vec<Point4>, Vec<(BiomeId, Point4)>) {
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
            let biomes_array = get_biomes_from_chunk_1_14(&c).unwrap();

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
            let biomes_array = get_biomes_from_chunk_1_15(&c).unwrap();

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

/// Get all the biomes present in the chunk. For version >= 1.15
pub fn get_biomes_from_chunk_1_15(chunk: &CompoundTag) -> Result<&Vec<i32>, String> {
    let level_compound_tag = chunk.get_compound_tag("Level").unwrap();
    let biomes_array = match level_compound_tag.get_i32_vec("Biomes") {
        Ok(x) => x,
        Err(nbt::CompoundTagError::TagNotFound { .. }) => {
            // Starting from 1.16, it is possible that the "Biomes" tag is missing from
            // chunks that are not fully generated yet. We simply ignore these chunks as if
            // they did not exist, by returning an empty list of biomes
            const EMPTY_VEC: &'static Vec<i32> = &Vec::new();
            return Ok(&EMPTY_VEC);
        }
        Err(e) => panic!("Unknown format for biomes array: {:?}", e),
    };
    match biomes_array.len() {
        0 => {}
        1024 => {}
        n => panic!("Unexpected biomes_array len: {}", n),
    }

    Ok(biomes_array)
}

/// Get all the biomes present in the chunk. For version <= 1.14
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

pub fn read_seed_from_level_dat_zip(input_zip: &Path, minecraft_version: Option<MinecraftVersion>) -> Result<i64, String> {
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
        Some(MinecraftVersion::Java1_16_1) | Some(MinecraftVersion::Java1_16) => read_seed_from_level_dat_1_16(&mut level_dat),
        Some(_) => read_seed_from_level_dat_1_15(&mut level_dat),
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
fn find_level_dat<R: Read + Seek>(zip_archive: &mut ZipArchive<R>) -> Result<String, String> {
    find_file_in_zip_exactly_once(zip_archive, "level.dat")
        .map(|x| x.to_string())
        .map_err(|e| format!("Failed to find level.dat in zip archive: {}", e))
}

pub fn find_dungeons<A: AnvilChunkProvider>(chunk_provider: &mut A) -> Result<Vec<((i64, i64, i64), SpawnerKind, Vec<String>)>, String> {
    let all_chunks = chunk_provider.list_chunks().expect("Error listing chunks");
    let mut dungeons = vec![];
    let mut overworld: Dimension<std::fs::File> = Dimension::new();
    let total_chunks = all_chunks.len();
    let mut processed_chunks_count = 0;

    for (chunk_x, chunk_z) in all_chunks {
        if processed_chunks_count % 1024 == 0 {
            log::debug!("{}/{} chunks processed, {} dungeons found", processed_chunks_count, total_chunks, dungeons.len());
        }
        processed_chunks_count += 1;
        let c = chunk_provider.load_chunk(chunk_x, chunk_z).expect("Error loading chunk");
        let spawners = get_all_dungeons_in_chunk(&c).expect("Error getting dungeons");
        let mut more_dungeons = vec![];

        for (x, y, z, kind) in spawners {
            match kind {
                // Ignore spawners that cannot be generated in dungeons
                SpawnerKind::Silverfish |
                SpawnerKind::CaveSpider => continue,
                _ => more_dungeons.push((x, y, z, kind)),
            }
        }

        if more_dungeons.is_empty() {
            continue;
        }

        // TODO: remove this hack:
        // Load this chunk and the 8 surrounding ones into dimension
        // Because we use a different NBT library to read blocks, we serialize the chunk NBT tag
        // using the named-binary-tag crate and deserialize it using the fastanvil crate
        let mut buf = Cursor::new(vec![]);
        nbt::encode::write_compound_tag(&mut buf, c).expect("Serialization failed");

        // To allow the reader to read...
        buf.set_position(0);
        overworld.add_chunk(chunk_x, chunk_z, &mut buf).expect("Failed to add chunk");

        // Load 8 neighbors
        for (chunk_x, chunk_z) in eight_connected((chunk_x, chunk_z)) {
            // Skip if chunk is already loaded
            if overworld.has_chunk(chunk_x, chunk_z) {
                continue;
            }
            // Here, some of the chunks may not exist, so ignore errors
            let c = chunk_provider.load_chunk(chunk_x, chunk_z);
            if c.is_err() {
                continue;
            }
            let c = c.unwrap();

            // TODO: remove this hack:
            // Because we use a different NBT library to read blocks, we serialize the chunk NBT tag
            // using the named-binary-tag crate and deserialize it using the fastanvil crate
            let mut buf = Cursor::new(vec![]);
            nbt::encode::write_compound_tag(&mut buf, c).expect("Serialization failed");

            // Set cursor position to 0 to allow the reader to read from the beginning
            buf.set_position(0);
            overworld.add_chunk(chunk_x, chunk_z, &mut buf).expect("Failed to add chunk neighbor");
        }

        for (x, y, z, kind) in more_dungeons {
            let x = i64::from(x);
            let y = i64::from(y);
            let z = i64::from(z);
            // Sanity check
            assert_eq!(overworld.get_block(x, y, z).unwrap().name, "minecraft:spawner");

            let mut floor = vec![];
            // Read 11x11 area just below the spawner
            let dy = -1;
            for dz in (-5)..=5 {
                for dx in (-5)..=5 {
                    let block = overworld.get_block(x + dx, y + dy, z + dz);
                    let block_name = block.map(|b| b.name).unwrap_or(
                        // If the block does not exist, use empty string instead of block name
                        // This seems to be common when a dungeon is near the edge of the world:
                        // part of the floor is missing
                        ""
                    );
                    // Map the block name to the equivalent DungeonFloor character
                    /*
                    let c = match block_name {
                        "minecraft:cobblestone" => "C",
                        "minecraft:mossy_cobblestone" => "M",
                        "minecraft:air" => "A",
                        _ => "?",
                    };
                    */
                    floor.push(block_name.to_string());
                }
            }

            /*
            let mut floor_string = String::with_capacity(11 * 11 + 11);
            for (i, f) in floor.iter().enumerate() {
                if i > 0 && i % 11 == 0 {
                    floor_string.push_str("\n");
                }
                floor_string.push_str(f);
            }
            println!("Floor below {:?}:", (x, y, z));
            println!("{}", floor_string);
            */

            dungeons.push(((x, y, z), kind, floor));
        }
    }
    log::debug!("All chunks processed, {} dungeons found", dungeons.len());

    Ok(dungeons)
}

pub fn find_spawners<A: AnvilChunkProvider>(chunk_provider: &mut A) -> Result<Vec<((i64, i64, i64), String)>, String> {
    let all_chunks = chunk_provider.list_chunks().expect("Error listing chunks");
    let mut dungeons = vec![];
    let total_chunks = all_chunks.len();
    let mut processed_chunks_count = 0;

    for (chunk_x, chunk_z) in all_chunks {
        if processed_chunks_count % 1024 == 0 {
            log::debug!("{}/{} chunks processed, {} dungeons found", processed_chunks_count, total_chunks, dungeons.len());
        }
        processed_chunks_count += 1;
        let c = chunk_provider.load_chunk(chunk_x, chunk_z).expect("Error loading chunk");
        let spawners = get_all_dungeons_in_chunk2(&c).expect("Error getting dungeons");

        for (x, y, z, kind) in spawners {
            dungeons.push(((x as i64, y as i64, z as i64), kind));
        }
    }
    log::debug!("All chunks processed, {} dungeons found", dungeons.len());

    Ok(dungeons)
}


/// The differents kinds of spawners that can be found in a vanilla minecraft world
#[derive(Copy, Clone, Debug)]
pub enum SpawnerKind {
    CaveSpider,
    Silverfish,
    Skeleton,
    Spider,
    Zombie,
}

impl FromStr for SpawnerKind {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "minecraft:cave_spider" => SpawnerKind::CaveSpider,
            "minecraft:silverfish" => SpawnerKind::Silverfish,
            "minecraft:skeleton" => SpawnerKind::Skeleton,
            "minecraft:spider" => SpawnerKind::Spider,
            "minecraft:zombie" => SpawnerKind::Zombie,
            s => return Err(s.to_string()),
        })
    }
}

impl std::fmt::Display for SpawnerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let static_str = match self {
            SpawnerKind::CaveSpider => "minecraft:cave_spider",
            SpawnerKind::Silverfish => "minecraft:silverfish",
            SpawnerKind::Skeleton => "minecraft:skeleton",
            SpawnerKind::Spider => "minecraft:spider",
            SpawnerKind::Zombie => "minecraft:zombie",
        };
        write!(f, "{}", static_str)
    }
}

pub fn get_all_dungeons_in_chunk(chunk: &CompoundTag) -> Result<Vec<(i32, i32, i32, SpawnerKind)>, String> {
    let mut dungeons = vec![];

    let level_tag = chunk.get_compound_tag("Level").map_err(|e| format!("Failed to read {:?} tag: {:?}", "Level", e))?;
    let tile_entities = level_tag.get_compound_tag_vec("TileEntities").map_err(|e| format!("Failed to read {:?} tag: {:?}", "TileEntities", e))?;

    for (i, tile_entity_tag) in tile_entities.into_iter().enumerate() {
        let id = tile_entity_tag.get_str("id").map_err(|e| format!("Failed to read {:?} tag at position {}: {:?}", "id", i, e))?;
        if id != "minecraft:mob_spawner" {
            continue;
        }

        let x = tile_entity_tag.get_i32("x").map_err(|e| format!("Failed to read {:?} tag at position {}: {:?}", "x", i, e))?;
        let y = tile_entity_tag.get_i32("y").map_err(|e| format!("Failed to read {:?} tag at position {}: {:?}", "y", i, e))?;
        let z = tile_entity_tag.get_i32("z").map_err(|e| format!("Failed to read {:?} tag at position {}: {:?}", "z", i, e))?;

        let spawn_potentials = tile_entity_tag.get_compound_tag_vec("SpawnPotentials").map_err(|e| format!("Failed to read {:?} tag at position {}: {:?}", "SpawnPotentials", i, e))?;
        assert_eq!(spawn_potentials.len(), 1);

        let entity_tag = spawn_potentials[0].get_compound_tag("Entity").map_err(|e| format!("Failed to read SpawnPotentials/{:?} tag at position {}: {:?}", "Entity", i, e))?;
        let entity_id = entity_tag.get_str("id").map_err(|e| format!("Failed to read SpawnPotentials/{:?} tag at position {}: {:?}", "id", i, e))?;

        let dungeon_kind = entity_id.parse().expect("Unknown entity id");
        dungeons.push((x, y, z, dungeon_kind));
    }

    Ok(dungeons)
}

pub fn get_all_dungeons_in_chunk2(chunk: &CompoundTag) -> Result<Vec<(i32, i32, i32, String)>, String> {
    let mut dungeons = vec![];

    let level_tag = chunk.get_compound_tag("Level").map_err(|e| format!("Failed to read {:?} tag: {:?}", "Level", e))?;
    let tile_entities = level_tag.get_compound_tag_vec("TileEntities").map_err(|e| format!("Failed to read {:?} tag: {:?}", "TileEntities", e))?;

    for (i, tile_entity_tag) in tile_entities.into_iter().enumerate() {
        let id = tile_entity_tag.get_str("id").map_err(|e| format!("Failed to read {:?} tag at position {}: {:?}", "id", i, e))?;
        if id != "minecraft:mob_spawner" {
            continue;
        }

        let x = tile_entity_tag.get_i32("x").map_err(|e| format!("Failed to read {:?} tag at position {}: {:?}", "x", i, e))?;
        let y = tile_entity_tag.get_i32("y").map_err(|e| format!("Failed to read {:?} tag at position {}: {:?}", "y", i, e))?;
        let z = tile_entity_tag.get_i32("z").map_err(|e| format!("Failed to read {:?} tag at position {}: {:?}", "z", i, e))?;

        let spawn_potentials = tile_entity_tag.get_compound_tag_vec("SpawnPotentials").map_err(|e| format!("Failed to read {:?} tag at position {}: {:?}", "SpawnPotentials", i, e))?;
        assert_eq!(spawn_potentials.len(), 1);

        let entity_tag = spawn_potentials[0].get_compound_tag("Entity").map_err(|e| format!("Failed to read SpawnPotentials/{:?} tag at position {}: {:?}", "Entity", i, e))?;
        let entity_id = entity_tag.get_str("id").map_err(|e| format!("Failed to read SpawnPotentials/{:?} tag at position {}: {:?}", "id", i, e))?;

        let dungeon_kind = entity_id.to_string();
        dungeons.push((x, y, z, dungeon_kind));
    }

    Ok(dungeons)
}

fn eight_connected((x, z): (i32, i32)) -> impl Iterator<Item = (i32, i32)> {
    vec![
        (x - 1, z - 1,),
        (x - 1, z,    ),
        (x - 1, z + 1,),
        (x, z - 1,    ),
        (x, z + 1,    ),
        (x + 1, z - 1,),
        (x + 1, z,    ),
        (x + 1, z + 1 ),
    ].into_iter()
}

fn chunk_square_around((chunk_x, chunk_z): (i32, i32), chunk_radius: u32) -> Vec<(i32, i32)> {
    let chunk_radius = i32::try_from(chunk_radius).unwrap_or(i32::MAX);
    let mut v = vec![];

    for x in chunk_x.saturating_sub(chunk_radius)..=chunk_x.saturating_add(chunk_radius) {
        for z in chunk_z.saturating_sub(chunk_radius)..=chunk_z.saturating_add(chunk_radius) {
            v.push((x, z));
        }
    }

    v
}

pub fn region_contains_at_least_one_of_this_chunks((region_x, region_z): (i32, i32), only_check_chunks: Option<&[(i32, i32)]>) -> bool {
    if let Some(only_check_chunks) = only_check_chunks {
        let region_start_x = region_x * 32;
        let region_end_x = (region_x + 1) * 32 - 1;
        let region_start_z = region_z * 32;
        let region_end_z = (region_z + 1) * 32 - 1;

        // Return true if at least one of the chunks in "only_check_chunks" is inside this region
        only_check_chunks.iter().any(|(chunk_x, chunk_z)| {
            region_start_x <= *chunk_x && *chunk_x <= region_end_x &&
            region_start_z <= *chunk_z && *chunk_z <= region_end_z
        })
    } else {
        // If only_check_chunks is None, we must check all the chunks. So return true
        true
    }
}

pub fn find_blocks_in_world<A: AnvilChunkProvider>(chunk_provider: &mut A, block_name: &str, center_position_and_chunk_radius: Option<((i64, i64, i64), u32)>) -> Result<Vec<(i64, i64, i64)>, String> {
    let only_check_chunks = center_position_and_chunk_radius.map(|((x, _y, z), chunk_radius)| {
        let chunk_x = i32::try_from(x >> 4).unwrap();
        let chunk_z = i32::try_from(z >> 4).unwrap();

        chunk_square_around((chunk_x, chunk_z), chunk_radius)
    });
    let mut found_blocks = vec![];
    for (region_x, region_z) in chunk_provider.list_regions().unwrap() {
        if !region_contains_at_least_one_of_this_chunks((region_x, region_z), only_check_chunks.as_deref()) {
            log::debug!("Skipping region {:?}", (region_x, region_z));
            continue;
        }
        log::debug!("Checking region {:?}", (region_x, region_z));
        let region = chunk_provider.get_region(region_x, region_z).unwrap();
        let more_blocks = find_blocks_in_region(region, (region_x, region_z), block_name, only_check_chunks.as_deref())?;
        found_blocks.extend(more_blocks);
    }

    Ok(found_blocks)
}

pub fn find_blocks_in_region<R: Read + Seek>(region: R, (region_x, region_z): (i32, i32), block_name: &str, only_check_chunks: Option<&[(i32, i32)]>) -> Result<Vec<(i64, i64, i64)>, String> {
    let mut dungeons = vec![];
    let mut region = fastanvil::Region::new(region);
    region_for_each_chunk(&mut region, |chunk_x, chunk_z, data| {
        if let Some(only_check_chunks) = only_check_chunks {
            let chunk_x = region_x * 32 + chunk_x as i32;
            let chunk_z = region_z * 32 + chunk_z as i32;
            // Skip chunk if not in "only_check_chunks"
            if !only_check_chunks.contains(&(chunk_x, chunk_z)) {
                return;
            }
        }

        let mut chunk: fastanvil::Chunk = fastnbt::de::from_bytes(data.as_slice()).unwrap();
        //println!("Another chunk");
        //println!("{:?}: {:?}", (chunk_x, chunk_z), chunk);
        for x in 0..16 {
            for y in 0..256 {
                for z in 0..16 {
                    if let Some(block) = chunk.block(x as usize, y as isize, z as usize) {
                        if block.name == block_name {
                            let (x, y, z) = (x as i64, y as i64, z as i64);
                            let (chunk_x, chunk_z) = (chunk_x as i64, chunk_z as i64);
                            let block_x = i64::from(region_x) * 16 * 32 + chunk_x * 16 + x;
                            let block_y = i64::from(y);
                            let block_z = i64::from(region_z) * 16 * 32 + chunk_z * 16 + z;
                            //println!("{:?}", (block_x, block_y, block_z));
                            //println!("block {:?} chunk {:?}", (x, y, z), (chunk_x, chunk_z));
                            //println!("{}", block_name);
                            //log::debug!("{:?}: {}", (block_x, block_y, block_z), block.name);
                            dungeons.push((block_x, block_y, block_z));
                        }
                    } else {
                        // TODO: check max y to avoid iterating from 0 to 255
                        //println!("No block?");
                    }
                }
            }
        }
    }).unwrap();

    Ok(dungeons)
}

fn segregate_into_buckets<V>(list: Vec<((i64, i64, i64), V)>, size: u64) -> HashMap<(i64, i64, i64), Vec<((i64, i64, i64), V)>> {
    let size = size as i64;
    let mut buckets: HashMap<(i64, i64, i64), Vec<_>> = HashMap::new();

    for el in list {
        let ((x, y, z), v) = el;
        let bucket_id = (x.div_euclid(size), y.div_euclid(size), z.div_euclid(size));
        buckets.entry(bucket_id).or_default().push(((x, y, z), v));
    }

    buckets
}

fn load_bucket_and_26_neighbors<'b, V>(buckets: &'b HashMap<(i64, i64, i64), Vec<((i64, i64, i64), V)>>, (x, y, z): &(i64, i64, i64)) -> Vec<&'b ((i64, i64, i64), V)> {
    let mut v = vec![];

    for i in -1 ..= 1 {
        for j in -1 ..= 1 {
            for k in -1 ..= 1 {
                if let Some(bucket) = buckets.get(&(x+i, y+j, z+k)) {
                    for el in bucket {
                        v.push(el);
                    }
                }
            }
        }
    }

    v
}

fn distance3dsquared(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> f64 {
    let x = (a.0 - b.0) as f64;
    let y = (a.1 - b.1) as f64;
    let z = (a.2 - b.2) as f64;

    x*x + y*y + z*z
}

fn remove_all_spawners_that_are_too_far<V>(v: &mut Vec<&((i64, i64, i64), V)>, max_distance: u64) {
    // TODO: this must be one of the most poorly written algorithms in this file
    let mut to_remove = vec![];
    let max_distance_squared = (max_distance as f64) * (max_distance as f64);

    for i in 0..v.len() {
        let mut found_any = false;
        for j in 0..v.len() {
            if i == j {
                continue;
            }
            let a = &v[i].0;
            let b = &v[j].0;
            if distance3dsquared(a, b) < max_distance_squared {
                found_any = true;
                break;
            }
        }

        if !found_any {
            to_remove.push(i);
        }
    }

    for i in to_remove.into_iter().rev() {
        v.remove(i);
    }
}

#[derive(Debug)]
pub struct FloatPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug)]
pub struct FindMultiSpawnersOutput {
    pub optimal_position: FloatPosition,
    pub spawners: Vec<((i64, i64, i64), String)>,
}

#[derive(Debug)]
struct BoundingBox {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64,
}

fn bounding_box<V>(p: &[&((i64, i64, i64), V)]) -> BoundingBox {
    use std::cmp::min;
    use std::cmp::max;

    let mut x_min = p[0].0.0;
    let mut x_max = p[0].0.0;
    let mut y_min = p[0].0.1;
    let mut y_max = p[0].0.1;
    let mut z_min = p[0].0.2;
    let mut z_max = p[0].0.2;

    for ((x, y, z), _) in p {
        x_min = min(x_min, *x);
        x_max = max(x_max, *x);
        y_min = min(y_min, *y);
        y_max = max(y_max, *y);
        z_min = min(z_min, *z);
        z_max = max(z_max, *z);
    }

    BoundingBox {
        x_min, x_max, y_min, y_max, z_min, z_max
    }
}

fn clamp_bb_to_bucket(bb: &mut BoundingBox, bucket: &(i64, i64, i64), bucket_side_length: u64) {
    use std::cmp::min;
    use std::cmp::max;

    let l = bucket_side_length as i64;
    let b_x_min = bucket.0 * l;
    let b_x_max = (bucket.0 + 1) * l;
    let b_y_min = bucket.1 * l;
    let b_y_max = (bucket.1 + 1) * l;
    let b_z_min = bucket.2 * l;
    let b_z_max = (bucket.2 + 1) * l;

    bb.x_min = max(bb.x_min, b_x_min);
    bb.x_max = min(bb.x_max, b_x_max);
    bb.y_min = max(bb.y_min, b_y_min);
    bb.y_max = min(bb.y_max, b_y_max);
    bb.z_min = max(bb.z_min, b_z_min);
    bb.z_max = min(bb.z_max, b_z_max);
}

fn a_is_subset_of_b(a: &[bool], b: &[bool]) -> bool {
    assert_eq!(a.len(), b.len());

    false
}

fn remove_duplicate_keys(v: &mut Vec<(Vec<bool>, (i64, i64, i64))>) {
    // Check if any v[j].0 is a subset of v[i].0
    let mut to_remove = HashSet::new();

    for i in 0..v.len() {
        for j in 0..v.len() {
            if i == j {
                continue;
            }

            if a_is_subset_of_b(&v[j].0, &v[i].0) {
                to_remove.insert(j);
            }
        }

    }

    let mut to_remove: Vec<_> = to_remove.into_iter().collect();
    to_remove.sort();

    for i in to_remove.into_iter().rev() {
        v.remove(i);
    }
}

fn a_is_subset_of_b_again<V>(a: &[((i64, i64, i64), V)], b: &[((i64, i64, i64), V)]) -> bool {
    for (a_pos, _) in a {
        // If all the spawners of a are also present in b
        let mut found = false;
        for (b_pos, _) in b {
            if b_pos == a_pos {
                found = true;
                break;
            }
        }

        if !found {
            // At least one spawner of a is not present in b
            return false;
        }
    }

    true
}

fn a_is_equal_to_b_again<V>(a: &[((i64, i64, i64), V)], b: &[((i64, i64, i64), V)]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    for (a_pos, _) in a {
        // If all the spawners of a are also present in b
        let mut found = false;
        for (b_pos, _) in b {
            if b_pos == a_pos {
                found = true;
                break;
            }
        }

        if !found {
            // At least one spawner of a is not present in b
            return false;
        }
    }

    true
}

fn remove_duplicate_keys_again(v: &mut Vec<FindMultiSpawnersOutput>) {
    use std::cmp::max;
    // Check if any v[j].0 is a subset of v[i].0
    let mut to_remove = HashSet::new();

    for i in 0..v.len() {
        for j in 0..v.len() {
            if i == j {
                continue;
            }

            // Sometimes it happens that a and b are equal, so they are both subsets of each other
            // and they disappear after removing duplicates. So handle that case by removing the
            // one with highest index.
            if a_is_equal_to_b_again(&v[j].spawners, &v[i].spawners) {
                to_remove.insert(max(i, j));
            } else if a_is_subset_of_b_again(&v[j].spawners, &v[i].spawners) {
                // If all the spawners of a are also present in b
                to_remove.insert(j);
            }
        }

    }

    let mut to_remove: Vec<_> = to_remove.into_iter().collect();
    to_remove.sort();

    for i in to_remove.into_iter().rev() {
        v.remove(i);
    }
}

fn find_multispawners_in_bb(bb: &BoundingBox, spawners: &Vec<&((i64, i64, i64), String)>, max_distance: u64) -> Vec<FindMultiSpawnersOutput> {
    let mut multispawners = vec![];

    let mut hm = HashMap::new();
    let max_distance_squared = (max_distance as f64) * (max_distance as f64);

    let distance_to_all = |(x, y, z)| {
        let mut key = Vec::with_capacity(spawners.len());
        let mut score = 0.0;
        for s in spawners.iter() {
            let dist = distance3dsquared(&(x, y, z), &s.0);
            if dist < max_distance_squared {
                key.push(true);
                score += dist;
            } else {
                key.push(false);
            }
        }
        (key, score)
    };

    let all_false = |v: &[bool]| v.iter().all(|x| *x == false);

    for x in bb.x_min..=bb.x_max {
        for y in bb.y_min..=bb.y_max {
            for z in bb.z_min..=bb.z_max {
                let (hm_key, score) = distance_to_all((x, y, z));
                if all_false(&hm_key) {
                    continue;
                } else if let Some((prev_score, _prev_pos)) = hm.get(&hm_key) {
                    if score < *prev_score {
                        // Smaller distance = better match
                        hm.insert(hm_key, (score, (x, y, z)));
                    }
                } else {
                    hm.insert(hm_key, (score, (x, y, z)));
                }
            }
        }
    }

    // Deduplicate matches: given [true, false, true] and [true, true, true] we only want to keep
    // [true, true, true]

    let mut key_list: Vec<_> = hm.into_iter().map(|(hm_key, (_score, pos))| (hm_key, pos)).collect();
    key_list.sort_by_key(|(k, _)| {
        let mut ones = 0;
        for b in k {
            if *b {
                ones += 1;
            }
        }
        ones
    });

    remove_duplicate_keys(&mut key_list);

    for (hm_key, pos) in key_list {
        let mut sp = vec![];

        for (i, b) in hm_key.iter().enumerate() {
            if *b {
                sp.push(spawners[i].clone());
            }
        }

        multispawners.push(FindMultiSpawnersOutput {
            optimal_position: FloatPosition { x: pos.0 as f64, y: pos.1 as f64, z: pos.2 as f64 },
            spawners: sp,
        });

    }


    multispawners
}

pub fn find_spawners_in_world<A: AnvilChunkProvider>(chunk_provider: &mut A, _center_position_and_chunk_radius: Option<((i64, i64, i64), u32)>) -> Result<Vec<FindMultiSpawnersOutput>, String> {
    let all_dungeons = find_spawners(chunk_provider)?;

    // Segregate dungeons into buckets such that two dungeons that can be active at once are always
    // in adjacent buckets
    let spawner_activation_radius = 16;
    let bucket_side_length = spawner_activation_radius * 2 + 4;

    let buckets = segregate_into_buckets(all_dungeons, bucket_side_length);
    let mut multispawners = vec![];

    // For each bucket: load all the dungeons from this bucket and the 26 adjacent ones
    // And try to find groups of dungeons that are close to each other
    for bucket in buckets.keys() {
        let mut spawners = load_bucket_and_26_neighbors(&buckets, bucket);
        let orig_len = spawners.len();
        remove_all_spawners_that_are_too_far(&mut spawners, spawner_activation_radius * 2);
        log::debug!("Found {} spawners in bucket {:?} ({} before removing unconnected ones)", spawners.len(), bucket, orig_len);

        match spawners.len() {
            // No double dungeons here
            0 | 1 => continue,
            2 => {
                // Simple case, just calculate the midpoint
                let a = &spawners[0];
                let b = &spawners[1];
                let midpoint = FloatPosition { x: (a.0.0 as f64 + b.0.0 as f64) / 2.0, y: (a.0.1 as f64 + b.0.1 as f64) / 2.0, z: (a.0.2 as f64 + b.0.2 as f64) / 2.0 };
                multispawners.push(FindMultiSpawnersOutput {
                    optimal_position: midpoint,
                    spawners: spawners.into_iter().cloned().collect(),
                });
                continue;
            }
            _n => {}
        }


        // General case: more than 2 dungeons
        // Check every possible block position inside this bucket, and see if it is within the
        // desired radius of any spawners
        // Calculate bounding box of all the spawners, and iterate all the possible positions
        // inside that bounding box and inside the current bucket
        let mut bb = bounding_box(spawners.as_slice());
        clamp_bb_to_bucket(&mut bb, bucket, bucket_side_length);

        let more_multispawners = find_multispawners_in_bb(&bb, &spawners, spawner_activation_radius);
        multispawners.extend(more_multispawners);
    }

    remove_duplicate_keys_again(&mut multispawners);

    multispawners.sort_by_key(|k| {
        // Number of spawners (higher first), then x coordinate, then z coordinate, then y coordinate
        // TODO: use OrderedFloat to sort floats
        (!k.spawners.len(), k.optimal_position.x as u64, k.optimal_position.z as u64, k.optimal_position.y as u64)
    });

    Ok(multispawners)
}

