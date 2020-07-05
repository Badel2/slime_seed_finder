//! This module contains functions related to reading saved words
//!
//! Anvil is the name of the format used by Minecraft to store world files.
//! It contains all the block data, entities, and biome info of each chunk.

pub use anvil_region::AnvilChunkProvider;
pub use anvil_region::FolderChunkProvider;
pub use anvil_region::ZipChunkProvider;
use anvil_region::ChunkLoadError;
use nbt::CompoundTag;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::PathBuf;
use log::*;
use crate::biome_layers::is_oceanic;
use crate::biome_info::biome_id;
use crate::chunk::Point;
use crate::chunk::Point4;
use crate::seed_info::BiomeId;

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
