use crate::mc_rng::McRng;
use crate::xoroshiro128plusplus::Xoroshiro128PlusPlus;
use crate::noise_generator::NoiseGeneratorPerlin;
use crate::noise_generator::NoiseGeneratorDoublePerlin128;
use crate::seed_info::BiomeId;
use crate::seed_info::MinecraftVersion;
use log::debug;
use ndarray::Array2;
use ndarray::Array3;
use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;
use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;
use crate::java_rng::JavaRng;
use crate::chunk::Point;
use crate::chunk::Point2;
use crate::chunk::Point4;
use crate::chunk::Point3D;
use crate::chunk::Point3D4;
use crate::biome_info::biome_id;
use crate::biome_info::BIOME_COLORS;
use crate::biome_info::BIOME_INFO;
use crate::biome_info::UNKNOWN_BIOME_ID;
use crate::spline::Spline;
use crate::climate::Climate;

// The different Map* layers are copied from
// https://github.com/Cubitect/cubiomes
// since it's easier to translate C to Rust than Java to Rust.

/// Hash function used by MapVoronoiZoom since Minecraft 1.15
pub fn sha256_long_to_long(x: i64) -> i64 {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    // Add long in little endian format
    hasher.update(x.to_le_bytes());
    let r = hasher.finalize();

    // Output the first 8 bytes of the hash interpreted as a little endian i64
    // The output of Sha256 is 32 bytes, so this cannot fail
    i64::from_le_bytes(r[..8].try_into().unwrap())
}

/// Full range: range_lo = 0, range_hi = u32::max_value()
pub fn seed_hash_bruteforce_26_java_range(expected_hash: i64, candidates_26: &[u64], range_lo: u32, range_hi: u32) -> Option<i64> {
    for candidate_26 in candidates_26 {
        for hi in range_lo..=range_hi {
            for lo in 0..(1 << 6) {
                let seed = ((hi as u64) << 32) | ((lo as u64) << 26) | candidate_26;
                if JavaRng::create_from_long(seed).is_none() {
                    continue;
                }
                let seed = seed as i64;
                if sha256_long_to_long(seed) == expected_hash {
                    debug!("Found seed: {} with hash {}", seed, expected_hash);
                    return Some(seed);
                }
            }
        }
    }

    None
}

/// Full range: range_lo = 0, range_hi = u32::max_value()
pub fn seed_hash_bruteforce_26_range(expected_hash: i64, candidates_26: &[u64], range_lo: u32, range_hi: u32) -> Option<i64> {
    for candidate_26 in candidates_26 {
        for hi in range_lo..=range_hi {
            for lo in 0..(1 << 6) {
                let seed = ((hi as u64) << 32) | ((lo as u64) << 26) | candidate_26;
                let seed = seed as i64;
                if sha256_long_to_long(seed) == expected_hash {
                    debug!("Found seed: {} with hash {}", seed, expected_hash);
                    return Some(seed);
                }
            }
        }
    }

    None
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Area {
    pub x: i64,
    pub z: i64,
    pub w: u64,
    pub h: u64,
}

impl Area {
    /// Returns true if (x, z) in inside this area
    pub fn contains(&self, x: i64, z: i64) -> bool {
        x >= self.x && x < self.x + self.w as i64 && z >= self.z && z < self.z + self.h as i64
    }

    /// Creates the smallest area that will contain all the coords
    pub fn from_coords<I>(c: I) -> Area
    where
        I: IntoIterator<Item = Point>
    {
        let mut c = c.into_iter();
        let c0 = c.next();
        if c0.is_none() {
            // On empty coords, return empty area
            return Area { x: 0, z: 0, w: 0, h: 0 }
        }

        let c0 = c0.unwrap();
        let Point { x: mut x_min, z: mut z_min } = c0;
        let Point { x: mut x_max, z: mut z_max } = c0;

        for Point {x, z} in c {
            use std::cmp::{min, max};
            x_min = min(x_min, x);
            z_min = min(z_min, z);
            x_max = max(x_max, x);
            z_max = max(z_max, z);
        }

        Area { x: x_min, z: z_min, w: (x_max - x_min + 1) as u64, h: (z_max - z_min + 1) as u64 }
    }

    /// Creates the smallest area that will contain all the coords
    pub fn from_coords2<I>(c: I) -> Area
    where
        I: IntoIterator<Item = Point2>
    {
        let mut c = c.into_iter();
        let c0 = c.next();
        if c0.is_none() {
            // On empty coords, return empty area
            return Area { x: 0, z: 0, w: 0, h: 0 }
        }

        let c0 = c0.unwrap();
        let Point2 { x: mut x_min, z: mut z_min } = c0;
        let Point2 { x: mut x_max, z: mut z_max } = c0;

        for Point2 {x, z} in c {
            use std::cmp::{min, max};
            x_min = min(x_min, x);
            z_min = min(z_min, z);
            x_max = max(x_max, x);
            z_max = max(z_max, z);
        }

        Area { x: x_min, z: z_min, w: (x_max - x_min + 1) as u64, h: (z_max - z_min + 1) as u64 }
    }

    /// Creates the smallest area that will contain all the coords
    pub fn from_coords4<I>(c: I) -> Area
    where
        I: IntoIterator<Item = Point4>
    {
        let mut c = c.into_iter();
        let c0 = c.next();
        if c0.is_none() {
            // On empty coords, return empty area
            return Area { x: 0, z: 0, w: 0, h: 0 }
        }

        let c0 = c0.unwrap();
        let Point4 { x: mut x_min, z: mut z_min } = c0;
        let Point4 { x: mut x_max, z: mut z_max } = c0;

        for Point4 {x, z} in c {
            use std::cmp::{min, max};
            x_min = min(x_min, x);
            z_min = min(z_min, z);
            x_max = max(x_max, x);
            z_max = max(z_max, z);
        }

        Area { x: x_min, z: z_min, w: (x_max - x_min + 1) as u64, h: (z_max - z_min + 1) as u64 }
    }

    pub fn intersects(&self, other: &Area) -> bool {
        // Rect intersection from stackoverflow
        // https://stackoverflow.com/a/306379
        fn value_in_range<T: PartialOrd>(value: T, min: T, max: T) -> bool {
            value >= min && value <= max
        }

        let x_overlap = value_in_range(self.x, other.x, other.x + other.w as i64) ||
                    value_in_range(other.x, self.x, self.x + self.w as i64);

        let z_overlap = value_in_range(self.z, other.z, other.z + other.h as i64) ||
                        value_in_range(other.z, self.z, self.z + self.h as i64);

        x_overlap && z_overlap
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Area3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub sx: u64,
    pub sy: u64,
    pub sz: u64,
}

impl Area3D {
    /// Creates the smallest area that will contain all the coords
    pub fn from_coords4<I>(c: I) -> Area3D
    where
        I: IntoIterator<Item = Point3D4>
    {
        let mut c = c.into_iter();
        let c0 = c.next();
        if c0.is_none() {
            // On empty coords, return empty area
            return Area3D { x: 0, y: 0, z: 0, sx: 0, sy: 0, sz: 0 };
        }

        let c0 = c0.unwrap();
        let Point3D4 { x: mut x_min, y: mut y_min, z: mut z_min } = c0;
        let Point3D4 { x: mut x_max, y: mut y_max, z: mut z_max } = c0;

        for Point3D4 {x, y, z} in c {
            use std::cmp::{min, max};
            x_min = min(x_min, x);
            y_min = min(y_min, y);
            z_min = min(z_min, z);
            x_max = max(x_max, x);
            y_max = max(y_max, y);
            z_max = max(z_max, z);
        }

        Area3D { x: x_min, y: y_min, z: z_min, sx: (x_max - x_min + 1) as u64, sy: (y_max - y_min + 1) as u64, sz: (z_max - z_min + 1) as u64 }
    }

    /// Create 3D area from 2D area and a single y level
    pub fn from_area2d_and_y_level(area: Area, y_level: i64) -> Area3D {
        Self {
            x: area.x,
            y: y_level,
            z: area.z,
            sx: area.w,
            sy: 1,
            sz: area.h,
        }
    }

    /// If the y dimension of this area is 1, convert it into a 2D area
    pub fn into_area2d(self) -> Area {
        assert_eq!(self.sy, 1);

        Area {
            x: self.x,
            z: self.z,
            w: self.sx,
            h: self.sz,
        }
    }
}


#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Map {
    pub x: i64,
    pub z: i64,
    pub a: Array2<i32>,
}

impl Map {
    pub fn new(a: Area) -> Self {
        Self { x: a.x, z: a.z, a: Array2::zeros((a.w as usize, a.h as usize)) }
    }
    /// Create map from generator function
    pub fn from_area_fn<F: FnMut((usize, usize)) -> i32>(a: Area, f: F) -> Self {
        Self { x: a.x, z: a.z, a: Array2::from_shape_fn((a.w as usize, a.h as usize), f) }
    }
    pub fn area(&self) -> Area {
        let (w, h) = self.a.dim();
        Area { x: self.x, z: self.z, w: w as u64, h: h as u64 }
    }
    /// Get value at real coordinate (x, z)
    pub fn get(&self, real_x: i64, real_z: i64) -> i32 {
        self.a[((real_x - self.x) as usize, (real_z - self.z) as usize)]
    }
    /// Set value at real coordinate (x, z)
    pub fn set(&mut self, real_x: i64, real_z: i64, value: i32) {
        self.a[((real_x - self.x) as usize, (real_z - self.z) as usize)] = value;
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SparseMap {
    pub x: i64,
    pub z: i64,
    pub a: Array2<Option<i32>>,
}

impl SparseMap {
    pub fn new(a: Area) -> Self {
        Self { x: a.x, z: a.z, a: Array2::default((a.w as usize, a.h as usize)) }
    }
    pub fn area(&self) -> Area {
        let (w, h) = self.a.dim();
        Area { x: self.x, z: self.z, w: w as u64, h: h as u64 }
    }
    pub fn unwrap_or(self, unknown_biome_id: i32) -> Map {
        let a = self.a.map(|x| x.unwrap_or(unknown_biome_id));
        Map {
            x: self.x,
            z: self.z,
            a,
        }
    }
}

impl From<Map> for SparseMap {
    fn from(m: Map) -> Self {
        let a = m.a.map(|x| Some(*x));

        Self {
            x: m.x,
            z: m.z,
            a,
        }
    }
}

pub struct CachedMap {
    pub parent: Rc<dyn GetMap>,
    pub cache: RefCell<HashMap<(i64, i64), i32>>,
}

impl CachedMap {
    fn new(parent: Rc<dyn GetMap>) -> Self {
        Self {
            parent, cache: Default::default()
        }
    }
    fn insert_into_cache(&self, m: &Map) {
        let area = m.area();
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                self.cache.borrow_mut().insert((area.x + x as i64, area.z + z as i64), m.a[(x, z)]);
            }
        }
    }
    fn get_all_from_cache(&self, area: Area) -> Option<Map> {
        let mut m = Map::new(area);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                if let Some(b) = self.cache.borrow().get(&(area.x + x as i64, area.z + z as i64)) {
                    m.a[(x, z)] = *b;
                } else {
                    return None;
                }
            }
        }

        Some(m)
    }
}

impl GetMap for CachedMap {
    fn get_map(&self, area: Area) -> Map {
        if let Some(m) = self.get_all_from_cache(area) {
            m
        } else {
            let m = self.parent.get_map(area);
            self.insert_into_cache(&m);

            m
        }
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let area = pmap.area();

        self.get_map(area)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Map3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub a: Array3<i32>,
}

impl Map3D {
    pub fn new(a: Area3D) -> Self {
        Self { x: a.x, y: a.y, z: a.z, a: Array3::zeros((a.sx as usize, a.sy as usize, a.sz as usize)) }
    }
    /// Create map from generator function
    pub fn from_area_fn<F: FnMut((usize, usize, usize)) -> i32>(a: Area3D, f: F) -> Self {
        Self { x: a.x, y: a.y, z: a.z, a: Array3::from_shape_fn((a.sx as usize, a.sy as usize, a.sz as usize), f) }
    }
    pub fn area(&self) -> Area3D {
        let (sx, sy, sz) = self.a.dim();
        Area3D { x: self.x, y: self.y, z: self.z, sx: sx as u64, sy: sy as u64, sz: sz as u64 }
    }
    /// Get value at real coordinate (x, y, z)
    pub fn get(&self, real_x: i64, real_y: i64, real_z: i64) -> i32 {
        self.a[((real_x - self.x) as usize, (real_y - self.y) as usize, (real_z - self.z) as usize)]
    }
    /// Set value at real coordinate (x, y, z)
    pub fn set(&mut self, real_x: i64, real_y: i64, real_z: i64, value: i32) {
        self.a[((real_x - self.x) as usize, (real_y - self.y) as usize, (real_z - self.z) as usize)] = value;
    }

    /// If the y dimension of this map is 1, convert it into a 2D map
    pub fn into_map2d(self) -> Map {
        let area = self.area();
        assert_eq!(area.sy, 1, "Cannot convert 3D map into 2D map because y size is {} instead of 1", area.sy);

        let mut m = Map::new(area.into_area2d());

        for i in 0..area.sx {
            for j in 0..area.sz {
                let (i, j) = (i as usize, j as usize);
                m.a[(i, j)] = self.a[(i, 0, j)];
            }
        }

        m
    }

    /// Convert 3D map into 2D map, by taking slices at every y level and concatenating the result
    /// along the z axis.
    pub fn flatten_into_2d(self) -> Map {
        let area = self.area();

        let mut m = Map::new(area.into_area2d());

        for k in 0..area.sy {
            for i in 0..area.sx {
                for j in 0..area.sz {
                    let (i, j, k) = (i as usize, j as usize, k as usize);
                    m.a[(i, j + k * area.sx as usize * area.sz as usize)] = self.a[(i, k, j)];
                }
            }
        }

        m
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Biome {
    pub id: i32,
    pub type_0: i32,
    pub height: f64,
    pub temp: f64,
    pub tempCat: i32,
}

#[deprecated = "use get_category instead"]
fn get_biome_type(id: i32) -> i32 {
    BIOME_INFO[id as usize].type_0
}
fn get_category(v: MinecraftVersion, id: i32) -> Option<i32> {
    use biome_id::*;

    let category = match id {
        ocean | frozenOcean | deepOcean | warmOcean | lukewarmOcean | coldOcean | warmDeepOcean | lukewarmDeepOcean | coldDeepOcean | frozenDeepOcean => Ocean,
        plains | sunflowerPlains => Plains,
        desert | desertHills | desertLakes => Desert,
        extremeHills | extremeHillsEdge | extremeHillsPlus | gravellyMountains | modifiedGravellyMountains => Hills,
        forest | forestHills | birchForest | birchForestHills | roofedForest | flowerForest | tallBirchForest | tallBirchHills | darkForestHills => Forest,
        taiga | taigaHills | coldTaiga | coldTaigaHills | megaTaiga | megaTaigaHills | taigaMountains | snowyTaigaMountains | giantSpruceTaiga | giantSpruceTaigaHills => Taiga,
        swampland | swampHills => Swamp,
        river | frozenRiver => River,
        hell => Hell,
        sky | skyIslandLow | skyIslandMedium | skyIslandHigh | skyIslandBarren => Sky,
        icePlains | iceMountains | iceSpikes => Snow,
        mushroomIsland | mushroomIslandShore => MushroomIsland,
        beach | coldBeach => Beach,
        jungle | jungleHills | jungleEdge | modifiedJungle | modifiedJungleEdge | bambooJungle | bambooJungleHills => Jungle,
        stoneBeach => StoneBeach,
        savanna | savannaPlateau | shatteredSavanna | shatteredSavannaPlateau => Savanna,
        mesa | erodedBadlands | modifiedWoodedBadlandsPlateau | modifiedBadlandsPlateau => Mesa,
        mesaPlateau_F | mesaPlateau => if v >= MinecraftVersion::Java1_16 { mesaPlateau } else { Mesa },
        _ => return None,
    };

    Some(category)
}
fn biome_exists(id: i32) -> bool {
    if id <= 0xFF {
        BIOME_INFO[id as usize].id & (!0xFF) == 0
    } else {
        false
    }
}
fn get_mutated(v: MinecraftVersion, id: i32) -> Option<i32> {
    use biome_id::*;

    if v == MinecraftVersion::Java1_9 {
        // Simulate https://bugs.mojang.com/browse/MC-98995
        if id == birchForest {
            return Some(id + 129);
        }
        if id == birchForestHills {
            return None;
        }
    }

    if biome_exists(id + 128) {
        Some(id + 128)
    } else {
        None
    }
}
pub fn is_oceanic(id: i32) -> bool {
    use biome_id::*;
    match id {
        ocean
        | deepOcean
        | warmOcean
        | warmDeepOcean
        | lukewarmOcean
        | lukewarmDeepOcean
        | coldOcean
        | coldDeepOcean
        | frozenOcean
        | frozenDeepOcean
        => true,
        _ => false
    }
}
fn is_mesa(id: i32) -> bool {
    use biome_id::*;
    match id {
        mesa | mesaPlateau_F | mesaPlateau | erodedBadlands | modifiedWoodedBadlandsPlateau | modifiedBadlandsPlateau => true,
        _ => false,
    }
}
fn is_biome_JFTO(id: i32) -> bool {
    use biome_id::*;
    biome_exists(id) && (get_category(MinecraftVersion::Java1_16, id) == Some(Jungle) || id == forest || id == taiga || is_oceanic(id))
}

fn is_biome_snowy(id: i32) -> bool {
    biome_exists(id) && BIOME_INFO[(id&0xff) as usize].temp < 0.1
}
pub fn biome_to_color(id: i32) -> [u8; 4] {
    let mut id = id as usize;

    if id > 255 {
        // Invalid biome but proceed anyway
        id &= 0xFF;
    }

    if (174..(174+20)).contains(&id) {
        // 1.18 biomes are not present in BIOME_COLORS map, hardcode them here for the moment
        // Colors from cubiomes util.c
        /*
        174 => "DripstoneCaves",
        175 => "FrozenPeaks",
        176 => "Grove",
        177 => "JaggedPeaks",
        178 => "LushCaves",
        179 => "Meadow",
        180 => "NetherWastes",
        181 => "OldGrowthBirchForest",
        182 => "OldGrowthPineTaiga",
        183 => "OldGrowthSpruceTaiga",
        184 => "SnowyPlains",
        185 => "SnowySlopes",
        186 => "SparseJungle",
        187 => "StonyPeaks",
        188 => "StonyShore",
        189 => "WindsweptForest",
        190 => "WindsweptGravellyHills",
        191 => "WindsweptHills",
        192 => "WindsweptSavanna",
        193 => "WoodedBadlands",
        */
        let [r, g, b] = match id {
            174 => [78, 48, 18],
            175 => [176, 179, 206],
            176 => [71, 114, 108],
            177 => [220, 220, 200],
            178 => [40, 60, 0],
            179 => [96, 164, 69],
            180 => [87, 37, 38],
            181 => [0x4f, 0x6c, 0x56],
            182 => [0x48, 0x65, 0x5f],
            183 => [0x38, 0x58, 0x4f],
            184 => [0xc0, 0xd2, 0xb0],
            185 => [196, 196, 196],
            186 => [0x38, 0x55, 0x04],
            187 => [123, 143, 116],
            188 => [0x71, 0x71, 0x7b],
            189 => [0x53, 0x4d, 0x48],
            190 => [0x43, 0x44, 0x43],
            191 => [0x59, 0x55, 0x53],
            192 => [0x7e, 0x79, 0x58],
            193 => [0x83, 0x38, 0x06],
            // Give unique colors to unknown biomes, so they can be shown as "Biome #id" in the web
            // demo
            id => [1, 255, id as u8],
        };
        return [r, g, b, 255];
    }

    let (r, g, b);
    if id >= 128 && id <= 167 {
        r = BIOME_COLORS[id][0].saturating_add(40);
        g = BIOME_COLORS[id][1].saturating_add(40);
        b = BIOME_COLORS[id][2].saturating_add(40);
    } else {
        r = BIOME_COLORS[id][0];
        g = BIOME_COLORS[id][1];
        b = BIOME_COLORS[id][2];
    }

    [r, g, b, 255]
}

pub fn color_to_biome_map() -> HashMap<[u8; 4], i32> {
    let num_biomes = 256;
    let mut h = HashMap::with_capacity(num_biomes);

    for biome_id in 0..num_biomes {
        let biome_id = i32::try_from(biome_id).unwrap();
        let color = biome_to_color(biome_id);
        //let [r, g, b, _a] = color;
        // Convert color [r, g, b, a] into #rrggbb
        //let color_string = format!("#{:02x}{:02x}{:02x}", r, g, b);
        h.insert(color, biome_id);
    }

    h
}

/*
type LayerFn = fn(l: &Layer) -> Vec<i32>;

#[derive(Clone, Debug)]
struct Layer {
    baseSeed: i64,
    worldSeed: i64,
    chunkSeed: i64,
    //oceanRnd
    scale: u32,
    //getMap: LayerFn,
    p: Option<Rc<Layer>>,
    p2: Option<Rc<Layer>>,
}
*/

pub trait GetMap {
    fn get_map(&self, area: Area) -> Map;
    fn get_map_from_pmap(&self, pmap: &Map) -> Map;
}

pub trait GetMap3D {
    fn get_map_3d(&self, area: Area3D) -> Map3D;
    fn get_map_from_pmap_3d(&self, pmap: &Map3D) -> Map3D;
}

/// Convert a 3D map into a 2D map by setting a fixed y value
pub struct Map3DToMap2D<T: GetMap3D> {
    pub map_3d: T,
    pub y_level: i64,
}

impl<T: GetMap3D> GetMap for Map3DToMap2D<T> {
    fn get_map(&self, area: Area) -> Map {
        let area_3d = Area3D::from_area2d_and_y_level(area, self.y_level);
        let map3d = self.map_3d.get_map_3d(area_3d);
        map3d.into_map2d()
    }

    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        panic!("3D map should never use pmap");
    }
}

impl GetMap3D for Box<dyn GetMap3D> {
    fn get_map_3d(&self, area: Area3D) -> Map3D {
        (**self).get_map_3d(area)
    }

    fn get_map_from_pmap_3d(&self, pmap: &Map3D) -> Map3D {
        (**self).get_map_from_pmap_3d(pmap)
    }
}

// Test layer which always generates a map consisting of only zeros.
// To be used as a parent for testing.
pub struct TestMapZero;

impl GetMap for TestMapZero {
    fn get_map(&self, area: Area) -> Map {
        Map::new(area)
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let area = pmap.area();

        self.get_map(area)
    }
}

pub struct TestMapCheckers;

impl GetMap for TestMapCheckers {
    fn get_map(&self, area: Area) -> Map {
        let colors = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        MapFn(|Point {x, z}| {
            let rx = x as usize % 4;
            let rz = z as usize % 4;

            colors[rz * 4 + rx]
        }).get_map(area)
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let area = pmap.area();

        self.get_map(area)
    }
}

pub struct TestMapXhz;

impl GetMap for TestMapXhz {
    fn get_map(&self, area: Area) -> Map {
        MapFn(|Point {x, z}| {
            (x.wrapping_mul(area.h as i64) + z) as i32
        }).get_map(area)
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let area = pmap.area();

        self.get_map(area)
    }
}

/// A map which panics when used.
pub struct PanicMap;

impl GetMap for PanicMap {
    fn get_map(&self, area: Area) -> Map {
        panic!("NoMap called with area {:?}", area);
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        panic!("NoMap called with pmap {:?}", pmap);
    }
}

/// Generetes a map given a function which takes (x, z) coordinates
pub struct MapFn<F: Fn(Point) -> i32>(F);

impl<F: Fn(Point) -> i32> GetMap for MapFn<F> {
    fn get_map(&self, area: Area) -> Map {
        Map::from_area_fn(area, |(x, z)| {
            (self.0)(Point { x: area.x + x as i64, z: area.z + z as i64 })
        })
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let area = pmap.area();

        self.get_map(area)
    }
}

/// A map which applies a function to its parent map, depending on the coordinates
/// f = |x, z, parent_value_at_x_z| { new_value_at_x_z };
pub struct MapParentFn<P: GetMap, F: Fn(i64, i64, i32) -> i32>(P, F);

impl<F: Fn(i64, i64, i32) -> i32, P: GetMap> GetMap for MapParentFn<P, F> {
    fn get_map(&self, area: Area) -> Map {
        let pmap = self.0.get_map(area);
        self.get_map_from_pmap(&pmap)
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let area = pmap.area();
        let mut m = Map::new(area);
        for x in 0..area.w {
            for z in 0..area.h {
                m.a[(x as usize, z as usize)] = (self.1)(area.x + x as i64, area.z + z as i64, pmap.a[(x as usize, z as usize)]);
            }
        }

        m
    }
}

// A map which applies a function to its parent map
// This is just a MapParentFn with f = |x, z, p| p
pub struct MapMap {
    pub parent: Rc<dyn GetMap>,
    pub f: fn(i32) -> i32,
}

impl GetMap for MapMap {
    fn get_map(&self, area: Area) -> Map {
        let pmap = self.parent.get_map(area);
        self.get_map_from_pmap(&pmap)
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let mut m = pmap.clone();
        m.a.mapv_inplace(self.f);
        m
    }
}

// A map which applies a function to its two parent maps
pub struct MapMap2 {
    pub parent1: Rc<dyn GetMap>,
    pub parent2: Rc<dyn GetMap>,
    pub f: fn(i32, i32) -> i32,
}

impl GetMap for MapMap2 {
    fn get_map(&self, area: Area) -> Map {
        let mut pmap1 = self.parent1.get_map(area);
        let pmap2 = self.parent2.get_map(area);

        pmap1.a.zip_mut_with(&pmap2.a, |a, b| *a = (self.f)(*a, *b));

        pmap1
    }
    fn get_map_from_pmap(&self, _pmap: &Map) -> Map {
        panic!("MapMap2 requires 2 pmaps!");
    }
}

pub struct MapHalfVoronoiZoom {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapHalfVoronoiZoom {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

// TODO: tests
impl GetMap for MapHalfVoronoiZoom {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x >> 1,
                z: area.z >> 1,
                w: (area.w >> 1) + 2,
                h: (area.h >> 1) + 2
            };
            let pmap = parent.get_map(parea);

            let mut map = self.get_map_from_pmap(&pmap);
            // TODO: is this correct?
            let (nx, nz) = ((area.x + 1) & 1, (area.z + 1) & 1);
            map.x -= nx;
            map.z -= nz;
            let (nx, nz) = (nx as usize, nz as usize);
            map.a.slice_collapse(s![
                    nx..nx + area.w as usize,
                    nz..nz + area.h as usize
            ]);

            map
        } else {
            panic!("Parent not set");
        }
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        // Naive implementation: apply MapVoronoiZoom and rescale
        let vmap = MapVoronoiZoom::new(self.base_seed, self.world_seed).get_map_from_pmap(pmap);
        // Scale from 1:1 to 2:1
        let varea = vmap.area();
        let marea = Area {
            x: varea.x >> 1,
            z: varea.z >> 1,
            w: varea.w >> 1,
            h: varea.h >> 1,
        };
        let mut m = Map::new(marea);
        for x in 0..marea.w as usize {
            for z in 0..marea.h as usize {
                // TODO: check if we need offset here
                m.a[(x, z)] = vmap.a[(x * 2, z * 2)];
            }
        }

        m
    }
}

pub struct MapHalfVoronoiZoom115 {
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapHalfVoronoiZoom115 {
    pub fn new(world_seed: i64) -> Self {
        Self { world_seed, parent: None }
    }
}

// TODO: tests
impl GetMap for MapHalfVoronoiZoom115 {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x >> 1,
                z: area.z >> 1,
                w: (area.w >> 1) + 2,
                h: (area.h >> 1) + 2
            };
            let pmap = parent.get_map(parea);

            let mut map = self.get_map_from_pmap(&pmap);
            // TODO: is this correct?
            let (nx, nz) = ((area.x + 1) & 1, (area.z + 1) & 1);
            map.x -= nx;
            map.z -= nz;
            let (nx, nz) = (nx as usize, nz as usize);
            map.a.slice_collapse(s![
                    nx..nx + area.w as usize,
                    nz..nz + area.h as usize
            ]);

            map
        } else {
            panic!("Parent not set");
        }
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        // Naive implementation: apply MapVoronoiZoom and rescale
        let vmap = MapVoronoiZoom115::new(self.world_seed).get_map_from_pmap(pmap);
        // Scale from 1:1 to 2:1
        let varea = vmap.area();
        let marea = Area {
            x: varea.x >> 1,
            z: varea.z >> 1,
            w: varea.w >> 1,
            h: varea.h >> 1,
        };
        let mut m = Map::new(marea);
        for x in 0..marea.w as usize {
            for z in 0..marea.h as usize {
                // TODO: check if we need offset here
                m.a[(x, z)] = vmap.a[(x * 2, z * 2)];
            }
        }

        m
    }
}

pub struct MapVoronoiZoom {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapVoronoiZoom {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

// TODO: tests
impl GetMap for MapVoronoiZoom {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            // TODO: Area::double(), Area::quadruple(), etc
            // Example 1:
            // area  1x1: we want to generate 1x1
            // parea 2x2: we need 2x2 from the previous layer
            // narea 4x4: instead of 8x8, we only zoom the top left corner
            // now we need to crop that 4x4 into 1x1
            //
            // Example 2:
            // area  10x10: we want to generate 10x10
            // parea 4x4: we need 4x4 from the previous layer
            // narea 12x12: instead of 16x16, we skip the bottom and right pixels
            // now we need to crop that 12x12 into 10x10
            // But wait. We actually need parea 5x5, for the worst case:
            // |...*|****|****|*...|....|
            // So it makes sense to rewrite this algorithm and account for that
            // cases, allowing some optimizations
            let parea = Area {
                x: (area.x - 2) >> 2,
                z: (area.z - 2) >> 2,
                w: (area.w >> 2) + 2 + 1, // TODO: without the +1 the slicing fails
                h: (area.h >> 2) + 2 + 1,
            };

            let narea = Area {
                w: (parea.w - 1) << 2,
                h: (parea.h - 1) << 2,
                ..area
            };

            let pmap = parent.get_map(parea);
            let mut map = self.get_map_from_pmap(&pmap);
            let (nw, nh) = map.a.dim();
            assert_eq!((nw, nh), (narea.w as usize, narea.h as usize));
            // TODO: is this correct?
            let (nx, nz) = ((area.x - 2) & 3, (area.z - 2) & 3);
            map.x += nx;
            map.z += nz;
            let (nx, nz) = (nx as usize, nz as usize);
            map.a.slice_collapse(s![
                    nx..nx + area.w as usize,
                    nz..nz + area.h as usize
            ]);

            map
        } else {
            panic!("Parent not set");
        }
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let mut r = McRng::new(self.base_seed, self.world_seed);
        let (p_w, p_h) = pmap.a.dim();
        let p_x = pmap.x;
        let p_z = pmap.z;
        // TODO: x and z are correct?
        let mut area: Area = Default::default();
        area.x = ((p_x + 0) << 2) + 2;
        area.z = ((p_z + 0) << 2) + 2;
        area.w = ((p_w - 1) << 2) as u64;
        area.h = ((p_h - 1) << 2) as u64;
        let mut m = Map::new(area);

        // From a 2x2 pmap we can only generate 1 tile, because we need a 1 margin
        // for x+1 and z+1
        // 2x2 => 4x4
        // 3x3 => 8x8
        // 4x4 => 12x12

        for x in 0..p_w - 1 {
            let mut v00 = pmap.a[(x, 0)];
            let mut v10 = pmap.a[(x+1, 0)];

            for z in 0..p_h - 1 {
                let v01 = pmap.a[(x, z+1)]; //& 0xFF;
                let v11 = pmap.a[(x+1, z+1)]; //& 0xFF;

                // Missed optimization (not present in Java):
                // if v00 == v01 == v10 == v11,
                // buf will always be set to the same value, so skip
                // all the calculations
                // Benchmark result: x10 speedup when pmap is all zeros
                if v00 == v01 && v00 == v10 && v00 == v11 {
                    for j in 0..4 {
                        for i in 0..4 {
                            let idx = ((x << 2) + i, (z << 2) + j);
                            m.a[idx] = v00;
                        }
                    }

                    v00 = v01;
                    v10 = v11;
                    continue;
                }

                let x = x as i64;
                let z = z as i64;

                // Randomly place each of the 4 points in a 3.6x3.6 square
                // centered at (0, 0) or (4, 0) or (0, 4) or (4, 4),
                // depending on the point.

                r.set_chunk_seed((x+p_x) << 2, (z+p_z) << 2);
                let da1 = ((r.next_int_n(1024) as f64) / 1024.0 - 0.5) * 3.6;
                let da2 = ((r.next_int_n(1024) as f64) / 1024.0 - 0.5) * 3.6;

                r.set_chunk_seed((x+p_x+1) << 2, (z+p_z) << 2);
                let db1 = ((r.next_int_n(1024) as f64) / 1024.0 - 0.5) * 3.6 + 4.0;
                let db2 = ((r.next_int_n(1024) as f64) / 1024.0 - 0.5) * 3.6;

                r.set_chunk_seed((x+p_x) << 2, (z+p_z+1) << 2);
                let dc1 = ((r.next_int_n(1024) as f64) / 1024.0 - 0.5) * 3.6;
                let dc2 = ((r.next_int_n(1024) as f64) / 1024.0 - 0.5) * 3.6 + 4.0;

                r.set_chunk_seed((x+p_x+1) << 2, (z+p_z+1) << 2);
                let dd1 = ((r.next_int_n(1024) as f64) / 1024.0 - 0.5) * 3.6 + 4.0;
                let dd2 = ((r.next_int_n(1024) as f64) / 1024.0 - 0.5) * 3.6 + 4.0;

                // For each pixel from pmap we want to generate 4x4 pixels in buf
                for j in 0..4 {
                    let x = x as usize;
                    let z = z as usize;
                    let mut idx = (x << 2, (z << 2) + j);
                    for i in 0..4 {
                        let i = i as f64;
                        let j = j as f64;

                        // Calculate distance from (i, j) to each point
                        let da = (j-da2)*(j-da2) + (i-da1)*(i-da1);
                        let db = (j-db2)*(j-db2) + (i-db1)*(i-db1);
                        let dc = (j-dc2)*(j-dc2) + (i-dc1)*(i-dc1);
                        let dd = (j-dd2)*(j-dd2) + (i-dd1)*(i-dd1);

                        // Set map pixel to value of nearest point
                        if da < db && da < dc && da < dd {
                            m.a[idx] = v00;
                        } else if db < da && db < dc && db < dd {
                            m.a[idx] = v10;
                        } else if dc < da && dc < db && dc < dd {
                            m.a[idx] = v01;
                        } else {
                            m.a[idx] = v11;
                        }

                        idx.0 += 1;
                    }
                }

                v00 = v01;
                v10 = v11;
            }
        }

        m
    }
}

pub struct MapVoronoiZoom115 {
    hashed_world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapVoronoiZoom115 {
    pub fn new(world_seed: i64) -> Self {
        Self { hashed_world_seed: sha256_long_to_long(world_seed), parent: None }
    }
}

// TODO: tests
impl GetMap for MapVoronoiZoom115 {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            // TODO: Area::double(), Area::quadruple(), etc
            // Example 1:
            // area  1x1: we want to generate 1x1
            // parea 2x2: we need 2x2 from the previous layer
            // narea 4x4: instead of 8x8, we only zoom the top left corner
            // now we need to crop that 4x4 into 1x1
            //
            // Example 2:
            // area  10x10: we want to generate 10x10
            // parea 4x4: we need 4x4 from the previous layer
            // narea 12x12: instead of 16x16, we skip the bottom and right pixels
            // now we need to crop that 12x12 into 10x10
            // But wait. We actually need parea 5x5, for the worst case:
            // |...*|****|****|*...|....|
            // So it makes sense to rewrite this algorithm and account for that
            // cases, allowing some optimizations
            let parea = Area {
                x: (area.x - 2) >> 2,
                z: (area.z - 2) >> 2,
                w: (area.w >> 2) + 2 + 1, // TODO: without the +1 the slicing fails
                h: (area.h >> 2) + 2 + 1,
            };

            let narea = Area {
                w: (parea.w - 1) << 2,
                h: (parea.h - 1) << 2,
                ..area
            };

            let pmap = parent.get_map(parea);
            let mut map = self.get_map_from_pmap(&pmap);
            let (nw, nh) = map.a.dim();
            assert_eq!((nw, nh), (narea.w as usize, narea.h as usize));
            // TODO: is this correct?
            let (nx, nz) = ((area.x - 2) & 3, (area.z - 2) & 3);
            map.x += nx;
            map.z += nz;
            let (nx, nz) = (nx as usize, nz as usize);
            map.a.slice_collapse(s![
                    nx..nx + area.w as usize,
                    nz..nz + area.h as usize
            ]);

            map
        } else {
            panic!("Parent not set");
        }
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let (p_w, p_h) = pmap.a.dim();
        let p_x = pmap.x;
        let p_z = pmap.z;
        // TODO: x and z are correct?
        let mut area: Area = Default::default();
        area.x = ((p_x + 0) << 2) + 2;
        area.z = ((p_z + 0) << 2) + 2;
        area.w = ((p_w - 1) << 2) as u64;
        area.h = ((p_h - 1) << 2) as u64;
        let mut m = Map::new(area);

        // From a 2x2 pmap we can only generate 1 tile, because we need a 1 margin
        // for x+1 and z+1
        // 2x2 => 4x4
        // 3x3 => 8x8
        // 4x4 => 12x12

        for x in 0..p_w - 1 {
            let mut v00 = pmap.a[(x, 0)];
            let mut v10 = pmap.a[(x+1, 0)];

            for z in 0..p_h - 1 {
                let v01 = pmap.a[(x, z+1)]; //& 0xFF;
                let v11 = pmap.a[(x+1, z+1)]; //& 0xFF;

                // Missed optimization (not present in Java):
                // if v00 == v01 == v10 == v11,
                // buf will always be set to the same value, so skip
                // all the calculations
                if v00 == v01 && v00 == v10 && v00 == v11 {
                    for j in 0..4 {
                        for i in 0..4 {
                            let idx = ((x << 2) + i, (z << 2) + j);
                            m.a[idx] = v00;
                        }
                    }

                    v00 = v01;
                    v10 = v11;
                    continue;
                }

                // For each pixel from pmap we want to generate 4x4 pixels in buf
                // Calculate the positions of the 2x2x2 points from (px, py, pz) to (px+1, py+1, pz+1)
                let pos_offset = {
                    let px = (p_x + x as i64) as i32;
                    let pz = (p_z + z as i64) as i32;
                    // y = 0; py = (y - 2) >> 2
                    let py = -1;
                    voronoi_1_15_pos_offset(self.hashed_world_seed, px, py, pz)
                };
                let biome_at = [v00, v01, v00, v01, v10, v11, v10, v11];
                for j in 0..4 {
                    for i in 0..4 {
                        let idx = ((x << 2) + i, (z << 2) + j);
                        // y = 0; y2 = (y - 2)
                        let y2 = -2;
                        m.a[idx] = map_voronoi_1_15(i as i32, y2, j as i32, &pos_offset, &biome_at);
                    }
                }

                v00 = v01;
                v10 = v11;
            }
        }

        m
    }
}

pub struct MapVoronoiZoom118 {
    hashed_world_seed: i64,
    pub parent: Option<Rc<dyn GetMap3D>>,
}

impl MapVoronoiZoom118 {
    pub fn new(world_seed: i64) -> Self {
        Self { hashed_world_seed: sha256_long_to_long(world_seed), parent: None }
    }
}

// TODO: tests
impl GetMap3D for MapVoronoiZoom118 {
    fn get_map_3d(&self, area: Area3D) -> Map3D {
        if let Some(ref parent) = self.parent {
            // TODO: Area::double(), Area::quadruple(), etc
            // Example 1:
            // area  1x1: we want to generate 1x1
            // parea 2x2: we need 2x2 from the previous layer
            // narea 4x4: instead of 8x8, we only zoom the top left corner
            // now we need to crop that 4x4 into 1x1
            //
            // Example 2:
            // area  10x10: we want to generate 10x10
            // parea 4x4: we need 4x4 from the previous layer
            // narea 12x12: instead of 16x16, we skip the bottom and right pixels
            // now we need to crop that 12x12 into 10x10
            // But wait. We actually need parea 5x5, for the worst case:
            // |...*|****|****|*...|....|
            // So it makes sense to rewrite this algorithm and account for that
            // cases, allowing some optimizations
            let parea = Area3D {
                x: (area.x - 2) >> 2,
                y: (area.y - 2) >> 2,
                z: (area.z - 2) >> 2,
                sx: (area.sx >> 2) + 2 + 1, // TODO: without the +1 the slicing fails
                sy: (area.sy >> 2) + 2 + 1,
                sz: (area.sz >> 2) + 2 + 1,
            };

            let narea = Area3D {
                sx: (parea.sx - 1) << 2,
                sy: (parea.sy - 1) << 2,
                sz: (parea.sz - 1) << 2,
                ..area
            };

            let pmap = parent.get_map_3d(parea);
            let mut map = self.get_map_from_pmap_3d(&pmap);
            let (nsx, nsy, nsz) = map.a.dim();
            assert_eq!((nsx, nsy, nsz), (narea.sx as usize, narea.sy as usize, narea.sz as usize));
            // TODO: is this correct?
            let (nx, ny, nz) = ((area.x - 2) & 3, (area.y - 2) & 3, (area.z - 2) & 3);
            map.x += nx;
            map.y += ny;
            map.z += nz;
            let (nx, ny, nz) = (nx as usize, ny as usize, nz as usize);
            map.a.slice_collapse(s![
                    nx..nx + area.sx as usize,
                    ny..ny + area.sy as usize,
                    nz..nz + area.sz as usize
            ]);

            map
        } else {
            panic!("Parent not set");
        }
    }
    fn get_map_from_pmap_3d(&self, pmap: &Map3D) -> Map3D {
        let (p_sx, p_sy, p_sz) = pmap.a.dim();
        let p_x = pmap.x;
        let p_y = pmap.y;
        let p_z = pmap.z;
        // TODO: x and z are correct?
        let mut area: Area3D = Default::default();
        area.x = ((p_x + 0) << 2) + 2;
        area.y = ((p_y + 0) << 2) + 2;
        area.z = ((p_z + 0) << 2) + 2;
        area.sx = ((p_sx - 1) << 2) as u64;
        area.sy = ((p_sy - 1) << 2) as u64;
        area.sz = ((p_sz - 1) << 2) as u64;
        let mut m = Map3D::new(area);

        // From a 2x2 pmap we can only generate 1 tile, because we need a 1 margin
        // for x+1 and z+1
        // 2x2 => 4x4
        // 3x3 => 8x8
        // 4x4 => 12x12

        // TODO: add y loop
        for x in 0..p_sx - 1 {
            let mut v00 = pmap.a[(x, 0, 0)];
            let mut v10 = pmap.a[(x+1, 0, 0)];

            for z in 0..p_sz - 1 {
                let v01 = pmap.a[(x, 0, z+1)]; //& 0xFF;
                let v11 = pmap.a[(x+1, 0, z+1)]; //& 0xFF;

                // Missed optimization (not present in Java):
                // if v00 == v01 == v10 == v11,
                // buf will always be set to the same value, so skip
                // all the calculations
                if v00 == v01 && v00 == v10 && v00 == v11 {
                    for j in 0..4 {
                        for i in 0..4 {
                            let idx = ((x << 2) + i, 0, (z << 2) + j);
                            m.a[idx] = v00;
                        }
                    }

                    v00 = v01;
                    v10 = v11;
                    continue;
                }

                // For each pixel from pmap we want to generate 4x4 pixels in buf
                // Calculate the positions of the 2x2x2 points from (px, py, pz) to (px+1, py+1, pz+1)
                let pos_offset = {
                    let px = (p_x + x as i64) as i32;
                    let pz = (p_z + z as i64) as i32;
                    // y = 0; py = (y - 2) >> 2
                    let py = -1;
                    voronoi_1_15_pos_offset(self.hashed_world_seed, px, py, pz)
                };
                let biome_at = [v00, v01, v00, v01, v10, v11, v10, v11];
                for j in 0..4 {
                    for i in 0..4 {
                        let idx = ((x << 2) + i, 0, (z << 2) + j);
                        // y = 0; y2 = (y - 2)
                        let y2 = -2;
                        m.a[idx] = map_voronoi_1_15(i as i32, y2, j as i32, &pos_offset, &biome_at);
                    }
                }

                v00 = v01;
                v10 = v11;
            }
        }

        m
    }
}

/// Overworld and Nether biome generator for 1.18
pub struct MapGenBiomeNoise3D118 {
    world_seed: i64,
    shift: NoiseGeneratorDoublePerlin128,
    temperature: NoiseGeneratorDoublePerlin128,
    humidity: NoiseGeneratorDoublePerlin128,
    continentalness: NoiseGeneratorDoublePerlin128,
    erosion: NoiseGeneratorDoublePerlin128,
    weirdness: NoiseGeneratorDoublePerlin128,
    sp: Arc<Spline>,
}

impl MapGenBiomeNoise3D118 {
    pub fn new(world_seed: i64) -> Self {
        let mut pxr = Xoroshiro128PlusPlus::with_u64_seed(world_seed as u64);
        let xlo = pxr.next_long();
        let xhi = pxr.next_long();

        let amp_s = [1.0, 1.0, 1.0, 0.0];
        // md5 "minecraft:offset"
        pxr = Xoroshiro128PlusPlus::new(xlo ^ 0x080518cf6af25384, xhi ^ 0x3f3dfb40a54febd5);
        let shift = NoiseGeneratorDoublePerlin128::new(&mut pxr, &amp_s, -3);

        let amp_t = [1.5, 0.0, 1.0, 0.0, 0.0, 0.0];
        // md5 "minecraft:temperature"
        pxr = Xoroshiro128PlusPlus::new(xlo ^ 0x5c7e6b29735f0d7f, xhi ^ 0xf7d86f1bbc734988);
        let temperature = NoiseGeneratorDoublePerlin128::new(&mut pxr, &amp_t, -10);

        let amp_h = [1.0, 1.0, 0.0, 0.0, 0.0, 0.0];
        // md5 "minecraft:vegetation"
        pxr = Xoroshiro128PlusPlus::new(xlo ^ 0x81bb4d22e8dc168e, xhi ^ 0xf1c8b4bea16303cd);
        let humidity = NoiseGeneratorDoublePerlin128::new(&mut pxr, &amp_h, -8);

        let amp_c = [1.0, 1.0, 2.0, 2.0, 2.0, 1.0, 1.0, 1.0, 1.0];
        // md5 "minecraft:continentalness"
        pxr = Xoroshiro128PlusPlus::new(xlo ^ 0x83886c9d0ae3a662, xhi ^ 0xafa638a61b42e8ad);
        let continentalness = NoiseGeneratorDoublePerlin128::new(&mut pxr, &amp_c, -9);

        let amp_e = [1.0, 1.0, 0.0, 1.0, 1.0];
        // md5 "minecraft:erosion"
        pxr = Xoroshiro128PlusPlus::new(xlo ^ 0xd02491e6058f6fd8, xhi ^ 0x4792512c94c17a80);
        let erosion = NoiseGeneratorDoublePerlin128::new(&mut pxr, &amp_e, -9);

        let amp_w = [1.0, 2.0, 1.0, 0.0, 0.0, 0.0];
        // md5 "minecraft:ridge"
        pxr = Xoroshiro128PlusPlus::new(xlo ^ 0xefc8ef4d36102b34, xhi ^ 0x1beeeb324a0f24ea);
        let weirdness = NoiseGeneratorDoublePerlin128::new(&mut pxr, &amp_w, -7);

        lazy_static! {
            static ref CONTINENTAL_SPLINE: Arc<Spline> = Arc::new(Spline::new_continental());
        }
        let sp = Arc::clone(&CONTINENTAL_SPLINE);

        Self { world_seed, shift, temperature, humidity, continentalness, erosion, weirdness, sp }
    }

    fn sample_biome_noise(&self, np: Option<&mut Climate>, pos: Point3D, dat: &mut u64) -> i32 {
        let x = pos.x as f64;
        let y = pos.y;
        let z = pos.z as f64;
        let mut px = x;
        let mut pz = z;

        px += self.shift.sample(x, 0.0, z) * 4.0;
        pz += self.shift.sample(z, x, 0.0) * 4.0;

        let c = self.continentalness.sample(px, 0.0, pz);
        let e = self.erosion.sample(px, 0.0, pz);
        let w = self.weirdness.sample(px, 0.0, pz);

        let np_param = [c, e, -3.0 * ((w.abs() - 0.6666667).abs() - 0.33333334), w];
        let np_param = np_param.map(|x| x as f32);
        let off = self.sp.get_spline(&np_param) + 0.015;

        //double py = y + sampleDoublePerlin(&bn->shift, y, z, x) * 4.0;
        let d = 1.0 - ((y << 2) as f32) / 128.0 - 83.0/160.0 + off;

        let t = self.temperature.sample(px, 0.0, pz);
        let h = self.humidity.sample(px, 0.0, pz);

        let mut l_np;
        let p_np = match np {
            Some(np) => np,
            None => {
                l_np = Climate::default();
                &mut l_np
            }
        };

        p_np.temperature = (10000.0*t) as i64;
        p_np.humidity = (10000.0*h) as i64;
        p_np.continentalness = (10000.0*c) as i64;
        p_np.erosion = (10000.0*e) as i64;
        p_np.depth = (10000.0*d) as i64;
        p_np.weirdness = (10000.0*w) as i64;

        let id = p2overworld(p_np, dat);

        id
    }

    /// part:
    /// 0: shift_x
    /// 1: shift_z
    /// 2: temperature
    /// 3: humidity
    /// 4: continentalness
    /// 5: erosion
    /// 6: weirdness
    /// 7: depth
    /// 8: biome
    /// 50: climate distance to next biome (for debugging biome_info_118.rs)
    /// 51: difference between search_bruteforce and search_tree (for debugging biome_info_118.rs)
    // TODO: ensure that the return value is always in range [0, 255], so that converting to
    // grayscale color is easy
    fn partial_sample_biome_noise(&self, np: Option<&mut Climate>, pos: Point3D, dat: &mut u64, part: u32) -> i32 {
        let clamp_float_to_u8_range = |f, fmin, fmax| {
            let mut updated_limits = false;
            lazy_static! {
                static ref LIMITS: RwLock<[(f64, f64); 100]> = RwLock::new([(f64::MAX, f64::MIN); 100]);
            }
            // First, read RwLock to see if the value is within limits. This is the most likely
            // case, and multiple threads can read a RwLock at the same time.
            if let Some((lmin, lmax)) = LIMITS.read().expect("rwlock error").get(part as usize) {
                if f > *lmax {
                    updated_limits = true;
                }
                if f < *lmin {
                    updated_limits = true;
                }
            }
            // If the previous check returned true, we need to update the limits. In that case,
            // lock the RwLock for writing and update the limit
            if updated_limits {
                if let Some((lmin, lmax)) = LIMITS.write().expect("rwlock error").get_mut(part as usize) {
                    // Another thread may have already updated the limits, so reset flag to avoid
                    // printing the same error message twice
                    updated_limits = false;
                    if f > *lmax {
                        *lmax = f;
                        updated_limits = true;
                    }
                    if f < *lmin {
                        *lmin = f;
                        updated_limits = true;
                    }
                }
            }
            let r = (f - fmin) * 256.0 / (fmax - fmin);
            let r = r as i32;

            // Check if the resulting value is a valid u8
            if !(0..=255).contains(&r) {
                // Invalid value, this means that the limits are not correct. Print error message
                // and return a gray pixel.
                let string_in_the_stack;
                let arg_name = match part {
                    0 => "shift_x",
                    1 => "shift_z",
                    2 => "temperature",
                    3 => "humidity",
                    4 => "continentalness",
                    5 => "erosion",
                    6 => "weirdness",
                    7 => "depth",
                    8 => "biome",
                    50 => "debug_distance_to_second_biome",
                    51 => "debug_search_bruteforce_xor_search_tree",
                    _ => {
                        string_in_the_stack = format!("arg_{}", part);
                        &string_in_the_stack
                    }
                };
                if updated_limits {
                    let limits_filter_max: Vec<_> = LIMITS.read().expect("rwlock error").iter().enumerate().filter_map(|(i, k)| if *k == (f64::MAX, f64::MIN) { None } else { Some((i, *k)) }).collect();
                    log::error!("{}={} set to {}. LIMITS: {:?}", arg_name, f, r, limits_filter_max);
                }

                // Return middle point value on overflow, to make it easier to spot on the map
                127
            } else {
                r
            }
        };

        let x = pos.x as f64;
        let y = pos.y;
        let z = pos.z as f64;
        let mut px = x;
        let mut pz = z;

        // Parts 0..=1 are independent of each other
        match part {
            0 => {
                let shift_x = self.shift.sample(x, 0.0, z) * 4.0;
                px += shift_x;

                if part == 0 {
                    return clamp_float_to_u8_range(shift_x, -6.0, 6.0);
                }
            }
            1 => {
                let shift_z = self.shift.sample(z, x, 0.0) * 4.0;
                pz += shift_z;

                if part == 1 {
                    return clamp_float_to_u8_range(shift_z, -6.0, 6.0);
                }
            }
            _ => {}
        }

        let shift_x = self.shift.sample(x, 0.0, z) * 4.0;
        px += shift_x;

        if part == 0 {
            return clamp_float_to_u8_range(shift_x, -6.0, 6.0);
        }

        let shift_z = self.shift.sample(z, x, 0.0) * 4.0;
        pz += shift_z;

        if part == 1 {
            return clamp_float_to_u8_range(shift_z, -6.0, 6.0);
        }

        let mut l_np;
        let p_np = match np {
            Some(np) => np,
            None => {
                l_np = Climate::default();
                &mut l_np
            }
        };

        // Parts 2..=6 only depend on parts 0 and 1, not on the previous part
        match part {
            2 => {
                let t = self.temperature.sample(px, 0.0, pz);
                p_np.temperature = (10000.0*t) as i64;

                if part == 2 {
                    return clamp_float_to_u8_range(p_np.temperature as f64, -12010.0, 12010.0);
                }
            }
            3 => {
                let h = self.humidity.sample(px, 0.0, pz);
                p_np.humidity = (10000.0*h) as i64;

                if part == 3 {
                    return clamp_float_to_u8_range(p_np.humidity as f64, -10500.0, 10000.0);
                }
            }
            4 => {
                let c = self.continentalness.sample(px, 0.0, pz);
                p_np.continentalness = (10000.0*c) as i64;

                if part == 4 {
                    return clamp_float_to_u8_range(p_np.continentalness as f64, -14200.0, 15000.0);
                }
            }
            5 => {
                let e = self.erosion.sample(px, 0.0, pz);
                p_np.erosion = (10000.0*e) as i64;

                if part == 5 {
                    return clamp_float_to_u8_range(p_np.erosion as f64, -13500.0, 13500.0);
                }
            }
            6 => {
                let w = self.weirdness.sample(px, 0.0, pz);
                p_np.weirdness = (10000.0*w) as i64;

                if part == 6 {
                    return clamp_float_to_u8_range(p_np.weirdness as f64, -16000.0, 16100.0);
                }
            }
            _ => {}
        }

        let t = self.temperature.sample(px, 0.0, pz);
        p_np.temperature = (10000.0*t) as i64;

        if part == 2 {
            return clamp_float_to_u8_range(p_np.temperature as f64, -12010.0, 12010.0);
        }

        let h = self.humidity.sample(px, 0.0, pz);
        p_np.humidity = (10000.0*h) as i64;

        if part == 3 {
            return clamp_float_to_u8_range(p_np.humidity as f64, -10500.0, 10000.0);
        }

        let c = self.continentalness.sample(px, 0.0, pz);
        p_np.continentalness = (10000.0*c) as i64;

        if part == 4 {
            return clamp_float_to_u8_range(p_np.continentalness as f64, -14200.0, 15000.0);
        }

        let e = self.erosion.sample(px, 0.0, pz);
        p_np.erosion = (10000.0*e) as i64;

        if part == 5 {
            return clamp_float_to_u8_range(p_np.erosion as f64, -13500.0, 13500.0);
        }

        let w = self.weirdness.sample(px, 0.0, pz);
        p_np.weirdness = (10000.0*w) as i64;

        if part == 6 {
            return clamp_float_to_u8_range(p_np.weirdness as f64, -16000.0, 16100.0);
        }

        let np_param = [c as f32, e as f32, -3.0 * (((w as f32).abs() - 0.6666667).abs() - 0.33333334), w as f32];
        let off = self.sp.get_spline(&np_param) + 0.015;

        //double py = y + sampleDoublePerlin(&bn->shift, y, z, x) * 4.0;
        let d = 1.0 - ((y << 2) as f32) / 128.0 - 83.0/160.0 + off;
        p_np.depth = (10000.0*d) as i64;

        if part == 7 {
            return clamp_float_to_u8_range(p_np.depth as f64, -25000.0, 25000.0);
        }

        if part == 50 {
            let dist = debug_distance_to_second_biome(p_np, dat);
            // +1 because in case of 0 distance log2(0) = -inf we want log2(1) = 0
            let dist_log2 = (1.0 + dist as f64).log2();
            return clamp_float_to_u8_range(dist_log2, 0.0, 12.5);
        }
        if part == 51 {
            let diff = debug_search_bruteforce_xor_search_tree(p_np, dat);
            // Binary image
            let dist = if diff == 0 { 0 } else { 255 };
            return clamp_float_to_u8_range(dist as f64, 0.0, 256.0);
        }

        let id = p2overworld(p_np, dat);

        if part == 8 {
            return id;
        }

        panic!("Invalid part in partial_sample_biome_noise: {}", part);
    }

    pub fn partial_get_map_3d(&self, area: Area3D, part: u32) -> Map3D {
        let mut dat = 0;
        Map3D::from_area_fn(area, |(x, y, z)| {
            let x = area.x + x as i64;
            let y = area.y + y as i64;
            let z = area.z + z as i64;
            self.partial_sample_biome_noise(None, Point3D { x, y, z }, &mut dat, part)
        })
    }
}

impl GetMap3D for MapGenBiomeNoise3D118 {
    fn get_map_3d(&self, area: Area3D) -> Map3D {
        let mut dat = 0;
        Map3D::from_area_fn(area, |(x, y, z)| {
            let x = area.x + x as i64;
            let y = area.y + y as i64;
            let z = area.z + z as i64;
            self.sample_biome_noise(None, Point3D { x, y, z }, &mut dat)
        })
    }

    // MapIsland is the first layer, so it does not need pmap
    fn get_map_from_pmap_3d(&self, pmap: &Map3D) -> Map3D {
        let area = pmap.area();

        self.get_map_3d(area)
    }
}

fn p2overworld(np: &Climate, dat: &mut u64) -> i32 {
    crate::biome_info_118::BIOME_LIST.search(np).unwrap().0
}

fn debug_distance_to_second_biome(np: &Climate, dat: &mut u64) -> i64 {
    crate::biome_info_118::BIOME_LIST.distance_to_second_biome(np).unwrap()
}

fn debug_search_bruteforce_xor_search_tree(np: &Climate, dat: &mut u64) -> i32 {
    let bb = crate::biome_info_118::BIOME_LIST.search_bruteforce(np).unwrap();
    let bt = crate::biome_info_118::BIOME_LIST.search_tree(np).unwrap();

    bb.0 ^ bt.0
}

// Return the index of the minimum element of the input array, or None if the array is empty.
// Panics if the input contains a NaN float.
// Note that in case of tie, the element with the lowest index should win
fn index_of_min_element(x: &[f64]) -> Option<usize> {
    x.iter().enumerate().min_by(|(_, a), (_, b)| a.partial_cmp(b).expect("NaN float")).map(|(i, _)| i)
}

fn voronoi_1_15_pos_offset(seed: i64, px: i32, py: i32, pz: i32) -> [(f64, f64, f64); 8] {
    // Negative position of the voronoi point
    let mut pos_offset = [(0.0, 0.0, 0.0); 8];

    for i in 0..8 {
        let flagx = (i & 4) == 0;
        let flagy = (i & 2) == 0;
        let flagz = (i & 1) == 0;

        let x1 = if flagx { px } else { px + 1 };
        let y1 = if flagy { py } else { py + 1 };
        let z1 = if flagz { pz } else { pz + 1 };

        pos_offset[i] = rand_offset_3d(seed, x1, y1, z1);
        // FIXME(voronoi_float_precision): these operations used to be performed
        // right before mod_squared_3d
        pos_offset[i].0 -= if flagx { 0.0 } else { 1.0 };
        pos_offset[i].1 -= if flagy { 0.0 } else { 1.0 };
        pos_offset[i].2 -= if flagz { 0.0 } else { 1.0 };
    }

    pos_offset
}

// Calculates the distance from (x, y, z) to each of the points in pos_offset,
// and returns the biome of the nearest point.
// (x, y, z) are the coordinates inside the 4x4x4 cube that will be generated
// by MapVoronoiZoom115, should be one of (0, 1, 2, 3).
fn map_voronoi_1_15(x: i32, y: i32, z: i32, pos_offset: &[(f64, f64, f64); 8], biome_at: &[i32; 8]) -> i32 {
    // dx is one of 0.00, 0.25, 0.50, 0.75
    let dx = f64::from(x & 3) / 4.0;
    let dy = f64::from(y & 3) / 4.0;
    let dz = f64::from(z & 3) / 4.0;
    let mut dists = [0.0; 8];

    for i in 0..8 {
        dists[i] = mod_squared_3d(pos_offset[i].0 + dx, pos_offset[i].1 + dy, pos_offset[i].2 + dz);
    }

    let min_index = index_of_min_element(&dists).unwrap();

    biome_at[min_index]
}

fn mod_squared_3d(x: f64, y: f64, z: f64) -> f64 {
    // FIXME(voronoi_float_precision): the order of the arguments is important,
    // but I don't have any test cases with the correct order. This may be a
    // problem when two points are at about the same distance from a third
    // point. In that case, the biome at the third point may be wrong because of
    // the floating point precision. We cannot use AMIDST to generate test cases
    // because we need the full resolution biome map.
    z * z + y * y + x * x
}

fn rand_offset_3d(seed: i64, x: i32, y: i32, z: i32) -> (f64, f64, f64) {
    // Returns number in range [-0.45, 0.45)
    fn rand_offset(seed: i64) -> f64 {
        // nextInt(1024) / 1024.0
        // Return a f64 between 0.0 and 1.0 with 10 bits of accuracy:
        // two different points cannot be closer than 2^-10
        let d = McRng::math_floor_div(seed >> 24, 1024) as f64 / 1024.0;

        (d - 0.5) * 0.9
    }

    let mut r = McRng::next_state(seed, i64::from(x));
    r = McRng::next_state(r, i64::from(y));
    r = McRng::next_state(r, i64::from(z));
    r = McRng::next_state(r, i64::from(x));
    r = McRng::next_state(r, i64::from(y));
    r = McRng::next_state(r, i64::from(z));
    let dx = rand_offset(r);

    r = McRng::next_state(r, seed);
    let dy = rand_offset(r);

    r = McRng::next_state(r, seed);
    let dz = rand_offset(r);

    (dx, dy, dz)
}

pub struct MapIsland {
    base_seed: i64,
    world_seed: i64,
}

impl MapIsland {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed }
    }
}

impl GetMap for MapIsland {
    fn get_map(&self, area: Area) -> Map {
        let r = McRng::new(self.base_seed, self.world_seed);
        let mut m = MapFn(|Point {x, z}| {
            let mut r = r;
            r.set_chunk_seed(x, z);

            if r.next_int_n(10) == 0 { 1 } else { 0 }
        }).get_map(area);

        // Force (0, 0) to island
        if area.x > -(area.w as i64) && area.x <= 0 && area.z > -(area.h as i64) && area.z <= 0 {
            m.a[(-area.x as usize, -area.z as usize)] = 1;
        }

        m
    }

    // MapIsland is the first layer, so it does not need pmap
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let area = pmap.area();

        self.get_map(area)
    }
}

// Random:
// v00: 0 bits
// v01: 25 bits, but gives us the bit 24 for free
// v10: 25 bits
// v11: 26 bits
//  0 bits for v00,
// 25 bits for v01,
// 25 bits for v10,
// 26 bits for v11.
pub struct MapZoom {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
    pub bug_world_seed_not_set: bool, // true if this layer is parent2 of MapHills
}

impl MapZoom {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None, bug_world_seed_not_set: false }
    }
}

impl GetMap for MapZoom {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x >> 1,
                z: area.z >> 1,
                w: (area.w >> 1) + 2,
                h: (area.h >> 1) + 2
            };
            let pmap = parent.get_map(parea);

            let mut map = self.get_map_from_pmap(&pmap);
            // TODO: is this correct?
            let (nx, nz) = ((area.x) & 1, (area.z) & 1);
            map.x += nx;
            map.z += nz;
            let (nx, nz) = (nx as usize, nz as usize);
            map.a.slice_collapse(s![
                    nx..nx + area.w as usize,
                    nz..nz + area.h as usize
            ]);

            map
        } else {
            panic!("Parent not set");
        }
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let mut r = McRng::default();
        r.set_base_seed(self.base_seed);
        if !self.bug_world_seed_not_set {
            r.set_world_seed(self.world_seed);
        }
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x << 1,
            z: pmap.z << 1,
            w: ((p_w - 1) << 1) as u64,
            h: ((p_h - 1) << 1) as u64
        };

        let mut map = Map::new(area);

        for x in 0..p_w - 1 {
            let mut a = pmap.a[(x+0, 0)];
            let mut a1 = pmap.a[(x+1, 0)];
            for z in 0..p_h - 1 {
                let b = pmap.a[(x+0, z+1)];
                let b1 = pmap.a[(x+1, z+1)];

                // Missed optimization (not present in Java):
                // if a == a1 == b
                // buf will always be set to the same value, so skip
                // all the calculations
                // This assumes that:
                // select_mode_or_random(a, a, a, X) == a
                // select_mode_or_random(a, a, X, a) == a
                // Benchmark result:
                /*
                map_zoom_xhz        x 0.93
                map_zoom_zeros      x 2.21
                */
                if a == a1 && a == b {
                    map.a[((x << 1) + 0, (z << 1) + 0)] = a;
                    map.a[((x << 1) + 0, (z << 1) + 1)] = a;
                    map.a[((x << 1) + 1, (z << 1) + 0)] = a;
                    map.a[((x << 1) + 1, (z << 1) + 1)] = a;

                    a = b;
                    a1 = b1;
                    continue;
                }

                let chunk_x = (x as i64 + pmap.x) << 1;
                let chunk_z = (z as i64 + pmap.z) << 1;

                r.set_chunk_seed(chunk_x, chunk_z);
                let a_or_b = r.choose2(a, b);
                map.a[((x << 1) + 0, (z << 1) + 0)] = a;
                map.a[((x << 1) + 0, (z << 1) + 1)] = a_or_b;

                let a_or_a1 = r.choose2(a, a1);
                map.a[((x << 1) + 1, (z << 1) + 0)] = a_or_a1;

                map.a[((x << 1) + 1, (z << 1) + 1)] = r.select_mode_or_random(a, a1, b, b1);

                a = b;
                a1 = b1;
            }
        }

        map
    }
}

pub struct MapZoomFuzzy {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapZoomFuzzy {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapZoomFuzzy {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x >> 1,
                z: area.z >> 1,
                w: (area.w >> 1) + 2,
                h: (area.h >> 1) + 2
            };
            let pmap = parent.get_map(parea);

            let mut map = self.get_map_from_pmap(&pmap);
            // TODO: is this correct?
            let (nx, nz) = ((area.x) & 1, (area.z) & 1);
            map.x += nx;
            map.z += nz;
            let (nx, nz) = (nx as usize, nz as usize);
            map.a.slice_collapse(s![
                    nx..nx + area.w as usize,
                    nz..nz + area.h as usize
            ]);

            map
        } else {
            panic!("Parent not set");
        }
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let mut r = McRng::default();
        r.set_base_seed(self.base_seed);
        r.set_world_seed(self.world_seed);
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x << 1,
            z: pmap.z << 1,
            w: ((p_w - 1) << 1) as u64,
            h: ((p_h - 1) << 1) as u64
        };

        let mut map = Map::new(area);

        for x in 0..p_w - 1 {
            let mut a = pmap.a[(x+0, 0)];
            let mut a1 = pmap.a[(x+1, 0)];
            for z in 0..p_h - 1 {
                let b = pmap.a[(x+0, z+1)];
                let b1 = pmap.a[(x+1, z+1)];

                // Missed optimization (not present in Java):
                // if a == a1 == b == b1,
                // buf will always be set to the same value, so skip
                // all the calculations
                // Benchmark result:
                /*
                map_zoom_fuzzy_island      x 2.68
                map_zoom_fuzzy_xhz         x 0.99
                map_zoom_fuzzy_zeros       x 5.03
                */
                if a == a1 && a == b && a == b1 {
                    map.a[((x << 1) + 0, (z << 1) + 0)] = a;
                    map.a[((x << 1) + 0, (z << 1) + 1)] = a;
                    map.a[((x << 1) + 1, (z << 1) + 0)] = a;
                    map.a[((x << 1) + 1, (z << 1) + 1)] = a;

                    a = b;
                    a1 = b1;
                    continue;
                }

                let chunk_x = (x as i64 + pmap.x) << 1;
                let chunk_z = (z as i64 + pmap.z) << 1;

                r.set_chunk_seed(chunk_x, chunk_z);
                let a_or_b = r.choose2(a, b);
                map.a[((x << 1) + 0, (z << 1) + 0)] = a;
                map.a[((x << 1) + 0, (z << 1) + 1)] = a_or_b;

                let a_or_a1 = r.choose2(a, a1);
                map.a[((x << 1) + 1, (z << 1) + 0)] = a_or_a1;

                map.a[((x << 1) + 1, (z << 1) + 1)] = r.choose4(a, a1, b, b1);

                a = b;
                a1 = b1;
            }
        }

        map
    }
}

/// Unlike the regular MapZoom, this one makes sure that v11 is different
/// from any of its neighbours. This way we can generate all the possible
/// edges (and therefore rivers) for this 25-bit seed.
// Update: this did not work as I expected, but could still be useful
// Note that the black spots are not necessarily points where no river can spawn
pub struct HelperMapZoomAllEdges {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
    pub fuzzy: bool, // true when parent is MapIsland
    pub bug_world_seed_not_set: bool, // true if this layer is parent2 of MapHills
}

impl HelperMapZoomAllEdges {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None, fuzzy: false, bug_world_seed_not_set: false }
    }
}

impl GetMap for HelperMapZoomAllEdges {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x >> 1,
                z: area.z >> 1,
                w: (area.w >> 1) + 2,
                h: (area.h >> 1) + 2
            };
            let pmap = parent.get_map(parea);

            let mut map = self.get_map_from_pmap(&pmap);
            // TODO: is this correct?
            let (nx, nz) = ((area.x) & 1, (area.z) & 1);
            map.x += nx;
            map.z += nz;
            let (nx, nz) = (nx as usize, nz as usize);
            map.a.slice_collapse(s![
                    nx..nx + area.w as usize,
                    nz..nz + area.h as usize
            ]);

            map
        } else {
            panic!("Parent not set");
        }
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let mut r = McRng::default();
        r.set_base_seed(self.base_seed);
        if !self.bug_world_seed_not_set {
            r.set_world_seed(self.world_seed);
        }
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x << 1,
            z: pmap.z << 1,
            w: ((p_w - 1) << 1) as u64,
            h: ((p_h - 1) << 1) as u64
        };

        let mut map = Map::new(area);

        for x in 0..p_w - 1 {
            let mut a = pmap.a[(x+0, 0)];
            let mut a1 = pmap.a[(x+1, 0)];
            for z in 0..p_h - 1 {
                let b = pmap.a[(x+0, z+1)];
                let b1 = pmap.a[(x+1, z+1)];

                // Missed optimization (not present in Java):
                // if a == a1 == b == b1,
                // buf will always be set to the same value, so skip
                // all the calculations
                // Benchmark result:
                /*
                map_zoom_fuzzy_xhz      45,678  x 0.93
                map_zoom_fuzzy_zeros    18,162  x 2.37
                map_zoom_fuzzy_island   25,166  x 1.70
                map_zoom_xhz            67,579  x 0.93
                map_zoom_zeros          17,544  x 1.57
                */
                if a == a1 && a == b && a == b1 {
                    map.a[((x << 1) + 0, (z << 1) + 0)] = a;
                    map.a[((x << 1) + 0, (z << 1) + 1)] = a;
                    map.a[((x << 1) + 1, (z << 1) + 0)] = a;
                    map.a[((x << 1) + 1, (z << 1) + 1)] = a;

                    a = b;
                    a1 = b1;
                    continue;
                }

                let chunk_x = (x as i64 + pmap.x) << 1;
                let chunk_z = (z as i64 + pmap.z) << 1;

                r.set_chunk_seed(chunk_x, chunk_z);
                let a_or_b = r.choose2(a, b);
                map.a[((x << 1) + 0, (z << 1) + 0)] = a;
                map.a[((x << 1) + 0, (z << 1) + 1)] = a_or_b;

                let a_or_a1 = r.choose2(a, a1);
                map.a[((x << 1) + 1, (z << 1) + 0)] = a_or_a1;

                map.a[((x << 1) + 1, (z << 1) + 1)] = if self.fuzzy {
                    // For mapIsland
                    r.choose4(a, a1, b, b1)
                } else {
                    // This is the one line different from MapZoom
                    a + a1 + b + b1
                };

                a = b;
                a1 = b1;
            }
        }

        map
    }
}

/// This layer uses 64 bits but only affects shores (regions near ocean).
/// Deep ocean is not affected.
/// This makes continental biome borders a good candidate for getting the seed.
/// Ocean islands also seem unaffected, but they are generated in layer 25.
pub struct MapAddIsland {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapAddIsland {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapAddIsland {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x - 1,
                z: area.z - 1,
                w: area.w + 2,
                h: area.h + 2
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        let mut r = McRng::new(self.base_seed, self.world_seed);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let v00 = pmap.a[(x+0, z+0)];
                let v20 = pmap.a[(x+2, z+0)];
                let v02 = pmap.a[(x+0, z+2)];
                let v22 = pmap.a[(x+2, z+2)];
                let v11 = pmap.a[(x+1, z+1)];

                m.a[(x, z)] = if v11 == 0 && (v00 != 0 || v20 != 0 || v02 != 0 || v22 != 0) {
                    let chunk_x = x as i64 + area.x;
                    let chunk_z = z as i64 + area.z;
                    r.set_chunk_seed(chunk_x, chunk_z);

                    let mut v = 1;
                    let mut inc = 1;

                    if v00 != 0 {
                        // nextInt(1) is always 0
                        if r.next_int_n(inc) == 0 {
                            v = v00;
                        }
                        inc += 1;
                    }
                    if v20 != 0 {
                        if r.next_int_n(inc) == 0 {
                            v = v20;
                        }
                        inc += 1;
                    }
                    if v02 != 0 {
                        if r.next_int_n(inc) == 0 {
                            v = v02;
                        }
                        inc += 1;
                    }
                    if v22 != 0 {
                        if r.next_int_n(inc) == 0 {
                            v = v22;
                        }
                    }
                    if r.next_int_n(3) == 0 {
                        v
                    } else if v == 4 {
                        4
                    } else {
                        0
                    }
                } else if v11 > 0 && (v00 == 0 || v20 == 0 || v02 == 0 || v22 == 0) {
                    let chunk_x = x as i64 + area.x;
                    let chunk_z = z as i64 + area.z;
                    r.set_chunk_seed(chunk_x, chunk_z);
                    if r.next_int_n(5) == 0 {
                        if v11 == 4 { 4 } else { 0 }
                    } else {
                        v11
                    }
                } else {
                    v11
                };
            }
        }

        m
    }
}

pub struct MapRemoveTooMuchOcean {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapRemoveTooMuchOcean {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapRemoveTooMuchOcean {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x - 1,
                z: area.z - 1,
                w: area.w + 2,
                h: area.h + 2
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        let mut r = McRng::new(self.base_seed, self.world_seed);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let v11 = pmap.a[(x+1, z+1)];
                m.a[(x, z)] = v11;

                /* X0X     X0X *
                 * 000  => 010 *
                 * X0X     X0X */
                if pmap.a[(x+1, z+0)] == 0 && pmap.a[(x+2, z+1)] == 0
                    && pmap.a[(x+0, z+1)] == 0 && pmap.a[(x+1, z+2)] == 0 && v11 == 0 {
                    let chunk_x = x as i64 + area.x;
                    let chunk_z = z as i64 + area.z;
                    r.set_chunk_seed(chunk_x, chunk_z);

                    if r.next_int_n(2) == 0 {
                        m.a[(x, z)] = 1;
                    }
                }
            }
        }

        m
    }
}

pub struct MapAddSnow {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapAddSnow {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapAddSnow {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let pmap = parent.get_map(area);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has no margin: pmap.w == map.w
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let r = McRng::new(self.base_seed, self.world_seed);
        MapParentFn(PanicMap, |x, z, v| {
            if v == 0 {
                0
            } else {
                let mut r = r;
                r.set_chunk_seed(x, z);
                let r = r.next_int_n(6);

                if r == 0 {
                    4
                } else if r <= 1 {
                    3
                } else {
                    1
                }
            }
        }).get_map_from_pmap(pmap)
    }
}

pub struct MapCoolWarm {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapCoolWarm {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapCoolWarm {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x - 1,
                z: area.z - 1,
                w: area.w + 2,
                h: area.h + 2
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let v11 = pmap.a[(x+1, z+1)];

                m.a[(x, z)] = v11;

                if v11 == 1 {
                    let v10 = pmap.a[(x+1, z+0)];
                    let v21 = pmap.a[(x+2, z+1)];
                    let v01 = pmap.a[(x+0, z+1)];
                    let v12 = pmap.a[(x+1, z+2)];

                    /* t = 3 || 4
                     *
                     * XtX     XtX *
                     * t1t  => t2t *
                     * XtX     XtX */
                    if v10 == 3 || v10 == 4 || v21 == 3 || v21 == 4 || v01 == 3 || v01 == 4 || v12 == 3 || v12 == 4 {
                        m.a[(x, z)] = 2;
                    }
                }
            }
        }

        m
    }
}

pub struct MapHeatIce {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapHeatIce {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapHeatIce {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x - 1,
                z: area.z - 1,
                w: area.w + 2,
                h: area.h + 2
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let v11 = pmap.a[(x+1, z+1)];

                m.a[(x, z)] = v11;

                if v11 == 4 {
                    let v10 = pmap.a[(x+1, z+0)];
                    let v21 = pmap.a[(x+2, z+1)];
                    let v01 = pmap.a[(x+0, z+1)];
                    let v12 = pmap.a[(x+1, z+2)];

                    /* t = 1 || 2
                     *
                     * XtX     XtX *
                     * t4t  => t3t *
                     * XtX     XtX */
                    if v10 == 1 || v10 == 2 || v21 == 1 || v21 == 2 || v01 == 1 || v01 == 2 || v12 == 1 || v12 == 2 {
                        m.a[(x, z)] = 3;
                    }
                }
            }
        }

        m
    }
}

pub struct MapSpecial {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapSpecial {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapSpecial {
    // 1 to 1 mapping with no borders
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let pmap = parent.get_map(area);
            self.get_map_from_pmap(&pmap)
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has no margin: pmap.w == map.w
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let r = McRng::new(self.base_seed, self.world_seed);
        MapParentFn(PanicMap, |x, z, v| {
            let mut r = r;
            let mut v = v;

            if v != 0 {
                r.set_chunk_seed(x, z);
                if r.next_int_n(13) == 0 {
                    // What does this mean?
                    // if v == 1 and here we set it to 0x101..0xF01
                    // then it won't trigger any v == 1 checks in the future
                    v |= (1 + r.next_int_n(15)) << 8 & 0xf00;
                }
            }

            v
        }).get_map_from_pmap(pmap)
    }
}

pub struct MapAddMushroomIsland {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapAddMushroomIsland {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapAddMushroomIsland {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x - 1,
                z: area.z - 1,
                w: area.w + 2,
                h: area.h + 2
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        let mut r = McRng::new(self.base_seed, self.world_seed);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let v00 = pmap.a[(x+0, z+0)];
                let v20 = pmap.a[(x+2, z+0)];
                let v02 = pmap.a[(x+0, z+2)];
                let v22 = pmap.a[(x+2, z+2)];
                let mut v11 = pmap.a[(x+1, z+1)];

                /* 0X0     0X0 *
                 * X0X  => XMX *
                 * 0X0     0X0 */
                if v11 == 0 && v00 == 0 && v20 == 0 && v02 == 0 && v22 == 0 {
                    let chunk_x = x as i64 + area.x;
                    let chunk_z = z as i64 + area.z;
                    r.set_chunk_seed(chunk_x, chunk_z);
                    // TODO: great attack surface, this is the only way to
                    // spawn a mushroom island, the scale is 1:256 so we
                    // don't need precise coordinates.
                    // Rewrite this as (r % 4 == 0 && r % 25 == 0)
                    if r.next_int_n(100) == 0 {
                        v11 = 14; // mushroomIsland
                    }
                }

                m.a[(x, z)] = v11;
            }
        }

        m
    }
}

pub struct MapDeepOcean {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapDeepOcean {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapDeepOcean {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x - 1,
                z: area.z - 1,
                w: area.w + 2,
                h: area.h + 2
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let v10 = pmap.a[(x+1, z+0)];
                let v21 = pmap.a[(x+2, z+1)];
                let v01 = pmap.a[(x+0, z+1)];
                let v12 = pmap.a[(x+1, z+2)];
                let mut v11 = pmap.a[(x+1, z+1)];

                if v11 == 0 && v10 == 0 && v21 == 0 && v01 == 0 && v12 == 0 {
                    v11 = 24; // deepOcean
                }

                m.a[(x, z)] = v11;
            }
        }

        m
    }
}

pub struct MapBiome {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapBiome {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapBiome {
    // 1 to 1 mapping with no borders
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let pmap = parent.get_map(area);
            self.get_map_from_pmap(&pmap)
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has no margin: pmap.w == map.w
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        use biome_id::*;
        let warmBiomes = [desert, desert, desert, savanna, savanna, plains];
        let lushBiomes = [forest, roofedForest, extremeHills, plains, birchForest, swampland];
        let coldBiomes = [forest, extremeHills, taiga, plains];
        let snowBiomes = [icePlains, icePlains, icePlains, coldTaiga];
        let r = McRng::new(self.base_seed, self.world_seed);

        MapParentFn(PanicMap, |x, z, v| {
            let mut r = r;
            let mut id = v;

            let has_high_bit = ((id & 0xf00) >> 8) != 0;
            id &= !0xf00;
            if get_category(MinecraftVersion::Java1_16, id) == Some(Ocean) || id == mushroomIsland {
                return id;
            }

            r.set_chunk_seed(x, z);

            match id {
                Warm => {
                    if has_high_bit {
                        if r.next_int_n(3) == 0 {
                            mesaPlateau
                        } else {
                            mesaPlateau_F
                        }
                    } else {
                        warmBiomes[r.next_int_n(6) as usize]
                    }
                }
                Lush => {
                    if has_high_bit {
                        jungle
                    } else {
                        lushBiomes[r.next_int_n(6) as usize]
                    }
                }
                Cold => {
                    if has_high_bit {
                        megaTaiga
                    } else {
                        coldBiomes[r.next_int_n(4) as usize]
                    }
                }
                Freezing => {
                    snowBiomes[r.next_int_n(4) as usize]
                }
                _ => {
                    mushroomIsland
                }
            }
        }).get_map_from_pmap(pmap)
    }
}

fn is_shallow_ocean(id: i32) -> bool {
    use biome_id::*;
    match id {
        ocean | warmOcean | lukewarmOcean | coldOcean | frozenOcean => true,
        _ => false,
    }
}

fn is_deep_ocean(id: i32) -> bool {
    use biome_id::*;
    match id {
        deepOcean | warmDeepOcean | lukewarmDeepOcean | coldDeepOcean | frozenDeepOcean => true,
        _ => false,
    }
}

fn equal_or_plateau(version: MinecraftVersion, id1: i32, id2: i32) -> bool {
    use biome_id::*;
    if id1 == id2 {
        return true;
    }
    if id1 == mesaPlateau_F || id1 == mesaPlateau {
        return id2 == mesaPlateau_F || id2 == mesaPlateau;
    }
    if !biome_exists(id1) || !biome_exists(id2) {
        return false;
    }
    // adjust for asymmetric equality (workaround to simulate a bug in the MC java code)
    if id1 >= 128 || id2 >= 128 {
        // skip biomes that did not overload the isEqualTo() method
        if id2 == 130 || id2 == 133 || id2 == 134 || id2 == 149 || id2 == 151 || id2 == 155 ||
           id2 == 156 || id2 == 157 || id2 == 158 || id2 == 163 || id2 == 164 {
               return false;
           }
    }

    get_category(version, id1) == get_category(version, id2)
}

fn replace_edge(version: MinecraftVersion, out: &mut i32, v10: i32, v21: i32, v01: i32, v12: i32, id: i32, base_id: i32, edge_id: i32) -> bool {
    if id != base_id {
        return false;
    }

    if [v10, v21, v01, v12].iter().all(|&x| equal_or_plateau(version, x, base_id)) {
        *out = id;
    } else {
        *out = edge_id;
    }

    true
}

pub struct MapBiomeEdge {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapBiomeEdge {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapBiomeEdge {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x - 1,
                z: area.z - 1,
                w: area.w + 2,
                h: area.h + 2
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        use biome_id::*;
        let version = MinecraftVersion::Java1_16;
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let v10 = pmap.a[(x+1, z+0)];
                let v21 = pmap.a[(x+2, z+1)];
                let v01 = pmap.a[(x+0, z+1)];
                let v12 = pmap.a[(x+1, z+2)];
                let v11 = pmap.a[(x+1, z+1)];

                if !replace_edge(version, &mut m.a[(x, z)], v10, v21, v01, v12, v11, mesaPlateau_F, mesa) &&
                !replace_edge(version, &mut m.a[(x, z)], v10, v21, v01, v12, v11, mesaPlateau, mesa) &&
                !replace_edge(version, &mut m.a[(x, z)], v10, v21, v01, v12, v11, megaTaiga, taiga)
                    {
                    m.a[(x, z)] = match v11 {
                        desert => {
                            if v10 != icePlains && v21 != icePlains && v01 != icePlains && v12 != icePlains {
                                v11
                            } else {
                                extremeHillsPlus
                            }
                        }
                        swampland => {
                            if v10 != desert && v21 != desert && v01 != desert && v12 != desert &&
                               v10 != coldTaiga && v21 != coldTaiga && v01 != coldTaiga && v12 != coldTaiga &&
                               v10 != icePlains && v21 != icePlains && v01 != icePlains && v12 != icePlains {
                                if v10 != jungle && v12 != jungle && v21 != jungle && v01 != jungle
                                    // TODO: bambooJungle is from 1.14
                                    && v10 != bambooJungle && v12 != bambooJungle && v21 != bambooJungle
                                    && v01 != bambooJungle {
                                    v11
                                } else {
                                    jungleEdge
                                }
                            } else {
                                plains
                            }
                        }
                        _ => {
                            v11
                        }
                    };
                }
            }
        }

        m
    }
}

pub struct MapRiverInit {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapRiverInit {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapRiverInit {
    // 1 to 1 mapping with no borders
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let pmap = parent.get_map(area);
            self.get_map_from_pmap(&pmap)
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has no margin: pmap.w == map.w
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let r = McRng::new(self.base_seed, self.world_seed);
        MapParentFn(PanicMap, |x, z, v| {
            if v > 0 {
                let mut r = r;
                r.set_chunk_seed(x, z);
                r.next_int_n(299999) + 2
            } else {
                0
            }
        }).get_map_from_pmap(pmap)
    }
}

pub fn pretty_biome_map_hills(id: i32) -> i32 {
    if id == 0 {
        0
    } else {
        match (id - 2) % 29 {
            0 => 2,
            1 => 3,
            _ => 255,
        }
    }
}

/// This layer uses 64 bits
pub struct MapHills {
    base_seed: i64,
    world_seed: i64,
    mc_version: MinecraftVersion,
    pub parent1: Option<Rc<dyn GetMap>>,
    pub parent2: Option<Rc<dyn GetMap>>,
}

impl MapHills {
    pub fn new(base_seed: i64, world_seed: i64, mc_version: MinecraftVersion) -> Self {
        Self { base_seed, world_seed, mc_version, parent1: None, parent2: None }
    }
    pub fn get_map_from_pmap12(&self, pmap1: &Map, pmap2: &Map) -> Map {
        use biome_id::*;
        let (p_w, p_h) = pmap1.a.dim();
        {
            // Check that both maps are of same size and coords
            assert_eq!(pmap1.area(), pmap2.area());
        }
        let area = Area {
            x: pmap1.x + 1,
            z: pmap1.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        let mut r = McRng::new(self.base_seed, self.world_seed);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let chunk_x = x as i64 + m.x;
                let chunk_z = z as i64 + m.z;
                r.set_chunk_seed(chunk_x, chunk_z);
                let a11 = pmap1.a[(x+1, z+1)]; // biome
                let b11 = pmap2.a[(x+1, z+1)]; // river

                let var12 = (b11 - 2) % 29 == 0;

                m.a[(x, z)] = if b11 >= 2 && (b11 - 2) % 29 == 1 && !is_shallow_ocean(a11) {
                    get_mutated(self.mc_version, a11).unwrap_or(a11)
                } else if r.next_int_n(3) != 0 && !var12 {
                    a11
                } else {
                    let mut hill_id = match a11 {
                        desert => desertHills,
                        forest => forestHills,
                        birchForest => birchForestHills,
                        roofedForest => plains,
                        taiga => taigaHills,
                        megaTaiga => megaTaigaHills,
                        coldTaiga => coldTaigaHills,
                        plains => if r.next_int_n(3) == 0 { forestHills } else { forest },
                        icePlains => iceMountains,
                        jungle => jungleHills,
                        bambooJungle => bambooJungleHills, // TODO: 1.14
                        ocean => deepOcean,
                        extremeHills => extremeHillsPlus,
                        savanna => savannaPlateau,
                        _ => if equal_or_plateau(self.mc_version, a11, mesaPlateau_F) {
                            mesa
                        } else if is_deep_ocean(a11) && r.next_int_n(3) == 0 {
                            // TODO: is_deep_ocean was introduced in 1.13
                            if r.next_int_n(2) == 0 { plains } else { forest }
                        } else {
                            a11
                        }
                    };

                    if var12 && hill_id != a11 {
                        hill_id = get_mutated(self.mc_version, hill_id).unwrap_or(a11);
                    }

                    if hill_id == a11 {
                        a11
                    } else {
                        let a10 = pmap1.a[(x+1, z+0)];
                        let a21 = pmap1.a[(x+2, z+1)];
                        let a01 = pmap1.a[(x+0, z+1)];
                        let a12 = pmap1.a[(x+1, z+2)];
                        let mut equals = 0;
                        if equal_or_plateau(self.mc_version, a10, a11) { equals += 1; }
                        if equal_or_plateau(self.mc_version, a21, a11) { equals += 1; }
                        if equal_or_plateau(self.mc_version, a01, a11) { equals += 1; }
                        if equal_or_plateau(self.mc_version, a12, a11) { equals += 1; }

                        if equals >= 3 {
                            hill_id
                        } else {
                            a11
                        }
                    }
                }
            }
        }

        m
    }
}

impl GetMap for MapHills {
    fn get_map(&self, area: Area) -> Map {
        if let (Some(ref parent1), Some(ref parent2)) = (&self.parent1, &self.parent2) {
            let parea = Area {
                x: area.x - 1,
                z: area.z - 1,
                w: area.w + 2,
                h: area.h + 2
            };
            let pmap1 = parent1.get_map(parea);
            let pmap2 = parent2.get_map(parea);

            let map = self.get_map_from_pmap12(&pmap1, &pmap2);

            // No need to crop
            map
        } else {
            panic!("Parents not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, _pmap: &Map) -> Map {
        panic!("MapHills requires 2 pmaps!");
    }
}

pub struct MapRareBiome {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapRareBiome {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapRareBiome {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let pmap = parent.get_map(area);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    // TODO: this layer does not need the margin?
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let r = McRng::new(self.base_seed, self.world_seed);
        MapParentFn(PanicMap, |x, z, v| {
            use biome_id::*;

            let mut r = r;
            r.set_chunk_seed(x, z);

            if r.next_int_n(57) == 0 && v == plains {
                // Sunflower Plains
                plains + 128
            } else {
                v
            }
        }).get_map_from_pmap(pmap)
    }
}

pub struct MapShore {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapShore {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapShore {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x - 1,
                z: area.z - 1,
                w: area.w + 2,
                h: area.h + 2
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        use biome_id::*;
        // Helper function to simplify repeated logic
        fn replace_ocean(out: &mut i32, v10: i32, v21: i32, v01: i32, v12: i32, id: i32, replace_id: i32) -> bool {
            if is_oceanic(id) {
                return false;
            }
            if !is_oceanic(v10) && !is_oceanic(v21) && !is_oceanic(v01) && !is_oceanic(v12) {
                *out = id;
            } else {
                *out = replace_id;
            }

            true
        }

        let get_category = |id| {
            get_category(MinecraftVersion::Java1_16, id)
        };

        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let v11 = pmap.a[(x+1, z+1)];

                let v10 = pmap.a[(x+1, z+0)];
                let v21 = pmap.a[(x+2, z+1)];
                let v01 = pmap.a[(x+0, z+1)];
                let v12 = pmap.a[(x+1, z+2)];

                let biome = if biome_exists(v11) { v11 } else { 0 };

                m.a[(x, z)] = if v11 == mushroomIsland {
                   if v10 != ocean && v21 != ocean && v01 != ocean && v12 != ocean {
                       v11
                   } else {
                       mushroomIslandShore
                   }
                } else if /* biome < 128 && */ get_category(biome) == Some(Jungle) {
                    if is_biome_JFTO(v10) && is_biome_JFTO(v21) && is_biome_JFTO(v01) && is_biome_JFTO(v12) {
                        if !is_oceanic(v10) && !is_oceanic(v21) && !is_oceanic(v01) && !is_oceanic(v12) {
                            v11
                        } else {
                            beach
                        }
                    } else {
                        jungleEdge
                    }
                } else if v11 != extremeHills && v11 != extremeHillsPlus && v11 != extremeHillsEdge {
                    if is_biome_snowy(biome) {
                        let mut x = v11;
                        replace_ocean(&mut x, v10, v21, v01, v12, v11, coldBeach);
                        x
                    } else if v11 != mesa && v11 != mesaPlateau_F {
                        if v11 != ocean && v11 != deepOcean && v11 != river && v11 != swampland {
                            if !is_oceanic(v10) && !is_oceanic(v21) && !is_oceanic(v01) && !is_oceanic(v12) {
                                v11
                            } else {
                                beach
                            }
                        } else {
                            v11
                        }
                    } else {
                        if !is_oceanic(v10) && !is_oceanic(v21) && !is_oceanic(v01) && !is_oceanic(v12) {
                            if is_mesa(v10) && is_mesa(v21) && is_mesa(v01) && is_mesa(v12) {
                                v11
                            } else {
                                desert
                            }
                        } else {
                            v11
                        }
                    }
                } else {
                    let mut x = v11;
                    replace_ocean(&mut x, v10, v21, v01, v12, v11, stoneBeach);
                    x
                };
            }
        }

        m
    }
}

pub struct MapSmooth {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapSmooth {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapSmooth {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x - 1,
                z: area.z - 1,
                w: area.w + 2,
                h: area.h + 2
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        let mut r = McRng::new(self.base_seed, self.world_seed);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let mut v11 = pmap.a[(x+1, z+1)];

                let v10 = pmap.a[(x+1, z+0)];
                let v21 = pmap.a[(x+2, z+1)];
                let v01 = pmap.a[(x+0, z+1)];
                let v12 = pmap.a[(x+1, z+2)];
                /*
                 0B0
                 AXa
                 0b0
                if A == a == B == b:
                    X = A
                else if A == a && B == b:
                    X = random.choose(A, B)
                else if A == a:
                    X = A
                else if B == b:
                    X = B
                else:
                    X = X
                 */
                if v01 == v21 && v10 == v12 {
                    let chunk_x = x as i64 + m.x;
                    let chunk_z = z as i64 + m.z;
                    r.set_chunk_seed(chunk_x, chunk_z);

                    if r.next_int_n(2) == 0 {
                        v11 = v01;
                    } else {
                        v11 = v10;
                    }
                } else {
                    if v01 == v21 { v11 = v01 };
                    if v10 == v12 { v11 = v10 };
                }

                m.a[(x, z)] = v11;
            }
        }

        m
    }
}

/// Helper function to classify an input into [0, 1, 2, 3]
/// Used by MapRiver
pub fn reduce_id(id: i32) -> i32 {
    if id >= 2 {
        2 + (id & 1)
    } else {
        id
    }
}

pub struct MapRiver {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapRiver {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapRiver {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x - 1,
                z: area.z - 1,
                w: area.w + 2,
                h: area.h + 2
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        use biome_id::*;
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let v11 = reduce_id(pmap.a[(x+1, z+1)]);
                let v10 = reduce_id(pmap.a[(x+1, z+0)]);
                let v21 = reduce_id(pmap.a[(x+2, z+1)]);
                let v01 = reduce_id(pmap.a[(x+0, z+1)]);
                let v12 = reduce_id(pmap.a[(x+1, z+2)]);

                m.a[(x, z)] = if v11 == v01 && v11 == v10 && v11 == v21 && v11 == v12 {
                    -1
                } else {
                    river
                };
            }
        }

        m
    }
}

/// Like MapRiver, but will generate all the possible rivers for this 26-bit seed
pub struct HelperMapRiverAll {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl HelperMapRiverAll {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for HelperMapRiverAll {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x - 1,
                z: area.z - 1,
                w: area.w + 2,
                h: area.h + 2
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        use biome_id::*;
        let (p_w, p_h) = pmap.a.dim();
        assert!(p_w > 2);
        assert!(p_h > 2);
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let v11 = pmap.a[(x+1, z+1)];
                let v10 = pmap.a[(x+1, z+0)];
                let v21 = pmap.a[(x+2, z+1)];
                let v01 = pmap.a[(x+0, z+1)];
                let v12 = pmap.a[(x+1, z+2)];

                m.a[(x, z)] = if v11 == v01 && v11 == v10 && v11 == v21 && v11 == v12 {
                    -1
                } else {
                    river
                };
            }
        }

        m
    }
}

pub struct MapRiverMix {
    base_seed: i64,
    world_seed: i64,
    // Map parent
    pub parent1: Option<Rc<dyn GetMap>>,
    // River parent
    pub parent2: Option<Rc<dyn GetMap>>,
}

impl MapRiverMix {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent1: None, parent2: None }
    }
    pub fn get_map_from_pmap12(&self, pmap1: &Map, pmap2: &Map) -> Map {
        use biome_id::*;
        let (p_w, p_h) = pmap1.a.dim();
        {
            // Check that both maps are of same size and coords
            assert_eq!(pmap1.area(), pmap2.area());
        }
        let mut m = pmap1.clone();
        for x in 0..p_w as usize {
            for z in 0..p_h as usize {
                let buf = pmap1.a[(x, z)];
                let out = pmap2.a[(x, z)];
                m.a[(x, z)] = if is_oceanic(buf) {
                    buf
                } else {
                    if out == river {
                        if buf == icePlains {
                            frozenRiver
                        } else if buf == mushroomIsland || buf == mushroomIslandShore {
                            mushroomIslandShore
                        } else {
                            out & 0xFF
                        }
                    } else {
                        buf
                    }
                };
            }
        }

        m
    }
}

impl GetMap for MapRiverMix {
    // 1 to 1 mapping with no borders
    fn get_map(&self, area: Area) -> Map {
        if let (Some(ref parent1), Some(ref parent2)) = (&self.parent1, &self.parent2) {
            let parea = Area {
                x: area.x,
                z: area.z,
                w: area.w,
                h: area.h
            };
            let pmap1 = parent1.get_map(parea);
            let pmap2 = parent2.get_map(parea);

            let map = self.get_map_from_pmap12(&pmap1, &pmap2);

            // No need to crop
            map
        } else {
            panic!("Parents not set");
        }
    }

    // pmap has no margin: pmap.w == map.w
    fn get_map_from_pmap(&self, _pmap: &Map) -> Map {
        panic!("MapRiverMix requires 2 pmaps!")
    }
}

pub struct MapOceanTemp {
    base_seed: i64,
    world_seed: i64,
    perlin: NoiseGeneratorPerlin,
}

impl MapOceanTemp {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, perlin: NoiseGeneratorPerlin::new(world_seed) }
    }
}

impl GetMap for MapOceanTemp {
    fn get_map(&self, area: Area) -> Map {
        MapFn(|Point {x, z}| {
            use biome_id::*;

            let tmp = self.perlin.get_ocean_temp(x as f64 / 8.0, z as f64 / 8.0, 0.0);

            if tmp > 0.4 {
                warmOcean
            } else if tmp > 0.2 {
                lukewarmOcean
            } else if tmp < -0.4 {
                frozenOcean
            } else if tmp < -0.2 {
                coldOcean
            } else {
                ocean
            }
        }).get_map(area)
    }

    // MapIsland is the first layer, so it does not need pmap
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let area = pmap.area();

        self.get_map(area)
    }
}


pub struct MapOceanMix {
    base_seed: i64,
    world_seed: i64,
    // Map parent
    pub parent1: Option<Rc<dyn GetMap>>,
    // Ocean parent
    pub parent2: Option<Rc<dyn GetMap>>,
}

impl MapOceanMix {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent1: None, parent2: None }
    }
    pub fn get_map_from_pmap12(&self, pmap1: &Map, pmap2: &Map) -> Map {
        use biome_id::*;
        let (p_w, p_h) = pmap2.a.dim();
        {
            // Check that both maps have the expected size and offset
            let area = pmap2.area();
            let land_area = Area {
                x: area.x - 8,
                z: area.z - 8,
                w: area.w + 17,
                h: area.h + 17,
            };
            assert_eq!(pmap1.area(), land_area);
        }

        let mut m = pmap2.clone();
        for x in 0..p_w as usize {
            'loop_z: for z in 0..p_h as usize {
                let land_id = pmap1.a[(x+8, z+8)];
                let mut ocean_id = pmap2.a[(x, z)];

                if !is_oceanic(land_id) {
                    m.a[(x, z)] = land_id;
                    continue;
                }

                // Optimization: this loop is only useful when ocean_id is
                // warm or frozen
                if ocean_id == warmOcean || ocean_id == frozenOcean {
                    for i in 0..=4 {
                        for j in 0..=4 {
                            let i = i * 4;
                            let j = j * 4;
                            let nearby_id = pmap1.a[(x + i, z + j)];

                            if is_oceanic(nearby_id) {
                                continue;
                            }

                            if ocean_id == warmOcean {
                                m.a[(x, z)] = lukewarmOcean;
                                continue 'loop_z;
                            }

                            if ocean_id == frozenOcean {
                                m.a[(x, z)] = coldOcean;
                                continue 'loop_z;
                            }
                        }
                    }
                }

                if land_id == deepOcean {
                    ocean_id = match ocean_id {
                        lukewarmOcean => lukewarmDeepOcean,
                        ocean => deepOcean,
                        coldOcean => coldDeepOcean,
                        frozenOcean => frozenDeepOcean,
                        _ => ocean_id,
                    };
                }

                m.a[(x, z)] = ocean_id;
            }
        }

        m
    }
}

impl GetMap for MapOceanMix {
    fn get_map(&self, area: Area) -> Map {
        if let (Some(ref parent1), Some(ref parent2)) = (&self.parent1, &self.parent2) {
            let land_area = Area {
                x: area.x - 8,
                z: area.z - 8,
                w: area.w + 17,
                h: area.h + 17,
            };
            let parea = Area {
                x: area.x,
                z: area.z,
                w: area.w,
                h: area.h
            };
            let pmap1 = parent1.get_map(land_area);
            let pmap2 = parent2.get_map(parea);

            let map = self.get_map_from_pmap12(&pmap1, &pmap2);

            // No need to crop
            map
        } else {
            panic!("Parents not set");
        }
    }

    // pmap has no margin: pmap.w == map.w
    fn get_map_from_pmap(&self, _pmap: &Map) -> Map {
        panic!("MapOceanMix requires 2 pmaps!")
    }
}

pub struct MapSkip {
    zoom_factor: u8,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapSkip {
    /// Zoom factor: n in 2^n
    /// 0: same as parent
    /// 1: 2x zoom in each direction
    /// 2: 4x zoom in each direction
    pub fn new(parent: Rc<dyn GetMap>, zoom_factor: u8) -> Self {
        if zoom_factor >= 2 {
            Self {
                zoom_factor: 1,
                parent: Some(Rc::new(Self::new(parent, zoom_factor - 1))),
            }
        } else {
            Self {
                zoom_factor,
                parent: Some(parent),
            }
        }
    }
}

// TODO: tests
impl GetMap for MapSkip {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            match self.zoom_factor {
                0 => parent.get_map(area),
                1 => {
                    let parea = Area {
                        x: area.x >> 1,
                        z: area.z >> 1,
                        w: (area.w >> 1) + 2,
                        h: (area.h >> 1) + 2
                    };
                    let pmap = parent.get_map(parea);

                    let mut map = self.get_map_from_pmap(&pmap);
                    // TODO: is this correct?
                    let (nx, nz) = ((area.x) & 1, (area.z) & 1);
                    map.x += nx;
                    map.z += nz;
                    let (nx, nz) = (nx as usize, nz as usize);
                    map.a.slice_collapse(s![
                            nx..nx + area.w as usize,
                            nz..nz + area.h as usize
                    ]);

                    map
                }
                _ => {
                    unimplemented!()
                }
            }
        } else {
            panic!("Parent not set");
        }
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        match self.zoom_factor {
            0 => pmap.clone(),
            1 => {
                let (p_w, p_h) = pmap.a.dim();
                let area = Area {
                    x: pmap.x << 1,
                    z: pmap.z << 1,
                    w: ((p_w - 1) << 1) as u64,
                    h: ((p_h - 1) << 1) as u64
                };

                let mut map = Map::new(area);

                for x in 0..p_w - 1 {
                    for z in 0..p_h - 1 {
                        let a = pmap.a[(x, z)];
                        map.a[((x << 1) + 0, (z << 1) + 0)] = a;
                        map.a[((x << 1) + 0, (z << 1) + 1)] = a;
                        map.a[((x << 1) + 1, (z << 1) + 0)] = a;
                        map.a[((x << 1) + 1, (z << 1) + 1)] = a;
                    }
                }

                map
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

pub struct MapAddBamboo {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapAddBamboo {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapAddBamboo {
    // 1 to 1 mapping with no borders
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let pmap = parent.get_map(area);
            self.get_map_from_pmap(&pmap)
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has no margin: pmap.w == map.w
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let r = McRng::new(self.base_seed, self.world_seed);
        MapParentFn(PanicMap, |x, z, v| {
            use biome_id::*;

            if v == jungle {
                let mut r = r;
                r.set_chunk_seed(x, z);
                if r.next_int_n(10) == 0 {
                    return bambooJungle;
                }
            }

            v
        }).get_map_from_pmap(pmap)
    }
}

pub struct MapAddIsland13 {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapAddIsland13 {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapAddIsland13 {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x - 1,
                z: area.z - 1,
                w: area.w + 2,
                h: area.h + 2
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        use biome_id::*;

        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        let mut r = McRng::new(self.base_seed, self.world_seed);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let v00 = pmap.a[(x+0, z+0)];
                let v20 = pmap.a[(x+2, z+0)];
                let v02 = pmap.a[(x+0, z+2)];
                let v22 = pmap.a[(x+2, z+2)];
                let v11 = pmap.a[(x+1, z+1)];

                m.a[(x, z)] = if v11 == 0 && (v00 != 0 || v20 != 0 || v02 != 0 || v22 != 0) {
                    let chunk_x = x as i64 + area.x;
                    let chunk_z = z as i64 + area.z;
                    r.set_chunk_seed(chunk_x, chunk_z);

                    let mut v = 1;
                    let mut inc = 1;

                    if v00 != 0 {
                        // nextInt(1) is always 0
                        if r.next_int_n(inc) == 0 {
                            v = v00;
                        }
                        inc += 1;
                    }
                    if v20 != 0 {
                        if r.next_int_n(inc) == 0 {
                            v = v20;
                        }
                        inc += 1;
                    }
                    if v02 != 0 {
                        if r.next_int_n(inc) == 0 {
                            v = v02;
                        }
                        inc += 1;
                    }
                    if v22 != 0 {
                        if r.next_int_n(inc) == 0 {
                            v = v22;
                        }
                    }
                    if r.next_int_n(3) == 0 {
                        v
                    } else if v == icePlains {
                        frozenOcean
                    } else {
                        0
                    }
                } else if v11 > 0 && (v00 == 0 || v20 == 0 || v02 == 0 || v22 == 0) {
                    let chunk_x = x as i64 + area.x;
                    let chunk_z = z as i64 + area.z;
                    r.set_chunk_seed(chunk_x, chunk_z);
                    if r.next_int_n(5) == 0 {
                        if v11 == icePlains { frozenOcean } else { 0 }
                    } else {
                        v11
                    }
                } else {
                    v11
                };
            }
        }

        m
    }
}

pub struct MapIcePlains {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapIcePlains {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapIcePlains {
    // 1 to 1 mapping with no borders
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let pmap = parent.get_map(area);
            self.get_map_from_pmap(&pmap)
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has no margin: pmap.w == map.w
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let r = McRng::new(self.base_seed, self.world_seed);
        MapParentFn(PanicMap, |x, z, v| {
            use biome_id::*;

            if v == 0 {
                0
            } else {
                let mut r = r;
                r.set_chunk_seed(x, z);
                if r.next_int_n(5) == 0 {
                    icePlains
                } else {
                    1
                }
            }
        }).get_map_from_pmap(pmap)
    }
}

pub struct MapBiome13 {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapBiome13 {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapBiome13 {
    // 1 to 1 mapping with no borders
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let pmap = parent.get_map(area);
            self.get_map_from_pmap(&pmap)
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has no margin: pmap.w == map.w
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        use biome_id::*;
        let biomes = [desert, forest, extremeHills, swampland, plains, taiga, jungle];

        let r = McRng::new(self.base_seed, self.world_seed);
        MapParentFn(PanicMap, |x, z, v| {
            let mut r = r;
            r.set_chunk_seed(x, z);
            if v == 0 {
                0
            } else if v == mushroomIsland {
                mushroomIsland
            } else if v == 1 {
                biomes[r.next_int_n(biomes.len() as i32) as usize]
            } else {
                let random_biome = biomes[r.next_int_n(biomes.len() as i32) as usize];
                if random_biome == taiga {
                    taiga
                } else {
                    icePlains
                }
            }
        }).get_map_from_pmap(pmap)
    }
}

pub struct MapRegionHills {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapRegionHills {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapRegionHills {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x - 1,
                z: area.z - 1,
                w: area.w + 2,
                h: area.h + 2
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let mut r = McRng::new(self.base_seed, self.world_seed);
        use biome_id::*;
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let a11 = pmap.a[(x+1, z+1)];
                let chunk_x = x as i64 + area.x;
                let chunk_z = z as i64 + area.z;
                r.set_chunk_seed(chunk_x, chunk_z);

                m.a[(x, z)] = if r.next_int_n(3) == 0 {
                    let hill_id = match a11 {
                        desert => desertHills,
                        forest => forestHills,
                        taiga => taigaHills,
                        plains => forest,
                        icePlains => iceMountains,
                        jungle => jungleHills,
                        _ => a11,
                    };

                    if hill_id == a11 {
                        a11
                    } else {
                        let a10 = pmap.a[(x+1, z+0)];
                        let a21 = pmap.a[(x+2, z+1)];
                        let a01 = pmap.a[(x+0, z+1)];
                        let a12 = pmap.a[(x+1, z+2)];

                        if a11 == a10 && a11 == a21 && a11 == a01 && a11 == a12 {
                            hill_id
                        } else {
                            a11
                        }
                    }

                } else {
                    a11
                };
            }
        }

        m
    }
}

pub struct MapMushroomShore {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapMushroomShore {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapMushroomShore {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x - 1,
                z: area.z - 1,
                w: area.w + 2,
                h: area.h + 2
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        use biome_id::*;

        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let v11 = pmap.a[(x+1, z+1)];

                let v10 = pmap.a[(x+1, z+0)];
                let v21 = pmap.a[(x+2, z+1)];
                let v01 = pmap.a[(x+0, z+1)];
                let v12 = pmap.a[(x+1, z+2)];

                m.a[(x, z)] = if v11 == mushroomIsland {
                    if v10 != ocean && v21 != ocean && v01 != ocean && v12 != ocean {
                        v11
                    } else {
                        mushroomIslandShore
                    }
                } else if v11 != ocean && v11 != river && v11 != swampland && v11 != extremeHills {
                    if v10 != ocean && v21 != ocean && v01 != ocean && v12 != ocean {
                        v11
                    } else {
                        beach
                    }
                } else if v11 == extremeHills {
                    if v10 == extremeHills && v21 == extremeHills && v01 == extremeHills && v12 == extremeHills {
                        v11
                    } else {
                        extremeHillsEdge
                    }
                } else {
                    v11
                };
            }
        }

        m
    }
}

pub struct MapSwampRivers {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapSwampRivers {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapSwampRivers {
    // 1 to 1 mapping with no borders
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let pmap = parent.get_map(area);
            self.get_map_from_pmap(&pmap)
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has no margin: pmap.w == map.w
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let r = McRng::new(self.base_seed, self.world_seed);
        MapParentFn(PanicMap, |x, z, v| {
            use biome_id::*;
            let mut r = r;
            r.set_chunk_seed(x, z);

            if (v != swampland || r.next_int_n(6) != 0) && (v != jungle && v != jungleHills || r.next_int_n(8) != 0) {
                v
            } else {
                river
            }
        }).get_map_from_pmap(pmap)
    }
}

pub struct MapRiverInit13 {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapRiverInit13 {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapRiverInit13 {
    // 1 to 1 mapping with no borders
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let pmap = parent.get_map(area);
            self.get_map_from_pmap(&pmap)
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has no margin: pmap.w == map.w
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let r = McRng::new(self.base_seed, self.world_seed);
        MapParentFn(PanicMap, |x, z, v| {
            if v > 0 {
                let mut r = r;
                r.set_chunk_seed(x, z);
                r.next_int_n(2) + 2
            } else {
                0
            }
        }).get_map_from_pmap(pmap)
    }
}

pub struct MapRiver13 {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<dyn GetMap>>,
}

impl MapRiver13 {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None }
    }
}

impl GetMap for MapRiver13 {
    fn get_map(&self, area: Area) -> Map {
        if let Some(ref parent) = self.parent {
            let parea = Area {
                x: area.x - 1,
                z: area.z - 1,
                w: area.w + 2,
                h: area.h + 2
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        use biome_id::*;
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let v11 = reduce_id(pmap.a[(x+1, z+1)]);
                let v10 = reduce_id(pmap.a[(x+1, z+0)]);
                let v21 = reduce_id(pmap.a[(x+2, z+1)]);
                let v01 = reduce_id(pmap.a[(x+0, z+1)]);
                let v12 = reduce_id(pmap.a[(x+1, z+2)]);

                m.a[(x, z)] = if v11 != 0 && v11 == v01 && v11 == v10 && v11 == v21 && v11 == v12 {
                    -1
                } else {
                    river
                };
            }
        }

        m
    }
}

pub struct MapRiverMix13 {
    base_seed: i64,
    world_seed: i64,
    // Map parent
    pub parent1: Option<Rc<dyn GetMap>>,
    // River parent
    pub parent2: Option<Rc<dyn GetMap>>,
}

impl MapRiverMix13 {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent1: None, parent2: None }
    }
    pub fn get_map_from_pmap12(&self, pmap1: &Map, pmap2: &Map) -> Map {
        use biome_id::*;
        let (p_w, p_h) = pmap1.a.dim();
        {
            // Check that both maps are of same size and coords
            assert_eq!(pmap1.area(), pmap2.area());
        }
        let mut m = pmap1.clone();
        for x in 0..p_w {
            for z in 0..p_h {
                let buf = pmap1.a[(x, z)];
                let out = pmap2.a[(x, z)];
                m.a[(x, z)] = if buf == ocean {
                    buf
                } else if out >= 0 {
                    if buf == icePlains {
                        frozenRiver
                    } else if buf == mushroomIsland || buf == mushroomIslandShore {
                        mushroomIslandShore
                    } else {
                        out
                    }
                } else {
                    buf
                };
            }
        }

        m
    }
}

impl GetMap for MapRiverMix13 {
    // 1 to 1 mapping with no borders
    fn get_map(&self, area: Area) -> Map {
        if let (Some(ref parent1), Some(ref parent2)) = (&self.parent1, &self.parent2) {
            let parea = Area {
                x: area.x,
                z: area.z,
                w: area.w,
                h: area.h
            };
            let pmap1 = parent1.get_map(parea);
            let pmap2 = parent2.get_map(parea);

            let map = self.get_map_from_pmap12(&pmap1, &pmap2);

            // No need to crop
            map
        } else {
            panic!("Parents not set");
        }
    }

    // pmap has no margin: pmap.w == map.w
    fn get_map_from_pmap(&self, _pmap: &Map) -> Map {
        panic!("MapRiverMix13 requires 2 pmaps!")
    }
}


/// We lose some information here :/
/// Returns a tuple (BiomeMap, RiverMap)
fn decompose_map_river_mix(map: &Map) -> (SparseMap, SparseMap) {
    use biome_id::*;
    let mut parent1 = SparseMap::new(map.area());
    let mut parent2 = SparseMap::new(map.area());
    for ((x, z), b) in map.a.indexed_iter() {
        match *b {
            frozenRiver => {
                parent1.a[(x, z)] = Some(icePlains);
                parent2.a[(x, z)] = Some(river);
            }
            mushroomIslandShore => {
                // We can not be sure that it was a river
                // It may have been generated by MapShore
            }
            river => {
                parent2.a[(x, z)] = Some(river);
            }
            anything_else => {
                parent1.a[(x, z)] = Some(anything_else);
            }
        }
    }

    (parent1, parent2)
}

// This is supposed to be MathHelper::sin in Minecraft, which uses a lookup
// table, but here it is not implemented and calls the native sin function
// instead
fn fast_sin(x: f32) -> f32 {
    x.sin()
}

// return biome height >= 0
// Used to draw treasure maps
fn is_land_biome(biome_id: i32) -> bool {
    BIOME_INFO[biome_id as usize].height >= 0.0
}

/// Set the border pixels to a particular value
/// This is used to add 1-pixel padding to treasure maps, since ingame maps always have 128x128
/// resolution, but treasure maps are 126x126
fn set_pixels_at_margin(map: &mut Map, value: i32) {
    let area = map.area();

    for x in 0..area.h as usize {
        map.a[(x, 0)] = value;
        map.a[(x, area.w as usize - 1)] = value;
    }

    for z in 0..area.w as usize {
        map.a[(0, z)] = value;
        map.a[(area.h as usize - 1, z)] = value;
    }
}

pub fn add_margin_to_map(map: &Map, value: i32) -> Map {
    let parea = map.area();
    let area = Area {
        x: parea.x - 1,
        z: parea.x - 1,
        w: parea.w + 2,
        h: parea.h + 2,
    };

    let mut m = Map::new(area);

    for x in 0..parea.w as usize {
        for z in 0..parea.h as usize {
            m.a[(x + 1, z + 1)] = map.a[(x, z)];
        }
    }

    set_pixels_at_margin(&mut m, value);

    m
}

/// Apply the unexplored treasure map filter
// This is not a world generation layer
// The output of this Map is not biome_id, but color_id.
// Use the treasure_map_to_color function to convert it to RGB values
pub struct MapTreasure {
    pub parent: Rc<dyn GetMap>,
}

impl GetMap for MapTreasure {
    fn get_map(&self, area: Area) -> Map {
        let parea = Area {
            x: area.x - 1,
            z: area.z - 1,
            w: area.w + 2,
            h: area.h + 2
        };
        let pmap = self.parent.get_map(parea);
        let map = self.get_map_from_pmap(&pmap);

        // No need to crop
        map
    }

    // pmap has 1 wide margin on each size: pmap.w == map.w + 2
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        // TODO: only 1:1 maps are implemented
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let coords_in_fragment = |x: i64, z: i64| -> (u8, u8) {
            // Input: from -32 + (128 * kx) to 95 + (128 * kz)
            // Output: from 0 to 127
            (((x + 32) & 0x7F) as u8, ((z + 32) & 0x7F) as u8)
        };
        let mut m = Map::new(area);

        for x in 0..area.w as usize {
            for z in 0..area.h as usize {
                let mut num_water_neighbors = 8;

                for i in 0..3 {
                    for j in 0..3 {
                        if i == 1 && j == 1 {
                            continue;
                        }
                        if is_land_biome(pmap.a[(x+i, z+j)]) {
                            num_water_neighbors -= 1;
                        }
                    }
                }

                // Land color. Default: black (transparent).
                let color_land = 0;
                // Water color.
                let color_water = 15;
                // Land-water border color.
                let color_shore = 26;
                let color;
                let color_variant;

                let v11 = pmap.a[(x+1, z+1)];

                if !is_land_biome(v11) {
                    // If v11 is water
                    // xf and zf are the coordinates inside the map fragment
                    // must be in range [0, 127]
                    let (xf, zf) = coords_in_fragment(area.x + x as i64, area.z + z as i64);
                    if num_water_neighbors > 7 && zf % 2 == 0 {
                        color = color_water;
                        let mut random_int_5 = (xf as i32 + (fast_sin((zf as f32) + 0.0) * 7.0) as i32) / 8 % 5;
                        // Map color_variant from (0, 1, 2, 3, 4) to (0, 1, 2, 1, 0)
                        if random_int_5 == 3 {
                            random_int_5 = 1;
                        } else if random_int_5 == 4 {
                            random_int_5 = 0;
                        }
                        color_variant = random_int_5;
                    } else if num_water_neighbors > 7 {
                        color = color_land;
                        color_variant = 3;
                    } else if num_water_neighbors > 5 {
                        color = color_water;
                        color_variant = 1;
                    } else if num_water_neighbors > 3 {
                        color = color_water;
                        color_variant = 0;
                    } else if num_water_neighbors > 1 {
                        color = color_water;
                        color_variant = 0;
                    } else {
                        color = color_water;
                        color_variant = 3;
                    }
                } else if num_water_neighbors > 0 {
                    // If v11 is land but at least one of the 8-connected neighbors is water
                    color = color_shore;
                    if num_water_neighbors > 3 {
                        color_variant = 1;
                    } else {
                        color_variant = 3;
                    }
                } else {
                    // If v11 is land and all of the 8-connected neighbors are also land
                    color = color_land;
                    color_variant = 3;
                }

                if color != color_land {
                    // color_variant is always in [0, 3]
                    m.a[(x, z)] = color * 4 + color_variant;
                }
            }
        }

        m
    }
}

pub fn reverse_map_treasure(m: &Map) -> Map {
    // Input: color and variant
    // Output: ocean or plains
    //
    // Possible inputs:
    // * color_land: v11 is land and all of the 8-connected neighbors are also land
    //     or v11 is water and all of the 8-connected neighbors are water
    // * color_shore: v11 is land but some of the 8-connected neighbors are water
    //     * variant=1: num_water_neighbors >= 4
    //     * variant=3: num_water_neighbors < 4
    // * color_water: v11 is water
    let area = m.area();
    let mut o = Map::new(area);

    // Land color. Default: black (transparent).
    const COLOR_LAND: i32 = 0;
    // Water color.
    const COLOR_WATER: i32 = 15;
    // Land-water border color.
    const COLOR_SHORE: i32 = 26;

    let into_color_and_variant = |x| {
        (x / 4, x % 4)
    };

    let water = 0;
    let land = 1;
    let unknown = 255;

    // Set output map to unknown
    for x in 0..area.w as usize {
        for z in 0..area.h as usize {
            o.a[(x, z)] = unknown;
        }
    }

    // Set shore to land and water to water
    for x in 0..area.w as usize {
        for z in 0..area.h as usize {
            match into_color_and_variant(m.a[(x, z)]) {
                (COLOR_SHORE, _variant) => o.a[(x, z)] = land,
                (COLOR_WATER, _variant) => o.a[(x, z)] = water,
                (63, 3) => {
                    // This pixel is unknown
                    // If it is surrounded by 7 shore and 1 water, set it to water
                    // This fixes some tools that do not support water3 color variant
                    // TODO: is it possible to be surrounded by 8 shore? If so, can we be sure that
                    // the pixel should be set to water?
                    let mut count_shore = 0;
                    let mut count_water = 0;

                    // Only check pixels that are not at the border
                    if x >= 1 && x < area.w as usize - 1 && z >= 1 && z < area.h as usize - 1 {
                        for i in 0..3 {
                            for j in 0..3 {
                                if i == 1 && j == 1 {
                                    continue;
                                }
                                let value = m.a[(x+i-1, z+j-1)];
                                match into_color_and_variant(value) {
                                    (COLOR_SHORE, _variant) => count_shore += 1,
                                    // TODO: is may be possible that the color variant is not 0
                                    (COLOR_WATER, 0) => count_water += 1,
                                    _ => {},
                                }
                            }
                        }
                        if count_shore == 7 && count_water == 1 {
                            o.a[(x, z)] = water;
                        }
                    }
                },
                _ => {}
            }
        }
    }

    // color_land indicates that the pixel should have the same value as all of its 8 neighbors
    // This is probably the least efficient way to implement this, but it is correct
    let mut map_changed = true;
    while map_changed {
        map_changed = false;
        for x in 1..(area.w - 1) as usize {
            for z in 1..(area.h - 1) as usize {
                match into_color_and_variant(m.a[(x, z)]) {
                    (COLOR_LAND, 0) => {
                        // Check the 8 neighbors to see if any of them has value different from
                        // "unknown". If so, assume that this is the value of the entire group.
                        let mut n_value = unknown;

                        for i in 0..3 {
                            for j in 0..3 {
                                let value = o.a[(x+i-1, z+j-1)];
                                if value != unknown {
                                    if n_value == unknown {
                                        n_value = value;
                                    } else if n_value != value {
                                        panic!("Not all 8 neighbors have the same value but they should. This treasure map is malformed");
                                    }
                                }
                            }
                        }

                        if n_value != unknown {
                            // Set value of pixel and of all its neighbors
                            for i in 0..3 {
                                for j in 0..3 {
                                    let old_value = o.a[(x+i-1, z+j-1)];
                                    if old_value != n_value {
                                        map_changed = true;
                                    }
                                    o.a[(x+i-1, z+j-1)] = n_value;
                                }
                            }
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    o
}

// TODO: this function must do the reverse of edge detection
pub fn reverse_map_river(m: &Map) -> Map {
    let (w, h) = m.a.dim();
    let (p_w, p_h) = (w - 2, h - 2);
    let (p_w, p_h) = (p_w as u64, p_h as u64);
    let mut pmap = Map::new(Area { x: m.x + 1, z: m.z + 1, w: p_w, h: p_h });

    for x in 0..p_w {
        for z in 0..p_h {
            // if v11 is not a river, then all of [v11, v10, v21, v01, v12] are equal
            let (x, z) = (x as usize, z as usize);
            pmap.a[(x, z)] = m.a[(x + 1, z + 1)];
        }
    }

    pmap
}

/// This returns the biome parent of MapRiverMix.
/// Since the rivers actually overwrite some of the info, it is incomplete.
/// The unknown biomes are represented as 0xFF
pub fn reverse_map_river_mix(m: &Map) -> Map {
    decompose_map_river_mix(m).0.unwrap_or(UNKNOWN_BIOME_ID)
}

/// Actually, this works 100% of the time
pub fn reverse_map_zoom(m: &Map) -> Map {
    let (w, h) = m.a.dim();
    let (p_w, p_h) = (w >> 1, h >> 1);
    let (p_w, p_h) = (p_w as u64, p_h as u64);
    let mut pmap = Map::new(Area { x: m.x >> 1, z: m.z >> 1, w: p_w, h: p_h });
    let (fx, fz) = ((m.x & 1) as usize, (m.z & 1) as usize);

    for x in 0..p_w {
        for z in 0..p_h {
            let (x, z) = (x as usize, z as usize);
            pmap.a[(x, z)] = m.a[(fx + (x << 1), fz + (z << 1))];
        }
    }

    pmap
}

pub fn reverse_map_half_voronoi(m: &Map) -> Map {
    // Same as reverse_map_zoom, but we keep odd coordinates instead
    let (w, h) = m.a.dim();
    let (p_w, p_h) = (w >> 1, h >> 1);
    let (p_w, p_h) = (p_w as u64, p_h as u64);
    let mut pmap = Map::new(Area { x: m.x >> 1, z: m.z >> 1, w: p_w, h: p_h });
    let (fx, fz) = ((!m.x & 1) as usize, (!m.z & 1) as usize);

    for x in 0..p_w {
        for z in 0..p_h {
            let (x, z) = (x as usize, z as usize);
            pmap.a[(x, z)] = m.a[(fx + (x << 1), fz + (z << 1))];
        }
    }

    pmap
}

/// Works at least 9/16*100 % of the time
pub fn reverse_map_smooth(m: &Map) -> Map {
    let (w, h) = m.a.dim();
    let (p_w, p_h) = (w - 2, h - 2);
    let (p_w, p_h) = (p_w as u64, p_h as u64);
    let mut pmap = Map::new(Area { x: (m.x + 1), z: (m.z + 1), w: p_w, h: p_h });
    let (fx, fz) = ((m.x & 1) as usize, (m.z & 1) as usize);

    for x in 0..p_w {
        for z in 0..p_h {
            let (x, z) = (x as usize, z as usize);
            // Set each pixel to the same color as the "parent" before MapZoom:
            // [0, 0] = [0, 0]
            // [0, 1] = [0, 0]
            // [1, 0] = [0, 0]
            // [1, 1] = [0, 0]
            // [2, 0] = [2, 0]
            pmap.a[(x, z)] = m.a[(fx + (x & !1), fz + (z & !1))];
        }
    }

    pmap
}

/// Works 99.9 % of the time*
/// p = 0.9992 for each tile
/// The probability of having at least one error in a 30x30 area is 50%
pub fn reverse_map_voronoi_zoom(m: &Map) -> Result<Map, ()> {
    // 0 => 0, 1 => 4, 2 => 4, 3 => 4, 4 => 4
    fn next_multiple_of_4(x: i64) -> i64 {
        (x + 3) & !0x03
    }
    let d4 = |x| x / 4;
    let m4 = |x| x * 4;
    let area = m.area();
    if area.w < 4 || area.h < 4 {
        return Err(());
    }
    // Adjust map so that m.a[(0, 0)] corresponds to (2+4k, 2+4k)
    // 261 => 262
    let (nx, nz) = (next_multiple_of_4(area.x - 2) + 2, next_multiple_of_4(area.z - 2) + 2);
    let (adj_x, adj_z) = (nx - area.x, nz - area.z);
    let (adj_x, adj_z) = (adj_x as usize, adj_z as usize);
    let area = Area { x: nx, z: nz, w: area.w - adj_x as u64, h: area.h - adj_z as u64 };
    let (p_x, p_z) = (d4(area.x - 2), d4(area.z - 2));
    //let (p_x_max, p_z_max) = (d4(area.x + area.w as i64 - 1), d4(area.z + area.h as i64 - 1));
    let (p_w, p_h) = ((area.w + 3) >> 2, (area.h + 3) >> 2);
    //let (p_w, p_h) = (p_x_max - p_x + 1, p_z_max - p_z + 1);
    //let (p_w, p_h) = (p_w as u64, p_h as u64);
    let parea = Area { x: p_x, z: p_z, w: p_w, h: p_h };
    if parea.w == 0 || parea.h == 0 {
        // A zero sized map is useless
        return Err(());
    }
    let mut pmap = Map::new(parea);
    //println!("{:?} vs {:?}", area, parea);

    let adjusted_map = m.a.slice(s![adj_x.., adj_z..]);
    for x in 0..p_w as usize {
        for z in 0..p_h as usize {
            let xx = m4(x as i64) as usize;
            let zz = m4(z as i64) as usize;
            //println!("{:?} => {:?}", (x, z), (xx, zz));
            pmap.a[(x, z)] = adjusted_map[(xx, zz)];
        }
    }

    Ok(pmap)
}

fn slice_to_area(mut m: Map, a: Area) -> Map {
    //debug!("{:?} vs {:?}", m.area(), a);
    let x_diff = a.x - m.x;
    let z_diff = a.z - m.z;
    m.x += x_diff;
    m.z += z_diff;
    let (x_diff, z_diff) = (x_diff as i32, z_diff as i32);
    let (new_w, new_h) = (a.w as i32 + x_diff, a.h as i32 + z_diff);
    //debug!("x_diff: {}, z_diff: {}, new_w: {}, new_h: {}", x_diff, z_diff, new_w, new_h);
    m.a.slice_collapse(s![x_diff..new_w, z_diff..new_h]);
    //debug!("{:?} vs {:?}", m.area(), a);
    assert_eq!(m.area(), a);

    m
}

/// Detect which points are being used for the last layer (hd)
/// and which are used for the reverse_voronoi (prevoronoi)
// TODO: this is mostly useless if we dont implement splitting of big areas:
// it is faster to check many candidates from a small prevoronoi area than to
// check few candidates that have been generated using double the area.
pub fn segregate_coords_prevoronoi_hd(coords: Vec<Point>) -> (Vec<Point>, Vec<Point>) {
    // First, segregate coordinates by their importance in reverse_voronoi:
    // x % 4 == 2 and z % 4 == 2
    let mut prevoronoi_coords = vec![];
    let mut hd_coords = vec![];
    for Point {x, z} in coords {
        if x as u8 % 4 == 2 && z as u8 % 4 == 2 {
            prevoronoi_coords.push(Point {x, z});
        } else {
            hd_coords.push(Point {x, z});
        }
    }

    // Now, try to build Area from other_coords, and duplicate all the
    // voronoi_coords which are inside this area
    let area = Area::from_coords(hd_coords.iter().copied());
    for &Point {x, z} in &prevoronoi_coords {
        if area.contains(x, z) {
            hd_coords.push(Point {x, z});
        }
    }

    (prevoronoi_coords, hd_coords)
}

// A.k.a reverse map voronoi zoom
pub fn convert_hd_coords_into_quarter_scale(coords: &[Point]) -> Vec<Point4> {
    let mut prevoronoi_coords = vec![];

    for p in coords {
        if let Some(p4) = p.into_quarter_scale() {
            prevoronoi_coords.push(p4);
        }
    }

    prevoronoi_coords
}

/// River Seed Finder
pub fn river_seed_finder(river_coords_voronoi: &[Point], extra_biomes: &[(BiomeId, Point)], version: MinecraftVersion) -> Vec<i64> {
    river_seed_finder_range(river_coords_voronoi, extra_biomes, version, 0, 1 << 24)
}

pub fn river_seed_finder_26_range(river_coords_quarter_scale: &[Point4], range_lo: u32, range_hi: u32) -> Vec<i64> {
    // This iterator has 2**24 elements
    let iter25 = McRng::similar_biome_seed_iterator_bits(25).skip(range_lo as usize).take((range_hi - range_lo) as usize);
    let mut target_maps_derived = vec![];
    let river_fragments = split_rivers_into_fragments4(river_coords_quarter_scale);
    let initial_num_river_fragments = river_fragments.len();
    for x in river_fragments {
        let rivers = count_rivers(&x);
        target_maps_derived.push((x, rivers));
    }

    // Sort target maps by river count: most rivers first
    target_maps_derived.sort_unstable_by_key(|(_map, rivers)| !rivers);

    // Keep at most 10 maps
    target_maps_derived.truncate(10);

    // Remove all the maps with less than 10 rivers
    target_maps_derived.retain(|(_map, rivers)| *rivers >= 10);

    // 2 bad maps are needed to discard the seed
    let bad_map_target = match target_maps_derived.len() {
        1 => 1,
        _ => 2,
    };

    let mut max_possible_score = 0;
    for (_map, rivers) in &target_maps_derived {
        max_possible_score += rivers;
    }

    debug!("Max score: {}", max_possible_score);
    debug!("Max bad maps: {}", bad_map_target);
    debug!("Using {} out of {} river maps. Total river count: {}\n{:?}", target_maps_derived.len(), initial_num_river_fragments, max_possible_score, target_maps_derived);

    let mut candidates_26 = vec![];

    'nextseed: for world_seed in iter25 {
        let mut good_maps0 = 0;
        let mut bad_maps0 = 0;
        let mut good_maps1 = 0;
        let mut bad_maps1 = 0;
        let mut check0 = true;
        let mut check1 = true;
        let mut score0 = 0;
        let mut score1 = 0;
        'nextmap: for (target_map, target_score) in &target_maps_derived {
            let area = target_map.area();

            if check0 {
                // Check with bit 25 set to 0
                let candidate_map = candidate_river_map(area, world_seed);
                //debug!("{}", draw_map(&candidate_map));

                // The candidate map will probably have more rivers than the target map
                // Basically, target_map is a subset of candidate_map
                // Except in some rare cases where target_map can have rivers not present
                // in candidate_map.
                let candidate_score = count_rivers_and(&candidate_map, target_map);
                score0 += candidate_score;
                if candidate_score >= target_score * 90 / 100 {
                    good_maps0 += 1;
                } else {
                    bad_maps0 += 1;
                }

                if bad_maps0 >= bad_map_target {
                    check0 = false;
                }
            }

            if check1 {
                // Check with bit 25 set to 1
                // If the area is large enough, we could skip this check if the map
                // with bit 25 set to 0 had very few matches, as the two maps are
                // usually pretty similar at large scales
                let world_seed = world_seed ^ (1 << 25);
                let candidate_map = candidate_river_map(area, world_seed);
                //debug!("{}", draw_map(&candidate_map));

                // The candidate map will probably have more rivers than the target map
                // Basically, target_map is a subset of candidate_map
                // Except in some rare cases where target_map can have rivers not present
                // in candidate_map.
                let candidate_score = count_rivers_and(&candidate_map, target_map);
                score1 += candidate_score;
                if candidate_score >= target_score * 90 / 100 {
                    good_maps1 += 1;
                } else {
                    bad_maps1 += 1;
                }

                if bad_maps1 >= bad_map_target {
                    check1 = false;
                }
            }

            if check0 == false && check1 == false {
                continue 'nextseed;
            }
        }

        if check0 {
            let similar_biome_seed = McRng::similar_biome_seed(world_seed) & ((1 << 26) - 1);
            debug!("{:08X}: {}/{} maps, {}/{} rivers", world_seed, good_maps0, target_maps_derived.len(), score0, max_possible_score);
            debug!("{:08X}: {}/{} maps, {}/{} rivers", similar_biome_seed, good_maps0, target_maps_derived.len(), score0, max_possible_score);
            candidates_26.push(world_seed);
            candidates_26.push(similar_biome_seed);
        }

        if check1 {
            let world_seed = world_seed ^ (1 << 25);
            let similar_biome_seed = McRng::similar_biome_seed(world_seed) & ((1 << 26) - 1);
            debug!("{:08X}: {}/{} maps, {}/{} rivers", world_seed, good_maps1, target_maps_derived.len(), score1, max_possible_score);
            debug!("{:08X}: {}/{} maps, {}/{} rivers", similar_biome_seed, good_maps1, target_maps_derived.len(), score1, max_possible_score);
            candidates_26.push(world_seed);
            candidates_26.push(similar_biome_seed);
        }
    }
    debug!("{:08X?}", candidates_26);
    debug!("26 bit candidates: {}", candidates_26.len());

    candidates_26
}

/// River Seed Finder
///
/// range_lo: 0
/// range_hi: 1 << 24
/// Even though this is a 26-bit bruteforce, we check 4 seeds at a time
pub fn river_seed_finder_range(river_coords_voronoi: &[Point], extra_biomes: &[(BiomeId, Point)], version: MinecraftVersion, range_lo: u32, range_hi: u32) -> Vec<i64> {
    // For the 34-bit voronoi phase we only want to compare hd_coords
    let mut target_maps_hd = vec![];
    let river_fragments = split_rivers_into_fragments(river_coords_voronoi);
    for target_map_voronoi_hd in river_fragments {
        match reverse_map_voronoi_zoom(&target_map_voronoi_hd) {
            Ok(target_map_derived_hd) => {
                // Compare resolution of original and reverse-voronoi + voronoi
                let g43 = MapVoronoiZoom::new(10, 1234);
                let target_rv_voronoi = g43.get_map_from_pmap(&target_map_derived_hd);

                let target_rv_voronoi_area = target_rv_voronoi.area();
                if target_rv_voronoi_area.w <= 2 || target_rv_voronoi_area.h <= 2 {
                    debug!("Map too small, skipping: {:?}", target_rv_voronoi_area);
                    continue;
                }

                let target_map_voronoi_sliced = slice_to_area(target_map_voronoi_hd.clone(), target_rv_voronoi.area());
                // Actually, we only want to compare borders, so use HelperMapRiverAll, which is actually an
                // edge detector
                let target_map_voronoi_sliced = HelperMapRiverAll::new(1, 0).get_map_from_pmap(&target_map_voronoi_sliced);
                let target_score_voronoi_sliced = count_rivers(&target_map_voronoi_sliced);

                target_maps_hd.push((target_map_derived_hd, target_map_voronoi_sliced, target_score_voronoi_sliced));
            }
            Err(()) => {
                debug!("Too few rivers, minimum map size is 8x8");
            },
        }
    }

    // Sort target maps by river count: most rivers first
    target_maps_hd.sort_unstable_by_key(|(_map, _map_sliced, rivers)| !rivers);

    // Keep at most 4 maps
    target_maps_hd.truncate(4);

    // Remove all the maps with less than 40 rivers
    target_maps_hd.retain(|(_map, _map_sliced, rivers)| *rivers >= 40);

    // Ok, begin bruteforce!

    let river_coords_quarter_scale = convert_hd_coords_into_quarter_scale(river_coords_voronoi);
    let candidates_26 = river_seed_finder_26_range(&river_coords_quarter_scale, range_lo, range_hi);

    //let target_maps_hd = vec![(target_map_hd, target_map_voronoi_sliced, target_score_voronoi_sliced)];
    // Now use voronoi zoom to bruteforce the remaining (34-26 = 8 bits)
    let candidates_34 = candidates_26.into_iter().flat_map(|x| {
        let mut v = vec![];
        'nextseed: for seed in 0..(1 << (34 - 26)) {
            let world_seed = x | (seed << 26);
            let g43 = MapVoronoiZoom::new(10, world_seed);
            for (target_map_hd, target_map_voronoi_sliced, target_score_voronoi_sliced) in &target_maps_hd {
                let candidate_voronoi = g43.get_map_from_pmap(target_map_hd);
                let candidate_voronoi = HelperMapRiverAll::new(1, 0).get_map_from_pmap(&candidate_voronoi);
                //debug!("{}", draw_map(&target_map_voronoi_sliced));
                //debug!("{}", draw_map(&candidate_voronoi));
                let candidate_score = count_rivers_and(&candidate_voronoi, target_map_voronoi_sliced);
                // One match is enough to mark this as a candidate
                if candidate_score >= target_score_voronoi_sliced * 90 / 100 {
                    debug!("{:09X}: {}", world_seed, candidate_score);
                    v.push(world_seed);
                    continue 'nextseed;
                }
            }
        }

        v
    }).collect::<Vec<_>>();
    debug!("{:09X?}", candidates_34);
    debug!("34 bit candidates: {}", candidates_34.len());

    // Can't use rivers to find 48 bits because rivers use 64 bits
    // Can't use biomes because biomes also use 64 bits
    // But we can use rivers + extend48 to end the search with a 2^14 bruteforce
    // TODO: insert a filter by structures before the extend48
    let mut candidates_64 = candidates_34.into_iter().flat_map(|x| {
        let mut v = vec![];
        for seed in 0..(1 << (48 - 34)) {
            let world_seed = x | (seed << 34);
            v.extend(JavaRng::extend_long_48(world_seed as u64));
        }

        v
    }).filter_map(|world_seed| {
        let world_seed = world_seed as i64;
        let last_layer = version.num_layers();
        for (target_map, _target_map_voronoi, _voronoi_score) in &target_maps_hd {
            let target_score = count_rivers(target_map);
            let area = target_map.area();
            // Compare only rivers
            //let g41 = generate_up_to_layer(MinecraftVersion::Java1_7, area, world_seed, 41);
            // Compare all biomes (slower)
            let g42 = generate_up_to_layer(version, area, world_seed, last_layer - 1, 0);
            let candidate_score = count_rivers_and(&g42, target_map);
            if candidate_score < target_score * 90 / 100 {
                // Skip this seed
                return None;
            }
        }

        // When most rivers match, try extra biomes
        let mut hits = 0;
        let mut misses = 0;
        let target = extra_biomes.len() * 90 / 100;
        let max_misses = extra_biomes.len() - target;
        for (biome, Point {x, z}) in extra_biomes.iter().cloned() {
            let area = Area { x, z, w: 1, h: 1 };
            let g43 = generate_up_to_layer(version, area, world_seed, last_layer, 0);
            if g43.a[(0, 0)] == biome.0 {
                hits += 1;
            } else {
                misses += 1;
                if misses > max_misses {
                    break;
                }
            }
        }

        if hits >= target {
            debug!("{:016X}: {}/{}", world_seed, hits, extra_biomes.len());
            Some(world_seed)
        } else {
            None
        }
    }).collect::<Vec<_>>();
    candidates_64.sort_unstable();
    debug!("{:016X?}", candidates_64);
    debug!("64 bit candidates: {}", candidates_64.len());

    candidates_64
}

pub fn filter_seeds_using_biomes(candidates: &[i64], extra_biomes: &[(BiomeId, Point)], version: MinecraftVersion) -> Vec<i64> {
    let mut valid_seeds = vec![];
    let last_layer = version.num_layers();

    for world_seed in candidates {
        let world_seed = *world_seed;
        // When most rivers match, try extra biomes
        let mut hits = 0;
        let mut misses = 0;
        let target = extra_biomes.len() * 90 / 100;
        let max_misses = extra_biomes.len() - target;
        for (biome, Point {x, z}) in extra_biomes.iter().cloned() {
            let area = Area { x, z, w: 1, h: 1 };
            let g43 = generate_up_to_layer(version, area, world_seed, last_layer, 0);
            if g43.a[(0, 0)] == biome.0 {
                hits += 1;
            } else {
                misses += 1;
                if misses > max_misses {
                    break;
                }
            }
        }

        if hits >= target {
            debug!("{:016X}: {}/{}", world_seed, hits, extra_biomes.len());
            valid_seeds.push(world_seed);
        }
    }

    valid_seeds
}

/// Treasure Map River Seed Finder
///
/// range_lo: 0
/// range_hi: 1 << 24
/// Even though this is a 26-bit bruteforce, we check 4 seeds at a time
pub fn treasure_map_river_seed_finder(treasure_map: &Map, version: MinecraftVersion, range_lo: u32, range_hi: u32) -> Vec<i64> {
    // Naming
    // _tm: treasure_map, indicates 1:2 scale
    // _pm: previous_map, indicates 1:4 scale, obtained as ReverseMapZoom(treasure_map)
    // _hv: half_voronoi, indicates 1:2 scale sliced as MapZoom(ReverseMapZoom(treasure_map))
    // _hd: indicates 1:1 scale

    let mut river_coords_hd = vec![];
    let mut river_coords_tm = vec![];
    let tarea = treasure_map.area();
    debug!("Treasure map area: {:?}", tarea);
    for x in 0..tarea.w as usize {
        for z in 0..tarea.h as usize {
            if treasure_map.a[(x, z)] == biome_id::river {
                let p = Point { x: (tarea.x + x as i64) * 2, z: (tarea.z + z as i64) * 2 };
                river_coords_hd.push(p);
                let p = Point2 { x: (tarea.x + x as i64) * 1, z: (tarea.z + z as i64) * 1 };
                river_coords_tm.push(p);
            }
        }
    }

    let river_coords_quarter_scale = convert_hd_coords_into_quarter_scale(&river_coords_hd);
    let candidates_26 = river_seed_finder_26_range(&river_coords_quarter_scale, range_lo, range_hi);

    let candidates = if version < MinecraftVersion::Java1_15 {
        let area_tm = Area::from_coords2(river_coords_tm.iter().copied());
        let target_map_tm = map_with_river_at2(&river_coords_tm, area_tm);
        // Reversing from a HalfVoronoiZoom is more or less equivalent to reversing a MapZoom
        let target_map_pm = reverse_map_half_voronoi(&target_map_tm);

        // Compare resolution of original and reverse-voronoi + voronoi
        let g43 = MapHalfVoronoiZoom::new(10, 1234);
        let target_rv_voronoi = g43.get_map_from_pmap(&target_map_pm);

        let target_map_hv = slice_to_area(target_map_tm.clone(), target_rv_voronoi.area());

        debug!("{}", draw_map(&target_map_tm));
        debug!("{}", draw_map(&target_map_pm));
        //debug!("{}", draw_map(&target_map_hv));

        // Actually, we only want to compare borders, so use HelperMapRiverAll, which is actually an
        // edge detector
        let target_map_hv_borders = HelperMapRiverAll::new(1, 0).get_map_from_pmap(&target_map_hv);
        //debug!("area_hv_borders: {:?}", area_hv_borders);
        let target_score_hv = count_rivers(&target_map_hv_borders);

        debug!("Target voronoi score: {}", target_score_hv);
        // Now use voronoi zoom to bruteforce the remaining (34-26 = 8 bits)
        let candidates_34 = candidates_26.into_iter().flat_map(|x| {
            let mut v = vec![];
            for seed in 0..(1 << (34 - 26)) {
                let world_seed = x | (seed << 26);
                let g43 = MapHalfVoronoiZoom::new(10, world_seed);
                let candidate_voronoi = g43.get_map_from_pmap(&target_map_pm);
                //debug!("{}", draw_map(&candidate_voronoi));
                let candidate_voronoi_borders = HelperMapRiverAll::new(1, 0).get_map_from_pmap(&candidate_voronoi);
                //debug!("expected, found");
                //debug!("{}", draw_map(&target_map_hv_borders));
                //debug!("{}", draw_map(&candidate_voronoi_borders));
                let candidate_score = count_rivers_exact(&candidate_voronoi_borders, &target_map_hv_borders);
                if candidate_score >= target_score_hv * 90 / 100 {
                    debug!("{:09X}: {}", world_seed, candidate_score);
                    v.push(world_seed);
                }
            }

            v
        }).collect::<Vec<_>>();
        debug!("{:09X?}", candidates_34);
        debug!("34 bit candidates: {}", candidates_34.len());
        candidates_34
    } else {
        // Starting from 1.15, we need the seed hash to continue the bruteforce, so just return
        // the 26-bit candidates
        candidates_26
    };

    return candidates;
}

fn count_rivers(m: &Map) -> u32 {
    m.a.fold(0, |acc, &x| if x == biome_id::river { acc + 1 } else { acc })
}

// The first map should have more rivers than the second one
fn count_rivers_and(a: &Map, b: &Map) -> u32 {
    assert_eq!(a.area(), b.area());
    ndarray::Zip::from(&a.a).and(&b.a).fold(0, |mut acc, &v11_a, &v11_b| {
        acc += if v11_b == biome_id::river && v11_a == v11_b {
            1
        } else {
            0
        };
        acc
    })
}

fn count_rivers_exact(a: &Map, b: &Map) -> u32 {
    assert_eq!(a.area(), b.area());
    let acc = ndarray::Zip::from(&a.a).and(&b.a).fold(0, |mut acc, &v11_a, &v11_b| {
        acc += if v11_a == biome_id::river && v11_a == v11_b {
            1
        } else if v11_a == biome_id::river || v11_b == biome_id::river {
            -1
        } else {
            0
        };
        acc
    });

    if acc < 0 { 0 } else { acc as u32 }
}

pub fn map_with_river_at(c: &[Point], area: Area) -> Map {
    let mut m = Map::new(area);
    for Point {x, z} in c {
        m.a[((x - area.x) as usize, (z - area.z) as usize)] = biome_id::river;
    }
    m
}

pub fn map_with_river_at2(c: &[Point2], area: Area) -> Map {
    let mut m = Map::new(area);
    for Point2 {x, z} in c {
        m.a[((x - area.x) as usize, (z - area.z) as usize)] = biome_id::river;
    }
    m
}

pub fn map_with_river_at4(c: &[Point4], area: Area) -> Map {
    let mut m = Map::new(area);
    for Point4 {x, z} in c {
        m.a[((x - area.x) as usize, (z - area.z) as usize)] = biome_id::river;
    }
    m
}

/// Segregate a list of river coordinates into small maps
pub fn split_rivers_into_fragments(points: &[Point]) -> Vec<Map> {
    let mut h: HashMap<(i64, i64), Vec<Point>> = HashMap::new();

    let frag_size_log2 = 6;
    // Split points into fragments of size 64x64
    for p in points {
        let (frag_x, frag_z) = (p.x >> frag_size_log2, p.z >> frag_size_log2);
        h.entry((frag_x, frag_z)).or_default().push(*p);
    }

    // Convert that fragments into maps
    let mut r = vec![];
    for ps in h.values() {
        let a = Area::from_coords(ps.iter().copied());
        let m = map_with_river_at(ps, a);
        r.push(m);
    }

    r
}

/// Segregate a list of river coordinates into small maps
pub fn split_rivers_into_fragments4(points: &[Point4]) -> Vec<Map> {
    let mut h: HashMap<(i64, i64), Vec<Point4>> = HashMap::new();

    let frag_size_log2 = 6;
    // Split points into fragments of size 64x64
    for p in points {
        let (frag_x, frag_z) = (p.x >> frag_size_log2, p.z >> frag_size_log2);
        h.entry((frag_x, frag_z)).or_default().push(*p);
    }

    // Convert that fragments into maps
    let mut r = vec![];
    for ps in h.values() {
        let a = Area::from_coords4(ps.iter().copied());
        let m = map_with_river_at4(ps, a);
        r.push(m);
    }

    r
}

/// Fast check to see if it is possible for a river to generate near this point.
///
/// False positives are expected (this function can return true even if a river cannot
/// generate near the point) but false negatives should be rare (if this function returns
/// false you can pretty confident that a river will not spawn near the point).
///
/// point resolution: 1:4
/// world_seed: 24-bit value
pub fn can_generate_river_near(pre_voronoi_point: Point, world_seed: i64) -> bool {
    can_generate_river_near_steps(pre_voronoi_point, world_seed) == 0
}

// Return how many checks we needed to perform before discarding this seed,
// or 0 if this seed can generate a river near this point
fn can_generate_river_near_steps(pre_voronoi_point: Point, world_seed: i64) -> u8 {
    fn prev_area(area: Area) -> Area {
        let parea = Area {
            x: area.x >> 1,
            z: area.z >> 1,
            w: (area.w >> 1) + 2,
            h: (area.h >> 1) + 2
        };
        parea
    }
    fn all_equal(m: &Map) -> bool {
        let first = m.a[(0, 0)];
        m.a.iter().all(|&x| x == first)
    }

    // TODO: this check can be performed for 2 seeds at once,
    // by leaving bit 25 undefined and taking the OR of the two possible maps
    // But currently we just execute this function twice
    if world_seed & (1 << 25) == 0 {
        let a = can_generate_river_near_steps(pre_voronoi_point, world_seed | (1 << 25));
        if a == 0 {
            return 0;
        }
    }

    // We can generate a 3x3 area for more or less the same cost that a 1x1 area
    let a39 = Area { x: pre_voronoi_point.x - 1, z: pre_voronoi_point.z - 1, w: 3, h: 3 };
    let a38 = prev_area(a39);
    let a37 = prev_area(a38);
    let a36 = prev_area(a37);
    let a35 = prev_area(a36);
    // Return false as soon as a map cannot generate rivers:
    // when all tiles are equal
    let g22 = TestMapCheckers;
    let mut g34 = MapZoom::new(1000, world_seed);
    g34.parent = Some(Rc::new(g22));
    // This is never true
    //let m34 = g34.get_map(a34);
    //if all_equal(&m34) {}
    let mut g35 = MapZoom::new(1001, world_seed);
    g35.parent = Some(Rc::new(g34));
    let m35 = g35.get_map(a35);
    if all_equal(&m35) {
        return 1;
    }
    let mut g36 = MapZoom::new(1000, world_seed);
    g36.parent = Some(Rc::new(g35));
    let m36 = slice_to_area(g36.get_map_from_pmap(&m35), a36);
    if all_equal(&m36) {
        return 2;
    }
    let mut g37 = MapZoom::new(1001, world_seed);
    g37.parent = Some(Rc::new(g36));
    let m37 = slice_to_area(g37.get_map_from_pmap(&m36), a37);
    if all_equal(&m37) {
        return 3;
    }
    let mut g38 = MapZoom::new(1002, world_seed);
    g38.parent = Some(Rc::new(g37));
    let m38 = slice_to_area(g38.get_map_from_pmap(&m37), a38);
    if all_equal(&m38) {
        return 4;
    }
    let mut g39 = MapZoom::new(1003, world_seed);
    g39.parent = Some(Rc::new(g38));
    let m39 = slice_to_area(g39.get_map_from_pmap(&m38), a39);
    // This check is probably not worth it, just return 0 to save some cycles
    if all_equal(&m39) {
        return 5;
    }

    0
}

pub fn candidate_river_map_generator(world_seed: i64) -> impl GetMap {
    let g22 = TestMapCheckers;
    let mut g34 = MapZoom::new(1000, world_seed);
    g34.parent = Some(Rc::new(g22));
    let mut g35 = MapZoom::new(1001, world_seed);
    g35.parent = Some(Rc::new(g34));
    let mut g36 = MapZoom::new(1000, world_seed);
    g36.parent = Some(Rc::new(g35));
    let mut g37 = MapZoom::new(1001, world_seed);
    g37.parent = Some(Rc::new(g36));
    let mut g38 = MapZoom::new(1002, world_seed);
    g38.parent = Some(Rc::new(g37));
    let mut g39 = MapZoom::new(1003, world_seed);
    g39.parent = Some(Rc::new(g38));
    let mut g40 = HelperMapRiverAll::new(1, world_seed);
    g40.parent = Some(Rc::new(g39));
    let mut g41 = MapSmooth::new(1000, world_seed);
    g41.parent = Some(Rc::new(g40));

    g41
}

pub fn candidate_river_map(a: Area, world_seed: i64) -> Map {
    candidate_river_map_generator(world_seed).get_map(a)
}

/// Check two similar seeds at once
pub fn candidate_river_map_bit_25_undefined(a: Area, world_seed: i64) -> Map {
    let gm1 = candidate_river_map_generator(world_seed);
    let gm2 = candidate_river_map_generator(world_seed ^ (1 << 25));
    let map_or = MapMap2 {
        f: |a, b| a | b,
        parent1: Rc::new(gm1),
        parent2: Rc::new(gm2),
    };

    map_or.get_map(a)
}

pub fn draw_map(map: &Map) -> String {
    let (w, h) = map.a.dim();
    let mut s = format!("MAP: x: {}, z: {}, {}x{}\n", map.x, map.z, w, h);
    for z in 0..h {
        for x in 0..w {
            //let c = if map.a[(x, z)] != 0 { "#" } else { "_" };
            let c = match map.a[(x, z)] {
                0 => "_",
                1 => "#",
                2 => "2",
                3 => "3",
                7 => "R",
                _ => "?",
            };
            s.push_str(c);
        }
        s.push_str("\n");
    }

    s
}

pub fn draw_map_image(map: &Map) -> Vec<u8> {
    let (w, h) = map.a.dim();
    let mut v = vec![0; w*h*4];
    for x in 0..w {
        for z in 0..h {
            let color = biome_to_color(map.a[(x, z)]);
            let i = z * w + x;
            v[i*4+0] = color[0];
            v[i*4+1] = color[1];
            v[i*4+2] = color[2];
            v[i*4+3] = color[3];
        }
    }

    v
}

pub fn draw_map_image_noise(map: &Map) -> Vec<u8> {
    let (w, h) = map.a.dim();
    let mut v = vec![0; w*h*4];
    for x in 0..w {
        for z in 0..h {
            let a = map.a[(x, z)];
            let gray = a as u8;
            let color = [gray, gray, gray, 0xFF];
            let i = z * w + x;
            v[i*4+0] = color[0];
            v[i*4+1] = color[1];
            v[i*4+2] = color[2];
            v[i*4+3] = color[3];
        }
    }

    v
}

static TREASURE_MAP_COLORS: [u32; 64] = [
    0x000000,
    0x7FB238,
    0xF7E9A3,
    0xC7C7C7,
    0xFF0000,
    0xA0A0FF,
    0xA7A7A7,
    0x007C00,
    0xFFFFFF,
    0xA4A8B8,
    0x976D4D,
    0x707070,
    0x4040FF,
    0x8F7748,
    0xFFFCF5,
    0xD87F33,
    0xB24CD8,
    0x6699D8,
    0xE5E533,
    0x7FCC19,
    0xF27FA5,
    0x4C4C4C,
    0x999999,
    0x4C7F99,
    0x7F3FB2,
    0x334CB2,
    0x664C33,
    0x667F33,
    0x993333,
    0x191919,
    0xFAEE4D,
    0x5CDBD5,
    0x4A80FF,
    0x00D93A,
    0x815631,
    0x700200,
    0xD1B1A1,
    0x9F5224,
    0x95576C,
    0x706C8A,
    0xBA8524,
    0x677535,
    0xA04D4E,
    0x392923,
    0x876B62,
    0x575C5C,
    0x7A4958,
    0x4C3E5C,
    0x4C3223,
    0x4C522A,
    0x8E3C2E,
    0x251610,
    0x000000,
    0x000000,
    0x000000,
    0x000000,
    0x000000,
    0x000000,
    0x000000,
    0x000000,
    0x000000,
    0x000000,
    0x000000,
    0x000000,
];

/// channel: one of r, g, b
/// variant: 0, 1, 2, 3
fn color_variant_channel(channel: u8, variant: u8) -> u8 {
    let mut x = channel as u16;
    x *= [180, 220, 255, 135][variant as usize];
    // Round to nearest integer
    x += 128;
    x /= 256;

    x as u8
}

/// Calculate RGB value of color variant.
fn color_variant(r: u8, g: u8, b: u8, variant: u8) -> (u8, u8, u8) {
    let r = color_variant_channel(r, variant);
    let g = color_variant_channel(g, variant);
    let b = color_variant_channel(b, variant);

    (r, g, b)
}

pub fn treasure_map_to_color(id: i32) -> [u8; 4] {
    let id = id as usize;
    let (id, variant) = (id / 4, id % 4);

    // Fake transparent map
    // Color with id 0 (black) is rendered as transparent, so here we try to
    // replicate the resulting color to resemble that of a map
    let rgb = if id == 0 {
        0xDBC6AC
    } else {
        TREASURE_MAP_COLORS[id]
    };

    let (r, g, b) = {
        let b = rgb as u8;
        let rg = rgb / 256;
        let g = rg as u8;
        let r = rg / 256;
        let r = r as u8;

        (r, g, b)
    };

    let (r, g, b) = color_variant(r, g, b, variant as u8);

    [r, g, b, 255]
}


pub fn draw_treasure_map_image(map: &Map) -> Vec<u8> {
    let (w, h) = map.a.dim();
    let mut v = vec![0; w*h*4];
    for x in 0..w {
        for z in 0..h {
            let color = treasure_map_to_color(map.a[(x, z)]);
            let i = z * w + x;
            v[i*4+0] = color[0];
            v[i*4+1] = color[1];
            v[i*4+2] = color[2];
            v[i*4+3] = color[3];
        }
    }

    v
}

/// Generate terrain with the same style as unexplored treasure maps.
pub fn generate_image_treasure_map(version: MinecraftVersion, area: Area, seed: i64) -> Vec<u8> {
    let map = generate_fragment_treasure_map(version, area, seed);

    draw_treasure_map_image(&map)
}

/// Generate a treasure map with the same scale and aligment as ingame maps.
pub fn generate_image_treasure_map_at(version: MinecraftVersion, seed: i64, fragment_x: i64, fragment_z: i64) -> Vec<u8> {
    let corner_x = (fragment_x * 256 - 64) >> 1;
    let corner_z = (fragment_z * 256 - 64) >> 1;
    let parea = Area {
        x: corner_x,
        z: corner_z,
        w: 128,
        h: 128,
    };
    // Generate a 128x128 treasure map
    let mut map = generate_fragment_treasure_map(version, parea, seed);
    // But treasure maps have 126x126 resulution, so delete border pixels
    set_pixels_at_margin(&mut map, 0);

    // And convert the resulting map to a RGBA image
    // We could generate a 126x126 map and add the padding during the conversion to image, but that
    // would require a draw_treasure_map_image_with_padding function
    draw_treasure_map_image(&map)
}

pub fn generate_fragment_treasure_map(version: MinecraftVersion, area: Area, seed: i64) -> Map {
    // mhv: MapHalfVoronoi
    // Its the result of replacing the last layer (MapVoronoiZoom) which performs a 1:4 scale
    // operation, with MapHalfVoronoiZoom which performs a 1:2 scale. This should be equivalent to
    // doing a 2:1 scale after MapVoronoiZoom, but MapHalfVoronoiZoom can be optimized.
    let mhv: Rc<dyn GetMap> = match version {
        MinecraftVersion::Java1_13 => {
            let mut mhv = MapHalfVoronoiZoom::new(10, seed);
            let parent = Rc::from(generator_up_to_layer_1_13(seed, 50));
            mhv.parent = Some(parent);

            Rc::from(mhv)
        }
        MinecraftVersion::Java1_14 => {
            let mut mhv = MapHalfVoronoiZoom::new(10, seed);
            let parent = Rc::from(generator_up_to_layer_1_14(seed, 50));
            mhv.parent = Some(parent);

            Rc::from(mhv)
        }
        MinecraftVersion::Java1_15 | MinecraftVersion::Java1_16_1 | MinecraftVersion::Java1_16 | MinecraftVersion::Java1_17 => {
            let mut mhv = MapHalfVoronoiZoom115::new(seed);
            let parent = Rc::from(generator_up_to_layer_1_15(seed, 50, version));
            mhv.parent = Some(parent);

            Rc::from(mhv)
        }
        MinecraftVersion::Java1_18 => {
            let mut mhv = MapHalfVoronoiZoom115::new(seed);
            let parent = Rc::from(Map3DToMap2D { map_3d: generator_up_to_layer_1_18(seed, 8, version), y_level: 63 });
            mhv.parent = Some(parent);

            Rc::from(mhv)
        }
        _ => panic!("Treasure map generation in version {:?} is not implemented", version),
    };
    let mt = MapTreasure {
        parent: mhv,
    };

    mt.get_map(area)
}

pub fn generate_image(version: MinecraftVersion, area: Area, seed: i64, y_offset: u32) -> Vec<u8> {
    let num_layers = version.num_layers();
    generate_image_up_to_layer(version, area, seed, num_layers, y_offset)
}

pub fn generate_image_up_to_layer(version: MinecraftVersion, area: Area, seed: i64, layer: u32, y_offset: u32) -> Vec<u8> {
    let map = generate_up_to_layer(version, area, seed, layer, y_offset);

    match (version, layer) {
        // Layers [0, 7] are used to visualize noise.
        // Layers 8 and 9 can use the default draw_map_image.
        // Layer 50 is similar to noise, except it should be easy to spot low values.
        // Layer 51 is a binary map of biome ids that are different in search_bruteforce and
        // search_tree
        (MinecraftVersion::Java1_18, 0..=7 | 50 | 51) => {
            draw_map_image_noise(&map)
        }
        _ => draw_map_image(&map),
    }
}

pub fn generate(version: MinecraftVersion, a: Area, world_seed: i64, y_offset: u32) -> Map {
    let num_layers = version.num_layers();
    generate_up_to_layer(version, a, world_seed, num_layers, y_offset)
}

pub fn generate_up_to_layer(version: MinecraftVersion, area: Area, seed: i64, num_layers: u32, y_offset: u32) -> Map {
    match version {
        MinecraftVersion::Java1_3 => generate_up_to_layer_1_3(area, seed, num_layers),
        MinecraftVersion::Java1_7 => generate_up_to_layer_1_7(area, seed, num_layers, version),
        // 1.9 has a small bug
        MinecraftVersion::Java1_9 => generate_up_to_layer_1_7(area, seed, num_layers, version),
        // 1.11 should be the same as 1.7
        MinecraftVersion::Java1_11 => generate_up_to_layer_1_7(area, seed, num_layers, version),
        MinecraftVersion::Java1_13 => generate_up_to_layer_1_13(area, seed, num_layers),
        MinecraftVersion::Java1_14 => generate_up_to_layer_1_14(area, seed, num_layers),
        MinecraftVersion::Java1_15 => generate_up_to_layer_1_15(area, seed, num_layers, version),
        // 1.16.1 is the same as 1.15
        MinecraftVersion::Java1_16_1 => generate_up_to_layer_1_15(area, seed, num_layers, version),
        // 1.16.2 and later is different
        MinecraftVersion::Java1_16 => generate_up_to_layer_1_15(area, seed, num_layers, version),
        MinecraftVersion::Java1_17 => generate_up_to_layer_1_15(area, seed, num_layers, version),
        // 1.18 introduces 3D biomes in the overworld
        MinecraftVersion::Java1_18 => {
            let y_level = y_offset as i64 - 16;
            let area = Area3D::from_area2d_and_y_level(area, y_level);
            let map3d = generate_up_to_layer_1_18(area, seed, num_layers, version);
            map3d.into_map2d()
        }
        _ => {
            panic!("Biome generation in version {:?} is not implemented", version);
        }
    }
}

pub fn generate_up_to_layer_1_3(a: Area, world_seed: i64, layer: u32) -> Map {
    if layer >= 200 {
        //return generate_up_to_layer_1_7_extra_2(a, world_seed, layer);
    }
    if layer >= 100 && layer <= 142 {
        // TODO: implement river layer visualization for 1.3
        //return generate_up_to_layer_1_7_extra(a, world_seed, layer);
    }

    generator_up_to_layer_1_3(world_seed, layer).get_map(a)
}

pub fn generator_up_to_layer_1_3(world_seed: i64, layer: u32) -> Box<dyn GetMap> {
    let g0 = MapIsland::new(1, world_seed);
    if layer == 0 { return Box::new(g0); }
    let mut g1 = MapZoomFuzzy::new(2000, world_seed);
    g1.parent = Some(Rc::new(g0));
    if layer == 1 { return Box::new(g1); }
    let mut g2 = MapAddIsland13::new(1, world_seed);
    g2.parent = Some(Rc::new(g1));
    if layer == 2 { return Box::new(g2); }
    let mut g3 = MapZoom::new(2001, world_seed);
    g3.parent = Some(Rc::new(g2));
    if layer == 3 { return Box::new(g3); }
    let mut g4 = MapAddIsland13::new(2, world_seed);
    g4.parent = Some(Rc::new(g3));
    if layer == 4 { return Box::new(g4); }
    let mut g5 = MapIcePlains::new(2, world_seed);
    g5.parent = Some(Rc::new(g4));
    if layer == 5 { return Box::new(g5); }
    let mut g6 = MapZoom::new(2002, world_seed);
    g6.parent = Some(Rc::new(g5));
    if layer == 6 { return Box::new(g6); }
    let mut g7 = MapAddIsland13::new(3, world_seed);
    g7.parent = Some(Rc::new(g6));
    if layer == 7 { return Box::new(g7); }
    let mut g8 = MapZoom::new(2003, world_seed);
    g8.parent = Some(Rc::new(g7));
    if layer == 8 { return Box::new(g8); }
    let mut g9 = MapAddIsland13::new(4, world_seed);
    g9.parent = Some(Rc::new(g8));
    if layer == 9 { return Box::new(g9); }
    let mut g10 = MapAddMushroomIsland::new(5, world_seed);
    g10.parent = Some(Rc::new(g9));
    if layer == 10 { return Box::new(g10); }
    let g10 = Rc::new(g10);

    let mut g11 = MapBiome13::new(200, world_seed);
    g11.parent = Some(g10.clone());
    if layer == 11 { return Box::new(g11); }
    let mut g12 = MapZoom::new(1000, world_seed);
    g12.parent = Some(Rc::new(g11));
    if layer == 12 { return Box::new(g12); }
    let mut g13 = MapZoom::new(1001, world_seed);
    g13.parent = Some(Rc::new(g12));
    if layer == 13 { return Box::new(g13); }
    let mut g14 = MapRegionHills::new(1000, world_seed);
    g14.parent = Some(Rc::new(g13));
    if layer == 14 { return Box::new(g14); }

    let mut g15 = MapZoom::new(1000, world_seed);
    g15.parent = Some(Rc::new(g14));
    if layer == 15 { return Box::new(g15); }
    let mut g16 = MapAddIsland13::new(3, world_seed);
    g16.parent = Some(Rc::new(g15));
    if layer == 16 { return Box::new(g16); }
    let mut g17 = MapZoom::new(1001, world_seed);
    g17.parent = Some(Rc::new(g16));
    if layer == 17 { return Box::new(g17); }
    let mut g18 = MapMushroomShore::new(1000, world_seed);
    g18.parent = Some(Rc::new(g17));
    if layer == 18 { return Box::new(g18); }
    let mut g19 = MapSwampRivers::new(1000, world_seed);
    g19.parent = Some(Rc::new(g18));
    if layer == 19 { return Box::new(g19); }
    let mut g20 = MapZoom::new(1002, world_seed);
    g20.parent = Some(Rc::new(g19));
    if layer == 20 { return Box::new(g20); }
    let mut g21 = MapZoom::new(1003, world_seed);
    g21.parent = Some(Rc::new(g20));
    if layer == 21 { return Box::new(g21); }

    let mut g22 = MapSmooth::new(1000, world_seed);
    g22.parent = Some(Rc::new(g21));
    if layer == 22 { return Box::new(g22); }

    let mut g23 = MapRiverInit13::new(100, world_seed);
    g23.parent = Some(g10.clone());
    if layer == 23 { return Box::new(g23); }
    let mut g24 = MapZoom::new(1000, world_seed);
    g24.parent = Some(Rc::new(g23));
    if layer == 24 { return Box::new(g24); }
    let mut g25 = MapZoom::new(1001, world_seed);
    g25.parent = Some(Rc::new(g24));
    if layer == 25 { return Box::new(g25); }
    let mut g26 = MapZoom::new(1002, world_seed);
    g26.parent = Some(Rc::new(g25));
    if layer == 26 { return Box::new(g26); }
    let mut g27 = MapZoom::new(1003, world_seed);
    g27.parent = Some(Rc::new(g26));
    if layer == 27 { return Box::new(g27); }
    let mut g28 = MapZoom::new(1004, world_seed);
    g28.parent = Some(Rc::new(g27));
    if layer == 28 { return Box::new(g28); }
    let mut g29 = MapZoom::new(1005, world_seed);
    g29.parent = Some(Rc::new(g28));
    if layer == 29 { return Box::new(g29); }
    let mut g30 = MapRiver13::new(1, world_seed);
    g30.parent = Some(Rc::new(g29));
    if layer == 30 { return Box::new(g30); }
    let mut g31 = MapSmooth::new(1000, world_seed);
    g31.parent = Some(Rc::new(g30));
    if layer == 31 { return Box::new(g31); }

    let mut g32 = MapRiverMix13::new(100, world_seed);
    g32.parent1 = Some(Rc::new(g22));
    g32.parent2 = Some(Rc::new(g31));
    if layer == 32 { return Box::new(g32); }

    let mut g33 = MapVoronoiZoom::new(10, world_seed);
    g33.parent = Some(Rc::new(g32));
    Box::new(g33)
}

pub fn generate_up_to_layer_1_7_extra_2(a: Area, world_seed: i64, layer: u32) -> Map {
    let g22 = TestMapCheckers;
    if layer == 222 { return g22.get_map(a); }
    let mut g34 = HelperMapZoomAllEdges::new(1000, world_seed);
    g34.parent = Some(Rc::new(g22));
    if layer == 234 { return g34.get_map(a); }
    let mut g35 = HelperMapZoomAllEdges::new(1001, world_seed);
    g35.parent = Some(Rc::new(g34));
    if layer == 235 { return g35.get_map(a); }
    let mut g36 = HelperMapZoomAllEdges::new(1000, world_seed);
    g36.parent = Some(Rc::new(g35));
    if layer == 236 { return g36.get_map(a); }
    let mut g37 = HelperMapZoomAllEdges::new(1001, world_seed);
    g37.parent = Some(Rc::new(g36));
    if layer == 237 { return g37.get_map(a); }
    let mut g38 = HelperMapZoomAllEdges::new(1002, world_seed);
    g38.parent = Some(Rc::new(g37));
    if layer == 238 { return g38.get_map(a); }
    let mut g39 = HelperMapZoomAllEdges::new(1003, world_seed);
    g39.parent = Some(Rc::new(g38));
    if layer == 239 { return g39.get_map(a); }
    let mut g40 = HelperMapRiverAll::new(1, world_seed);
    g40.parent = Some(Rc::new(g39));
    if layer == 240 { return g40.get_map(a); }
    let mut g41 = MapSmooth::new(1000, world_seed);
    g41.parent = Some(Rc::new(g40));
    if layer == 241 { return g41.get_map(a); }

    TestMapZero.get_map(a)
}

pub fn generate_up_to_layer_1_7_extra(a: Area, world_seed: i64, layer: u32) -> Map {
    /* RIVER LAYERS */
    let g22 = TestMapCheckers;
    let g22 = Rc::new(g22);
    if layer == 122 { return g22.get_map(a); }
    let mut g34 = MapZoom::new(1000, world_seed);
    g34.parent = Some(g22.clone());
    if layer == 134 { return g34.get_map(a); }
    let mut g35 = MapZoom::new(1001, world_seed);
    g35.parent = Some(Rc::new(g34));
    if layer == 135 { return g35.get_map(a); }
    let mut g36 = MapZoom::new(1000, world_seed);
    g36.parent = Some(Rc::new(g35));
    if layer == 136 { return g36.get_map(a); }
    let mut g37 = MapZoom::new(1001, world_seed);
    g37.parent = Some(Rc::new(g36));
    if layer == 137 { return g37.get_map(a); }
    let mut g38 = MapZoom::new(1002, world_seed);
    g38.parent = Some(Rc::new(g37));
    if layer == 138 { return g38.get_map(a); }
    let mut g39 = MapZoom::new(1003, world_seed);
    g39.parent = Some(Rc::new(g38));
    if layer == 139 { return g39.get_map(a); }
    let mut g40 = HelperMapRiverAll::new(1, world_seed);
    g40.parent = Some(Rc::new(g39));
    if layer == 140 { return g40.get_map(a); }
    let mut g41 = MapSmooth::new(1000, world_seed);
    g41.parent = Some(Rc::new(g40));
    if layer == 141 { return g41.get_map(a); }
    /* END RIVER LAYERS */

    /* BIOME LAYERS */
    let g12 = TestMapCheckers;
    if layer <= 112 { return g12.get_map(a); }
    let mut g13 = MapZoom::new(2002, world_seed);
    g13.parent = Some(Rc::new(g12));
    if layer == 113 { return g13.get_map(a); }
    let mut g14 = MapZoom::new(2003, world_seed);
    g14.parent = Some(Rc::new(g13));
    if layer == 114 { return g14.get_map(a); }
    //let mut g15 = MapAddIsland::new(4, world_seed);
    //g15.parent = Some(Rc::new(g14));
    //if layer == 115 { return g15.get_map(a); }
    //let mut g16 = MapAddMushroomIsland::new(5, world_seed);
    //g16.parent = Some(Rc::new(g15));
    //if layer == 116 { return g16.get_map(a); }
    //let mut g17 = MapDeepOcean::new(4, world_seed);
    //g17.parent = Some(Rc::new(g16));
    //let g17 = Rc::new(g17);
    //if layer == 117 { return g17.get_map(a); }
    //let mut g18 = MapBiome::new(200, world_seed);
    //g18.parent = Some(g17.clone());
    //if layer == 118 { return g18.get_map(a); }
    let mut g19 = MapZoom::new(1000, world_seed);
    g19.parent = Some(Rc::new(g14));
    if layer <= 119 { return g19.get_map(a); }
    let mut g20 = MapZoom::new(1001, world_seed);
    g20.parent = Some(Rc::new(g19));
    if layer == 120 { return g20.get_map(a); }
    //let mut g21 = MapBiomeEdge::new(1000, world_seed);
    //g21.parent = Some(Rc::new(g20));
    //if layer == 121 { return g21.get_map(a); }

    let mut g23 = MapZoom::new(1000, world_seed);
    g23.parent = Some(g22.clone());
    g23.bug_world_seed_not_set = true;
    if layer == 123 { return g23.get_map(a); }
    let mut g24 = MapZoom::new(1001, world_seed);
    g24.parent = Some(Rc::new(g23));
    g24.bug_world_seed_not_set = true;
    if layer == 124 { return g24.get_map(a); }
    //let mut g25 = MapHills::new(1000, world_seed);
    //g25.parent1 = Some(Rc::new(g20));
    //g25.parent2 = Some(Rc::new(g24));
    //if layer == 25 { return g25.get_map(a); }
    //let mut g26 = MapRareBiome::new(1001, world_seed);
    //g26.parent = Some(Rc::new(g25));
    //if layer == 26 { return g26.get_map(a); }
    let mut g27 = MapZoom::new(1000, world_seed);
    g27.parent = Some(Rc::new(g24));
    // Target deep ocean islands:
    //g27.parent = Some(g22.clone());
    if layer == 127 { return g27.get_map(a); }
    //let mut g28 = MapAddIsland::new(3, world_seed);
    //g28.parent = Some(Rc::new(g27));
    //if layer == 28 { return g28.get_map(a); }
    let mut g29 = MapZoom::new(1001, world_seed);
    g29.parent = Some(Rc::new(g27));
    if layer == 129 { return g29.get_map(a); }
    //let mut g30 = MapShore::new(1000, world_seed);
    //g30.parent = Some(Rc::new(g29));
    //if layer == 30 { return g30.get_map(a); }
    let mut g31 = MapZoom::new(1002, world_seed);
    g31.parent = Some(Rc::new(g29));
    // Target MapShore:
    //g31.parent = Some(g22.clone());
    if layer == 131 { return g31.get_map(a); }
    let mut g32 = MapZoom::new(1003, world_seed);
    g32.parent = Some(Rc::new(g31));
    if layer == 132 { return g32.get_map(a); }
    let mut g33 = MapSmooth::new(1000, world_seed);
    g33.parent = Some(Rc::new(g32));
    if layer == 133 { return g33.get_map(a); }

    let mut g42 = MapRiverMix::new(100, world_seed);
    g42.parent1 = Some(Rc::new(g33));
    g42.parent2 = Some(Rc::new(g41));
    if layer == 142 { return g42.get_map(a); }
    let mut g43 = MapVoronoiZoom::new(10, world_seed);
    g43.parent = Some(Rc::new(g42));
    if layer == 143 { return g43.get_map(a); }

    if layer == 170 { return MapFn(|p| {
        can_generate_river_near_steps(p, world_seed) as i32
    }).get_map(a); }

    TestMapZero.get_map(a)
}

pub fn generate_up_to_layer_1_7(a: Area, world_seed: i64, layer: u32, version: MinecraftVersion) -> Map {
    if layer >= 200 {
        return generate_up_to_layer_1_7_extra_2(a, world_seed, layer);
    }
    if layer >= 100 {
        return generate_up_to_layer_1_7_extra(a, world_seed, layer);
    }

    generator_up_to_layer_1_7(world_seed, layer, version).get_map(a)
}

pub fn generator_up_to_layer_1_7(world_seed: i64, layer: u32, version: MinecraftVersion) -> Box<dyn GetMap> {
    let g0 = MapIsland::new(1, world_seed);
    if layer == 0 { return Box::new(g0); }
    let mut g1 = MapZoomFuzzy::new(2000, world_seed);
    g1.parent = Some(Rc::new(g0));
    if layer == 1 { return Box::new(g1); }
    let mut g2 = MapAddIsland::new(1, world_seed);
    g2.parent = Some(Rc::new(g1));
    if layer == 2 { return Box::new(g2); }
    let mut g3 = MapZoom::new(2001, world_seed);
    g3.parent = Some(Rc::new(g2));
    if layer == 3 { return Box::new(g3); }
    let mut g4 = MapAddIsland::new(2, world_seed);
    g4.parent = Some(Rc::new(g3));
    if layer == 4 { return Box::new(g4); }
    let mut g5 = MapAddIsland::new(50, world_seed);
    g5.parent = Some(Rc::new(g4));
    if layer == 5 { return Box::new(g5); }
    let mut g6 = MapAddIsland::new(70, world_seed);
    g6.parent = Some(Rc::new(g5));
    if layer == 6 { return Box::new(g6); }
    let mut g7 = MapRemoveTooMuchOcean::new(2, world_seed);
    g7.parent = Some(Rc::new(g6));
    if layer == 7 { return Box::new(g7); }
    let mut g8 = MapAddSnow::new(2, world_seed);
    g8.parent = Some(Rc::new(g7));
    if layer == 8 { return Box::new(g8); }
    let mut g9 = MapAddIsland::new(3, world_seed);
    g9.parent = Some(Rc::new(g8));
    if layer == 9 { return Box::new(g9); }
    let mut g10 = MapCoolWarm::new(2, world_seed);
    g10.parent = Some(Rc::new(g9));
    if layer == 10 { return Box::new(g10); }
    let mut g11 = MapHeatIce::new(2, world_seed);
    g11.parent = Some(Rc::new(g10));
    if layer == 11 { return Box::new(g11); }
    let mut g12 = MapSpecial::new(3, world_seed);
    g12.parent = Some(Rc::new(g11));
    if layer == 12 { return Box::new(g12); }
    let mut g13 = MapZoom::new(2002, world_seed);
    g13.parent = Some(Rc::new(g12));
    if layer == 13 { return Box::new(g13); }
    let mut g14 = MapZoom::new(2003, world_seed);
    g14.parent = Some(Rc::new(g13));
    if layer == 14 { return Box::new(g14); }
    let mut g15 = MapAddIsland::new(4, world_seed);
    g15.parent = Some(Rc::new(g14));
    if layer == 15 { return Box::new(g15); }
    let mut g16 = MapAddMushroomIsland::new(5, world_seed);
    g16.parent = Some(Rc::new(g15));
    if layer == 16 { return Box::new(g16); }
    let mut g17 = MapDeepOcean::new(4, world_seed);
    g17.parent = Some(Rc::new(g16));
    if layer == 17 { return Box::new(g17); }
    let g17 = Rc::new(g17);
    let mut g18 = MapBiome::new(200, world_seed);
    g18.parent = Some(g17.clone());
    if layer == 18 { return Box::new(g18); }
    let mut g19 = MapZoom::new(1000, world_seed);
    g19.parent = Some(Rc::new(g18));
    if layer == 19 { return Box::new(g19); }
    let mut g20 = MapZoom::new(1001, world_seed);
    g20.parent = Some(Rc::new(g19));
    if layer == 20 { return Box::new(g20); }
    let mut g21 = MapBiomeEdge::new(1000, world_seed);
    g21.parent = Some(Rc::new(g20));
    if layer == 21 { return Box::new(g21); }
    let mut g22 = MapRiverInit::new(100, world_seed);
    g22.parent = Some(g17.clone());
    if layer == 22 { return Box::new(g22); }
    let g22 = Rc::new(g22);
    // TODO: use some special color palette for MapRiverInit?
    //if layer == 23 { return Box::new(MapMap { parent: Rc::new(g23), f: pretty_biome_map_hills }); }
    let mut g23 = MapZoom::new(1000, world_seed);
    g23.parent = Some(g22.clone());
    g23.bug_world_seed_not_set = true;
    if layer == 23 { return Box::new(MapMap { parent: Rc::new(g23), f: pretty_biome_map_hills }); }
    let mut g24 = MapZoom::new(1001, world_seed);
    g24.parent = Some(Rc::new(g23));
    g24.bug_world_seed_not_set = true;
    if layer == 24 { return Box::new(MapMap { parent: Rc::new(g24), f: pretty_biome_map_hills }); }
    let mut g25 = MapHills::new(1000, world_seed, version);
    g25.parent1 = Some(Rc::new(g21));
    g25.parent2 = Some(Rc::new(g24));
    if layer == 25 { return Box::new(g25); }
    let mut g26 = MapRareBiome::new(1001, world_seed);
    g26.parent = Some(Rc::new(g25));
    if layer == 26 { return Box::new(g26); }
    let mut g27 = MapZoom::new(1000, world_seed);
    g27.parent = Some(Rc::new(g26));
    if layer == 27 { return Box::new(g27); }
    let mut g28 = MapAddIsland::new(3, world_seed);
    g28.parent = Some(Rc::new(g27));
    if layer == 28 { return Box::new(g28); }
    let mut g29 = MapZoom::new(1001, world_seed);
    g29.parent = Some(Rc::new(g28));
    if layer == 29 { return Box::new(g29); }
    let mut g30 = MapShore::new(1000, world_seed);
    g30.parent = Some(Rc::new(g29));
    if layer == 30 { return Box::new(g30); }
    let mut g31 = MapZoom::new(1002, world_seed);
    g31.parent = Some(Rc::new(g30));
    if layer == 31 { return Box::new(g31); }
    let mut g32 = MapZoom::new(1003, world_seed);
    g32.parent = Some(Rc::new(g31));
    if layer == 32 { return Box::new(g32); }
    let mut g33 = MapSmooth::new(1000, world_seed);
    g33.parent = Some(Rc::new(g32));
    if layer == 33 { return Box::new(g33); }
    let mut g34 = MapZoom::new(1000, world_seed);
    g34.parent = Some(g22.clone());
    if layer == 34 { return Box::new(MapMap { parent: Rc::new(g34), f: reduce_id }); }
    let mut g35 = MapZoom::new(1001, world_seed);
    g35.parent = Some(Rc::new(g34));
    if layer == 35 { return Box::new(MapMap { parent: Rc::new(g35), f: reduce_id }); }
    let mut g36 = MapZoom::new(1000, world_seed);
    g36.parent = Some(Rc::new(g35));
    if layer == 36 { return Box::new(MapMap { parent: Rc::new(g36), f: reduce_id }); }
    let mut g37 = MapZoom::new(1001, world_seed);
    g37.parent = Some(Rc::new(g36));
    if layer == 37 { return Box::new(MapMap { parent: Rc::new(g37), f: reduce_id }); }
    let mut g38 = MapZoom::new(1002, world_seed);
    g38.parent = Some(Rc::new(g37));
    if layer == 38 { return Box::new(MapMap { parent: Rc::new(g38), f: reduce_id }); }
    let mut g39 = MapZoom::new(1003, world_seed);
    g39.parent = Some(Rc::new(g38));
    if layer == 39 { return Box::new(MapMap { parent: Rc::new(g39), f: reduce_id }); }
    let mut g40 = MapRiver::new(1, world_seed);
    g40.parent = Some(Rc::new(g39));
    if layer == 40 { return Box::new(g40); }
    let mut g41 = MapSmooth::new(1000, world_seed);
    g41.parent = Some(Rc::new(g40));
    if layer == 41 { return Box::new(g41); }
    let mut g42 = MapRiverMix::new(100, world_seed);
    g42.parent1 = Some(Rc::new(g33));
    g42.parent2 = Some(Rc::new(g41));
    if layer == 42 { return Box::new(g42); }
    let mut g43 = MapVoronoiZoom::new(10, world_seed);
    g43.parent = Some(Rc::new(g42));

    Box::new(g43)
}

pub fn generate_up_to_layer_1_13(a: Area, world_seed: i64, layer: u32) -> Map {
    if layer >= 200 {
        //return generate_up_to_layer_1_7_extra_2(a, world_seed, layer);
    }
    if layer >= 100 && layer <= 142 {
        // The first 42 layers are almost equal in 1.7 and 1.13
        // The main difference being the MapHills bug, which does
        // not affect the river generation code
        return generate_up_to_layer_1_7_extra(a, world_seed, layer);
    }

    generator_up_to_layer_1_13(world_seed, layer).get_map(a)
}

pub fn generator_up_to_layer_1_13(world_seed: i64, layer: u32) -> Box<dyn GetMap> {
    let g0 = MapIsland::new(1, world_seed);
    if layer == 0 { return Box::new(g0); }
    let mut g1 = MapZoomFuzzy::new(2000, world_seed);
    g1.parent = Some(Rc::new(g0));
    if layer == 1 { return Box::new(g1); }
    let mut g2 = MapAddIsland::new(1, world_seed);
    g2.parent = Some(Rc::new(g1));
    if layer == 2 { return Box::new(g2); }
    let mut g3 = MapZoom::new(2001, world_seed);
    g3.parent = Some(Rc::new(g2));
    if layer == 3 { return Box::new(g3); }
    let mut g4 = MapAddIsland::new(2, world_seed);
    g4.parent = Some(Rc::new(g3));
    if layer == 4 { return Box::new(g4); }
    let mut g5 = MapAddIsland::new(50, world_seed);
    g5.parent = Some(Rc::new(g4));
    if layer == 5 { return Box::new(g5); }
    let mut g6 = MapAddIsland::new(70, world_seed);
    g6.parent = Some(Rc::new(g5));
    if layer == 6 { return Box::new(g6); }
    let mut g7 = MapRemoveTooMuchOcean::new(2, world_seed);
    g7.parent = Some(Rc::new(g6));
    if layer == 7 { return Box::new(g7); }
    let mut g8 = MapAddSnow::new(2, world_seed);
    g8.parent = Some(Rc::new(g7));
    if layer == 8 { return Box::new(g8); }
    let mut g9 = MapAddIsland::new(3, world_seed);
    g9.parent = Some(Rc::new(g8));
    if layer == 9 { return Box::new(g9); }
    let mut g10 = MapCoolWarm::new(2, world_seed);
    g10.parent = Some(Rc::new(g9));
    if layer == 10 { return Box::new(g10); }
    let mut g11 = MapHeatIce::new(2, world_seed);
    g11.parent = Some(Rc::new(g10));
    if layer == 11 { return Box::new(g11); }
    let mut g12 = MapSpecial::new(3, world_seed);
    g12.parent = Some(Rc::new(g11));
    if layer == 12 { return Box::new(g12); }
    let mut g13 = MapZoom::new(2002, world_seed);
    g13.parent = Some(Rc::new(g12));
    if layer == 13 { return Box::new(g13); }
    let mut g14 = MapZoom::new(2003, world_seed);
    g14.parent = Some(Rc::new(g13));
    if layer == 14 { return Box::new(g14); }
    let mut g15 = MapAddIsland::new(4, world_seed);
    g15.parent = Some(Rc::new(g14));
    if layer == 15 { return Box::new(g15); }
    let mut g16 = MapAddMushroomIsland::new(5, world_seed);
    g16.parent = Some(Rc::new(g15));
    if layer == 16 { return Box::new(g16); }
    let mut g17 = MapDeepOcean::new(4, world_seed);
    g17.parent = Some(Rc::new(g16));
    if layer == 17 { return Box::new(g17); }
    let g17 = Rc::new(g17);
    let mut g18 = MapBiome::new(200, world_seed);
    g18.parent = Some(g17.clone());
    if layer == 18 { return Box::new(g18); }
    let mut g19 = MapZoom::new(1000, world_seed);
    g19.parent = Some(Rc::new(g18));
    if layer == 19 { return Box::new(g19); }
    let mut g20 = MapZoom::new(1001, world_seed);
    g20.parent = Some(Rc::new(g19));
    if layer == 20 { return Box::new(g20); }
    let mut g21 = MapBiomeEdge::new(1000, world_seed);
    g21.parent = Some(Rc::new(g20));
    if layer == 21 { return Box::new(g21); }
    let mut g22 = MapRiverInit::new(100, world_seed);
    g22.parent = Some(g17.clone());
    if layer == 22 { return Box::new(g22); }
    let g22 = Rc::new(g22);
    // TODO: use some special color palette for MapRiverInit?
    //if layer == 23 { return Box::new(MapMap { parent: Rc::new(g23), f: pretty_biome_map_hills }); }
    let mut g23 = MapZoom::new(1000, world_seed);
    g23.parent = Some(g22.clone());
    if layer == 23 { return Box::new(MapMap { parent: Rc::new(g23), f: pretty_biome_map_hills }); }
    let mut g24 = MapZoom::new(1001, world_seed);
    g24.parent = Some(Rc::new(g23));
    if layer == 24 { return Box::new(MapMap { parent: Rc::new(g24), f: pretty_biome_map_hills }); }
    let mut g25 = MapHills::new(1000, world_seed, MinecraftVersion::Java1_13);
    g25.parent1 = Some(Rc::new(g21));
    g25.parent2 = Some(Rc::new(g24));
    if layer == 25 { return Box::new(g25); }
    let mut g26 = MapRareBiome::new(1001, world_seed);
    g26.parent = Some(Rc::new(g25));
    if layer == 26 { return Box::new(g26); }
    let mut g27 = MapZoom::new(1000, world_seed);
    g27.parent = Some(Rc::new(g26));
    if layer == 27 { return Box::new(g27); }
    let mut g28 = MapAddIsland::new(3, world_seed);
    g28.parent = Some(Rc::new(g27));
    if layer == 28 { return Box::new(g28); }
    let mut g29 = MapZoom::new(1001, world_seed);
    g29.parent = Some(Rc::new(g28));
    if layer == 29 { return Box::new(g29); }
    let mut g30 = MapShore::new(1000, world_seed);
    g30.parent = Some(Rc::new(g29));
    if layer == 30 { return Box::new(g30); }
    let mut g31 = MapZoom::new(1002, world_seed);
    g31.parent = Some(Rc::new(g30));
    if layer == 31 { return Box::new(g31); }
    let mut g32 = MapZoom::new(1003, world_seed);
    g32.parent = Some(Rc::new(g31));
    if layer == 32 { return Box::new(g32); }
    let mut g33 = MapSmooth::new(1000, world_seed);
    g33.parent = Some(Rc::new(g32));
    if layer == 33 { return Box::new(g33); }
    let mut g34 = MapZoom::new(1000, world_seed);
    g34.parent = Some(g22.clone());
    if layer == 34 { return Box::new(MapMap { parent: Rc::new(g34), f: reduce_id }); }
    let mut g35 = MapZoom::new(1001, world_seed);
    g35.parent = Some(Rc::new(g34));
    if layer == 35 { return Box::new(MapMap { parent: Rc::new(g35), f: reduce_id }); }
    let mut g36 = MapZoom::new(1000, world_seed);
    g36.parent = Some(Rc::new(g35));
    if layer == 36 { return Box::new(MapMap { parent: Rc::new(g36), f: reduce_id }); }
    let mut g37 = MapZoom::new(1001, world_seed);
    g37.parent = Some(Rc::new(g36));
    if layer == 37 { return Box::new(MapMap { parent: Rc::new(g37), f: reduce_id }); }
    let mut g38 = MapZoom::new(1002, world_seed);
    g38.parent = Some(Rc::new(g37));
    if layer == 38 { return Box::new(MapMap { parent: Rc::new(g38), f: reduce_id }); }
    let mut g39 = MapZoom::new(1003, world_seed);
    g39.parent = Some(Rc::new(g38));
    if layer == 39 { return Box::new(MapMap { parent: Rc::new(g39), f: reduce_id }); }
    let mut g40 = MapRiver::new(1, world_seed);
    g40.parent = Some(Rc::new(g39));
    if layer == 40 { return Box::new(g40); }
    let mut g41 = MapSmooth::new(1000, world_seed);
    g41.parent = Some(Rc::new(g40));
    if layer == 41 { return Box::new(g41); }
    let mut g42 = MapRiverMix::new(100, world_seed);
    g42.parent1 = Some(Rc::new(g33));
    g42.parent2 = Some(Rc::new(g41));
    if layer == 42 { return Box::new(g42); }

    // 1.13 ocean layers
    let g43 = MapOceanTemp::new(2, world_seed);
    if layer == 43 { return Box::new(g43); }
    let mut g44 = MapZoom::new(2001, world_seed);
    g44.parent = Some(Rc::new(g43));
    if layer == 44 { return Box::new(g44); }
    let mut g45 = MapZoom::new(2002, world_seed);
    g45.parent = Some(Rc::new(g44));
    if layer == 45 { return Box::new(g45); }
    let mut g46 = MapZoom::new(2003, world_seed);
    g46.parent = Some(Rc::new(g45));
    if layer == 46 { return Box::new(g46); }
    let mut g47 = MapZoom::new(2004, world_seed);
    g47.parent = Some(Rc::new(g46));
    if layer == 47 { return Box::new(g47); }
    let mut g48 = MapZoom::new(2005, world_seed);
    g48.parent = Some(Rc::new(g47));
    if layer == 48 { return Box::new(g48); }
    let mut g49 = MapZoom::new(2006, world_seed);
    g49.parent = Some(Rc::new(g48));
    if layer == 49 { return Box::new(g49); }

    let mut g50 = MapOceanMix::new(100, world_seed);
    g50.parent1 = Some(Rc::new(g42)); // MapRiverMix
    g50.parent2 = Some(Rc::new(g49)); // MapZoom <- MapOceanTemp
    if layer == 50 { return Box::new(g50); }

    let mut g51 = MapVoronoiZoom::new(10, world_seed);
    g51.parent = Some(Rc::new(g50));

    Box::new(g51)
}

pub fn generate_up_to_layer_1_14(a: Area, world_seed: i64, layer: u32) -> Map {
    if layer >= 200 {
        //return generate_up_to_layer_1_7_extra_2(a, world_seed, layer);
    }
    if layer >= 100 && layer <= 142 {
        // The first 42 layers are almost equal in 1.7 and 1.13
        // The main difference being the MapHills bug, which does
        // not affect the river generation code
        return generate_up_to_layer_1_7_extra(a, world_seed, layer);
    }

    generator_up_to_layer_1_14(world_seed, layer).get_map(a)
}

pub fn generator_up_to_layer_1_14(world_seed: i64, layer: u32) -> Box<dyn GetMap> {
    let g0 = MapIsland::new(1, world_seed);
    if layer == 0 { return Box::new(g0); }
    let mut g1 = MapZoomFuzzy::new(2000, world_seed);
    g1.parent = Some(Rc::new(g0));
    if layer == 1 { return Box::new(g1); }
    let mut g2 = MapAddIsland::new(1, world_seed);
    g2.parent = Some(Rc::new(g1));
    if layer == 2 { return Box::new(g2); }
    let mut g3 = MapZoom::new(2001, world_seed);
    g3.parent = Some(Rc::new(g2));
    if layer == 3 { return Box::new(g3); }
    let mut g4 = MapAddIsland::new(2, world_seed);
    g4.parent = Some(Rc::new(g3));
    if layer == 4 { return Box::new(g4); }
    let mut g5 = MapAddIsland::new(50, world_seed);
    g5.parent = Some(Rc::new(g4));
    if layer == 5 { return Box::new(g5); }
    let mut g6 = MapAddIsland::new(70, world_seed);
    g6.parent = Some(Rc::new(g5));
    if layer == 6 { return Box::new(g6); }
    let mut g7 = MapRemoveTooMuchOcean::new(2, world_seed);
    g7.parent = Some(Rc::new(g6));
    if layer == 7 { return Box::new(g7); }
    let mut g8 = MapAddSnow::new(2, world_seed);
    g8.parent = Some(Rc::new(g7));
    if layer == 8 { return Box::new(g8); }
    let mut g9 = MapAddIsland::new(3, world_seed);
    g9.parent = Some(Rc::new(g8));
    if layer == 9 { return Box::new(g9); }
    let mut g10 = MapCoolWarm::new(2, world_seed);
    g10.parent = Some(Rc::new(g9));
    if layer == 10 { return Box::new(g10); }
    let mut g11 = MapHeatIce::new(2, world_seed);
    g11.parent = Some(Rc::new(g10));
    if layer == 11 { return Box::new(g11); }
    let mut g12 = MapSpecial::new(3, world_seed);
    g12.parent = Some(Rc::new(g11));
    if layer == 12 { return Box::new(g12); }
    let mut g13 = MapZoom::new(2002, world_seed);
    g13.parent = Some(Rc::new(g12));
    if layer == 13 { return Box::new(g13); }
    let mut g14 = MapZoom::new(2003, world_seed);
    g14.parent = Some(Rc::new(g13));
    if layer == 14 { return Box::new(g14); }
    let mut g15 = MapAddIsland::new(4, world_seed);
    g15.parent = Some(Rc::new(g14));
    if layer == 15 { return Box::new(g15); }
    let mut g16 = MapAddMushroomIsland::new(5, world_seed);
    g16.parent = Some(Rc::new(g15));
    if layer == 16 { return Box::new(g16); }
    let mut g17 = MapDeepOcean::new(4, world_seed);
    g17.parent = Some(Rc::new(g16));
    if layer == 17 { return Box::new(g17); }
    let g17 = Rc::new(g17);
    let mut g18 = MapBiome::new(200, world_seed);
    g18.parent = Some(g17.clone());
    //if layer == 18 { return Box::new(g18); }
    // 1.14: bamboo
    let mut g18b = MapAddBamboo::new(1001, world_seed);
    g18b.parent = Some(Rc::new(g18));
    if layer == 18 { return Box::new(g18b); }
    let mut g19 = MapZoom::new(1000, world_seed);
    g19.parent = Some(Rc::new(g18b));
    if layer == 19 { return Box::new(g19); }
    let mut g20 = MapZoom::new(1001, world_seed);
    g20.parent = Some(Rc::new(g19));
    if layer == 20 { return Box::new(g20); }
    let mut g21 = MapBiomeEdge::new(1000, world_seed);
    g21.parent = Some(Rc::new(g20));
    if layer == 21 { return Box::new(g21); }
    let mut g22 = MapRiverInit::new(100, world_seed);
    g22.parent = Some(g17.clone());
    if layer == 22 { return Box::new(g22); }
    let g22 = Rc::new(g22);
    // TODO: use some special color palette for MapRiverInit?
    //if layer == 23 { return Box::new(MapMap { parent: Rc::new(g23), f: pretty_biome_map_hills }); }
    let mut g23 = MapZoom::new(1000, world_seed);
    g23.parent = Some(g22.clone());
    if layer == 23 { return Box::new(MapMap { parent: Rc::new(g23), f: pretty_biome_map_hills }); }
    let mut g24 = MapZoom::new(1001, world_seed);
    g24.parent = Some(Rc::new(g23));
    if layer == 24 { return Box::new(MapMap { parent: Rc::new(g24), f: pretty_biome_map_hills }); }
    let mut g25 = MapHills::new(1000, world_seed, MinecraftVersion::Java1_14);
    g25.parent1 = Some(Rc::new(g21));
    g25.parent2 = Some(Rc::new(g24));
    if layer == 25 { return Box::new(g25); }
    let mut g26 = MapRareBiome::new(1001, world_seed);
    g26.parent = Some(Rc::new(g25));
    if layer == 26 { return Box::new(g26); }
    let mut g27 = MapZoom::new(1000, world_seed);
    g27.parent = Some(Rc::new(g26));
    if layer == 27 { return Box::new(g27); }
    let mut g28 = MapAddIsland::new(3, world_seed);
    g28.parent = Some(Rc::new(g27));
    if layer == 28 { return Box::new(g28); }
    let mut g29 = MapZoom::new(1001, world_seed);
    g29.parent = Some(Rc::new(g28));
    if layer == 29 { return Box::new(g29); }
    let mut g30 = MapShore::new(1000, world_seed);
    g30.parent = Some(Rc::new(g29));
    if layer == 30 { return Box::new(g30); }
    let mut g31 = MapZoom::new(1002, world_seed);
    g31.parent = Some(Rc::new(g30));
    if layer == 31 { return Box::new(g31); }
    let mut g32 = MapZoom::new(1003, world_seed);
    g32.parent = Some(Rc::new(g31));
    if layer == 32 { return Box::new(g32); }
    let mut g33 = MapSmooth::new(1000, world_seed);
    g33.parent = Some(Rc::new(g32));
    if layer == 33 { return Box::new(g33); }
    let mut g34 = MapZoom::new(1000, world_seed);
    g34.parent = Some(g22.clone());
    if layer == 34 { return Box::new(MapMap { parent: Rc::new(g34), f: reduce_id }); }
    let mut g35 = MapZoom::new(1001, world_seed);
    g35.parent = Some(Rc::new(g34));
    if layer == 35 { return Box::new(MapMap { parent: Rc::new(g35), f: reduce_id }); }
    let mut g36 = MapZoom::new(1000, world_seed);
    g36.parent = Some(Rc::new(g35));
    if layer == 36 { return Box::new(MapMap { parent: Rc::new(g36), f: reduce_id }); }
    let mut g37 = MapZoom::new(1001, world_seed);
    g37.parent = Some(Rc::new(g36));
    if layer == 37 { return Box::new(MapMap { parent: Rc::new(g37), f: reduce_id }); }
    let mut g38 = MapZoom::new(1002, world_seed);
    g38.parent = Some(Rc::new(g37));
    if layer == 38 { return Box::new(MapMap { parent: Rc::new(g38), f: reduce_id }); }
    let mut g39 = MapZoom::new(1003, world_seed);
    g39.parent = Some(Rc::new(g38));
    if layer == 39 { return Box::new(MapMap { parent: Rc::new(g39), f: reduce_id }); }
    let mut g40 = MapRiver::new(1, world_seed);
    g40.parent = Some(Rc::new(g39));
    if layer == 40 { return Box::new(g40); }
    let mut g41 = MapSmooth::new(1000, world_seed);
    g41.parent = Some(Rc::new(g40));
    if layer == 41 { return Box::new(g41); }
    let mut g42 = MapRiverMix::new(100, world_seed);
    g42.parent1 = Some(Rc::new(g33));
    g42.parent2 = Some(Rc::new(g41));
    if layer == 42 { return Box::new(g42); }

    // 1.13 ocean layers
    let g43 = MapOceanTemp::new(2, world_seed);
    if layer == 43 { return Box::new(g43); }
    let mut g44 = MapZoom::new(2001, world_seed);
    g44.parent = Some(Rc::new(g43));
    if layer == 44 { return Box::new(g44); }
    let mut g45 = MapZoom::new(2002, world_seed);
    g45.parent = Some(Rc::new(g44));
    if layer == 45 { return Box::new(g45); }
    let mut g46 = MapZoom::new(2003, world_seed);
    g46.parent = Some(Rc::new(g45));
    if layer == 46 { return Box::new(g46); }
    let mut g47 = MapZoom::new(2004, world_seed);
    g47.parent = Some(Rc::new(g46));
    if layer == 47 { return Box::new(g47); }
    let mut g48 = MapZoom::new(2005, world_seed);
    g48.parent = Some(Rc::new(g47));
    if layer == 48 { return Box::new(g48); }
    let mut g49 = MapZoom::new(2006, world_seed);
    g49.parent = Some(Rc::new(g48));
    if layer == 49 { return Box::new(g49); }

    let mut g50 = MapOceanMix::new(100, world_seed);
    g50.parent1 = Some(Rc::new(g42)); // MapRiverMix
    g50.parent2 = Some(Rc::new(g49)); // MapZoom <- MapOceanTemp
    if layer == 50 { return Box::new(g50); }

    let mut g51 = MapVoronoiZoom::new(10, world_seed);
    g51.parent = Some(Rc::new(g50));

    Box::new(g51)
}

pub fn generate_up_to_layer_1_15(a: Area, world_seed: i64, layer: u32, version: MinecraftVersion) -> Map {
    if layer >= 200 {
        //return generate_up_to_layer_1_7_extra_2(a, world_seed, layer);
    }
    if layer >= 100 && layer <= 142 {
        // The first 42 layers are almost equal in 1.7 and 1.13
        // The main difference being the MapHills bug, which does
        // not affect the river generation code
        return generate_up_to_layer_1_7_extra(a, world_seed, layer);
    }

    generator_up_to_layer_1_15(world_seed, layer, version).get_map(a)
}

pub fn generator_up_to_layer_1_15(world_seed: i64, layer: u32, version: MinecraftVersion) -> Box<dyn GetMap> {
    let g0 = MapIsland::new(1, world_seed);
    if layer == 0 { return Box::new(g0); }
    let mut g1 = MapZoomFuzzy::new(2000, world_seed);
    g1.parent = Some(Rc::new(g0));
    if layer == 1 { return Box::new(g1); }
    let mut g2 = MapAddIsland::new(1, world_seed);
    g2.parent = Some(Rc::new(g1));
    if layer == 2 { return Box::new(g2); }
    let mut g3 = MapZoom::new(2001, world_seed);
    g3.parent = Some(Rc::new(g2));
    if layer == 3 { return Box::new(g3); }
    let mut g4 = MapAddIsland::new(2, world_seed);
    g4.parent = Some(Rc::new(g3));
    if layer == 4 { return Box::new(g4); }
    let mut g5 = MapAddIsland::new(50, world_seed);
    g5.parent = Some(Rc::new(g4));
    if layer == 5 { return Box::new(g5); }
    let mut g6 = MapAddIsland::new(70, world_seed);
    g6.parent = Some(Rc::new(g5));
    if layer == 6 { return Box::new(g6); }
    let mut g7 = MapRemoveTooMuchOcean::new(2, world_seed);
    g7.parent = Some(Rc::new(g6));
    if layer == 7 { return Box::new(g7); }
    let mut g8 = MapAddSnow::new(2, world_seed);
    g8.parent = Some(Rc::new(g7));
    if layer == 8 { return Box::new(g8); }
    let mut g9 = MapAddIsland::new(3, world_seed);
    g9.parent = Some(Rc::new(g8));
    if layer == 9 { return Box::new(g9); }
    let mut g10 = MapCoolWarm::new(2, world_seed);
    g10.parent = Some(Rc::new(g9));
    if layer == 10 { return Box::new(g10); }
    let mut g11 = MapHeatIce::new(2, world_seed);
    g11.parent = Some(Rc::new(g10));
    if layer == 11 { return Box::new(g11); }
    let mut g12 = MapSpecial::new(3, world_seed);
    g12.parent = Some(Rc::new(g11));
    if layer == 12 { return Box::new(g12); }
    let mut g13 = MapZoom::new(2002, world_seed);
    g13.parent = Some(Rc::new(g12));
    if layer == 13 { return Box::new(g13); }
    let mut g14 = MapZoom::new(2003, world_seed);
    g14.parent = Some(Rc::new(g13));
    if layer == 14 { return Box::new(g14); }
    let mut g15 = MapAddIsland::new(4, world_seed);
    g15.parent = Some(Rc::new(g14));
    if layer == 15 { return Box::new(g15); }
    let mut g16 = MapAddMushroomIsland::new(5, world_seed);
    g16.parent = Some(Rc::new(g15));
    if layer == 16 { return Box::new(g16); }
    let mut g17 = MapDeepOcean::new(4, world_seed);
    g17.parent = Some(Rc::new(g16));
    if layer == 17 { return Box::new(g17); }
    let g17 = Rc::new(g17);
    let mut g18 = MapBiome::new(200, world_seed);
    g18.parent = Some(g17.clone());
    //if layer == 18 { return Box::new(g18); }
    // 1.14: bamboo
    let mut g18b = MapAddBamboo::new(1001, world_seed);
    g18b.parent = Some(Rc::new(g18));
    if layer == 18 { return Box::new(g18b); }
    let mut g19 = MapZoom::new(1000, world_seed);
    g19.parent = Some(Rc::new(g18b));
    if layer == 19 { return Box::new(g19); }
    let mut g20 = MapZoom::new(1001, world_seed);
    g20.parent = Some(Rc::new(g19));
    if layer == 20 { return Box::new(g20); }
    let mut g21 = MapBiomeEdge::new(1000, world_seed);
    g21.parent = Some(Rc::new(g20));
    if layer == 21 { return Box::new(g21); }
    let mut g22 = MapRiverInit::new(100, world_seed);
    g22.parent = Some(g17.clone());
    if layer == 22 { return Box::new(g22); }
    let g22 = Rc::new(g22);
    // TODO: use some special color palette for MapRiverInit?
    //if layer == 23 { return Box::new(MapMap { parent: Rc::new(g23), f: pretty_biome_map_hills }); }
    let mut g23 = MapZoom::new(1000, world_seed);
    g23.parent = Some(g22.clone());
    if layer == 23 { return Box::new(MapMap { parent: Rc::new(g23), f: pretty_biome_map_hills }); }
    let mut g24 = MapZoom::new(1001, world_seed);
    g24.parent = Some(Rc::new(g23));
    if layer == 24 { return Box::new(MapMap { parent: Rc::new(g24), f: pretty_biome_map_hills }); }
    let mut g25 = MapHills::new(1000, world_seed, version);
    g25.parent1 = Some(Rc::new(g21));
    g25.parent2 = Some(Rc::new(g24));
    if layer == 25 { return Box::new(g25); }
    let mut g26 = MapRareBiome::new(1001, world_seed);
    g26.parent = Some(Rc::new(g25));
    if layer == 26 { return Box::new(g26); }
    let mut g27 = MapZoom::new(1000, world_seed);
    g27.parent = Some(Rc::new(g26));
    if layer == 27 { return Box::new(g27); }
    let mut g28 = MapAddIsland::new(3, world_seed);
    g28.parent = Some(Rc::new(g27));
    if layer == 28 { return Box::new(g28); }
    let mut g29 = MapZoom::new(1001, world_seed);
    g29.parent = Some(Rc::new(g28));
    if layer == 29 { return Box::new(g29); }
    let mut g30 = MapShore::new(1000, world_seed);
    g30.parent = Some(Rc::new(g29));
    if layer == 30 { return Box::new(g30); }
    let mut g31 = MapZoom::new(1002, world_seed);
    g31.parent = Some(Rc::new(g30));
    if layer == 31 { return Box::new(g31); }
    let mut g32 = MapZoom::new(1003, world_seed);
    g32.parent = Some(Rc::new(g31));
    if layer == 32 { return Box::new(g32); }
    let mut g33 = MapSmooth::new(1000, world_seed);
    g33.parent = Some(Rc::new(g32));
    if layer == 33 { return Box::new(g33); }
    let mut g34 = MapZoom::new(1000, world_seed);
    g34.parent = Some(g22.clone());
    if layer == 34 { return Box::new(MapMap { parent: Rc::new(g34), f: reduce_id }); }
    let mut g35 = MapZoom::new(1001, world_seed);
    g35.parent = Some(Rc::new(g34));
    if layer == 35 { return Box::new(MapMap { parent: Rc::new(g35), f: reduce_id }); }
    let mut g36 = MapZoom::new(1000, world_seed);
    g36.parent = Some(Rc::new(g35));
    if layer == 36 { return Box::new(MapMap { parent: Rc::new(g36), f: reduce_id }); }
    let mut g37 = MapZoom::new(1001, world_seed);
    g37.parent = Some(Rc::new(g36));
    if layer == 37 { return Box::new(MapMap { parent: Rc::new(g37), f: reduce_id }); }
    let mut g38 = MapZoom::new(1002, world_seed);
    g38.parent = Some(Rc::new(g37));
    if layer == 38 { return Box::new(MapMap { parent: Rc::new(g38), f: reduce_id }); }
    let mut g39 = MapZoom::new(1003, world_seed);
    g39.parent = Some(Rc::new(g38));
    if layer == 39 { return Box::new(MapMap { parent: Rc::new(g39), f: reduce_id }); }
    let mut g40 = MapRiver::new(1, world_seed);
    g40.parent = Some(Rc::new(g39));
    if layer == 40 { return Box::new(g40); }
    let mut g41 = MapSmooth::new(1000, world_seed);
    g41.parent = Some(Rc::new(g40));
    if layer == 41 { return Box::new(g41); }
    let mut g42 = MapRiverMix::new(100, world_seed);
    g42.parent1 = Some(Rc::new(g33));
    g42.parent2 = Some(Rc::new(g41));
    if layer == 42 { return Box::new(g42); }

    // 1.13 ocean layers
    let g43 = MapOceanTemp::new(2, world_seed);
    if layer == 43 { return Box::new(g43); }
    let mut g44 = MapZoom::new(2001, world_seed);
    g44.parent = Some(Rc::new(g43));
    if layer == 44 { return Box::new(g44); }
    let mut g45 = MapZoom::new(2002, world_seed);
    g45.parent = Some(Rc::new(g44));
    if layer == 45 { return Box::new(g45); }
    let mut g46 = MapZoom::new(2003, world_seed);
    g46.parent = Some(Rc::new(g45));
    if layer == 46 { return Box::new(g46); }
    let mut g47 = MapZoom::new(2004, world_seed);
    g47.parent = Some(Rc::new(g46));
    if layer == 47 { return Box::new(g47); }
    let mut g48 = MapZoom::new(2005, world_seed);
    g48.parent = Some(Rc::new(g47));
    if layer == 48 { return Box::new(g48); }
    let mut g49 = MapZoom::new(2006, world_seed);
    g49.parent = Some(Rc::new(g48));
    if layer == 49 { return Box::new(g49); }

    let mut g50 = MapOceanMix::new(100, world_seed);
    g50.parent1 = Some(Rc::new(g42)); // MapRiverMix
    g50.parent2 = Some(Rc::new(g49)); // MapZoom <- MapOceanTemp
    if layer == 50 { return Box::new(g50); }

    let mut g51 = MapVoronoiZoom115::new(world_seed);
    g51.parent = Some(Rc::new(g50));

    Box::new(g51)
}

pub fn generate_up_to_layer_1_18(a: Area3D, world_seed: i64, layer: u32, version: MinecraftVersion) -> Map3D {
    match layer {
        0..=8 | 50 | 51 => {
            let g0 = MapGenBiomeNoise3D118::new(world_seed);
            g0.partial_get_map_3d(a, layer)
        }
        _ => {
            generator_up_to_layer_1_18(world_seed, layer, version).get_map_3d(a)
        }
    }
}

pub fn generator_up_to_layer_1_18(world_seed: i64, layer: u32, version: MinecraftVersion) -> Box<dyn GetMap3D> {
    // Layers:
    // [0, 8]: before voronoi
    // 9: after voronoi
    // TODO: use helper layer to call partial_get_map_3d
    // currently this function just assumes that layers [0, 8] are always 8
    let g0 = MapGenBiomeNoise3D118::new(world_seed);
    if layer <= 8 { return Box::new(g0); }
    let mut g1 = MapVoronoiZoom118::new(world_seed);
    g1.parent = Some(Rc::new(g0));
    if layer == 9 { return Box::new(g1); }

    Box::new(g1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seed_info::BiomeId;

    #[ignore]
    #[test]
    fn all_candidate_river_maps() {
        let area = Area { x: 0, z: 0, w: 30, h: 30 };
        //let world_seed = 1234;
        for world_seed in 0..(1 << 26) {
            candidate_river_map(area, world_seed);
        }
    }

    #[ignore]
    #[test]
    fn river_seed_finder() {
        let world_seed = 2251799825931796;
        let (w, h) = (200, 25);
        let area = Area { x: 0, z: 0, w, h };
        let y_offset = 0;
        let m33 = generate_up_to_layer(MinecraftVersion::Java1_7, area, world_seed, 33, y_offset);

        let g34 = HelperMapZoomAllEdges::new(1000, world_seed);
        let g35 = HelperMapZoomAllEdges::new(1001, world_seed);
        let g36 = HelperMapZoomAllEdges::new(1000, world_seed);
        let g37 = HelperMapZoomAllEdges::new(1001, world_seed);
        let g38 = HelperMapZoomAllEdges::new(1002, world_seed);
        let g39 = HelperMapZoomAllEdges::new(1003, world_seed);
        let g40 = MapRiver::new(1, world_seed);
        let g41 = MapSmooth::new(1000, world_seed);

        let m34 = g34.get_map_from_pmap(&m33);
        let m35 = g35.get_map_from_pmap(&m34);
        let m36 = g36.get_map_from_pmap(&m35);
        let m37 = g37.get_map_from_pmap(&m36);
        let m38 = g38.get_map_from_pmap(&m37);
        let m39 = g39.get_map_from_pmap(&m38);
        let m40 = g40.get_map_from_pmap(&m39);
        let m41 = g41.get_map_from_pmap(&m40);

        let r40 = reverse_map_smooth(&m41);
        let r39 = reverse_map_river(&r40);
        let r38 = reverse_map_zoom(&r39);
        let r37 = reverse_map_zoom(&r38);
        let r36 = reverse_map_zoom(&r37);
        let r35 = reverse_map_zoom(&r36);
        let r34 = reverse_map_zoom(&r35);
        let r33 = reverse_map_zoom(&r34);
        let a_r = r33.a.clone();
        let a_s = m33.a.slice(s![1..-2, 1..-2]);
        /*
        println!("{}", draw_map(&m));
        println!("BUT GOT");
        println!("{}", draw_map(&r0));
        println!("{:?}", (m.area(), r0.area()));

        let mut diff = r0.clone();
        diff.a = &a_s ^ &a_r;
        println!("{}", draw_map(&diff));
        panic!(";D");
        */
        //assert!(a_s == a_r, format!("{:#?}", &a_s ^ &a_r));
        //assert_eq!(a_s, a_r);
        let different = (&a_s ^ &a_r).fold(0, |acc, &x| if x != 0 { acc + 1 } else { acc });
        // This fails because reverse_map_river is not implemented
        assert_eq!(different, 0);
    }

    #[test]
    fn smooth_zoom_magic_reverse_plus() {
        let world_seed = 2251799825931796;
        let (w, h) = (200, 25);
        let area = Area { x: 0, z: 0, w, h };
        let y_offset = 0;
        let m = generate_up_to_layer(MinecraftVersion::Java1_7, area, world_seed, 30, y_offset);

        let g31 = MapZoom::new(1002, world_seed);
        let g32 = MapZoom::new(1003, world_seed);
        let g33 = MapSmooth::new(1000, world_seed);

        let m_1 = g31.get_map_from_pmap(&m);
        let m1 = g32.get_map_from_pmap(&m_1);
        let m2 = g33.get_map_from_pmap(&m1);
        let b = m2;

        let r1 = reverse_map_smooth(&b);
        let r_0 = reverse_map_zoom(&r1);
        let r0 = reverse_map_zoom(&r_0);
        let a_r = r0.a.clone();
        let a_s = m.a.slice(s![1..-2, 1..-2]);
        /*
        println!("{}", draw_map(&m));
        println!("BUT GOT");
        println!("{}", draw_map(&r0));
        println!("{:?}", (m.area(), r0.area()));

        let mut diff = r0.clone();
        diff.a = &a_s ^ &a_r;
        println!("{}", draw_map(&diff));
        panic!(";D");
        */
        //assert!(a_s == a_r, format!("{:#?}", &a_s ^ &a_r));
        //assert_eq!(a_s, a_r);
        let different = (&a_s ^ &a_r).fold(0, |acc, &x| if x != 0 { acc + 1 } else { acc });
        // In this configuration we got 5 errors :(
        assert_eq!(different, 5);
    }

    #[test]
    fn smooth_zoom_magic_reverse() {
        let world_seed = 2251799825931796;
        let (w, h) = (200, 25);
        let area = Area { x: 0, z: 0, w, h };
        let y_offset = 0;
        let m = generate_up_to_layer(MinecraftVersion::Java1_7, area, world_seed, 31, y_offset);

        let g32 = HelperMapZoomAllEdges::new(1003, world_seed);
        let g33 = MapSmooth::new(1000, world_seed);

        let m1 = g32.get_map_from_pmap(&m);
        let m2 = g33.get_map_from_pmap(&m1);
        let b = m2;

        let r1 = reverse_map_smooth(&b);
        let r0 = reverse_map_zoom(&r1);
        let a_r = r0.a.clone();
        let a_s = m.a.slice(s![1..-2, 1..-2]);
        /*
        println!("{}", draw_map(&m));
        println!("BUT GOT");
        println!("{}", draw_map(&r0));
        println!("{:?}", (m.area(), r0.area()));

        let mut diff = r0.clone();
        diff.a = &a_s ^ &a_r;
        println!("{}", draw_map(&diff));
        panic!(";D");
        */
        //assert!(a_s == a_r, format!("{:#?}", &a_s ^ &a_r));
        //assert_eq!(a_s, a_r);
        let different = (&a_s ^ &a_r).fold(0, |acc, &x| if x != 0 { acc + 1 } else { acc });
        // In this configuration we got 15 errors :(
        assert_eq!(different, 15);
    }

    #[ignore]
    #[test]
    fn exists_unique_smooth() {
        use std::collections::HashMap;
        // Is there any output of MapSmooth that can only be produced by a very small number of
        // inputs?
        let (w, h) = (5, 5);
        let area = Area { x: 0, z: 0, w, h };

        // Helper function used to iterate over all possible 2-color maps
        fn next_map(m: &Map) -> Map {
            let mut n = m.clone();
            let area = n.area();
            'all: for z in 0..area.h {
                for x in 0..area.w {
                    let (x, z) = (x as usize, z as usize);
                    match m.a[(x, z)] {
                        0 => {
                            n.a[(x, z)] = 1;
                            break 'all;
                        }
                        1 => {
                            n.a[(x, z)] = 0;
                        }
                        _ => {
                            panic!("Only 2-color maps supported");
                        }
                    }
                }
            }
            n
        }

        let mut sm = HashMap::with_capacity(1 << 9);
        // Try 16 different world seeds, because MapSmooth has randomness
        // which depends on the lower 25 bits of the world_seed
        for world_seed in 0..16 {
            let map_smooth = MapSmooth::new(1000, world_seed);
            let mut m = Map::new(area);
            // Iterate over all the possible 5x5 2-color maps
            for _ in 0..=(1 << 25) {
                //println!("{}", draw_map(&m));
                // Generate the smooth of this map
                let a = map_smooth.get_map_from_pmap(&m);
                // Insert into the hashmap
                //sm.entry(a.clone()).or_insert(vec![]).push(m.clone());
                *sm.entry(a.clone()).or_insert(0) += 1;
                m = next_map(&m);
            }
        }

        let mut smv: Vec<_> = sm.into_iter().collect();
        // Sort by uniqueness
        //smv.sort_by(|(ka, va), (kb, vb)| va.len().cmp(&vb.len()));
        //smv.sort_by(|(ka, va), (kb, vb)| vb.len().cmp(&va.len()));
        smv.sort_unstable_by(|(_ka, va), (_kb, vb)| vb.cmp(&va));

        for (m3x3, v_m5x5) in smv {
            print!("{}", draw_map(&m3x3));
            println!("{} inputs", v_m5x5);
        }
        panic!(":D");
    }

    #[test]
    fn rev_map_zoom() {
        let zoom = MapZoom::new(10, 0);
        let (w, h) = (300, 300);
        let mut m = Map::new(Area { x: 0, z: 0, w, h });
        for x in 0..w {
            for z in 0..h {
                m.a[(x as usize, z as usize)] = (x * h + z) as i32;
            }
        }

        let b = zoom.get_map_from_pmap(&m);
        let r = reverse_map_zoom(&b);
        let a_r = r.a;
        let a_s = m.a.slice(s![0..-1, 0..-1]);

        //assert!(a_s == a_r, format!("{:#?}", &a_s ^ &a_r));
        //assert_eq!(a_s, a_r);
        let different = (&a_s ^ &a_r).fold(0, |acc, &x| if x != 0 { acc + 1 } else { acc });
        // In this configuration we got 1 error :(
        assert_eq!(different, 0);
    }

    #[ignore]
    #[test]
    fn smooth_is_stable() {
        // Maybe applying MapSmooth twice should be the same as applying it once?
        // Obviously not
        let world_seed = 0;
        let map_smooth = MapSmooth::new(1000, world_seed);
        let (w, h) = (300, 300);
        let area = Area { x: 0, z: 0, w, h };
        let y_offset = 0;
        let m = generate_up_to_layer(MinecraftVersion::Java1_7, area, world_seed, 32, y_offset);

        let b = map_smooth.get_map_from_pmap(&m);
        let c = map_smooth.get_map_from_pmap(&b);

        let b_s = b.a.slice(s![1..-2, 1..-2]);
        let c_s = c.a.slice(s![0..-1, 0..-1]);

        //assert!(b_s == c_s, format!("{:#?}", &b_s ^ &c_s));
        //assert_eq!(a_s, a_r);
        let different = (&b_s ^ &c_s).fold(0, |acc, &x| if x != 0 { acc + 1 } else { acc });
        assert_eq!(different, 0);
    }

    #[test]
    fn vzoom2() {
        let voronoi_zoom = MapVoronoiZoom::new(10, 0);
        let (w, h) = (30, 30);
        let mut m = Map::new(Area { x: 0, z: 0, w, h });
        //a[(0, 0)] = 1;
        for x in 0..w {
            for z in 0..h {
                m.a[(x as usize, z as usize)] = (x * h + z) as i32;
            }
        }

        let b = voronoi_zoom.get_map_from_pmap(&m);
        let a_r = reverse_map_voronoi_zoom(&b).unwrap().a;
        let a_s = m.a.slice(s![0..-1, 0..-1]);

        //assert!(a_s == a_r, format!("{:#?}", &a_s ^ &a_r));
        //assert_eq!(a_s, a_r);
        let different = (&a_s ^ &a_r).fold(0, |acc, &x| if x != 0 { acc + 1 } else { acc });
        // In this configuration we got 1 error :(
        assert_eq!(different, 1);
    }

    #[test]
    fn voronoi_zoom_parent() {
        let mut voronoi_zoom = MapVoronoiZoom::new(10, 0);
        voronoi_zoom.parent = Some(Rc::new(TestMapZero));

        let (w, h) = (10, 10);
        let map = voronoi_zoom.get_map(Area { x: 1, z: 1, w, h });
        assert_eq!(map.a.dim(), (w as usize, h as usize));
        assert_eq!(map.x, 1);
        assert_eq!(map.z, 1);

        voronoi_zoom.parent = Some(Rc::new(TestMapXhz));
        let (x, z) = (4, 4);
        let map = voronoi_zoom.get_map(Area { x, z, w, h });
        assert_eq!(map.a.dim(), (w as usize, h as usize));
        assert_eq!(map.a[(0, 0)], (((x - 2) >> 2) * ((h >> 2) + 2) as i64 + ((z - 2) >> 2)) as i32);
        assert_eq!(map.x, x);
        assert_eq!(map.z, z);
    }

    #[test]
    fn islands_match() {
        let world_seed = 9223090561890311698;
        let base_seed = 1;
        let gen = MapIsland::new(base_seed, world_seed);
        let m = gen.get_map(Area { x: -2, z: -1, w: 4, h: 2 });
        let mut t = Array2::zeros((4, 2));
        t[(1, 1)] = 1;
        t[(2, 1)] = 1;
        assert_eq!(t, m.a);
    }

    #[test]
    fn island_one_big_equals_many_small() {
        let world_seed = 9223090561890311698;
        let base_seed = 1;
        let gen = MapIsland::new(base_seed, world_seed);
        let (x, z) = (-4, -4);
        let (w, h) = (10, 10);
        let mbig = gen.get_map(Area { x, z, w, h }).a;
        let (w, h) = (w as usize, h as usize);
        let mut msmall = Array2::zeros((w, h));

        for i in 0..w {
            for j in 0..h {
                let nx = x + i as i64;
                let nz = z + j as i64;
                msmall[(i, j)] = gen.get_map(Area { x: nx, z: nz, w: 1, h: 1 }).a[(0, 0)];
            }
        }

        assert_eq!(mbig, msmall);
    }

    #[test]
    fn zoom_island_match() {
        let world_seed = 9223090561890311698;
        let base_seed = 2000;
        let mut gen = MapZoomFuzzy::new(base_seed, world_seed);
        let island = MapIsland::new(1, world_seed);
        gen.parent = Some(Rc::new(island));
        let (x, z) = (-3, -2);
        let (w, h) = (6, 4);
        let map = gen.get_map(Area { x, z, w, h });
        assert_eq!(map.a.dim(), (w as usize, h as usize));
        assert_eq!(map.x, x);
        assert_eq!(map.z, z);
        let mut t = Array2::zeros(map.a.dim());
        t[(0, 3)] = 1;
        t[(1, 2)] = 1;
        t[(2, 3)] = 1;
        t[(2, 2)] = 1;
        t[(3, 2)] = 1;
        assert_eq!(t, map.a);
    }

    // Helper function to check that a layer generates the correct area
    fn preserve_area(g: &dyn GetMap) {
        let mut av = Vec::with_capacity(9*9*10*10 * 2 + 2);
        av.push(Area { x: 0, z: 0, w: 0, h: 0 });
        av.push(Area { x: 1, z: 2, w: 0, h: 0 });
        for x in -5..5 {
            for z in -5..5 {
                for w in 1..10 {
                    for h in 1..10 {
                        av.push(Area { x, z, w, h });
                    }
                }
            }
        }
        for x in 1000..1010 {
            for z in 1000..1010 {
                for w in 1..10 {
                    for h in 1..10 {
                        av.push(Area { x, z, w, h });
                    }
                }
            }
        }
        for a in &av {
            let map = g.get_map(*a);
            assert_eq!(map.area(), *a);
        }
    }

    #[test]
    fn preserve_area_t() {
        let world_seed = 9223090561890311698;
        let base_seed = 2000;
        let parent: Option<Rc<dyn GetMap>> = Some(Rc::new(TestMapZero));
        let g0 = MapIsland::new(base_seed, world_seed);
        preserve_area(&g0);

        let mut g1 = MapZoom::new(base_seed, world_seed);
        g1.parent = parent.clone();
        preserve_area(&g1);

        let mut g2 = MapAddIsland::new(base_seed, world_seed);
        g2.parent = parent.clone();
        preserve_area(&g2);

        let mut g3 = MapVoronoiZoom::new(base_seed, world_seed);
        g3.parent = parent.clone();
        preserve_area(&g3);
    }

    #[test]
    fn preserve_area_simple() {
        preserve_area(&TestMapZero);
        preserve_area(&TestMapXhz);
    }

    #[test]
    fn preserve_area_half_voronoi() {
        let seed = 1234;
        let mut g0 = MapHalfVoronoiZoom::new(10, seed);
        g0.parent = Some(Rc::new(TestMapZero));
        preserve_area(&g0);

        let mut g1 = MapHalfVoronoiZoom115::new(seed);
        g1.parent = Some(Rc::new(TestMapZero));
        preserve_area(&g1);
    }

    #[test]
    fn generate_t() {
        let a = Area { x: 0, z: 0, w: 100, h: 100 };
        let y_offset = 0;
        generate(MinecraftVersion::Java1_7, a, 1234, y_offset);
    }

    #[test]
    fn bamboo_jungle() {
        // This is a regression test for
        // https://github.com/Cubitect/cubiomes/issues/23
        let a = Area { x: -3000, z: -3000, w: 1, h: 1 };
        let y_offset = 0;
        let m = generate(MinecraftVersion::Java1_14, a, 5010, y_offset);
        assert_eq!(m.a[(0, 0)], biome_id::bambooJungle);
    }

    #[test]
    fn reverse_voronoi_small_map() {
        fn rcoords(c: &[(i64, i64)]) -> Result<Map, ()> {
            let c: Vec<_> = c.iter().map(|c| Point { x: c.0, z: c.1 }).collect();
            let area_voronoi = Area::from_coords(c.iter().copied());
            let target_map_voronoi = map_with_river_at(&c, area_voronoi);
            reverse_map_voronoi_zoom(&target_map_voronoi)
        }
        assert!(rcoords(&[]).is_err());
        assert!(rcoords(&[(1, 1)]).is_err());
        assert!(rcoords(&[(1, 1), (1, 2)]).is_err());
        assert!(rcoords(&[(1, 1), (1, 2), (1, 3)]).is_err());
        assert!(rcoords(&[(1, 1), (1, 2), (1, 3), (1, 4)]).is_err());
        assert!(rcoords(&[(1, 1), (1, 2), (1, 3), (1, 4), (2, 1)]).is_err());
        assert!(rcoords(&[(1, 1), (1, 2), (1, 3), (1, 4), (2, 1), (3, 1)]).is_err());
        // The minimum map size is 4x4
        assert!(rcoords(&[(1, 1), (1, 2), (1, 3), (1, 4), (2, 1), (3, 1), (4, 1)]).is_ok());
    }

    #[test]
    fn reverse_voronoi_river() {
        use crate::seed_info::SeedInfo;
        let s = SeedInfo::read("seedinfo_tests/long_river_1_7.json").unwrap();

        let river_coords_voronoi = &s.biomes[&BiomeId(7)];
        let river_coords_voronoi = river_coords_voronoi.iter().cloned().collect::<Vec<_>>();
        let area_voronoi = Area::from_coords(river_coords_voronoi.iter().copied());
        let target_map_voronoi = map_with_river_at(&river_coords_voronoi, area_voronoi);
        let target_map_derived = reverse_map_voronoi_zoom(&target_map_voronoi).unwrap();
        let target_map = target_map_derived;
        println!("{}", draw_map(&target_map));

        let river_coords_rv_expected_value = s.options.other["expectedRiversPreviousLayer"].clone();
        let river_coords_rv_expected: Vec<Point> = serde_json::from_value(river_coords_rv_expected_value).unwrap();
        let area_rv = Area::from_coords(river_coords_rv_expected.iter().copied());
        let expected_rv_map = map_with_river_at(&river_coords_rv_expected, area_rv);
        println!("{}", draw_map(&expected_rv_map));
        assert_eq!(target_map, expected_rv_map);
    }

    #[test]
    fn biomes_with_negative_height() {
        // These biomes are important for treasure maps
        use biome_id::*;
        let deep = [frozenDeepOcean, coldDeepOcean, lukewarmDeepOcean, warmDeepOcean, deepOcean];
        for id in &deep { // -1.8
            assert!(BIOME_INFO[*id as usize].height < 0.0, "{}", id);
        }
        let normal = [frozenOcean, coldOcean, lukewarmOcean, warmOcean, ocean];
        for id in &normal { // -1.0
            assert!(BIOME_INFO[*id as usize].height < 0.0, "{}", id);
        }
        let rivers = [river, frozenRiver];
        for id in &rivers { // -0.5
            assert!(BIOME_INFO[*id as usize].height < 0.0, "{}", id);
        }
        let swamps = [swampland, swampland | 0x80];
        for id in &swamps { // -0.2, -0.1
            assert!(BIOME_INFO[*id as usize].height < 0.0, "{}", id);
        }
    }

    #[test]
    fn voronoi_1_15() {
        use crate::seed_info::SeedInfo;
        let s = SeedInfo::read("seedinfo_tests/voronoi_1_15.json").unwrap();
        let river_coords = &s.biomes[&BiomeId(7)];
        println!("{}", draw_map(&map_with_river_at(river_coords, Area::from_coords(river_coords.iter().copied()))));
        let y_offset = 0;
        let m = generate(MinecraftVersion::Java1_15, Area::from_coords(river_coords.iter().copied()), s.world_seed.unwrap(), y_offset);
        println!("{}", draw_map(&m));
        for r in river_coords {
            let gr = m.get(r.x, r.z);
            assert!(gr == 7, "{:?} should be river but is {}", r, gr);
        }
    }

    #[test]
    fn sha256_byte_order() {
        let input = 2499980394650691929u64 as i64;
        let expected = 11169288594992569606u64 as i64;

        assert_eq!(sha256_long_to_long(input), expected);
    }

    #[test]
    fn sha256_collision_34_bits() {
        use crate::java_rng::mask;

        // These values of the world seed result in the same lower 34 bits
        // after being hashed. This doesn't seed to be useful, but here they
        // are anyway.
        let x = [19424995491, 55159191312, 47814887371];
        for &i in &x {
            assert_eq!(sha256_long_to_long(i) & mask(34) as i64, i & mask(34) as i64);
        }
    }

    #[test]
    fn sha256_collision_similar_biome_seeds() {
        use crate::java_rng::mask;

        // The similar biomes trick doesn't work as well in 1.15 because of the
        // changes in MapVoronoiZoom. However, it is still possible to find
        // some seeds that share the lower 34 bits when hashed
        let x = [36159317779, 54081671403, 3734291918, 41695158396];
        for &i in &x {
            let similar = McRng::similar_biome_seed(i);
            assert_eq!(sha256_long_to_long(i) & mask(34) as i64, sha256_long_to_long(similar) & mask(34) as i64);
        }
    }

    #[test]
    fn index_of_min_element_tie() {
        assert_eq!(index_of_min_element(&[0.0, 1.0]).unwrap(), 0);
        assert_eq!(index_of_min_element(&[1.0, 0.0]).unwrap(), 1);
        assert_eq!(index_of_min_element(&[0.0, 0.0]).unwrap(), 0);
        assert_eq!(index_of_min_element(&[0.1, 0.0, 0.0]).unwrap(), 1);
    }

    #[test]
    fn candidates_26() {
        use crate::seed_info::SeedInfo;
        let s = SeedInfo::read("seedinfo_tests/long_river_1_7.json").unwrap();

        let river_coords_voronoi = &s.biomes[&BiomeId(7)];
        let river_coords_voronoi = river_coords_voronoi.iter().cloned().collect::<Vec<_>>();
        let seed26: u32 = 0x03A1F4CC;
        let range_lo = 0xf84c80;
        let river_coords_quarter_scale = convert_hd_coords_into_quarter_scale(&river_coords_voronoi);
        let candidates = river_seed_finder_26_range(&river_coords_quarter_scale, range_lo, range_lo + (1 << 7));
        assert!(candidates.contains(&(seed26 as i64)), "{:?}", candidates);
    }

    #[test]
    fn split_rivers_into_fragments_integer_division() {
        let p = vec![Point { x: 0, z: 0 }, Point { x: -1, z: 0 }];
        // This function used to make one big fragment near (0, 0) because of using / instead of >>
        let x = split_rivers_into_fragments(&p);
        // The correct implementation should create 2 fragments
        assert_eq!(x.len(), 2);
    }

    #[test]
    fn split_rivers_into_fragments4_integer_division() {
        let p = vec![Point4 { x: 0, z: 0 }, Point4 { x: -1, z: 0 }];
        // This function used to make one big fragment near (0, 0) because of using / instead of >>
        let x = split_rivers_into_fragments4(&p);
        // The correct implementation should create 2 fragments
        assert_eq!(x.len(), 2);
    }

    #[test]
    fn treasure_map_1_13_test_case() {
        let seed = -7014733495468514438;
        let fragment_x = 2;
        let fragment_z = -2;
        let map_bytes = generate_image_treasure_map_at(MinecraftVersion::Java1_13, seed, fragment_x, fragment_z);

        assert_eq!(map_bytes.len(), 128 * 128 * 4);

        // TODO: instead of using the hash of the image, store the treasure map in SeedInfo format
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&map_bytes);
        let r = hasher.finalize();
        assert_eq!(r.to_vec(), vec![2, 164, 56, 89, 18, 20, 220, 103, 112, 230, 180, 212, 73, 255, 209, 131, 125, 156, 151, 14, 110, 104, 67, 247, 31, 50, 114, 198, 244, 85, 4, 116]);
    }

    #[test]
    fn treasure_map_1_15_test_case() {
        let seed = -7014733495468514438;
        let fragment_x = 2;
        let fragment_z = -2;
        let map_bytes = generate_image_treasure_map_at(MinecraftVersion::Java1_15, seed, fragment_x, fragment_z);

        assert_eq!(map_bytes.len(), 128 * 128 * 4);

        // TODO: instead of using the hash of the image, store the treasure map in SeedInfo format
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(&map_bytes);
        let r = hasher.finalize();
        assert_eq!(r.to_vec(), vec![22, 23, 27, 71, 87, 221, 197, 105, 176, 3, 90, 34, 222, 117, 239, 165, 169, 117, 157, 35, 0, 177, 27, 253, 76, 154, 247, 248, 197, 175, 50, 246]);
    }

    #[test]
    fn reverse_treasure_map() {
        let seed = 1239;
        let parea = Area {
            x: -32,
            z: -32,
            w: 128,
            h: 128,
        };
        // Generate a 128x128 treasure map
        let mhv: Rc<dyn GetMap> = {
            let mut mhv = MapHalfVoronoiZoom::new(10, seed);
            let parent = Rc::from(generator_up_to_layer_1_13(seed, 50));
            mhv.parent = Some(parent);

            Rc::from(mhv)
        };

        fn is_land_biome_12(x: i32) -> i32 {
            if is_land_biome(x) {
                1
            } else {
                0
            }
        }

        let mm = MapMap {
            parent: Rc::clone(&mhv),
            f: is_land_biome_12,
        };
        let parea2 = Area {
            // TODO: the x,z coords are wrong?
            x: -34,
            z: -34,
            w: 128,
            h: 128,
        };
        let mut pmap = mm.get_map(parea2);
        // Set the pixels at margin to unknown, we will ignore them anyway
        set_pixels_at_margin(&mut pmap, 2);

        let mt = MapTreasure {
            parent: mhv,
        };

        let mut map = mt.get_map(parea);

        // But treasure maps have 126x126 resulution, so delete border pixels
        set_pixels_at_margin(&mut map, 0);

        let mut reversed_map = reverse_map_treasure(&map);
        // Set the pixels at margin to unknown, we will ignore them anyway
        set_pixels_at_margin(&mut reversed_map, 2);

        println!("{}", draw_map(&pmap));
        println!("{}", draw_map(&reversed_map));

        // TODO: the x,z coords are wrong?
        // The bug is probably in MapHalfVoronoiZoom or in reverse_map_treasure
        // Compare map array only, ignore coords

        assert_eq!(reversed_map.a, pmap.a);
    }

    #[test]
    fn test_generation_mc_98995() {
        // Maybe https://bugs.mojang.com/browse/MC-98995
        // This is a test to handle changes in version 1.9.4 and 1.10.2
        let a = Area { x: 157, z: -573, w: 1, h: 1 };
        // 1.7: got 6 (correct)
        // 1.8.9: got 6 (correct)
        let version = MinecraftVersion::Java1_7;
        let y_offset = 0;
        let m = generate_up_to_layer(version, a, 2727174149152569210, version.num_layers(), y_offset);
        assert_eq!(m.a[(0, 0)], biome_id::swampland);

        // 1.9.4: got 6 (expected 27)
        // 1.10.2: got 6 (expected 27)
        let version = MinecraftVersion::Java1_9;
        let y_offset = 0;
        let m = generate_up_to_layer(version, a, 2727174149152569210, version.num_layers(), y_offset);
        assert_eq!(m.a[(0, 0)], biome_id::birchForest);

        // 1.11.2: got 6 (correct)
        let version = MinecraftVersion::Java1_11;
        let y_offset = 0;
        let m = generate_up_to_layer(version, a, 2727174149152569210, version.num_layers(), y_offset);
        assert_eq!(m.a[(0, 0)], biome_id::swampland);
    }

    #[test]
    fn test_generation_1_16_1() {
        // This is a regression test for
        // https://github.com/Cubitect/cubiomes/issues/51
        let a = Area { x: 180, z: 171, w: 1, h: 1 };
        let version = MinecraftVersion::Java1_16_1;
        let y_offset = 0;
        let m = generate_up_to_layer(version, a, 1437905338718953247, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], biome_id::mesa);
    }

    #[test]
    fn test_generation_1_16_1_b() {
        // This is a regression test for
        // https://github.com/Cubitect/cubiomes/issues/51
        let a = Area { x: -158, z: 23, w: 1, h: 1 };
        let version = MinecraftVersion::Java1_16_1;
        let y_offset = 0;
        let m = generate_up_to_layer(version, a, 84, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], biome_id::mesa);
    }

    #[test]
    fn test_generation_1_16_2() {
        // This is a regression test for
        // https://github.com/Cubitect/cubiomes/issues/51
        let a = Area { x: 180, z: 171, w: 1, h: 1 };
        let version = MinecraftVersion::Java1_16;
        let y_offset = 0;
        let m = generate_up_to_layer(version, a, 1437905338718953247, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], biome_id::mesaPlateau_F);
    }

    #[test]
    fn test_generation_1_16_2_b() {
        // This is a regression test for
        // https://github.com/Cubitect/cubiomes/issues/51
        let a = Area { x: -158, z: 23, w: 1, h: 1 };
        let version = MinecraftVersion::Java1_16;
        let y_offset = 0;
        let m = generate_up_to_layer(version, a, 84, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], biome_id::mesaPlateau_F);
    }

    #[test]
    fn get_category_replaces_get_biome_type() {
        for biome_id in 0..256 {
            #[allow(deprecated)]
            let old = get_biome_type(biome_id);
            // Using 1.16.1 because 1.16.2 changes the category of mesaPlateau
            let new = get_category(MinecraftVersion::Java1_16_1, biome_id);
            //assert_eq!(new, Some(old), "{}", biome_id);
            if new.unwrap_or(0) != old {
                panic!("{} should be {} and is {:?}", biome_id, old, new);
            }
        }
    }

    #[test]
    fn color_to_biome_map_works() {
        let cbm = color_to_biome_map();

        let color_plains = biome_to_color(biome_id::plains);

        assert_eq!(cbm[&color_plains], biome_id::plains);
    }

    #[test]
    fn all_biomes_have_unique_colors() {
        let num_biomes = 256;
        let mut h: HashMap<_, Vec<i32>> = HashMap::with_capacity(num_biomes);

        for biome_id in 0..num_biomes {
            let biome_id = i32::try_from(biome_id).unwrap();
            let color = biome_to_color(biome_id);
            h.entry(color).or_default().push(biome_id);
        }

        let mut bad_colors = vec![];

        let color_black = biome_to_color(255);
        for (colors, biomes) in h.iter() {
            if biomes.len() > 1 {
                // Biomes that don't exist are black
                if *colors != color_black {
                    bad_colors.push((colors, biomes.clone()));
                }
            }
        }

        assert_eq!(bad_colors, vec![]);
    }

    #[test]
    fn test_generation_bamboo_hills_fail1() {
        let a = Area { x: 188, z: -71, w: 1, h: 1 };
        let version = MinecraftVersion::Java1_16;
        let y_offset = 0;
        let m = generate_up_to_layer(version, a, 797383349100663716, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 168);
        // This generation has not changed since 1.14
        let version = MinecraftVersion::Java1_14;
        let y_offset = 0;
        let m = generate_up_to_layer(version, a, 797383349100663716, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 168);
    }

    #[test]
    fn test_generation_bamboo_hills_fail2() {
        let a = Area { x: -23, z: 163, w: 1, h: 1 };
        let version = MinecraftVersion::Java1_16;
        let y_offset = 0;
        let m = generate_up_to_layer(version, a, -5450423930667436192, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 169);
        // This generation has not changed since 1.14
        let version = MinecraftVersion::Java1_14;
        let y_offset = 0;
        let m = generate_up_to_layer(version, a, -5450423930667436192, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 169);
    }

    // 1.18 test cases from
    // https://github.com/Cubitect/cubiomes/issues/61

    #[test]
    #[ignore] // this test fails
    fn test_generation_1_18_test1() {
        let world_seed = 1234;
        // Biome mismatch at (-95, -16, -56), expected 4 got 7
        // Biome mismatch at (-79, -16, -53), expected 7 got 0
        // Biome mismatch at (-136, -16, 13), expected 4 got 1
        let version = MinecraftVersion::Java1_18;
        let y_offset = 0;
        let a = Area { x: -95, z: -56, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 4);

        let a = Area { x: -79, z: -53, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 7);

        let a = Area { x: -136, z: 13, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 4);
    }

    #[test]
    fn test_generation_1_18_test2() {
        let world_seed = -4100855569562546563;
        // Biome mismatch at (640, -16, -16), expected 4 got 1
        let version = MinecraftVersion::Java1_18;
        let y_offset = 0;
        let a = Area { x: 640, z: -16, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 4);
    }

    #[test]
    fn test_generation_1_18_test3() {
        let world_seed = -7088473816240932309;
        // Biome mismatch at (-30, -7, -120), expected 175 got 45
        // Biome mismatch at (38, -5, -104), expected 175 got 45
        // Biome mismatch at (38, -5, -103), expected 175 got 45
        // Biome mismatch at (38, -5, -102), expected 175 got 45
        // 175 is lush_caves
        let lush_caves_id = BiomeId(fastanvil::biome::Biome::LushCaves as i32).0;
        let version = MinecraftVersion::Java1_18;
        let y_offset = 16-7;
        let a = Area { x: -30, z: -120, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], lush_caves_id);

        let y_offset = 16-5;
        let a = Area { x: 38, z: -104, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], lush_caves_id);
        let a = Area { x: 38, z: -103, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], lush_caves_id);
        let a = Area { x: 38, z: -102, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], lush_caves_id);
    }

    // Additional tests from manually generated worlds
    #[test]
    #[ignore] // this test fails
    fn test_generation_1_18_test4() {
        let world_seed = 1234;
        // thread 'main' panicked at 'Mismatch at (-98, 46, -49): expected 7 generated 4', src/main.rs:1115:29
        let version = MinecraftVersion::Java1_18;
        let y_offset = 16+46;
        let a = Area { x: -98, z: -49, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 7);
    }

    #[test]
    fn test_generation_1_18_test5() {
        let world_seed = 1234;
        // thread 'main' panicked at 'Mismatch at (-154, 1, 2): expected 174 generated 4 (distance to 2nd biome: 156)', src/main.rs:1410:33
        let version = MinecraftVersion::Java1_18;
        let y_offset = 16+1;
        let a = Area { x: -154, z: 2, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 174);

        // thread 'main' panicked at 'Mismatch at (-159, 0, -1): expected 4 generated 174 (distance to 2nd biome: 161)', src/main.rs:1410:33
        let version = MinecraftVersion::Java1_18;
        let y_offset = 16+0;
        let a = Area { x: -159, z: -1, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 4);

        // thread 'main' panicked at 'Mismatch at (-158, 2, 1): expected 174 generated 4 (distance to 2nd biome: 145)', src/main.rs:1410:33
        let version = MinecraftVersion::Java1_18;
        let y_offset = 16+2;
        let a = Area { x: -158, z: 1, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 174);
    }

    #[test]
    fn test_generation_1_18_test6() {
        let world_seed = -4100855569562546563;
        // thread 'main' panicked at 'Mismatch at (145, 18, -10): expected 179 generated 174 (distance to 2nd biome: 225)', src/main.rs:1410:33
        let version = MinecraftVersion::Java1_18;
        let y_offset = 16+18;
        let a = Area { x: 145, z: -10, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 179);
    }

    // thread 'main' panicked at 'Mismatch at (-22, -1, -107): expected 16 generated 178 (distance to 2nd biome: 133)', src/main.rs:1410:33
    // thread 'main' panicked at 'Mismatch at (-22, -1, -106): expected 16 generated 178 (distance to 2nd biome: 133)', src/main.rs:1410:33
    // thread 'main' panicked at 'Mismatch at (30, -3, -97): expected 45 generated 178 (distance to 2nd biome: 135)', src/main.rs:1410:33
    #[test]
    #[ignore] // this test fails
    fn test_generation_1_18_test7() {
        let world_seed = -7088473816240932309;
        let version = MinecraftVersion::Java1_18;
        let y_offset = 16-1;
        let a = Area { x: -22, z: -107, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 16);

        let version = MinecraftVersion::Java1_18;
        let y_offset = 16-1;
        let a = Area { x: -22, z: -106, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 16);

        let version = MinecraftVersion::Java1_18;
        let y_offset = 16-3;
        let a = Area { x: 30, z: -97, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 45);
    }

    // These test cases were generated by making small changes to the generation code, and
    // comparing biomes that would have been different.

    #[test]
    fn test_generation_1_18_test8() {
        // TestCase { world_seed: 1234, y_offset: 23, x: -11215, z: 49355, b1: 30, b2: 4 }', examples/test_case_finder.rs:43:13
        let world_seed = 1234;
        let version = MinecraftVersion::Java1_18;
        let y_offset = 23;
        let a = Area { x: -11215, z: 49355, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 30);
    }

    #[test]
    fn test_generation_1_18_test9() {
        // TestCase { world_seed: 1234, y_offset: 11, x: 15002, z: -70700, b1: 27, b2: 178 }', examples/test_case_finder.rs:43:13
        let world_seed = 1234;
        let version = MinecraftVersion::Java1_18;
        let y_offset = 11;
        let a = Area { x: 15002, z: -70700, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 27);
    }

    #[test]
    fn test_generation_1_18_test10() {
        // TestCase { world_seed: 1234, y_offset: 13, x: 88062, z: -86044, b1: 4, b2: 174 }', examples/test_case_finder.rs:43:13
        let world_seed = 1234;
        let version = MinecraftVersion::Java1_18;
        let y_offset = 13;
        let a = Area { x: 88062, z: -86044, w: 1, h: 1 };
        let m = generate_up_to_layer(version, a, world_seed, version.num_layers() - 1, y_offset);
        assert_eq!(m.a[(0, 0)], 4);
    }
}
