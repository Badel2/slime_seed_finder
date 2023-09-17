use colorgrad::{Color, CustomGradient, Gradient};
use log::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_file_reader::WebSysFile;

use slime_seed_finder::biome_info::biome_id;
use slime_seed_finder::biome_info::biome_name;
use slime_seed_finder::biome_layers;
use slime_seed_finder::biome_layers::Area;
use slime_seed_finder::biome_layers::GetMap;
use slime_seed_finder::biome_layers::Map;
use slime_seed_finder::biome_layers::MapTreasure;
use slime_seed_finder::biome_layers::PanicMap;
use slime_seed_finder::chunk::Point;
use slime_seed_finder::java_rng::JavaRng;
use slime_seed_finder::mc_rng::McRng;
use slime_seed_finder::patterns::BlockPattern;
use slime_seed_finder::patterns::BlockPatternItem;
use slime_seed_finder::seed_info;
use slime_seed_finder::seed_info::BiomeId;
use slime_seed_finder::seed_info::MinecraftVersion;
use slime_seed_finder::seed_info::SeedInfo;
use slime_seed_finder::slime::SlimeChunks;
use slime_seed_finder::*;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;

#[wasm_bindgen]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    seed_info: SeedInfo,
    range: Option<(u32, u32)>,
}

#[wasm_bindgen]
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DrawRivers {
    l43_area: Area,
    l43: Vec<u8>,
    l42_area: Area,
    l42: Vec<u8>,
}

#[wasm_bindgen]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateRiversCandidate {
    version: String,
    seed: String,
    area: Area,
}

#[wasm_bindgen]
pub fn slime_seed_finder(o: JsValue) -> String {
    let o: Options = match serde_wasm_bindgen::from_value(o) {
        Ok(o) => o,
        Err(e) => {
            return format!("Error parsing args: {}", e);
        }
    };
    debug!("Hello from Rust");
    let r = find_seed(o);

    format!("Found {} seeds!\n{:#?}", r.len(), r)
}

#[wasm_bindgen]
pub fn river_seed_finder(o: String) -> Vec<JsValue> {
    let o: Result<Options, _> = serde_json::from_str(&o);
    let o = o.unwrap();
    debug!("Hello from Rust");
    let r = find_seed_rivers(o);

    r.into_iter()
        .map(|seed| JsValue::from_str(&format!("{}", seed)))
        .collect()
}

#[wasm_bindgen]
pub fn draw_rivers(o: String) -> JsValue {
    // TODO: detect when there are two separate river areas and return a vec of maps?
    let o: Result<Options, _> = serde_json::from_str(&o);
    let o = o.unwrap();
    let (_prevoronoi_coords, hd_coords) = biome_layers::segregate_coords_prevoronoi_hd(
        o.seed_info.biomes[&BiomeId(biome_id::river)].clone(),
    );
    let area_rivers = Area::from_coords(
        o.seed_info.biomes[&BiomeId(biome_id::river)]
            .iter()
            .copied(),
    );
    let target_map = biome_layers::map_with_river_at(
        &o.seed_info.biomes[&BiomeId(biome_id::river)],
        area_rivers,
    );
    let m = biome_layers::reverse_map_voronoi_zoom(&target_map).unwrap_or_default();

    let area_hd = Area::from_coords(hd_coords.iter().copied());
    let target_map_hd = biome_layers::map_with_river_at(&hd_coords, area_hd);

    let ret = DrawRivers {
        l43_area: target_map_hd.area(),
        l43: biome_layers::draw_map_image(&target_map_hd),
        l42_area: m.area(),
        l42: biome_layers::draw_map_image(&m),
    };
    serde_wasm_bindgen::to_value(&ret)
        .unwrap_or_else(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
}

#[wasm_bindgen]
/// Returns `DrawRivers` object
pub fn generate_rivers_candidate(o: String) -> JsValue {
    let o: Result<GenerateRiversCandidate, _> = serde_json::from_str(&o);
    let o = o.unwrap();

    // TODO: only works for version 1.7
    let magic_layer_river_candidate = 141;

    let ret = DrawRivers {
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
            0,
        ),
    };
    serde_wasm_bindgen::to_value(&ret)
        .unwrap_or_else(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
}

#[wasm_bindgen]
pub fn count_candidates(o: JsValue) -> String {
    let o: Options = match serde_wasm_bindgen::from_value(o) {
        Ok(o) => o,
        Err(e) => {
            return format!("Error parsing args: {}", e);
        }
    };
    let c: Vec<_> = o.seed_info.positive.slime_chunks;
    let nc: Vec<_> = o.seed_info.negative.slime_chunks;

    if (c.len() == 0) && (nc.len() == 0) {
        return format!("{} * 2^30 candidates", 1 << 18);
    }
    let sc = SlimeChunks::new(&c, 0, &nc, 0);
    let num_cand = sc.num_low_18_candidates() as u32;
    return format!("{} * 2^30 candidates", num_cand);
}

#[wasm_bindgen]
pub fn draw_reverse_voronoi(o: JsValue) -> Vec<u8> {
    let o: Options = match serde_wasm_bindgen::from_value(o) {
        Ok(o) => o,
        Err(e) => {
            error!("Error parsing args: {}", e);
            // Return empty vec as error
            return vec![];
        }
    };
    let target_map = seed_info::biomes_to_map(o.seed_info.biomes);

    let m = biome_layers::reverse_map_voronoi_zoom(&target_map).unwrap_or_default();
    biome_layers::draw_map_image(&m)
}

#[wasm_bindgen]
pub fn extend48(s: &str) -> String {
    let mut r = vec![];
    for s in s.lines() {
        let x = match s.parse() {
            Ok(x) => {
                if x < (1u64 << 48) {
                    x
                } else {
                    let error_string = format!("Input must be lower than 2^48");
                    error!("{}", error_string);
                    return error_string;
                }
            }
            Err(e) => {
                let error_string = format!("{}", e);
                error!("{}", error_string);
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

#[wasm_bindgen]
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
        error!("Can't find seed without chunks");
        return vec![];
    }
    let sc = SlimeChunks::new(&c, 0, &nc, 0);
    let num_cand = sc.num_low_18_candidates() as u32;
    info!("Found {} * 2^30 candidates", num_cand);
    info!("ETA: about {} seconds", num_cand * 7);
    let seeds = sc.find_seed();

    {
        // Display only seeds that could be generated by java (empty box)
        let java_seeds: Vec<_> = seeds.iter().map(|&s| JavaRng::extend_long_48(s)).collect();

        info!("Java seeds: \n{:#?}", java_seeds);
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
        error!("Can't find seed without rivers");
        vec![]
    }
}

#[wasm_bindgen]
pub fn generate_fragment(
    version: String,
    fx: i32,
    fy: i32,
    seed: String,
    frag_size: usize,
    y_offset: u32,
) -> Vec<u8> {
    let empty_map_as_error = || vec![0; frag_size * frag_size * 4];
    let version1: MinecraftVersion = match version.parse() {
        Ok(s) => s,
        Err(_) => {
            if version.starts_with("TreasureMap") {
                let seed = if let Ok(s) = seed.parse() {
                    s
                } else {
                    error!("{} is not a valid seed", seed);
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
                    error!("{} is not a valid treasure map version", version);
                    return empty_map_as_error();
                };
                return biome_layers::generate_image_treasure_map(mc_version, area, seed);
            } else {
                error!("{} is not a valid version", version);
                return empty_map_as_error();
            }
        }
    };
    let num_layers = version1.num_layers();
    generate_fragment_up_to_layer(version, fx, fy, seed, frag_size, num_layers, y_offset)
}

#[wasm_bindgen]
pub fn generate_fragment_up_to_layer(
    version: String,
    fx: i32,
    fy: i32,
    seed: String,
    frag_size: usize,
    layer: u32,
    // y_offset used to render slice of 3D biome
    y_offset: u32,
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
                    error!("{} is not a valid seed", seed);
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
                    error!("{} is not a valid treasure map version", version);
                    return empty_map_as_error();
                };
                return biome_layers::generate_image_treasure_map(mc_version, area, seed);
            } else {
                error!("{} is not a valid version", version);
                return empty_map_as_error();
            }
        }
    };
    let seed = if let Ok(s) = seed.parse() {
        s
    } else {
        error!("{} is not a valid seed", seed);
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
    let v = biome_layers::generate_image_up_to_layer(version, area, seed, layer, y_offset);

    v
}

pub fn slime_to_color(id: u32, total: u32, grad1: &Gradient) -> [u8; 4] {
    assert!(id <= total);

    if id == 0 {
        // black
        [0x00, 0x00, 0x00, 0xFF]
    } else if id == total {
        // white
        [0xFF, 0xFF, 0xFF, 0xFF]
    } else {
        let color = grad1.at(id as f64 / total as f64);
        [
            (color.r * 255.0) as u8,
            (color.g * 255.0) as u8,
            (color.b * 255.0) as u8,
            (color.a * 255.0) as u8,
        ]
    }
}

#[wasm_bindgen]
pub fn generate_fragment_slime_map(
    fx: i32,
    fy: i32,
    seeds: Vec<JsValue>,
    frag_size: usize,
) -> Vec<u8> {
    let seeds = seeds.into_iter().map(|s| {
        s.as_string()
            .unwrap_or_else(|| String::new())
            .parse()
            .unwrap_or_else(|s| {
                error!("{} is not a valid seed", s);
                panic!("{} is not a valid seed", s);
            })
    });

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
        info!("This may take a while");
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

    let grad1 = CustomGradient::new()
        .colors(&[
            Color::from_rgba8(0, 51, 0, 255),
            //Color::from_rgba8(255, 255, 0, 255),
            Color::from_rgba8(0, 255, 0, 255),
        ])
        .build()
        .unwrap();
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

#[wasm_bindgen]
pub fn is_i64(seed: String) -> String {
    match seed.parse::<i64>() {
        Ok(_) => format!("OK"),
        Err(e) => format!("ERROR: {}", e.to_string()),
    }
}

#[wasm_bindgen]
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

#[wasm_bindgen]
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

#[wasm_bindgen]
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

#[wasm_bindgen]
pub fn similar_biome_seed(seed: String) -> String {
    if let Ok(s) = seed.parse::<i64>() {
        format!("{}", McRng::similar_biome_seed(s))
    } else {
        seed
    }
}

#[wasm_bindgen]
pub fn draw_treasure_map(o: String) -> Vec<u8> {
    debug!("Parsing options: {}", o);
    let o: Result<Options, _> = serde_json::from_str(&o);
    let o = o.unwrap();
    let first_treasure_map = &o.seed_info.treasure_maps[0];
    let mut pmap = Map::new(Area {
        x: -32,
        z: -32,
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
            // Unknown biome
            255 => 255,
            _ => panic!("Invalid id: {}", v),
        };
    }
    let mt = MapTreasure {
        parent: Rc::from(PanicMap),
    };

    let tmap_no_margin = mt.get_map_from_pmap(&pmap);

    // tmap_no_margin has 126x126 size but the output of this function should have 128x128 size
    let tmap = biome_layers::add_margin_to_map(&tmap_no_margin, 0);

    biome_layers::draw_treasure_map_image(&tmap)
}

#[wasm_bindgen]
pub fn treasure_map_seed_finder(o: String) -> Vec<JsValue> {
    debug!("Parsing options: {}", o);
    let o: Result<Options, _> = serde_json::from_str(&o);
    let o = o.unwrap();
    let version = o.seed_info.version.parse().unwrap();
    let first_treasure_map = &o.seed_info.treasure_maps[0];
    let mut pmap = Map::new(Area {
        x: 0,
        z: 0,
        w: 128,
        h: 128,
    });
    info!("First treasure map len: {}", first_treasure_map.map.len());
    assert_eq!(first_treasure_map.map.len(), 128 * 128);
    for (i, v) in first_treasure_map.map.iter().enumerate() {
        let (x, z) = (i % 128, i / 128);
        pmap.a[(x, z)] = match v {
            0 => biome_id::ocean,
            1 => biome_id::plains,
            2 => biome_id::river,
            // Unknown biome
            255 => 255,
            _ => panic!("Invalid id: {}", v),
        };
    }
    let r = if let Some((range_lo, range_hi)) = o.range {
        biome_layers::treasure_map_river_seed_finder(&pmap, version, range_lo, range_hi)
    } else {
        biome_layers::treasure_map_river_seed_finder(&pmap, version, 0, 1 << 24)
    };

    r.into_iter()
        .map(|seed| JsValue::from_str(&format!("{}", seed)))
        .collect()
}

#[wasm_bindgen]
pub fn anvil_region_to_river_seed_finder(
    zip_file: web_sys::File,
    is_minecraft_1_15: bool,
) -> String {
    use slime_seed_finder::anvil::ZipChunkProvider;
    // TODO: check if the input is actually a zipped_world, as it also may be a raw region file
    let wf = WebSysFile::new(zip_file);
    let mut zip_chunk_provider = ZipChunkProvider::new(wf).unwrap();
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

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtractMapResult {
    cropped_scaled_img: Vec<u8>,
    treasure_map_img: Vec<u8>,
    land_water: Vec<u8>,
}

#[wasm_bindgen]
/// Returns `Option<ExtractMapResult>`
pub fn extract_map_from_screenshot(width: u32, height: u32, screenshot: &[u8]) -> JsValue {
    let img = image::RgbaImage::from_raw(width, height, Vec::from(screenshot)).unwrap();
    let mut img = image::DynamicImage::ImageRgba8(img);
    minecraft_screenshot_parser::crosshair::remove_crosshair(&mut img);
    let detected_map = minecraft_screenshot_parser::map::detect_map(&mut img);

    if let Some(detected_map) = detected_map {
        let palette_image =
            minecraft_screenshot_parser::map_color_correct::extract_unexplored_treasure_map(
                &detected_map.cropped_scaled_img,
            );
        // Convert image::GrayScale into Map
        let palette_treasure_map = image_grayscale_into_map(palette_image);

        let treasure_map_img = biome_layers::draw_treasure_map_image(&palette_treasure_map);
        let land_water_map = biome_layers::reverse_map_treasure(&palette_treasure_map);
        // Convert land-water (only contains 0 or 1) into Vec<u8>
        let mut land_water = Vec::with_capacity(128 * 128);
        let area = land_water_map.area();
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
                let v = land_water_map.a[(x, z)];
                let v = u8::try_from(v).unwrap();
                land_water.push(v);
            }
        }

        let ret = ExtractMapResult {
            cropped_scaled_img: detected_map.cropped_scaled_img.into_bytes(),
            treasure_map_img,
            land_water,
        };
        serde_wasm_bindgen::to_value(&ret)
            .unwrap_or_else(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
    } else {
        JsValue::NULL
    }
}

fn image_grayscale_into_map(img: image::GrayImage) -> Map {
    let (w, h) = img.dimensions();
    let area = Area {
        x: -64,
        z: -64,
        w: 128,
        h: 128,
    };
    let mut m = Map::new(area);

    for x in 0..w {
        for z in 0..h {
            m.a[(x as usize, z as usize)] = i32::from(img.get_pixel(x, z).0[0]);
        }
    }

    m
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundDungeon {
    pub position: Position,
    pub kind: String,
    pub floor: Vec<String>,
}

#[wasm_bindgen]
/// Returns `Vec<FoundDungeon>`
pub fn read_dungeons(zip_file: web_sys::File) -> Vec<JsValue> {
    use slime_seed_finder::anvil::ZipChunkProvider;
    // TODO: check if the input is actually a zipped_world, as it also may be a raw region file
    let wf = WebSysFile::new(zip_file);
    let mut chunk_provider = ZipChunkProvider::new(wf).unwrap();
    let dungeons = anvil::find_dungeons(&mut chunk_provider).unwrap();
    // Convert DungeonKind to string in order to serialize it
    let dungeons: Vec<_> = dungeons
        .into_iter()
        .map(|((x, y, z), kind, floor)| FoundDungeon {
            position: Position { x, y, z },
            kind: kind.to_string(),
            floor,
        })
        .map(|found_dungeon| {
            serde_wasm_bindgen::to_value(&found_dungeon)
                .unwrap_or_else(|e| JsValue::from_str(&format!("Error serializing result: {}", e)))
        })
        .collect();

    dungeons
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindBlocksInWorldParams {
    pub block_name: String,
    pub center_position_and_chunk_radius: Option<(Position, u32)>,
    pub dimension: Option<String>,
}

#[wasm_bindgen]
/// Returns `Vec<Position>`
pub fn find_blocks_in_world(zip_file: web_sys::File, params: JsValue) -> Vec<JsValue> {
    let params: FindBlocksInWorldParams = match serde_wasm_bindgen::from_value(params) {
        Ok(x) => x,
        Err(e) => {
            error!("Failed to parse params: {}", e);
            // Return empty vector as error
            return vec![];
        }
    };
    use slime_seed_finder::anvil::ZipChunkProvider;
    // TODO: check if the input is actually a zipped_world, as it also may be a raw region file
    let wf = WebSysFile::new(zip_file);
    let mut chunk_provider =
        ZipChunkProvider::new_with_dimension(wf, params.dimension.as_deref()).unwrap();
    let blocks = anvil::find_blocks_in_world(
        &mut chunk_provider,
        &params.block_name,
        params
            .center_position_and_chunk_radius
            .map(|(position, radius)| ((position.x, position.y, position.z), radius)),
    )
    .unwrap();
    let blocks: Vec<_> = blocks
        .into_iter()
        .map(|(x, y, z)| Position { x, y, z })
        .map(|pos| {
            serde_wasm_bindgen::to_value(&pos).unwrap_or_else(|e| {
                JsValue::from_str(&format!(
                    "Failed to serialize position {:?}, error was: {}",
                    pos, e
                ))
            })
        })
        .collect();

    blocks
}

fn block_pattern_item_from_json(x: serde_json::Value) -> Result<BlockPatternItem, String> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum X {
        BlockName { block_name: String },
        Not { not: Box<X> },
        Or { or: Vec<X> },
        S(String),
    }

    impl TryFrom<X> for BlockPatternItem {
        type Error = String;

        fn try_from(x: X) -> Result<Self, String> {
            Ok(match x {
                X::BlockName { block_name } => BlockPatternItem::BlockName(block_name),
                X::Not { not } => BlockPatternItem::Not(Box::new((*not).try_into()?)),
                X::Or { or } => BlockPatternItem::Or(
                    or.into_iter()
                        .map(|p| p.try_into())
                        .collect::<Result<_, _>>()?,
                ),
                X::S(s) if s == "any" => BlockPatternItem::Any,
                X::S(s) => return Err(format!("Invalid pattern item: {:?}", s)),
            })
        }
    }

    let lx: X = serde_json::from_value(x).map_err(|e| format!("{:?}", e))?;

    lx.try_into().map_err(|e| format!("{:?}", e))
}

fn get_char_from_string(s: &str) -> Option<char> {
    if s.chars().count() != 1 {
        None
    } else {
        Some(s.chars().next().unwrap())
    }
}

fn palette_from_json(x: &serde_json::Value) -> Result<HashMap<char, BlockPatternItem>, String> {
    let x_obj = x.as_object().unwrap();
    let mut h = HashMap::new();

    for (key, value) in x_obj {
        if let Some(k) = get_char_from_string(key) {
            let v = block_pattern_item_from_json(value.clone())?;
            h.insert(k, v);
        } else {
            return Err(format!(
                "{:?} is not a valid key, must be a single characted",
                key
            ));
        }
    }

    Ok(h)
}

fn rotation_list_from_json(x: Option<&serde_json::Value>) -> Result<Vec<u8>, String> {
    if x.is_none() {
        return Ok(vec![0]);
    }
    let x = x.unwrap();
    let x_arr = x.as_array().unwrap();
    let mut rots = vec![];

    for y in x_arr {
        let y_u = y.as_u64().unwrap();
        // Only valid values are 0..=47
        if y_u >= 48 {
            return Err(format!("Invalid rotation index: {}", y_u));
        }
        rots.push(u8::try_from(y_u).unwrap());
    }

    // Remove duplicates
    rots.sort_unstable();
    rots.dedup();

    Ok(rots)
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindBlockPatternInWorldParams {
    pub pattern: String,
    pub center_position_and_chunk_radius: Option<(Position, u32)>,
    pub dimension: Option<String>,
    pub y_range: Option<(i32, i32)>,
}

#[wasm_bindgen]
/// Returns `Vec<Position>`
pub fn find_block_pattern_in_world(zip_file: web_sys::File, params: JsValue) -> Vec<JsValue> {
    let params: FindBlockPatternInWorldParams = match serde_wasm_bindgen::from_value(params) {
        Ok(x) => x,
        Err(e) => {
            error!("Failed to parse params: {}", e);
            // Return empty vector as error
            return vec![];
        }
    };
    let pattern_js_value: serde_json::Value = serde_json::from_str(&params.pattern).unwrap();
    let palette = palette_from_json(&pattern_js_value["palette"]).unwrap();
    let map_str = &pattern_js_value["map"];
    let rotations = pattern_js_value.get("rotations");
    let block_pattern = BlockPattern {
        palette,
        map: slime_seed_finder::patterns::parse_block_pattern_map(map_str.as_str().unwrap())
            .unwrap(),
        rotations: rotation_list_from_json(rotations).unwrap(),
    };
    use slime_seed_finder::anvil::ZipChunkProvider;
    // TODO: check if the input is actually a zipped_world, as it also may be a raw region file
    let wf = WebSysFile::new(zip_file);
    let mut chunk_provider =
        ZipChunkProvider::new_with_dimension(wf, params.dimension.as_deref()).unwrap();
    let blocks = anvil::find_block_pattern_in_world(
        &mut chunk_provider,
        &block_pattern.compile(),
        params
            .center_position_and_chunk_radius
            .map(|(position, radius)| ((position.x, position.y, position.z), radius)),
        params.y_range.map(|(y_min, y_max)| y_min..=y_max),
    )
    .unwrap();
    let blocks: Vec<_> = blocks
        .into_iter()
        .map(|(x, y, z)| Position { x, y, z })
        .map(|pos| {
            serde_wasm_bindgen::to_value(&pos).unwrap_or_else(|e| {
                JsValue::from_str(&format!(
                    "Failed to serialize position {:?}, error was: {}",
                    pos, e
                ))
            })
        })
        .collect();

    blocks
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindMultiDungeonsParams {
    pub center_position_and_chunk_radius: Option<(Position, u32)>,
    pub dimension: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundDungeon1 {
    pub position: Position,
    pub kind: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FloatPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FindMultiDungeonsOutput {
    // Optimal standing position for farming
    pub optimal_position: FloatPosition,
    // Dungeons in radius of position
    pub spawners: Vec<FoundDungeon1>,
}

#[wasm_bindgen]
/// Returns `Vec<FindMultiDungeonsOutput>`
pub fn find_spawners_in_world(zip_file: web_sys::File, params: JsValue) -> Vec<JsValue> {
    let params: FindMultiDungeonsParams = match serde_wasm_bindgen::from_value(params) {
        Ok(x) => x,
        Err(e) => {
            error!("Failed to parse params: {}", e);
            // Return empty vector as error
            return vec![];
        }
    };
    use slime_seed_finder::anvil::ZipChunkProvider;
    // TODO: check if the input is actually a zipped_world, as it also may be a raw region file
    let wf = WebSysFile::new(zip_file);
    let mut chunk_provider =
        ZipChunkProvider::new_with_dimension(wf, params.dimension.as_deref()).unwrap();
    let blocks = anvil::find_spawners_in_world(
        &mut chunk_provider,
        params
            .center_position_and_chunk_radius
            .map(|(position, radius)| ((position.x, position.y, position.z), radius)),
    )
    .unwrap();
    let blocks: Vec<_> = blocks
        .into_iter()
        .map(
            |anvil::FindMultiSpawnersOutput {
                 optimal_position,
                 spawners,
             }| FindMultiDungeonsOutput {
                optimal_position: FloatPosition {
                    x: optimal_position.x,
                    y: optimal_position.y,
                    z: optimal_position.z,
                },
                spawners: spawners
                    .into_iter()
                    .map(|(pos, kind)| FoundDungeon1 {
                        position: Position {
                            x: pos.0,
                            y: pos.1,
                            z: pos.2,
                        },
                        kind,
                    })
                    .collect(),
            },
        )
        .map(|x| {
            serde_wasm_bindgen::to_value(&x).unwrap_or_else(|e| {
                JsValue::from_str(&format!("Failed to serialize result: {}", e))
            })
        })
        .collect();

    blocks
}

#[derive(Debug, Serialize)]
pub struct NbtSearchResult {
    filename: String,
    nbt_path: String,
}

#[wasm_bindgen]
/// Returns `Vec<NbtSearchResult>`
pub fn nbt_search(_zip_file: web_sys::File, _block_name: &str) -> Vec<JsValue> {
    unimplemented!("nbt search not supported yet")
}

#[wasm_bindgen]
/// Returns HashMap<String, i32>
pub fn get_color_to_biome_map() -> JsValue {
    let rgba_to_biome = biome_layers::color_to_biome_map();

    let ret: HashMap<String, i32> = rgba_to_biome
        .into_iter()
        .map(|(rgba, biome_id)| {
            let [r, g, b, _a] = rgba;
            // Convert color [r, g, b, a] into #rrggbb
            let color_string = format!("#{:02x}{:02x}{:02x}", r, g, b);

            (color_string, biome_id)
        })
        .collect();
    serde_wasm_bindgen::to_value(&ret)
        .unwrap_or_else(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
}

#[wasm_bindgen]
/// Returns HashMap<String, String>
pub fn get_biome_id_to_biome_name_map() -> JsValue {
    let num_biomes = 256;
    let mut h = HashMap::with_capacity(usize::try_from(num_biomes).unwrap());

    for biome_id in 0..num_biomes {
        if let Some(name) = biome_name(biome_id) {
            h.insert(biome_id.to_string(), name.to_string());
        }
    }

    let ret = h;
    serde_wasm_bindgen::to_value(&ret)
        .unwrap_or_else(|e| JsValue::from_str(&format!("Failed to serialize result: {}", e)))
}

#[wasm_bindgen]
pub fn read_fragment_biome_map(
    zip_file: web_sys::File,
    version_str: String,
    fx: i32,
    fy: i32,
    frag_size: usize,
    y_offset: u32,
) -> Vec<u8> {
    use slime_seed_finder::anvil::ZipChunkProvider;
    let version: MinecraftVersion = match version_str.parse() {
        Ok(s) => s,
        Err(e) => {
            error!("{:?} is not a valid version: {}", version_str, e);
            // Return empty vec as error
            return vec![];
        }
    };
    // TODO: check if the input is actually a zipped_world, as it also may be a raw region file
    let wf = WebSysFile::new(zip_file);
    let mut zip_chunk_provider = ZipChunkProvider::new(wf).unwrap();

    let frag_size = frag_size as u64;
    let area = Area {
        x: fx as i64 * frag_size as i64,
        z: fy as i64 * frag_size as i64,
        w: frag_size,
        h: frag_size,
    };

    let biomes;
    // TODO: assuming that version >= 1.15
    match version {
        MinecraftVersion::Java1_15
        | MinecraftVersion::Java1_16
        | MinecraftVersion::Java1_16_1
        | MinecraftVersion::Java1_17 => {
            biomes = anvil::get_biomes_from_area_1_15(&mut zip_chunk_provider, area, y_offset);
        }
        MinecraftVersion::Java1_18 => {
            // Convert offset into level: offset goes from [0, 95], level goes from [-64, 319]
            let y_level: i64 = -64 + y_offset as i64 * 4;
            biomes = anvil::get_biomes_from_area_1_18(&mut zip_chunk_provider, area, y_level);
        }
        _ => {
            error!("Version {:?} is not supported", version_str);
            // Return empty vec as error
            return vec![];
        }
    }

    let mut map = Map::from_area_fn(area, |(_, _)| biome_info::UNKNOWN_BIOME_ID);
    for (expected_biome_id, p) in &biomes {
        if area.contains(p.x, p.z) {
            map.set(p.x, p.z, expected_biome_id.0);
        } else {
            // TODO: print error when this is fixed
        }
    }

    biome_layers::draw_map_image(&map)
}
