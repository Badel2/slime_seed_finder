
use slime_seed_finder::biome_layers::candidate_river_map;
use slime_seed_finder::biome_layers::generate_up_to_layer;
use slime_seed_finder::biome_layers::Area;
use slime_seed_finder::biome_layers::Map;
use slime_seed_finder::biome_layers::biome_id;

fn map_river_and(mut a: Map, b: &Map) -> Map {
    assert_eq!(a.area(), b.area());
    let area = a.area();
    for z in 0..area.h as usize {
        for x in 0..area.w as usize {
            let v11_a = a.a[(x, z)];
            let v11_b = b.a[(x, z)];
            a.a[(x, z)] = if v11_a == biome_id::river && v11_a == v11_b {
                biome_id::river
            } else {
                -1
            }
        }
    }

    a
}

fn count_rivers(m: &Map) -> u32 {
    m.a.fold(0, |acc, &x| if x == biome_id::river { acc + 1 } else { acc })
}

fn all_candidate_river_maps() {
    let area = Area { x: 250, z: 50, w: 20, h: 20 };
    let world_seed = 0x00ABCDEF;
    let target_map = generate_up_to_layer(area, world_seed, 41);
    let target_score = count_rivers(&target_map);
    println!("Target score: {}", target_score);
    // Bruteforcing 2^25 should be enough: there will be a very high similarity
    // So we need to store the most similar seeds
    for world_seed in 0..(1 << 25) {
        let candidate_map = candidate_river_map(area, world_seed);
        let and_map = map_river_and(candidate_map, &target_map);
        let candidate_score = count_rivers(&and_map);
        if candidate_score >= target_score * 9 / 10 {
            println!("{:08X}: {}", world_seed, candidate_score);
        }
    }
}

fn main() {
    all_candidate_river_maps();
}

