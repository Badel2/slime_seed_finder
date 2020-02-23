use palette::{Gradient, LinSrgb};
use serde::{Deserialize, Serialize};
use stdweb::console;
use stdweb::js_export;
use stdweb::serde::Serde;
use stdweb::web::TypedArray;

use slime_seed_finder::biome_info::biome_id;
use slime_seed_finder::biome_layers::Area;
use slime_seed_finder::biome_layers::Map;
use slime_seed_finder::chunk::Point;
use slime_seed_finder::java_rng::JavaRng;
use slime_seed_finder::mc_rng::McRng;
use slime_seed_finder::seed_info::BiomeId;
use slime_seed_finder::seed_info::MinecraftVersion;
use slime_seed_finder::seed_info::SeedInfo;
use slime_seed_finder::slime::SlimeChunks;
use slime_seed_finder::*;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    seed_info: SeedInfo,
    range: Option<(u32, u32)>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawRivers {
    l43_area: Area,
    l43: Vec<u8>,
    l42_area: Area,
    l42: Vec<u8>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateRiversCandidate {
    version: String,
    seed: String,
    area: Area,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnvilOptions {
    range: Option<(u32, u32)>,
    version: String,
}

#[js_export]
//pub fn slime_seed_finder(chunks_str: &str, no_chunks_str: &str) -> String {
//    let r = find_seed(chunks_str, no_chunks_str);
pub fn slime_seed_finder(o: Serde<Options>) -> String {
    let o = o.0;
    console!(log, "Hello from Rust");
    let r = find_seed(o);

    format!("Found {} seeds!\n{:#?}", r.len(), r)
}

#[js_export]
pub fn river_seed_finder(o: String) -> Vec<String> {
    let o: Result<Options, _> = serde_json::from_str(&o);
    let o = o.unwrap();
    console!(log, "Hello from Rust");
    let r = find_seed_rivers(o);

    r.into_iter().map(|seed| format!("{}", seed)).collect()
}

#[js_export]
pub fn draw_rivers(o: String) -> Serde<DrawRivers> {
    // TODO: detect when there are two separate river areas and return a vec of maps?
    let o: Result<Options, _> = serde_json::from_str(&o);
    let o = o.unwrap();
    let (_prevoronoi_coords, hd_coords) = biome_layers::segregate_coords_prevoronoi_hd(
        o.seed_info.biomes[&BiomeId(biome_id::river)].clone(),
    );
    let area_rivers = Area::from_coords(&o.seed_info.biomes[&BiomeId(biome_id::river)]);
    let target_map = biome_layers::map_with_river_at(
        &o.seed_info.biomes[&BiomeId(biome_id::river)],
        area_rivers,
    );
    let m = biome_layers::reverse_map_voronoi_zoom(&target_map).unwrap_or_default();

    let area_hd = Area::from_coords(&hd_coords);
    let target_map_hd = biome_layers::map_with_river_at(&hd_coords, area_hd);

    Serde(DrawRivers {
        l43_area: target_map_hd.area(),
        l43: biome_layers::draw_map_image(&target_map_hd),
        l42_area: m.area(),
        l42: biome_layers::draw_map_image(&m),
    })
}

#[js_export]
pub fn generate_rivers_candidate(o: String) -> Serde<DrawRivers> {
    let o: Result<GenerateRiversCandidate, _> = serde_json::from_str(&o);
    let o = o.unwrap();

    // TODO: only works for version 1.7
    let magic_layer_river_candidate = 141;

    Serde(DrawRivers {
        l43_area: Area {
            x: 0,
            z: 0,
            w: 0,
            h: 0,
        },
        l43: vec![],
        l42_area: o.area,
        l42: biome_layers::generate_image_up_to_layer(
            o.version.parse().unwrap(),
            o.area,
            o.seed.parse().unwrap(),
            magic_layer_river_candidate,
        ),
    })
}

#[js_export]
pub fn count_candidates(o: Serde<Options>) -> String {
    let o = o.0;
    let c: Vec<_> = o.seed_info.positive.slime_chunks;
    let nc: Vec<_> = o.seed_info.negative.slime_chunks;

    if (c.len() == 0) && (nc.len() == 0) {
        return format!("{} * 2^30 candidates", 1 << 18);
    }
    let sc = SlimeChunks::new(&c, 0, &nc, 0);
    let num_cand = sc.num_low_18_candidates() as u32;
    return format!("{} * 2^30 candidates", num_cand);
}

#[js_export]
pub fn draw_reverse_voronoi(o: Serde<Options>) -> Vec<u8> {
    let o = o.0;
    let target_map = seed_info::biomes_to_map(o.seed_info.biomes);

    let m = biome_layers::reverse_map_voronoi_zoom(&target_map).unwrap_or_default();
    biome_layers::draw_map_image(&m)
}

#[js_export]
pub fn extend48(s: &str) -> String {
    let mut r = vec![];
    for s in s.lines() {
        let x = match s.parse() {
            Ok(x) => {
                if x < (1u64 << 48) {
                    x
                } else {
                    let error_string = format!("Input must be lower than 2^48");
                    console!(error, &error_string);
                    return error_string;
                }
            }
            Err(e) => {
                let error_string = format!("{}", e);
                console!(error, &error_string);
                return error_string;
            }
        };

        let rr = JavaRng::extend_long_48(x);
        r.extend(rr.into_iter().map(|seed| seed as i64));
    }

    let mut s = format!("Found {} seeds!\n", r.len());
    r.sort();
    s.push_str(&format!("{:#?}\n", r));

    s
}

#[js_export]
pub fn count_rivers(o: String) -> String {
    let o: Result<Options, _> = serde_json::from_str(&o);
    let o = o.unwrap();
    if let Some(rivers) = o.seed_info.biomes.get(&BiomeId(biome_id::river)) {
        format!("{} rivers", rivers.len())
    } else {
        format!("No rivers :(")
    }
}

pub fn find_seed(o: Options) -> Vec<u64> {
    let c: Vec<_> = o.seed_info.positive.slime_chunks;
    let nc: Vec<_> = o.seed_info.negative.slime_chunks;

    if (c.len() == 0) && (nc.len() == 0) {
        console!(log, "Can't find seed without chunks");
        return vec![];
    }
    let sc = SlimeChunks::new(&c, 0, &nc, 0);
    let num_cand = sc.num_low_18_candidates() as u32;
    console!(log, format!("Found {} * 2^30 candidates", num_cand));
    console!(log, format!("ETA: about {} seconds", num_cand * 7));
    let seeds = sc.find_seed();

    {
        // Display only seeds that could be generated by java (empty box)
        let java_seeds: Vec<_> = seeds.iter().map(|&s| JavaRng::extend_long_48(s)).collect();

        console!(log, format!("Java seeds: \n{:#?}", java_seeds));
    }

    seeds
}

pub fn find_seed_rivers(o: Options) -> Vec<i64> {
    let extra_biomes: Vec<_> = o
        .seed_info
        .biomes
        .iter()
        .flat_map(|(id, vec_xz)| {
            if *id == BiomeId(biome_id::river) {
                vec![]
            } else {
                vec_xz.iter().map(|p| (*id, *p)).collect()
            }
        })
        .collect();
    let version = o.seed_info.version.parse().unwrap();
    if let Some(rivers) = o.seed_info.biomes.get(&BiomeId(biome_id::river)) {
        if let Some((range_lo, range_hi)) = o.range {
            biome_layers::river_seed_finder_range(
                rivers,
                &extra_biomes,
                version,
                range_lo,
                range_hi,
            )
        } else {
            biome_layers::river_seed_finder(rivers, &extra_biomes, version)
        }
    } else if let Some(rivers) = o
        .seed_info
        .biomes_quarter_scale
        .get(&BiomeId(biome_id::river))
    {
        // Only quarter scale biomes: find 26-bit candidates and exit
        if let Some((range_lo, range_hi)) = o.range {
            biome_layers::river_seed_finder_26_range(rivers, range_lo, range_hi)
        } else {
            biome_layers::river_seed_finder_26_range(rivers, 0, 1 << 24)
        }
    } else {
        console!(error, "Can't find seed without rivers");
        vec![]
    }
}

#[js_export]
pub fn generate_fragment(
    version: String,
    fx: i32,
    fy: i32,
    seed: String,
    frag_size: usize,
) -> Vec<u8> {
    let empty_map_as_error = || vec![0; frag_size * frag_size * 4];
    let version1: MinecraftVersion = match version.parse() {
        Ok(s) => s,
        Err(_) => {
            if version.starts_with("TreasureMap") {
                let seed = if let Ok(s) = seed.parse() {
                    s
                } else {
                    console!(error, format!("{} is not a valid seed", seed));
                    return empty_map_as_error();
                };
                let frag_size = frag_size as u64;
                let area = Area {
                    x: fx as i64 * frag_size as i64,
                    z: fy as i64 * frag_size as i64,
                    w: frag_size,
                    h: frag_size,
                };
                let mc_version = if version == "TreasureMap13" {
                    MinecraftVersion::Java1_13
                } else if version == "TreasureMap14" {
                    MinecraftVersion::Java1_14
                } else if version == "TreasureMap15" {
                    MinecraftVersion::Java1_15
                } else {
                    console!(
                        error,
                        format!("{} is not a valid treasure map version", version)
                    );
                    return empty_map_as_error();
                };
                return biome_layers::generate_image_treasure_map(mc_version, area, seed);
            } else {
                console!(error, format!("{} is not a valid version", version));
                return empty_map_as_error();
            }
        }
    };
    let num_layers = version1.num_layers();
    generate_fragment_up_to_layer(version, fx, fy, seed, frag_size, num_layers)
}

#[js_export]
pub fn generate_fragment_up_to_layer(
    version: String,
    fx: i32,
    fy: i32,
    seed: String,
    frag_size: usize,
    layer: u32,
) -> Vec<u8> {
    let empty_map_as_error = || vec![0; frag_size * frag_size * 4];
    let frag_size = frag_size as usize;
    let version = match version.parse() {
        Ok(s) => s,
        Err(_) => {
            if version.starts_with("TreasureMap") {
                let seed = if let Ok(s) = seed.parse() {
                    s
                } else {
                    console!(error, format!("{} is not a valid seed", seed));
                    return empty_map_as_error();
                };
                let frag_size = frag_size as u64;
                let area = Area {
                    x: fx as i64 * frag_size as i64,
                    z: fy as i64 * frag_size as i64,
                    w: frag_size,
                    h: frag_size,
                };
                let mc_version = if version == "TreasureMap13" {
                    MinecraftVersion::Java1_13
                } else if version == "TreasureMap14" {
                    MinecraftVersion::Java1_14
                } else if version == "TreasureMap15" {
                    MinecraftVersion::Java1_15
                } else {
                    console!(
                        error,
                        format!("{} is not a valid treasure map version", version)
                    );
                    return empty_map_as_error();
                };
                return biome_layers::generate_image_treasure_map(mc_version, area, seed);
            } else {
                console!(error, format!("{} is not a valid version", version));
                return empty_map_as_error();
            }
        }
    };
    let seed = if let Ok(s) = seed.parse() {
        s
    } else {
        console!(error, format!("{} is not a valid seed", seed));
        return empty_map_as_error();
    };

    let frag_size = frag_size as u64;
    let area = Area {
        x: fx as i64 * frag_size as i64,
        z: fy as i64 * frag_size as i64,
        w: frag_size,
        h: frag_size,
    };
    //let last_layer = 43;
    //let map = cubiomes_test::call_layer(last_layer, seed, area);
    let v = biome_layers::generate_image_up_to_layer(version, area, seed, layer);

    v
}

pub fn slime_to_color(id: u32, total: u32, grad1: &Gradient<LinSrgb>) -> [u8; 4] {
    assert!(id <= total);
    // Gradient from red to green
    // http://blogs.perl.org/users/ovid/2010/12/perl101-red-to-green-gradient.html

    if id == 0 {
        // red
        [0xFF, 0x00, 0x00, 0xFF]
    } else if id == total {
        // white
        [0xFF, 0xFF, 0xFF, 0xFF]
    } else {
        let color = grad1.get(id as f32 / total as f32);
        [
            (color.red * 255.0) as u8,
            (color.green * 255.0) as u8,
            (color.blue * 255.0) as u8,
            0xFF,
        ]
    }
}

#[js_export]
pub fn generate_fragment_slime_map(
    fx: i32,
    fy: i32,
    seeds: Vec<String>,
    frag_size: usize,
) -> Vec<u8> {
    let seeds: Vec<u64> = seeds
        .into_iter()
        .map(|s| {
            s.parse().unwrap_or_else(|s| {
                console!(error, format!("{} is not a valid seed", s));
                panic!("{} is not a valid seed", s);
            })
        })
        .collect();

    let frag_size = frag_size as u64;
    let area = Area {
        x: fx as i64 * frag_size as i64,
        z: fy as i64 * frag_size as i64,
        w: frag_size,
        h: frag_size,
    };
    //let last_layer = 43;
    let num_seeds = seeds.len();
    if num_seeds > (0x10000) {
        // 65k seeds
        console!(log, "This may take a while");
    }
    let (w, h) = (area.w as usize, area.h as usize);
    let mut map_sum = vec![0; w * h];
    for seed in seeds {
        let map = slime::gen_map_from_seed(area, seed);
        for x in 0..w {
            for z in 0..h {
                let is_slime_chunk = map.a[(x, z)] != 0;
                if is_slime_chunk {
                    let i = z * h + x;
                    map_sum[i] += 1;
                }
            }
        }
    }

    let grad1 = Gradient::new(vec![
        LinSrgb::new(0.0, 0.0, 0.0),
        LinSrgb::new(1.0, 1.0, 0.0),
        LinSrgb::new(0.0, 1.0, 0.0),
    ]);
    let mut v = vec![0; w * h * 4];
    for i in 0..w * h {
        let color = slime_to_color(map_sum[i], num_seeds as u32, &grad1);
        v[i * 4 + 0] = color[0];
        v[i * 4 + 1] = color[1];
        v[i * 4 + 2] = color[2];
        v[i * 4 + 3] = color[3];
    }

    v
}

#[js_export]
pub fn is_i64(seed: String) -> String {
    match seed.parse::<i64>() {
        Ok(_) => format!("OK"),
        Err(e) => format!("ERROR: {}", e.to_string()),
    }
}

#[js_export]
pub fn add_2_n(seed: String, n: u8) -> String {
    if n >= 64 {
        return seed;
    }

    if let Ok(s) = seed.parse::<i64>() {
        format!("{}", s.wrapping_add(1 << n))
    } else {
        seed
    }
}

#[js_export]
pub fn sub_2_n(seed: String, n: u8) -> String {
    if n >= 64 {
        return seed;
    }

    if let Ok(s) = seed.parse::<i64>() {
        format!("{}", s.wrapping_sub(1 << n))
    } else {
        seed
    }
}

#[js_export]
pub fn gen_test_seed_base_n_bits(base: String, n: String, bits: String) -> String {
    let base: i64 = base.parse().unwrap();
    let n: i64 = n.parse().unwrap();
    let bits: usize = bits.parse().unwrap();

    let sign = if n > 0 { 1 } else { -1 };
    let n = n * sign; //abs(n)

    let mut s = String::new();
    for i in 0..n {
        let x = base + i * sign * (1 << bits);
        s.push_str(&format!("{},\n", x));
    }

    s
}

#[js_export]
pub fn similar_biome_seed(seed: String) -> String {
    if let Ok(s) = seed.parse::<i64>() {
        format!("{}", McRng::similar_biome_seed(s))
    } else {
        seed
    }
}

#[js_export]
pub fn draw_treasure_map(o: String) -> Vec<u8> {
    console!(log, format!("Parsing options: {}", o));
    let o: Result<Options, _> = serde_json::from_str(&o);
    let o = o.unwrap();
    let first_treasure_map = &o.seed_info.treasure_maps[0];
    let mut pmap = Map::new(Area {
        x: 0,
        z: 0,
        w: 128,
        h: 128,
    });
    assert_eq!(first_treasure_map.map.len(), 128 * 128);
    for (i, v) in first_treasure_map.map.iter().enumerate() {
        let (x, z) = (i % 128, i / 128);
        pmap.a[(x, z)] = match v {
            0 => biome_id::ocean,
            1 => biome_id::plains,
            2 => biome_id::river,
            _ => panic!("Invalid id: {}", v),
        };
    }
    // Double map size from 128x128 to 256x256
    let mut pmap2 = Map::new(Area {
        x: 0,
        z: 0,
        w: 256,
        h: 256,
    });
    for x in 0..256 {
        for z in 0..256 {
            pmap2.a[(x, z)] = pmap.a[(x / 2, z / 2)];
        }
    }
    let tmap = biome_layers::treasure_map_at(
        first_treasure_map.fragment_x,
        first_treasure_map.fragment_z,
        &pmap2,
    );

    biome_layers::draw_treasure_map_image(&tmap)
}

#[js_export]
pub fn treasure_map_seed_finder(o: String) -> Vec<String> {
    console!(log, format!("Parsing options: {}", o));
    let o: Result<Options, _> = serde_json::from_str(&o);
    let o = o.unwrap();
    let first_treasure_map = &o.seed_info.treasure_maps[0];
    let mut pmap = Map::new(Area {
        x: 0,
        z: 0,
        w: 128,
        h: 128,
    });
    console!(
        log,
        format!("First treasure map len: {}", first_treasure_map.map.len())
    );
    assert_eq!(first_treasure_map.map.len(), 128 * 128);
    for (i, v) in first_treasure_map.map.iter().enumerate() {
        let (x, z) = (i % 128, i / 128);
        pmap.a[(x, z)] = match v {
            0 => biome_id::ocean,
            1 => biome_id::plains,
            2 => biome_id::river,
            _ => panic!("Invalid id: {}", v),
        };
    }
    let r = biome_layers::treasure_map_river_seed_finder(&pmap, 0, 1 << 24);

    r.into_iter().map(|seed| format!("{}", seed)).collect()
}

#[js_export]
pub fn anvil_region_to_river_seed_finder(
    zipped_world: TypedArray<u8>,
    is_minecraft_1_15: bool,
) -> String {
    use slime_seed_finder::anvil::ZipChunkProvider;
    use std::io::Cursor;
    // TODO: check if the input is actually a zipped_world, as it also may be a raw region file
    let mut zip_chunk_provider =
        ZipChunkProvider::new(Cursor::new(Vec::from(zipped_world))).unwrap();
    let center_block = Point { x: 0, z: 0 };
    let s = if is_minecraft_1_15 {
        let (rivers, _extra_biomes) =
            anvil::get_rivers_and_some_extra_biomes_1_15(&mut zip_chunk_provider, center_block);

        let mut s = SeedInfo::default();
        s.biomes_quarter_scale.insert(BiomeId(7), rivers);

        s
    } else {
        let (rivers, extra_biomes) =
            anvil::get_rivers_and_some_extra_biomes(&mut zip_chunk_provider, center_block);

        let mut s = SeedInfo::default();
        s.biomes.insert(BiomeId(7), rivers);

        for (b_id, b_coords) in extra_biomes {
            // Adding more rivers here breaks bounding box detection...
            if b_id != BiomeId(7) {
                s.biomes.entry(b_id).or_default().push(b_coords);
            }
        }

        s
    };

    serde_json::to_string(&s).unwrap()
}
