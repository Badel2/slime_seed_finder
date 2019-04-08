#[macro_use]
extern crate ndarray;
use slime_seed_finder::biome_layers::candidate_river_map;
use slime_seed_finder::biome_layers::generate_up_to_layer;
use slime_seed_finder::biome_layers::Area;
use slime_seed_finder::biome_layers::Map;
use slime_seed_finder::biome_layers::GetMap;
use slime_seed_finder::biome_layers::biome_id;
use slime_seed_finder::biome_layers::reverse_map_voronoi_zoom;
use slime_seed_finder::biome_layers::MapVoronoiZoom;
use slime_seed_finder::biome_layers::HelperMapRiverAll;

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

fn area_from_coords(c: &[(i64, i64)]) -> Area {
    if c.is_empty() {
        // On empty coords, return empty area
        return Area { x: 0, z: 0, w: 0, h: 0 }
    }

    let (mut x_min, mut z_min) = c[0];
    let (mut x_max, mut z_max) = c[0];

    for &(x, z) in c.iter().skip(1) {
        use std::cmp::{min, max};
        x_min = min(x_min, x);
        z_min = min(z_min, z);
        x_max = max(x_max, x);
        z_max = max(z_max, z);
    }

    let area = Area { x: x_min, z: z_min, w: (x_max - x_min + 1) as u64, h: (z_max - z_min + 1) as u64 };
    area
}

fn map_with_river_at(c: &[(i64, i64)], area: Area) -> Map {
    let mut m = Map::new(area);
    for (x, z) in c {
        m.a[((x - area.x) as usize, (z - area.z) as usize)] = biome_id::river;
    }
    m
}

fn seed_99999999() {
    let river_coords = vec![
        (-8, 7), (-9, 7),
        (-7, 6), (-8, 6), (-9, 6),
        (-7, 5), (-8, 5), (-9, 5),
        (-6, 4), (-7, 4),
        (-3, 3), (-4, 3), (-5, 3), (-6, 3), (-7, 3),
        (-2, 2), (-3, 2), (-4, 2), (-5, 2), (-6, 2),
        (-1, 1), (-2, 1), (-3, 1), (-4, 1),
        (-1, 0), (-2, 0), (-3, 0),
        (-1, -1), (-2, -1), (-3, -1), (-4, -1), (-5, -1),
        (-2, -2), (-3, -2), (-4, -2), (-5, -2),
        (-5, -3), (-6, -3), (-7, -3), (-8, -3), (-9, -3), 
        (-6, -4), (-7, -4), (-8, -4), (-9, -4), 
    ];
    let river_coords_voronoi = vec![
    [-1, 0],
    [-1, 1],
    [-1, 2],
    [-1, 3],
    [-1, 4],
    [-1, 5],
    [-2, 5],
    [-3, 5],
    [-5, 5],
    [-4, 5],
    [-3, 6],
    [-4, 6],
    [-5, 6],
    [-6, 6],
    [-6, 7],
    [-5, 7],
    [-1, 6],
    [-2, 6],
    [-3, 7],
    [-4, 7],
    [-5, 8],
    [-6, 8],
    [-4, 9],
    [-4, 10],
    [-5, 9],
    [-5, 10],
    [-4, 11],
    [-3, 11],
    [-5, 11],
    [-6, 10],
    [-7, 10],
    [-6, 9],
    [-7, 9],
    [0, -1],
    [-1, -1],
    [-1, -2],
    [-2, -2],
    [-3, -2],
    [-4, -2],
    [-5, -2],
    [-5, -3],
    [-5, -4],
    [-5, -5],
    [-5, -6],
    [-4, -6],
    [-4, -7],
    [-4, -8],
    [-5, -8],
    [-5, -7],
    [-6, -7],
    [-8, -7],
    [-7, -7],
    [-9, -7],
    [-9, -8],
    [-10, -8],
    [-11, -8],
    [-12, -8],
    [-12, -7],
    [-11, -7],
    [-10, -7],
    [-13, -7],
    [-14, -7],
    [-15, -7],
    [-16, -7],
    [-16, -8],
    [-8, 11],
    [-8, 10],
    [-9, 11],
    [-9, 12],
    [-9, 13],
    [-9, 14],
    [-8, 14],
    [-10, 14],
    [-10, 15],
    [-11, 15],
    [-12, 15],
    [-13, 15],
    [-14, 14],
    [-13, 14],
    [-15, 14],
    [-15, 15],
    [-16, 15],
    [-17, 15],
    [-17, 16],
    [-18, 16],
    [-19, 16],
    [-19, 17],
    [-20, 17],
    [-20, 18],
    [-20, 19],
    [-19, 19],
    [-20, 20],
    [-21, 20],
    [-22, 20],
    [-23, 20],
    [-24, 20],
    [-25, 20],
    [-25, 21],
    [-25, 22],
    [-24, 22],
    [-24, 23],
    [-24, 24],
    [-23, 24],
    [-24, 25],
    [-25, 25],
    [-25, 26],
    [-26, 26],
    [-26, 27],
    [-26, 28],
    [-27, 28],
    [-28, 28],
    [-28, 29],
    [-29, 29],
    [-29, 30],
    [-29, 31],
    [-30, 31],
    [-31, 31],
    [-32, 31],
    [-33, 31],
    [-34, 31],
    [-34, 30],
    [-35, 30],
    [-35, 29],
    [-36, 29],
    [-36, 28],
    [-36, 27],
    [-36, 26],
    [-36, 25],
    [-36, 24],
    [-36, 23],
    [-36, 22],
    [-36, 21],
    [-35, 21],
    [-35, 22],
    [-34, 22],
    [-33, 22],
    [-32, 21],
    [-32, 22],
    [-31, 21],
    [-31, 20],
    [-30, 21],
    [-29, 21],
    [-28, 20],
    [-28, 21],
    [-27, 21],
    [-26, 21],
    [-26, 20],
    [-27, 20],
    [-28, 19],
    [-28, 18],
    [-28, 17],
    [-27, 17],
    [-27, 16],
    [-27, 15],
    [-27, 14],
    [-27, 13],
    [-27, 12],
    [-25, 12],
    [-26, 12],
    [-24, 12],
    [-23, 12],
    [-23, 11],
    [-23, 10],
    [-23, 9],
    [-22, 9],
    [-21, 9],
    [-21, 8],
    [-20, 8],
    [-20, 9],
    [-19, 9],
    [-18, 9],
    [-17, 9],
    [-16, 9],
    [-15, 9],
    [-15, 8],
    [-15, 6],
    [-14, 6],
    [-14, 5],
    [-13, 5],
    [-12, 5],
    [-12, 4],
    [-12, 3],
    [-11, 3],
    [-11, 2],
    [-10, 2],
    [-10, 1],
    [-10, 0],
    [-11, 0],
    [-12, -1],
    [-11, -1],
    [-13, -1],
    [-14, -1],
    [-15, -1],
    [-15, 0],
    [-16, 0],
    [-17, 0],
    [-18, 0],
    [-18, 1],
    [-19, 1],
    [-19, 0],
    [-19, -1],
    [-19, -2],
    [-18, -2],
    [-18, -3],
    [-18, -4],
    [-19, -4],
    [-20, -4],
    [-20, -5],
    [-20, -6],
    [-21, -6],
    [-21, -7],
    [-21, -8],
    [-22, -8],
    [-22, -9],
    [-23, -9],
    [-24, -9],
    [-25, -9],
    [-26, -9],
    [-27, -9],
    [-28, -9],
    [-29, -9],
    [-30, -9],
    [-32, -9],
    [-31, -9],
    [-36, -9],
    [-35, -9],
    [-34, -9],
    [-33, -9],
    [-36, -8],
    [-35, -8],
    [-34, -8],
    [-33, -8],
    [-32, -8],
    [-31, -8],
    [-36, -10],
    [-35, -11],
    [-36, -12],
    [-35, -12],
    [-35, -10],
    [-36, -13],
    [-36, -14],
    [-36, -15],
    [-36, -16],
    [-35, -16],
    [-35, -15],
    [-34, -15],
    [-33, -15],
    [-32, -15],
    [-31, -15],
    [-31, -16],
    [-30, -16],
    [-30, -15],
    [-29, -15],
    [-28, -15],
    [-27, -15],
    [-26, -15],
    [-25, -15],
    [-24, -15],
    [-23, -15],
    [-23, -14],
    [-22, -14],
    [-21, -14],
    [-21, -13],
    [-21, -12],
    [-20, -12],
    [-20, -11],
    [-19, -11],
    [-19, -10],
    [-18, -10],
    [-17, -10],
    [-16, -10],
    [-16, -9],
    [-17, -11],
    [-18, -11],
    [-35, -14],
    [-35, -13],
    [-34, -14],
    [-34, -13],
    [-34, -12],
    [-34, -11],
    [-34, -10],
    [-33, -14],
    [-33, -13],
    [-33, -12],
    [-33, -11],
    [-33, -10],
    [-32, -14],
    [-31, -14],
    [-30, -14],
    [-29, -14],
    [-27, -14],
    [-28, -14],
    [-26, -14],
    [-25, -14],
    [-24, -14],
    [-22, -13],
    [-22, -12],
    [-22, -11],
    [-22, -10],
    [-23, -13],
    [-24, -13],
    [-32, -13],
    [-31, -13],
    [-32, -12],
    [-31, -12],
    [-32, -11],
    [-31, -11],
    [-32, -10],
    [-31, -10],
    [-30, -13],
    [-29, -13],
    [-28, -13],
    [-27, -13],
    [-26, -13],
    [-25, -13],
    [-23, -12],
    [-24, -12],
    [-25, -12],
    [-26, -12],
    [-27, -12],
    [-28, -12],
    [-29, -12],
    [-30, -12],
    [-30, -11],
    [-30, -10],
    [-29, -11],
    [-29, -10],
    [-28, -11],
    [-23, -11],
    [-24, -11],
    [-25, -11],
    [-26, -11],
    [-27, -11],
    [-23, -10],
    [-24, -10],
    [-25, -10],
    [-26, -10],
    [-27, -10],
    [-28, -10],
    [-21, -11],
    [-21, -10],
    [-21, -9],
    [-20, -10],
    [-20, -9],
    [-20, -8],
    [-20, -7],
    [-19, -9],
    [-19, -8],
    [-19, -7],
    [-18, -9],
    [-17, -9],
    [-18, -8],
    [-17, -8],
    [-17, -7],
    [-18, -6],
    [-18, -7],
    [-19, -6],
    [-18, -5],
    [-19, -5],
    [-18, -1],
    [-17, -6],
    [-17, -5],
    [-17, -4],
    [-17, -3],
    [-17, -2],
    [-17, -1],
    [-16, -1],
    [-16, -2],
    [-16, -3],
    [-16, -4],
    [-16, -5],
    [-16, -6],
    [-15, -6],
    [-15, -5],
    [-15, -4],
    [-15, -3],
    [-15, -2],
    [-14, -6],
    [-14, -5],
    [-14, -4],
    [-14, -3],
    [-14, -2],
    [-13, -6],
    [-13, -5],
    [-13, -4],
    [-13, -2],
    [-13, -3],
    [-12, -6],
    [-11, -6],
    [-11, -5],
    [-12, -5],
    [-12, -4],
    [-11, -4],
    [-11, -3],
    [-12, -3],
    [-12, -2],
    [-11, -2],
    [-10, -6],
    [-9, -5],
    [-8, -4],
    [-8, -3],
    [-7, -3],
    [-7, -2],
    [-6, -1],
    [-6, -2],
    [-6, -3],
    [-8, -5],
    [-9, -6],
    [-8, -6],
    [-7, -6],
    [-6, -6],
    [-6, -5],
    [-7, -5],
    [-6, -4],
    [-7, -4],
    [-10, -5],
    [-10, -4],
    [-9, -4],
    [-10, -3],
    [-9, -3],
    [-10, -2],
    [-9, -1],
    [-9, -2],
    [-10, -1],
    [-8, -2],
    [-8, -1],
    [-7, -1],
    [-5, -1],
    [-4, -1],
    [-3, -1],
    [-2, -1],
    [-2, 0],
    [-3, 0],
    [-4, 0],
    [-5, 0],
    [-6, 0],
    [-7, 0],
    [-8, 0],
    [-9, 0],
    [-9, 1],
    [-8, 1],
    [-7, 1],
    [-6, 1],
    [-5, 1],
    [-4, 1],
    [-3, 1],
    [-2, 1],
    [-2, 2],
    [-3, 2],
    [-4, 2],
    [-5, 2],
    [-6, 2],
    [-7, 2],
    [-8, 2],
    [-9, 2],
    [-10, 3],
    [-9, 3],
    [-8, 3],
    [-7, 3],
    [-6, 3],
    [-5, 3],
    [-4, 3],
    [-3, 3],
    [-2, 3],
    [-2, 4],
    [-3, 4],
    [-4, 4],
    [-5, 4],
    [-6, 4],
    [-7, 4],
    [-8, 4],
    [-9, 4],
    [-10, 4],
    [-11, 4],
    [-11, 5],
    [-10, 5],
    [-9, 5],
    [-8, 5],
    [-7, 5],
    [-6, 5],
    [-7, 6],
    [-8, 6],
    [-9, 6],
    [-10, 6],
    [-11, 6],
    [-12, 6],
    [-13, 6],
    [-15, 10],
    [-15, 11],
    [-15, 12],
    [-15, 13],
    [-12, 14],
    [-11, 14],
    [-11, 13],
    [-10, 12],
    [-10, 13],
    [-15, 7],
    [-14, 7],
    [-13, 8],
    [-12, 9],
    [-11, 10],
    [-10, 11],
    [-13, 7],
    [-11, 7],
    [-12, 7],
    [-10, 7],
    [-7, 7],
    [-9, 7],
    [-8, 7],
    [-7, 8],
    [-8, 8],
    [-9, 8],
    [-10, 8],
    [-11, 8],
    [-12, 8],
    [-11, 9],
    [-10, 9],
    [-9, 9],
    [-8, 9],
    [-9, 10],
    [-10, 10],
    [-14, 8],
    [-14, 9],
    [-14, 10],
    [-13, 9],
    [-13, 10],
    [-12, 10],
    [-11, 11],
    [-12, 11],
    [-13, 11],
    [-14, 11],
    [-14, 12],
    [-13, 12],
    [-12, 12],
    [-11, 12],
    [-12, 13],
    [-13, 13],
    [-14, 13],
    [-16, 10],
    [-17, 10],
    [-18, 10],
    [-19, 10],
    [-20, 10],
    [-21, 10],
    [-22, 10],
    [-22, 11],
    [-21, 11],
    [-20, 11],
    [-19, 11],
    [-18, 11],
    [-17, 11],
    [-16, 11],
    [-16, 12],
    [-17, 12],
    [-18, 12],
    [-19, 12],
    [-20, 12],
    [-21, 12],
    [-22, 12],
    [-26, 13],
    [-27, 18],
    [-27, 19],
    [-16, 14],
    [-16, 13],
    [-17, 13],
    [-17, 14],
    [-18, 13],
    [-18, 14],
    [-19, 13],
    [-19, 14],
    [-18, 15],
    [-19, 15],
    [-20, 13],
    [-20, 14],
    [-20, 15],
    [-20, 16],
    [-25, 13],
    [-24, 13],
    [-23, 13],
    [-22, 13],
    [-21, 13],
    [-21, 14],
    [-22, 14],
    [-23, 14],
    [-24, 14],
    [-25, 14],
    [-26, 14],
    [-26, 15],
    [-25, 16],
    [-24, 16],
    [-23, 15],
    [-22, 16],
    [-21, 17],
    [-21, 18],
    [-22, 19],
    [-23, 19],
    [-24, 19],
    [-25, 19],
    [-26, 19],
    [-26, 18],
    [-26, 17],
    [-26, 16],
    [-25, 15],
    [-24, 15],
    [-23, 16],
    [-22, 15],
    [-21, 15],
    [-21, 16],
    [-21, 19],
    [-22, 18],
    [-22, 17],
    [-23, 17],
    [-24, 17],
    [-25, 17],
    [-25, 18],
    [-24, 18],
    [-23, 18],
    [-35, 23],
    [-34, 23],
    [-33, 23],
    [-32, 23],
    [-31, 22],
    [-30, 22],
    [-29, 22],
    [-28, 22],
    [-27, 22],
    [-26, 22],
    [-25, 23],
    [-25, 24],
    [-26, 23],
    [-27, 23],
    [-28, 23],
    [-29, 23],
    [-30, 23],
    [-31, 23],
    [-35, 24],
    [-34, 24],
    [-33, 24],
    [-32, 24],
    [-31, 24],
    [-30, 24],
    [-29, 24],
    [-28, 24],
    [-27, 24],
    [-26, 24],
    [-26, 25],
    [-27, 25],
    [-28, 25],
    [-29, 25],
    [-30, 25],
    [-31, 25],
    [-32, 25],
    [-33, 25],
    [-34, 25],
    [-35, 25],
    [-35, 26],
    [-34, 26],
    [-33, 26],
    [-32, 26],
    [-31, 26],
    [-30, 26],
    [-29, 26],
    [-28, 26],
    [-27, 26],
    [-27, 27],
    [-28, 27],
    [-29, 28],
    [-29, 27],
    [-30, 27],
    [-31, 27],
    [-32, 27],
    [-33, 27],
    [-34, 27],
    [-35, 27],
    [-35, 28],
    [-34, 28],
    [-34, 29],
    [-33, 29],
    [-32, 29],
    [-31, 29],
    [-30, 28],
    [-30, 30],
    [-33, 30],
    [-33, 28],
    [-32, 28],
    [-31, 28],
    [-30, 29],
    [-31, 30],
    [-32, 30],
    ];
	let river_coords_voronoi = river_coords_voronoi.into_iter().map(|x| (x[0], x[1])).collect::<Vec<_>>();
    let area_voronoi = area_from_coords(&river_coords_voronoi);
    let target_map_voronoi = map_with_river_at(&river_coords_voronoi, area_voronoi);
    let target_map_derived = reverse_map_voronoi_zoom(&target_map_voronoi);

    // Using manual layer42 coords works fine and finishes in 3 minutes
    // How about using layer43 coords (voronoi)? Even if reverse_map_voronoi_zoom
    // is far from perfect, it should work
    let target_score = river_coords.len() as u32;
    let area = area_from_coords(&river_coords);
    let target_map = map_with_river_at(&river_coords, area);

    // reverse_map_voronoi_zoom is not perfect yet, 3 errors :(
    // target_map_derived.a[(8, 5)] = biome_id::river;
    // target_map_derived.a[(6, 7)] = biome_id::river;
    // target_map_derived.a[(2, 10)] = biome_id::river;
    assert_eq!(target_map, target_map_derived);


    let target_map = target_map_derived;
    let area = target_map.area();
    let target_score = count_rivers(&target_map);

    let mut target_map_voronoi_sliced = target_map_voronoi.clone();
    target_map_voronoi_sliced.x += 2;
    target_map_voronoi_sliced.z += 2;
    target_map_voronoi_sliced.a.slice_collapse(s![2..-3, 2..-2]);
    println!("{:?} vs {:?}", target_map_voronoi.area(), target_map_voronoi_sliced.area());
    // Actually, we only want to compare borders, so use HelperMapRiverAll, which is actually an
    // edge detector
    let target_map_voronoi_sliced = HelperMapRiverAll::new(1, 0).get_map_from_pmap(&target_map_voronoi_sliced);
    let target_score_voronoi_sliced = count_rivers(&target_map_voronoi_sliced);
    
    println!("{:?}", target_map);
    println!("Target score: {}", target_score);
    let mut candidates_25 = vec![];

    for world_seed in 0..(1 << 25) {
        let candidate_map = candidate_river_map(area, world_seed);

        let and_map = map_river_and(candidate_map, &target_map);
        let candidate_score = count_rivers(&and_map);
        if candidate_score >= target_score * 98 / 100 {
            println!("{:08X}: {}", world_seed, candidate_score);
            candidates_25.push(world_seed);
        }
    }
    println!("{:08X?}", candidates_25);
    println!("25 bit candidates: {}", candidates_25.len());

    // Use the river map again to find bit 25
    let candidates_26 = candidates_25.into_iter().flat_map(|x| {
        let mut v = vec![x];
        let world_seed = x | (1 << 25);
        let candidate_map = candidate_river_map(area, world_seed);

        let and_map = map_river_and(candidate_map, &target_map);
        let candidate_score = count_rivers(&and_map);
        if candidate_score >= target_score * 95 / 100 {
            println!("{:08X}: {}", world_seed, candidate_score);
            v.push(world_seed);
        }

        v
    }).collect::<Vec<_>>();
    println!("{:08X?}", candidates_26);
    println!("26 bit candidates: {}", candidates_26.len());

    println!("Target voronoi score: {}", target_score_voronoi_sliced);
    // Now use voronoi zoom to bruteforce the remaining (34-26 = 8 bits)
    let candidates_34 = candidates_26.into_iter().flat_map(|x| {
        let mut v = vec![];
        for seed in 0..(1 << (34 - 26)) {
            let world_seed = x | (seed << 26);
            let g43 = MapVoronoiZoom::new(10, world_seed);
            let candidate_voronoi = g43.get_map_from_pmap(&target_map);
            let candidate_voronoi = HelperMapRiverAll::new(1, 0).get_map_from_pmap(&candidate_voronoi);
            let and_map = map_river_and(candidate_voronoi, &target_map_voronoi_sliced);
            let candidate_score = count_rivers(&and_map);
            if candidate_score >= target_score_voronoi_sliced * 90 / 100 {
                println!("{:09X}: {}", world_seed, candidate_score);
                v.push(world_seed);
            }
        }

        v
    }).collect::<Vec<_>>();
    println!("{:08X?}", candidates_34);
    println!("34 bit candidates: {}", candidates_34.len());

    // Can't use rivers to find 48 bits because rivers use 64 bits
    // Can't use biomes because biomes also use 64 bits
}

fn main() {
    //all_candidate_river_maps();
    seed_99999999();
}

