#![feature(custom_attribute)]

extern crate slime_seed_finder;
#[macro_use]
extern crate stdweb;
extern crate serde;
extern crate serde_json;
extern crate palette;
extern crate log;

mod stdweb_logger;

use stdweb::serde::Serde;
use palette::{Gradient, LinSrgb};
use serde::{Serialize, Deserialize};

use slime_seed_finder::*;
use slime_seed_finder::slime::SlimeChunks;
use slime_seed_finder::biome_layers::Area;
use slime_seed_finder::biome_layers::biome_id;
use slime_seed_finder::seed_info::SeedInfo;

#[cfg(feature = "wasm")]
fn main(){
    // Init console logger
    stdweb_logger::Logger::init_with_level(::log::LevelFilter::Debug);
    // Don't start, wait for user to press button
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    seed_info: SeedInfo,
    range: Option<(u32, u32)>,
}

js_serializable!(DrawRivers);
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
    seed: String,
    area: Area,
}

#[cfg(feature = "wasm")]
#[js_export]
//pub fn slime_seed_finder(chunks_str: &str, no_chunks_str: &str) -> String {
//    let r = find_seed(chunks_str, no_chunks_str);
pub fn slime_seed_finder(o: Serde<Options>) -> String {
    let o = o.0;
    console!(log, "Hello from Rust");
    let r = find_seed(o);

    format!("Found {} seeds!\n{:#?}", r.len(), r)
}

#[cfg(feature = "wasm")]
#[js_export]
pub fn river_seed_finder(o: String) -> Vec<String> {
    let o: Result<Options, _> = serde_json::from_str(&o);
    let o = o.unwrap();
    console!(log, "Hello from Rust");
    let r = find_seed_rivers(o);

    r.into_iter().map(|seed| format!("{}", seed)).collect()
}

#[cfg(feature = "wasm")]
#[js_export]
pub fn draw_rivers(o: String) -> DrawRivers {
    // TODO: detect when there are two separate river areas and return a vec of maps?
    let o: Result<Options, _> = serde_json::from_str(&o);
    let o = o.unwrap();
    let (_prevoronoi_coords, hd_coords) = biome_layers::segregate_coords_prevoronoi_hd(o.seed_info.biomes[&biome_id::river].clone());
    let area_rivers = biome_layers::area_from_coords(&o.seed_info.biomes[&biome_id::river]);
    let target_map = biome_layers::map_with_river_at(&o.seed_info.biomes[&biome_id::river], area_rivers);
    let m = biome_layers::reverse_map_voronoi_zoom(&target_map).unwrap_or_default();

    let area_hd = biome_layers::area_from_coords(&hd_coords);
    let target_map_hd = biome_layers::map_with_river_at(&hd_coords, area_hd);

    DrawRivers {
        l43_area: target_map_hd.area(),
        l43: biome_layers::draw_map_image(&target_map_hd),
        l42_area: m.area(),
        l42: biome_layers::draw_map_image(&m),
    }
}

#[cfg(feature = "wasm")]
#[js_export]
pub fn generate_rivers_candidate(o: String) -> DrawRivers {
    let o: Result<GenerateRiversCandidate, _> = serde_json::from_str(&o);
    let o = o.unwrap();

    DrawRivers {
        l43_area: Area { x: 0, z: 0, w: 0, h: 0 },
        l43: vec![],
        l42_area: o.area,
        l42: biome_layers::generate_image_up_to_layer(o.area, o.seed.parse().unwrap(), 141),
    }
}

#[cfg(feature = "wasm")]
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

#[cfg(feature = "wasm")]
#[js_export]
pub fn draw_reverse_voronoi(o: Serde<Options>) -> Vec<u8> {
    let o = o.0;
    let target_map = seed_info::biomes_to_map(o.seed_info.biomes);

    let m = biome_layers::reverse_map_voronoi_zoom(&target_map).unwrap_or_default();
    biome_layers::draw_map_image(&m)
}

#[cfg(feature = "wasm")]
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

        let rr = Rng::extend_long_48(x);
        r.extend(rr.into_iter().map(|seed| seed as i64));
    }

    let mut s = format!("Found {} seeds!\n", r.len());
    r.sort();
    s.push_str(&format!("{:#?}\n", r));

    s
}

#[cfg(feature = "wasm")]
#[js_export]
pub fn count_rivers(o: String) -> String {
    let o: Result<Options, _> = serde_json::from_str(&o);
    let o = o.unwrap();
    if let Some(rivers) = o.seed_info.biomes.get(&biome_id::river) {
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
        let java_seeds: Vec<_> = seeds
            .iter()
            .map(|&s| Rng::extend_long_48(s))
            .collect();

        console!(log, format!("Java seeds: \n{:#?}", java_seeds));
    }

    seeds
}

pub fn find_seed_rivers(o: Options) -> Vec<i64> {
    let extra_biomes: Vec<_> = o.seed_info.biomes.iter().flat_map(|(id, vec_xz)| {
        if *id == biome_id::river {
            vec![]
        } else {
            vec_xz.iter().map(|(x, z)| (*id, *x, *z)).collect()
        }
    }).collect();
    if let Some(rivers) = o.seed_info.biomes.get(&biome_id::river) {
        if let Some((range_lo, range_hi)) = o.range {
            biome_layers::river_seed_finder_range(rivers, &extra_biomes, range_lo, range_hi)
        } else {
            biome_layers::river_seed_finder(rivers, &extra_biomes)
        }
    } else {
        console!(error, "Can't find seed without rivers");
        vec![]
    }
}

#[cfg(feature = "wasm")]
#[js_export]
pub fn generate_fragment(fx: i32, fy: i32, seed: String, frag_size: usize) -> Vec<u8> {
    generate_fragment_up_to_layer(fx, fy, seed, frag_size, biome_layers::NUM_LAYERS)
}

#[cfg(feature = "wasm")]
#[js_export]
pub fn generate_fragment_up_to_layer(fx: i32, fy: i32, seed: String, frag_size: usize, layer: u32) -> Vec<u8> {
    let frag_size = frag_size as usize;
    let seed = if let Ok(s) = seed.parse() {
        s
    } else {
        console!(error, format!("{} is not a valid seed", seed));
        return vec![0; frag_size*frag_size*4];
    };

    let frag_size = frag_size as u64;
    let area = Area { x: fx as i64 * frag_size as i64, z: fy as i64 * frag_size as i64, w: frag_size, h: frag_size};
    //let last_layer = 43;
    //let map = cubiomes_test::call_layer(last_layer, seed, area);
    let v = biome_layers::generate_image_up_to_layer(area, seed, layer);

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
        [(color.red * 255.0) as u8, (color.green * 255.0) as u8, (color.blue * 255.0) as u8, 0xFF]
    }
}

#[cfg(feature = "wasm")]
#[js_export]
pub fn generate_fragment_slime_map(fx: i32, fy: i32, seeds: Vec<String>, frag_size: usize) -> Vec<u8> {
    let seeds: Vec<u64> = seeds.into_iter().map(|s| s.parse().unwrap_or_else(|s| {
        console!(error, format!("{} is not a valid seed", s));
        panic!("{} is not a valid seed", s);
    })).collect();

    let frag_size = frag_size as u64;
    let area = Area { x: fx as i64 * frag_size as i64, z: fy as i64 * frag_size as i64, w: frag_size, h: frag_size};
    //let last_layer = 43;
    let num_seeds = seeds.len();
    if num_seeds > (0x10000) { // 65k seeds
        console!(log, "This may take a while");
    }
    let (w, h) = (area.w as usize, area.h as usize);
    let mut map_sum = vec![0; w*h];
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
    let mut v = vec![0; w*h*4];
    for i in 0..w*h {
        let color = slime_to_color(map_sum[i], num_seeds as u32, &grad1);
        v[i*4+0] = color[0];
        v[i*4+1] = color[1];
        v[i*4+2] = color[2];
        v[i*4+3] = color[3];
    }

    v
}

#[cfg(feature = "wasm")]
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

#[cfg(feature = "wasm")]
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

#[cfg(feature = "wasm")]
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
