use slime_seed_finder::biome_layers;
use slime_seed_finder::biome_layers::biome_id;
use slime_seed_finder::seed_info::SeedInfo;
use log::*;
use std::env;
use std::fs::File;
use pretty_env_logger;

// TODO: this example is mainly used for testing
// In the future, the main app will have all the functionality

fn main() {
    pretty_env_logger::init();

    let args: Vec<_> = env::args().collect();
    let seed_info = read_seed_info_file(&args[1]).unwrap();
    let seeds = find_seed_rivers(&seed_info);
    info!("Found {} seeds: ", seeds.len());
    println!("{:#?}", seeds);
}

fn read_seed_info_file(path: &str) -> Result<SeedInfo, Box<std::error::Error>> {
    let f = File::open(path)?;
    //let file = BufReader::new(&f);
    Ok(serde_json::from_reader(f)?)
}

fn find_seed_rivers(seed_info: &SeedInfo) -> Vec<i64> {
    let extra_biomes: Vec<_> = seed_info.biomes.iter().flat_map(|(id, vec_xz)| {
        if *id == biome_id::river {
            vec![]
        } else {
            vec_xz.iter().map(|(x, z)| (*id, *x, *z)).collect()
        }
    }).collect();

    if let Some(rivers) = seed_info.biomes.get(&biome_id::river) {
        biome_layers::river_seed_finder(rivers, &extra_biomes)
    } else {
        error!("No rivers in seedInfo");
        vec![]
    }
}
