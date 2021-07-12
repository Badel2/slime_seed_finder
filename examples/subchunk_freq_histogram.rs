//! Write block frequency data to subchunk_freq_all.json
//!
//! Usage:
//!
//!     RUST_LOG=info cargo run --release --example subchunk_freq_histogram path_to_world_zip_1 path_to_world_zip_2
//!
//! See https://github.com/Badel2/mc_block_stats for example and visualization.

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
    let filename = format!("subchunk_freq_all.json");
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
        anvil::iterate_blocks_in_world(
            &mut chunk_provider,
            center_position_and_chunk_radius,
            &mut |(x, y, z), block: &fastanvil::Block| {
                // Sort the resulting blocks in 256 bins, according to their sub-chunk x and z coordinates
                let sub_x = x & 0xF;
                let sub_z = z & 0xF;
                let idx = sub_z as usize * 16 + sub_x as usize;
                counts.entry(block.name().to_string()).or_insert_with(|| vec![0; 256])[idx] += 1;
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
