//! This module contains functions related to reading saved words
//!
//! Anvil is the name of the format used by Minecraft to store word files.
//! It contains all the block data, entities, and biome info of each chunk.

pub use anvil_region::AnvilChunkProvider;
pub use anvil_region::FolderChunkProvider;
pub use anvil_region::ZipChunkProvider;
use anvil_region::ChunkLoadError;
use anvil_region::AnvilRegion;
use nbt::CompoundTag;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::path::PathBuf;
use std::io::BufReader;
use log::*;

/// Read all the existing chunks in a `area_size*area_size` block area around
/// `(block_x, block_z)`.
pub fn read_area_around<A: AnvilChunkProvider>(chunk_provider: &mut A, area_size: u64, (block_x, block_z): (i64, i64)) -> Result<Vec<CompoundTag>, ChunkLoadError> {
    let mut r = vec![];
    let start_x = (block_x / 16) as i32;
    let start_z = (block_z / 16) as i32;
    let ahc = i32::try_from((area_size / 16) / 2).unwrap();
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

use std::io::{Seek, Read, Write};
/// Read all the chunks in region file
/// 32*32 chunks
pub fn read_from_region_file<F: Seek+Read+Write>(region: &mut AnvilRegion<F>) -> Result<Vec<CompoundTag>, ChunkLoadError> {
    let mut r = vec![];

    for x in 0..32 {
        for z in 0..32 {
            match region.read_chunk(x, z) {
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
pub fn get_rivers_and_some_extra_biomes_folder(input_dir: &PathBuf, center_block_arg: (i64, i64)) -> (Vec<(i64, i64)>, Vec<(i32, i64, i64)>) {
    let mut chunk_provider = FolderChunkProvider::new(input_dir.to_str().unwrap());

    get_rivers_and_some_extra_biomes(&mut chunk_provider, center_block_arg)
}

pub fn get_rivers_and_some_extra_biomes_zip(input_zip: &PathBuf, center_block_arg: (i64, i64)) -> (Vec<(i64, i64)>, Vec<(i32, i64, i64)>) {
    let file = std::fs::File::open(&input_zip).unwrap();
    let reader = BufReader::new(file);
    let mut chunk_provider = ZipChunkProvider::new(reader);

    get_rivers_and_some_extra_biomes(&mut chunk_provider, center_block_arg)
}

pub fn get_rivers_and_some_extra_biomes<A: AnvilChunkProvider>(chunk_provider: &mut A, center_block_arg: (i64, i64)) -> (Vec<(i64, i64)>, Vec<(i32, i64, i64)>) {
    let blocks_around_center: u32 = 1_000;

    let cheb = spiral::ChebyshevIterator::new(0, 0, u16::max_value());
    for (cheb_i, (cheb_x, cheb_z)) in cheb.enumerate() {
        if cheb_i == 10 {
            warn!("This is taking longer than expected");
            if center_block_arg == (0, 0) {
                warn!("Please feel free to specify some center coordinates to speed up the process.");
            } else {
                warn!("The provided coordinates are probably wrong: {:?}", center_block_arg);
                warn!("Please double check that there are rivers near this block coordinates");
            }
        }
        let center_block = (center_block_arg.0 + i64::from(cheb_x) * i64::from(blocks_around_center), center_block_arg.1 + i64::from(cheb_z) * i64::from(blocks_around_center));
        debug!("Trying to find chunks around {:?}", center_block);
        let chunks = read_area_around(chunk_provider, u64::from(blocks_around_center), center_block).unwrap();
        if chunks.is_empty() {
            debug!("Area around {:?} is not present in the saved world", center_block);
            continue;
        }

        let mut biome_data = HashMap::new();
        let mut river_chunks: HashMap<(i32, i32), u8> = HashMap::new();
        for c in chunks {
            let level_compound_tag = c.get_compound_tag("Level").unwrap();
            let chunk_x = level_compound_tag.get_i32("xPos").unwrap();
            let chunk_z = level_compound_tag.get_i32("zPos").unwrap();
            let biomes_array = level_compound_tag.get_i32_vec("Biomes").unwrap();
            match biomes_array.len() {
                0 => {}
                256 => {}
                n => panic!("Unexpected biomes_array len: {}", n),
            }

            for (i_b, b) in biomes_array.into_iter().enumerate() {
                let block_x = i64::from(chunk_x) * 16 + (i_b % 16) as i64;
                let block_z = i64::from(chunk_z) * 16 + (i_b / 16) as i64;
                let b = b.clone();

                match b {
                    127 => {
                        // Ignore void biome (set by WorldDownloader for unknown biomes)
                    }
                    b => {
                        if b == 7 {
                            // River: store as potential river_seed_finder starting point
                            let a = river_chunks.entry((chunk_x, chunk_z)).or_default();
                            *a = a.saturating_add(1);
                        }
                        biome_data.insert((block_x, block_z), b);
                    }
                }
            }
        }

        if biome_data.is_empty() {
            debug!("No chunks found around {:?}. Maybe that part of the map is not generated?", center_block);
            continue;
        }

        if river_chunks.is_empty() {
            debug!("No rivers found around {:?}. Please try again with different coords.", center_block);
            continue;
        }

        debug!("biome_data.len(): {}", biome_data.len());
        debug!("river_chunks: {:?}", river_chunks);
        let (brc_x, brc_z) = best_river_chunk(&river_chunks).unwrap();

        let mut rivers = vec![];
        {
            let start_x = i64::from((brc_x - 1) * 16);
            let start_z = i64::from((brc_z - 1) * 16);

            for x in 0..16*3 {
                for z in 0..16*3 {
                    if biome_data.get(&(start_x+x, start_z+z)) == Some(&7) {
                        rivers.push((start_x+x, start_z+z));
                    }
                }
            }
        }
        debug!("rivers: {:?}", rivers);

        let mut extra_biomes = vec![];
        // Hashmap iteration follows a random order, so take some random biomes
        extra_biomes.extend(biome_data.iter().map(|((x, z), b)| (*b, *x, *z)).take(30));
        debug!("extra_biomes: {:?}", extra_biomes);

        return (rivers, extra_biomes);
    }

    error!("Found zero valid chunks. Is this even a minecraft save?");

    (vec![], vec![])
}
