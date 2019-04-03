extern crate slime_seed_finder;
use slime_seed_finder::biome_layers::candidate_river_map;
use slime_seed_finder::biome_layers::Area;

fn all_candidate_river_maps() {
    let area = Area { x: 0, z: 0, w: 30, h: 30 };
    //let world_seed = 1234;
    for world_seed in 0..(1 << 22) {
        candidate_river_map(area, world_seed);
    }
}

fn main() {
    all_candidate_river_maps();
}

