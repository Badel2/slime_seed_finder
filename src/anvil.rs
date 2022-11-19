//! This module contains functions related to reading saved words
//!
//! Anvil is the name of the format used by Minecraft to store world files.
//! It contains all the block data, entities, and biome info of each chunk.

pub use fastanvil::Chunk;
use crate::fastanvil_ext::CompoundTag;
use crate::fastanvil_ext::CompoundTagError;
use crate::fastanvil_ext::read_gzip_compound_tag;
use zip::ZipArchive;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::Path;
use std::fs::OpenOptions;
use log::*;
use crate::biome_layers::Area;
use crate::biome_layers::is_oceanic;
use crate::biome_info::biome_id;
use crate::biome_info::UNKNOWN_BIOME_ID;
use crate::chunk::Point;
use crate::chunk::Point4;
use crate::chunk::Point3D4;
use crate::patterns::CompiledBlockPattern;
use crate::seed_info::BiomeId;
use crate::seed_info::MinecraftVersion;
use crate::fastanvil_ext::Dimension;
use crate::fastanvil_ext::region_for_each_chunk;
use crate::fastanvil_ext::FolderChunkProvider;
pub use crate::fastanvil_ext::ZipChunkProvider;
use crate::fastanvil_ext::AnvilChunkProvider;
use crate::fastanvil_ext::ChunkLoadError;
use crate::zip_ext::find_file_in_zip_exactly_once;
pub use crate::multi_spawners::find_multi_spawners;
pub use crate::multi_spawners::FindMultiSpawnersOutput;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::str::FromStr;
use std::convert::TryInto;
use std::ops::RangeInclusive;
use serde::Deserialize;

/// Read all the existing chunks in a `area_size*area_size` block area around
/// `(block_x, block_z)`.
pub fn read_area_around<A: AnvilChunkProvider>(chunk_provider: &mut A, area_size: u64, Point { x: block_x, z: block_z }: Point) -> Result<Vec<Vec<u8>>, ChunkLoadError> {
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
    let mut chunk_provider = FolderChunkProvider::new(input_dir.to_owned());

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
            let c = CompoundTag::from_bytes(&c).unwrap();
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
            let c = CompoundTag::from_bytes(&c).unwrap();
            let biomes_array = get_biomes_from_chunk_1_15(&c).unwrap();
            let level_compound_tag = c.get_compound_tag("Level").unwrap();

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

/// Get all the biomes present in the chunk. For version >= 1.15 but < 1.18
pub fn get_biomes_from_chunk_1_15(chunk: &CompoundTag) -> Result<Vec<i32>, String> {
    let level_compound_tag = chunk.get_compound_tag("Level").unwrap();
    let biomes_array = match level_compound_tag.get_i32_vec("Biomes") {
        Ok(x) => x.to_vec(),
        Err(CompoundTagError::TagNotFound) => {
            // Starting from 1.16, it is possible that the "Biomes" tag is missing from
            // chunks that are not fully generated yet. We simply ignore these chunks as if
            // they did not exist, by returning an empty list of biomes
            return Ok(vec![]);
        }
        Err(e) => panic!("Unknown format for biomes array: {:?}", e),
    };
    match biomes_array.len() {
        0 => {}
        1024 => {}
        // TODO: this is used by experimental 1.18 snapshots
        1536 => {}
        n => panic!("Unexpected biomes_array len: {}", n),
    }

    Ok(biomes_array)
}

/// Get all the biomes present in the chunk. For version <= 1.14
pub fn get_biomes_from_chunk_1_14(chunk: &CompoundTag) -> Result<Vec<i32>, String> {
    let level_compound_tag = chunk.get_compound_tag("Level").unwrap();
    let biomes_array = match level_compound_tag.get_i32_vec("Biomes") {
        Ok(x) => x.to_vec(),
        Err(_e) => {
            match level_compound_tag.get_i8_vec("Biomes") {
                Ok(x) => {
                    // Important: before 1.13 biomes was a byte array,
                    // i8 is wrong, u8 is correct
                    x.iter().map(|byte| i32::from(*byte as u8)).collect()
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

/// Get all the biomes present in the save. For version >= 1.15 but < 1.18
pub fn get_all_biomes_1_15<A: AnvilChunkProvider>(chunk_provider: &mut A) -> Vec<(BiomeId, Point4)> {
    let mut biome_data = HashMap::new();
    let all_chunks = chunk_provider.list_chunks().expect("Error listing chunks");
    for (chunk_x, chunk_z) in all_chunks {
        let c = chunk_provider.load_chunk(chunk_x, chunk_z).expect("Error loading chunk");
        let c = CompoundTag::from_bytes(&c).unwrap();

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

/// Get all the biomes present in the save. For version >= 1.18
pub fn get_all_biomes_1_18<A: AnvilChunkProvider>(chunk_provider: &mut A) -> Vec<(BiomeId, Point3D4)> {
    let mut biome_data = HashMap::new();
    let all_regions = chunk_provider.list_regions().expect("Error listing regions");
    for (region_x, region_z) in all_regions {
        let r = chunk_provider.get_region(region_x, region_z).expect("Error loading region");
        let mut rb = fastanvil::Region::from_stream(r).expect("Failed to initialize region");

        region_for_each_chunk(&mut rb, |rel_chunk_x, rel_chunk_z, data| {
            let chunk = fastanvil::JavaChunk::from_bytes(data.as_slice()).unwrap();
            //log::debug!("chunk {:?}: {:?}", (region_x, region_z, rel_chunk_x, rel_chunk_z), chunk);
            // TODO: biomes are stored in 1:4 scale, so we don't need to iterate over all y values,
            // we could iterate in steps of 4. Test this.
            let y_range = chunk.y_range();
            for y in y_range.step_by(4) {
                for ix in 0..4 {
                    for iz in 0..4 {
                        let x = ix * 4;
                        let z = iz * 4;
                        // TODO: some chunk sections have 1 biome only, we could skip some
                        // calculations in that case.
                        // TODO: there is a bug, chunks at the border that have not fully generated
                        // yet seem to have biome: plains. Maybe we could detect that case and set
                        // the biome to unknown or "not generated". But note that some chunks may
                        // have only 1 biome plains because they are actually plains, so the check
                        // cannot be "if all biomes == plains".
                        let b = chunk.biome(x, y, z).unwrap_or_else(|| {
                            panic!("biome not present, what to do? coords: {:?}", (region_x, region_z, rel_chunk_x, rel_chunk_z, x, y, z));
                        });
                        let block_x: i64 = (region_x as i64 * 512) + (rel_chunk_x as i64 * 16) + x as i64;
                        let block_z: i64 = (region_z as i64 * 512) + (rel_chunk_z as i64 * 16) + z as i64;
                        let block_y: i64 = y.try_into().unwrap();
                        // Divide by 4 to get 1:4 scale
                        let block_x = block_x >> 2;
                        let block_y = block_y >> 2;
                        let block_z = block_z >> 2;
                        let biome_id = match b {
                            fastanvil::biome::Biome::Unknown => BiomeId(UNKNOWN_BIOME_ID),
                            b => BiomeId(i32::from(b)),
                        };
                        biome_data.insert(Point3D4 { x: block_x, y: block_y, z: block_z }, biome_id);
                    }
                }
            }
        }).expect("for_each_chunk error");
    }

    debug!("biome_data.len(): {}", biome_data.len());

    let mut extra_biomes = vec![];
    extra_biomes.extend(biome_data.iter().map(|(p, b)| (*b, *p)));
    //debug!("extra_biomes: {:?}", extra_biomes);

    return extra_biomes;
}

fn area4_contains_chunk(area: Area, chunk_x: i32, chunk_z: i32) -> bool {
    // Create area from chunk, in 1:4 scale
    let chunk_area = Area { x: chunk_x as i64 * 4, z: chunk_z as i64 * 4, w: 4, h: 4 };

    area.intersects(&chunk_area)
}

fn area4_contains_region(area: Area, region_x: i32, region_z: i32) -> bool {
    // Create area from region, in 1:4 scale
    // 1 region = 32x32 chunks, so multiply code of area4_contains_chunk by 32
    let region_area = Area { x: region_x as i64 * 4 * 32, z: region_z as i64 * 4 * 32, w: 4 * 32, h: 4 * 32 };

    area.intersects(&region_area)
}

/// Get the biomes present in the area, reading from the world save. For version >= 1.15 but < 1.18
pub fn get_biomes_from_area_1_15<A: AnvilChunkProvider>(chunk_provider: &mut A, area: Area, y_offset: u32) -> Vec<(BiomeId, Point4)> {
    let mut biome_data = HashMap::new();
    let all_chunks = chunk_provider.list_chunks().expect("Error listing chunks");
    for (chunk_x, chunk_z) in all_chunks {
        // TODO: area uses coordinates in 1:4 scale
        // chunks are 1:16 scale
        // How to ensure that this chunk is not inside the area?
        if !area4_contains_chunk(area, chunk_x, chunk_z) {
            continue;
        }
        let c = chunk_provider.load_chunk(chunk_x, chunk_z).expect("Error loading chunk");
        let c = CompoundTag::from_bytes(&c).unwrap();

        let biomes_array = get_biomes_from_chunk_1_15(&c).unwrap();

        // Since 1.15, the biomes array is 3D, so we need to select the "y offset".
        // In 1.15 - 1.17 the y offset can be 0 - 64, and since 1.18 it is 0 - 96.
        // We return an empty list of biomes if the y offset is out of bounds.
        let y_skip = usize::try_from(y_offset).unwrap_or(usize::MAX).saturating_mul(4 * 4);
        for (i_b, b) in biomes_array.into_iter().enumerate().skip(y_skip).take(4 * 4) {
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

/// Get the biomes present in the area, reading from the world save. For version >= 1.18
pub fn get_biomes_from_area_1_18<A: AnvilChunkProvider>(chunk_provider: &mut A, area: Area, y_level: i64) -> Vec<(BiomeId, Point4)> {
    let mut biome_data = HashMap::new();
    let all_regions = chunk_provider.list_regions().expect("Error listing regions");
    for (region_x, region_z) in all_regions {
        // TODO: area uses coordinates in 1:4 scale
        // chunks are 1:16 scale
        if !area4_contains_region(area, region_x, region_z) {
            continue;
        }
        let r = chunk_provider.get_region(region_x, region_z).expect("Error loading region");
        let mut rb = fastanvil::Region::from_stream(r).expect("Failed to initialize region");

        region_for_each_chunk(&mut rb, |rel_chunk_x, rel_chunk_z, data| {
            // TODO: area uses coordinates in 1:4 scale
            // chunks are 1:16 scale
            // How to ensure that this chunk is not inside the area?
            // 1 region = 32x32 chunks
            let chunk_x = 32 * region_x + rel_chunk_x as i32;
            let chunk_z = 32 * region_z + rel_chunk_z as i32;
            if !area4_contains_chunk(area, chunk_x, chunk_z) {
                return;
            }

            let chunk = fastanvil::JavaChunk::from_bytes(data.as_slice()).unwrap();
            //log::debug!("chunk {:?}: {:?}", (region_x, region_z, rel_chunk_x, rel_chunk_z), chunk);
            // TODO: biomes are stored in 1:4 scale, so we don't need to iterate over all y values,
            // we could iterate in steps of 4. Test this.
            let y_range = chunk.y_range();
            let y = y_level as isize;
            if !y_range.contains(&y) {
                return;
            }
            for ix in 0..4 {
                for iz in 0..4 {
                    let x = ix * 4;
                    let z = iz * 4;
                    // TODO: some chunk sections have 1 biome only, we could skip some
                    // calculations in that case.
                    // TODO: there is a bug, chunks at the border that have not fully generated
                    // yet seem to have biome: plains. Maybe we could detect that case and set
                    // the biome to unknown or "not generated". But note that some chunks may
                    // have only 1 biome plains because they are actually plains, so the check
                    // cannot be "if all biomes == plains".
                    let b = chunk.biome(x, y, z).unwrap_or_else(|| {
                        panic!("biome not present, what to do? coords: {:?}", (region_x, region_z, rel_chunk_x, rel_chunk_z, x, y, z));
                    });
                    let block_x: i64 = (region_x as i64 * 512) + (rel_chunk_x as i64 * 16) + x as i64;
                    let block_z: i64 = (region_z as i64 * 512) + (rel_chunk_z as i64 * 16) + z as i64;
                    // Divide by 4 to get 1:4 scale
                    let block_x = block_x >> 2;
                    let block_z = block_z >> 2;
                    let biome_id = match b {
                        fastanvil::biome::Biome::Unknown => BiomeId(UNKNOWN_BIOME_ID),
                        b => BiomeId(i32::from(b)),
                    };
                    biome_data.insert(Point4 { x: block_x, z: block_z }, biome_id);
                }
            }
        }).expect("for_each_chunk error");
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
        let c = CompoundTag::from_bytes(&c).unwrap();

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
        Some(MinecraftVersion::Java1_16_1) | Some(MinecraftVersion::Java1_16) | Some(MinecraftVersion::Java1_17) => read_seed_from_level_dat_1_16(&mut level_dat),
        Some(version) if version <= MinecraftVersion::Java1_15 => read_seed_from_level_dat_1_15(&mut level_dat),
        _ => {
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
    let root_tag = read_gzip_compound_tag(r).map_err(|e| format!("Failed to read gzip compount tag: {:?}", e))?;
    let data_tag = root_tag.get_compound_tag("Data").map_err(|e| format!("Failed to read {:?} tag: {:?}", "Data", e))?;
    let seed = data_tag.get_i64("RandomSeed").map_err(|e| format!("Failed to read {:?} tag: {:?}", "RandomSeed", e))?;

    Ok(seed)
}

/// Read seed from level.dat in version 1.16 and above.
///
/// root["Data"]["WorldGenSettings"]["seed"]
pub fn read_seed_from_level_dat_1_16<R: Read>(r: &mut R) -> Result<i64, String> {
    let root_tag = read_gzip_compound_tag(r).map_err(|e| format!("Failed to read gzip compount tag: {:?}", e))?;
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
        let c_bytes = chunk_provider.load_chunk(chunk_x, chunk_z).expect("Error loading chunk");
        let c = CompoundTag::from_bytes(&c_bytes).unwrap();
        // Store all the errors
        let mut errs = vec![];
        let spawners = Result::<_, String>::Err(Default::default()).or_else(|_| {
            get_all_dungeons_in_chunk_118(&c)
        }).or_else(|e| {
            errs.push(("1.18", e));
            get_all_dungeons_in_chunk(&c)
        }).map_err(|e| {
            errs.push(("1.17", e));
        }).map_err(|_| {
            // Convert the list of errors into a String because sadly that's our error type
            let mut s = String::new();
            s.push_str(&"Failed to read dungeons from chunk: unsupported version or corrupted file. Detailed list of errors:\n");
            for (version, err) in errs {
                s.push_str(&format!("* {}: {}\n", version, err));
            }

            s
        });

        let spawners = spawners?;
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

        // Load this chunk and the 8 surrounding ones into dimension
        overworld.add_chunk(chunk_x, chunk_z, &mut Cursor::new(&c_bytes)).expect("Failed to add chunk");

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
            let c_bytes = c.unwrap();
            overworld.add_chunk(chunk_x, chunk_z, &mut Cursor::new(&c_bytes)).expect("Failed to add chunk neighbor");
        }

        for (x, y, z, kind) in more_dungeons {
            let x = i64::from(x);
            let y = i64::from(y);
            let z = i64::from(z);
            // Sanity check
            assert_eq!(overworld.get_block(x, y, z).unwrap().name(), "minecraft:spawner");

            let mut floor = vec![];
            // Read 11x11 area just below the spawner
            let dy = -1;
            for dz in (-5)..=5 {
                for dx in (-5)..=5 {
                    let block = overworld.get_block(x + dx, y + dy, z + dz);
                    let block_name = block.map(|b| b.name()).unwrap_or(
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
        let c = CompoundTag::from_bytes(&c).unwrap();

        // Store all the errors
        let mut errs = vec![];
        let spawners = Result::<_, String>::Err(Default::default()).or_else(|_| {
            get_all_dungeons_in_chunk2_118(&c)
        }).or_else(|e| {
            errs.push(("1.18", e));
            get_all_dungeons_in_chunk2(&c)
        }).map_err(|e| {
            errs.push(("1.17", e));
        }).map_err(|_| {
            // Convert the list of errors into a String because sadly that's our error type
            let mut s = String::new();
            s.push_str(&"Failed to read spawners from chunk: unsupported version or corrupted file. Detailed list of errors:\n");
            for (version, err) in errs {
                s.push_str(&format!("* {}: {}\n", version, err));
            }

            s
        });

        let spawners = spawners?;

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
    get_all_dungeons_in_chunk2(chunk).map(|v| {
        v.into_iter().map(|(x, y, z, entity_id)| {
            (x, y, z, entity_id.parse().expect("Unknown entity id"))
        }).collect()
    })
}

pub fn get_all_dungeons_in_chunk_118(chunk: &CompoundTag) -> Result<Vec<(i32, i32, i32, SpawnerKind)>, String> {
    get_all_dungeons_in_chunk2_118(chunk).map(|v| {
        v.into_iter().map(|(x, y, z, entity_id)| {
            (x, y, z, entity_id.parse().expect("Unknown entity id"))
        }).collect()
    })
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

pub fn get_all_dungeons_in_chunk2_118(chunk: &CompoundTag) -> Result<Vec<(i32, i32, i32, String)>, String> {
    let mut dungeons = vec![];

    let tile_entities = chunk.get_compound_tag_vec("block_entities").map_err(|e| format!("Failed to read {:?} tag: {:?}", "block_entities", e))?;

    for (i, tile_entity_tag) in tile_entities.into_iter().enumerate() {
        let id = tile_entity_tag.get_str("id").map_err(|e| format!("Failed to read {:?} tag at position {}: {:?}", "id", i, e))?;
        if id != "minecraft:mob_spawner" {
            continue;
        }

        let x = tile_entity_tag.get_i32("x").map_err(|e| format!("Failed to read {:?} tag at position {}: {:?}", "x", i, e))?;
        let y = tile_entity_tag.get_i32("y").map_err(|e| format!("Failed to read {:?} tag at position {}: {:?}", "y", i, e))?;
        let z = tile_entity_tag.get_i32("z").map_err(|e| format!("Failed to read {:?} tag at position {}: {:?}", "z", i, e))?;

        let spawn_data = tile_entity_tag.get_compound_tag("SpawnData").map_err(|e| format!("Failed to read {:?} tag at position {}: {:?}", "SpawnData", i, e))?;
        let entity_tag = spawn_data.get_compound_tag("entity").map_err(|e| format!("Failed to read SpawnData/{:?} tag at position {}: {:?}", "entity", i, e))?;
        let entity_id = entity_tag.get_str("id").map_err(|e| format!("Failed to read SpawnData/entity/{:?} tag at position {}: {:?}", "id", i, e))?;

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

pub fn iterate_blocks_in_world<A: AnvilChunkProvider, F: FnMut((i64, i64, i64), &fastanvil::Block)>(chunk_provider: &mut A, center_position_and_chunk_radius: Option<((i64, i64, i64), u32)>, mut f: F) -> Result<(), String> {
    let only_check_chunks = center_position_and_chunk_radius.map(|((x, _y, z), chunk_radius)| {
        let chunk_x = i32::try_from(x >> 4).unwrap();
        let chunk_z = i32::try_from(z >> 4).unwrap();

        chunk_square_around((chunk_x, chunk_z), chunk_radius)
    });
    for (region_x, region_z) in chunk_provider.list_regions().unwrap() {
        if !region_contains_at_least_one_of_this_chunks((region_x, region_z), only_check_chunks.as_deref()) {
            log::debug!("Skipping region {:?}", (region_x, region_z));
            continue;
        }
        log::debug!("Checking region {:?}", (region_x, region_z));
        let region = chunk_provider.get_region(region_x, region_z).unwrap();
        iterate_blocks_in_region(region, (region_x, region_z), only_check_chunks.as_deref(), &mut f)?;
    }

    Ok(())
}

pub fn find_blocks_in_world<A: AnvilChunkProvider>(chunk_provider: &mut A, block_name: &str, center_position_and_chunk_radius: Option<((i64, i64, i64), u32)>) -> Result<Vec<(i64, i64, i64)>, String> {
    let mut found_blocks = vec![];

    iterate_blocks_in_world(chunk_provider, center_position_and_chunk_radius, |(x, y, z), block| {
        if block.name() == block_name {
            found_blocks.push((x, y, z));
        }
    })?;

    Ok(found_blocks)
}

pub fn find_block_pattern_in_world<A: AnvilChunkProvider>(chunk_provider: &mut A, block_pattern: &CompiledBlockPattern, center_position_and_chunk_radius: Option<((i64, i64, i64), u32)>, y_range: Option<RangeInclusive<i32>>) -> Result<Vec<(i64, i64, i64)>, String> {
    iterate_find_block_pattern(chunk_provider, block_pattern, center_position_and_chunk_radius, y_range).map(|res| res.1)
}


pub fn iterate_chunks_in_world<A: AnvilChunkProvider, F: FnMut((i32, i32), &fastanvil::JavaChunk)>(chunk_provider: &mut A, center_position_and_chunk_radius: Option<((i64, i64, i64), u32)>, mut f: F) -> Result<(), String> {
    let only_check_chunks = center_position_and_chunk_radius.map(|((x, _y, z), chunk_radius)| {
        let chunk_x = i32::try_from(x >> 4).unwrap();
        let chunk_z = i32::try_from(z >> 4).unwrap();

        chunk_square_around((chunk_x, chunk_z), chunk_radius)
    });
    for (region_x, region_z) in chunk_provider.list_regions().unwrap() {
        if !region_contains_at_least_one_of_this_chunks((region_x, region_z), only_check_chunks.as_deref()) {
            log::debug!("Skipping region {:?}", (region_x, region_z));
            continue;
        }
        log::debug!("Checking region {:?}", (region_x, region_z));
        let region = chunk_provider.get_region(region_x, region_z).unwrap();
        iterate_chunks_in_region(region, (region_x, region_z), only_check_chunks.as_deref(), &mut f)?;
    }

    Ok(())
}


pub fn iterate_chunks_in_region<R: Read + Seek, F: FnMut((i32, i32), &fastanvil::JavaChunk)>(region: R, (region_x, region_z): (i32, i32), only_check_chunks: Option<&[(i32, i32)]>, mut f: F) -> Result<(), String> {
    let mut rb = fastanvil::Region::from_stream(region).expect("Failed to initialize region");
    region_for_each_chunk(&mut rb, |chunk_x, chunk_z, data| {
        let chunk_x = region_x * 32 + chunk_x as i32;
        let chunk_z = region_z * 32 + chunk_z as i32;
        if let Some(only_check_chunks) = only_check_chunks {
            // Skip chunk if not in "only_check_chunks"
            if !only_check_chunks.contains(&(chunk_x, chunk_z)) {
                return;
            }
        }

        let chunk = match fastanvil::JavaChunk::from_bytes(data.as_slice()) {
            Ok(x) => x,
            Err(e) => {
                log::warn!("Error when deserializing chunk {:?}: {:?}", (chunk_x, chunk_z), e);
                return;
            }
        };

        f((chunk_x, chunk_z), &chunk);
    }).unwrap();

    Ok(())
}

pub fn iterate_blocks_in_region<R: Read + Seek, F: FnMut((i64, i64, i64), &fastanvil::Block)>(region: R, (region_x, region_z): (i32, i32), only_check_chunks: Option<&[(i32, i32)]>, mut f: F) -> Result<(), String> {
    iterate_chunks_in_region(region, (region_x, region_z), only_check_chunks, |(chunk_x, chunk_z), chunk| {
        let y_range = chunk.y_range();

        for y in y_range {
            for x in 0..16 {
                for z in 0..16 {
                    let (x, y, z) = (x as i64, y as i64, z as i64);
                    let block_x = chunk_x as i64 * 16 + x;
                    let block_y = i64::from(y);
                    let block_z = chunk_z as i64 * 16 + z;
                    if let Some(block) = chunk.block(x as usize, y as isize, z as usize) {
                        f((block_x, block_y, block_z), block);
                    } else {
                        // chunk.block() should never return None because we checked the y_range
                        log::warn!("iterate_blocks_in_region: Failed to get block at {:?}", (block_x, block_y, block_z));
                    }
                }
            }
        }
    })
}

/// Returns the list of multi-spawners in the given dimension, sorted by number of spawners that
/// can be activated at the same time.
pub fn find_spawners_in_world<A: AnvilChunkProvider>(chunk_provider: &mut A, _center_position_and_chunk_radius: Option<((i64, i64, i64), u32)>) -> Result<Vec<FindMultiSpawnersOutput>, String> {
    let all_dungeons = find_spawners(chunk_provider)?;
    let multi_spawners = find_multi_spawners(all_dungeons);

    Ok(multi_spawners)
}

#[derive(Debug)]
pub struct Box3D {
    pub x_min: i64,
    pub x_max: i64,
    pub y_min: i64,
    pub y_max: i64,
    pub z_min: i64,
    pub z_max: i64,
}

impl Box3D {
    /// Returns an infinitely sized 3d area
    pub fn max_size() -> Self {
        Self {
            x_min: i64::MIN,
            x_max: i64::MAX,
            y_min: i64::MIN,
            y_max: i64::MAX,
            z_min: i64::MIN,
            z_max: i64::MAX,
        }
    }

    /// Returns true if any block inside the Box3D belongs to the chunk
    pub fn contains_chunk(&self, chunk_x: i32, chunk_z: i32) -> bool {
        // This can be done as a 2D rectangle intersection using (chunk_min_x, chunk_min_z) and (chunk_max_z, chunk_max_z) as the edges
        // TODO: implement a generic rectangle-point intersection function
        let chunk1 = crate::chunk::Chunk::from_point(Point { x: self.x_min, z: self.z_min });
        let chunk2 = crate::chunk::Chunk::from_point(Point { x: self.x_max, z: self.z_max });

        // chunk1.x <= chunk_x <= chunk2.x && chunk1.z <= chunk_z <= chunk2.z
        chunk1.x <= chunk_x && chunk_x <= chunk2.x && chunk1.z <= chunk_z && chunk_z <= chunk2.z
    }
}

#[derive(Debug)]
pub enum SearchBounds {
    Everywhere,
    CenterAndRadius { center: (i64, i64, i64), radius: u32 },
    BoundingBox(Box3D),
    Intersection(Vec<SearchBounds>),
    Union(Vec<SearchBounds>),
    Not(Box<SearchBounds>),
    // TODO: add modulo conditions such as x%16 == 9 for treasure chests
}

impl SearchBounds {
    pub fn contains_chunk(&self, chunk_x: i32, chunk_z: i32) -> bool {
        match self {
            SearchBounds::Everywhere => true,
            SearchBounds::CenterAndRadius { .. } => todo!(),
            SearchBounds::BoundingBox(bb) => bb.contains_chunk(chunk_x, chunk_z),
            SearchBounds::Intersection(v) => {
                v.iter().all(|bound| bound.contains_chunk(chunk_x, chunk_z))
            }
            SearchBounds::Union(v) => {
                v.iter().any(|bound| bound.contains_chunk(chunk_x, chunk_z))
            }
            SearchBounds::Not(bound) => !bound.contains_chunk(chunk_x, chunk_z),
        }
    }
}

pub enum SearchMode {
    /// Return the coordinates of all the matches
    FindAll,
    /// Return the first n matches only
    FindSome(u32),
    /// Count the number of matches
    CountAll,
    /// Count the number of matches, up to the first n
    CountSome(u32),
}

impl SearchMode {
    fn counter(&self) -> SearchModeCounter {
        match self {
            SearchMode::FindSome(n) => {
                SearchModeCounter {
                    limit: *n,
                    num_matches: 0,
                    matches: Some(vec![]),
                }
            }
            SearchMode::FindAll => {
                SearchModeCounter {
                    limit: u32::MAX,
                    num_matches: 0,
                    matches: Some(vec![]),
                }
            }
            SearchMode::CountSome(n) => {
                SearchModeCounter {
                    limit: *n,
                    num_matches: 0,
                    matches: None,
                }
            }
            SearchMode::CountAll => {
                SearchModeCounter {
                    limit: u32::MAX,
                    num_matches: 0,
                    matches: None,
                }
            }
        }
    }
}

struct SearchModeCounter {
    limit: u32,
    num_matches: u32,
    matches: Option<Vec<(i64, i64, i64)>>,
}

impl SearchModeCounter {
    fn push(&mut self, pos: (i64, i64, i64)) -> Result<(), ()> {
        self.num_matches += 1;
        if let Some(m) = &mut self.matches {
            m.push(pos);
        }

        if self.num_matches >= self.limit {
            Err(())
        } else {
            Ok(())
        }
    }
}

pub trait WorldSearchInterface {
    fn list_chunks(&mut self) -> Vec<(i32, i32)>;
    fn chunk_y_range(&mut self, chunk_x: i32, chunk_z: i32) -> std::ops::Range<isize>;
    fn get_block_name(&mut self, x: i64, y: i64, z: i64) -> Option<&str>;
    // None: the block is missing
    // Some(None): the block does not have that property
    fn get_block_property(&mut self, x: i64, y: i64, z: i64, key: &str) -> Option<Option<&str>>;
}

struct FirstWorldSearchInterface<'a, 'b, A> {
    chunk_provider: &'a mut A,
    dimension: &'b mut Dimension<std::fs::File>,
}

impl<'a, 'b, A: AnvilChunkProvider> WorldSearchInterface for FirstWorldSearchInterface<'a, 'b, A> {
    fn list_chunks(&mut self) -> Vec<(i32, i32)> {
        self.chunk_provider.list_chunks().unwrap()
    }
    fn chunk_y_range(&mut self, chunk_x: i32, chunk_z: i32) -> std::ops::Range<isize> {
        // TODO: read y range from chunk list
        -64..256
    }
    fn get_block_name(&mut self, x: i64, y: i64, z: i64) -> Option<&str> {
        self.dimension.get_block(x, y, z).map(|block| block.name())
    }
    fn get_block_property(&mut self, x: i64, y: i64, z: i64, key: &str) -> Option<Option<&str>> {
        fn get_property_from_encoded_description<'a, 'b>(key: &'a str, x: &'b str) -> Option<&'b str> {
            // Block::encoded_description returns:
            // A string of the format “id|prop1=val1,prop2=val2”. The properties are ordered lexigraphically. This somewhat matches the way Minecraft stores variants in blockstates, but with the block ID/name prepended.
            // Get string after first |
            let x_kv = x.splitn(2, '|').skip(1).next()?;

            for kv in x_kv.split_terminator(',') {
                // TODO: maybe rewrite this as kv.strip_prefix(key + '=')
                let mut ikv = kv.splitn(2, '=');
                let k = ikv.next().unwrap();
                if k != key {
                    continue;
                }
                let v = ikv.next().unwrap();
                return Some(v);
            }

            None
        }
        self.dimension.get_block(x, y, z).map(|block| get_property_from_encoded_description(key, block.encoded_description()))
    }
}

pub fn search_pattern_in_world<W>(block_pattern: &CompiledBlockPattern, bounds: &SearchBounds, mode: &SearchMode, world: &mut W) -> Result<(u32, Vec<(i64, i64, i64)>), String>
where W: WorldSearchInterface
{
    let mut counter = mode.counter();
    let mut processed_chunks_count = 0;
    let ys = block_pattern.max_y_size();
    let all_chunks = world.list_chunks();
    let total_chunks = all_chunks.len();

    for (chunk_x, chunk_z) in all_chunks {
        if processed_chunks_count % 1024 == 0 {
            log::debug!("{}/{} chunks processed, {} matches found", processed_chunks_count, total_chunks, counter.num_matches);
        }
        processed_chunks_count += 1;
        if !bounds.contains_chunk(chunk_x, chunk_z) {
            continue;
        }

        // TODO: find the optimal way to iterate over the chunk depending on the WorldBounds.

        let mut y_range = world.chunk_y_range(chunk_x, chunk_z);
        y_range.start -= ys as isize;
        y_range.end += ys as isize;
        'next_world_block_y: for y in y_range.clone() {
            for cz in 0..16 {
                'next_world_block: for cx in 0..16 {
                    let x = i64::from(chunk_x * 16 + cx);
                    let y = i64::try_from(y).unwrap();
                    let z = i64::from(chunk_z * 16 + cz);

                    let good = block_pattern.check_position(x, y, z, y_range.clone(), world);

                    if good {
                        let reached_limit = counter.push((x, y, z)).is_err();
                        if reached_limit {
                            return Ok((counter.num_matches, counter.matches.unwrap_or_default()));
                        }
                    }
                }
            }
        }
    }

    Ok((counter.num_matches, counter.matches.unwrap_or_default()))
}

pub fn iterate_find_block_pattern<A: AnvilChunkProvider>(chunk_provider: &mut A, block_pattern: &CompiledBlockPattern, _center_position_and_chunk_radius: Option<((i64, i64, i64), u32)>, limit_y_range: Option<RangeInclusive<i32>>) -> Result<(u32, Vec<(i64, i64, i64)>), String> {
    let all_chunks = chunk_provider.list_chunks().expect("Error listing chunks");
    let mut overworld: Dimension<std::fs::File> = Dimension::new();

    // Load all chunks into memory
    for (chunk_x, chunk_z) in all_chunks.iter().copied() {
        let c_bytes = chunk_provider.load_chunk(chunk_x, chunk_z).expect("Error loading chunk");
        let c = CompoundTag::from_bytes(&c_bytes).unwrap();
        overworld.add_chunk(chunk_x, chunk_z, &mut Cursor::new(&c_bytes)).expect("Failed to add chunk");
    }

    let mut search_bounds = SearchBounds::Everywhere;

    if let Some(limit_y_range) = limit_y_range {
        search_bounds = SearchBounds::BoundingBox(Box3D { y_min: i64::from(*limit_y_range.start()), y_max: i64::from(*limit_y_range.end()), ..Box3D::max_size() });
    }
    let search_mode = SearchMode::FindAll;

    let mut world_interface = FirstWorldSearchInterface { chunk_provider, dimension: &mut overworld };

    let (num_matches, matches) = search_pattern_in_world(block_pattern, &search_bounds, &search_mode, &mut world_interface)?;

    log::debug!("All chunks processed, {} matches found", num_matches);

    Ok((num_matches, matches))
}

pub fn iterate_find_block_pattern_callback<A: AnvilChunkProvider, R, F: FnOnce(Dimension<std::fs::File>, Vec<(i32, i32)>) -> R>(chunk_provider: &mut A, cb: F, _center_position_and_chunk_radius: Option<((i64, i64, i64), u32)>) -> R {
    let all_chunks = chunk_provider.list_chunks().expect("Error listing chunks");
    let mut overworld: Dimension<std::fs::File> = Dimension::new();
    let total_chunks = all_chunks.len();

    // Load all chunks into memory
    for (chunk_x, chunk_z) in all_chunks.iter().copied() {
        let c_bytes = chunk_provider.load_chunk(chunk_x, chunk_z).expect("Error loading chunk");
        let c = CompoundTag::from_bytes(&c_bytes).unwrap();
        overworld.add_chunk(chunk_x, chunk_z, &mut Cursor::new(&c_bytes)).expect("Failed to add chunk");
    }

    cb(overworld, all_chunks)
}
