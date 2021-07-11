use fastanvil::Chunk;
use serde::{Serialize, Deserialize};
use slime_seed_finder::anvil;
use slime_seed_finder::anvil::ZipChunkProvider;
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::path::Path;

#[derive(Debug, Default, Serialize, Deserialize)]
struct Counts {
    per_seed: Vec<(i64, BTreeMap<String, Vec<u64>>)>,
    total: BTreeMap<String, Vec<u64>>,
}

fn main() {
    pretty_env_logger::init();

    let args: Vec<_> = std::env::args().collect();

    let mut c = Counts::default();
    let dimension = None;
    let skip_odd_chunks = true;
    let filename = format!("subchunk_freq_centered_at_diamond{}.json", if skip_odd_chunks { "_odd_chunks" } else { "" });
    //let dimension = Some("DIM-1");
    //let filename = format!("subchunk_freq_all_nether.json");
    //let dimension = Some("DIM1");
    //let filename = format!("subchunk_freq_all_end.json");

    for zip_path in &args[1..] {
        let center_position_and_chunk_radius = None;
        println!("Opening {}", zip_path);

        let seed = anvil::read_seed_from_level_dat_zip(Path::new(zip_path), None).expect("failed to read seed from level.dat");
        println!("Seed: {}", seed);

        let zip_file = OpenOptions::new().write(false).read(true).create(false).open(zip_path).expect("failed to open zip file");

        let mut chunk_provider = ZipChunkProvider::new_with_dimension(zip_file, dimension).unwrap();

        let mut counts: BTreeMap<String, Vec<u64>> = Default::default();
        anvil::iterate_chunks_in_world(
            &mut chunk_provider,
            center_position_and_chunk_radius,
            &mut |(chunk_x, chunk_z), chunk: &fastanvil::JavaChunk| {
                if skip_odd_chunks {
                    if ((chunk_x & 1) ^ (chunk_z & 1)) != 0 {
                        // Skip odd chunks for this run
                        return;
                    }
                }
                let diamonds_position = find_diamonds_in_chunk(chunk);
                //println!("Chunk {:?}, diamonds at {:?}", (chunk_x, chunk_z), diamonds_position);
                let diamonds_position = match diamonds_position {
                    Some(x) => x,
                    None => return,
                };

                for x in 0..16 {
                    for y in 0..256 {
                        for z in 0..16 {
                            if let Some(block) = chunk.block(x as usize, y as isize, z as usize) {
                                let (x, y, z) = (x as i64, y as i64, z as i64);
                                let (chunk_x, chunk_z) = (chunk_x as i64, chunk_z as i64);
                                let block_x = chunk_x * 16 + x;
                                let block_y = i64::from(y);
                                let block_z = chunk_z * 16 + z;

                                // Sort the resulting blocks in 256 bins, according to their sub-chunk x and z coordinates
                                let sub_x = (x - diamonds_position.0 + 8) & 0xF;
                                let sub_z = (z - diamonds_position.2 + 8) & 0xF;
                                let idx = sub_z as usize * 16 + sub_x as usize;
                                counts.entry(block.name.to_string()).or_insert_with(|| vec![0; 256])[idx] += 1;
                            } else {
                                // TODO: check max y to avoid iterating from 0 to 255
                                //println!("No block?");
                            }
                        }
                    }
                }
            }
        )
        .unwrap();

        //println!("Hist: {:?}", counts);

        c.per_seed.push((seed, counts));
    }

    // Calculate total
    let mut total: BTreeMap<String, Vec<u64>> = Default::default();
    for (_seed, m) in &c.per_seed {
        for (block_name, counts) in m.iter() {
            let t = total.entry(block_name.to_string()).or_insert_with(|| vec![0; 256]);
            for i in 0..256 {
                t[i] += counts[i];
            }
        }
    }
    c.total = total;

    serde_json::to_writer(&std::fs::File::create(&filename).expect("failed to create file"), &c).expect("error writing file");
}

fn find_diamonds_in_chunk(chunk: &fastanvil::JavaChunk) -> Option<(i64, i64, i64)> {
    let mut all_diamonds = vec![];

    for x in 0..16 {
        for y in 0..256 {
            for z in 0..16 {
                if let Some(block) = chunk.block(x as usize, y as isize, z as usize) {
                    if block.name == "minecraft:diamond_ore" || block.name == "minecraft:deepslate_diamond_ore" {
                        all_diamonds.push((x, y, z));
                    }
                } else {
                    // TODO: check max y to avoid iterating from 0 to 255
                    //println!("No block?");
                }
            }
        }
    }

    // Count how many clusters of diamonds are there
    //println!("All diamonds: {:?}", all_diamonds);

    // Return the diamond with lowest x and z coordinates
    all_diamonds.into_iter().min_by_key(|(x, y, z)| (*x, *z, *y))
}
