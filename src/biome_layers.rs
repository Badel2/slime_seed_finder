use crate::mc_rng::McRng;
use crate::noise_generator::NoiseGeneratorPerlin;
use crate::seed_info::MinecraftVersion;
// TODO: Array2[(x, z)] is a nice syntax, but the fastest dimension to iterate
// is the z dimension, but in the Java code it is the x dimension, as the arrays
// are defined as (z * w + x).
use log::{debug, error};
use ndarray::Array2;
use serde::{Serialize, Deserialize};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use crate::java_rng::JavaRng;
use crate::seed_info::Point;

// The different Map* layers are copied from
// https://github.com/Cubitect/cubiomes
// since it's easier to translate C to Rust than Java to Rust.

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Area {
    pub x: i64,
    pub z: i64,
    pub w: u64,
    pub h: u64,
}

impl Area {
    /// Returns true if (x, z) in inside this area
    fn contains(&self, x: i64, z: i64) -> bool {
        x >= self.x && x < self.x + self.w as i64 && z >= self.z && z < self.z + self.h as i64
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
    pub fn area(&self) -> Area {
        let (w, h) = self.a.dim();
        Area { x: self.x, z: self.z, w: w as u64, h: h as u64 }
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
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
                self.cache.borrow_mut().insert((area.x as i64 + x as i64, area.z as i64 + z as i64), m.a[(x, z)]);
            }
        }
    }
    fn get_all_from_cache(&self, area: Area) -> Option<Map> {
        let mut m = Map::new(area);
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
                if let Some(b) = self.cache.borrow().get(&(area.x as i64 + x as i64, area.z as i64 + z as i64)) {
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

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Biome {
    pub id: i32,
    pub type_0: i32,
    pub height: f64,
    pub temp: f64,
    pub tempCat: i32,
}

fn get_biome_type(id: i32) -> i32 {
    BIOME_INFO[id as usize].type_0
}
fn biome_exists(id: i32) -> bool {
    BIOME_INFO[(id & 0xff) as usize].id & (!0xff) == 0
}
fn is_oceanic(id: i32) -> bool {
    use self::biome_id::*;
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
fn is_biome_JFTO(id: i32) -> bool {
    use self::biome_id::*;
    biome_exists(id) && (get_biome_type(id) == Jungle || id == forest || id == taiga || is_oceanic(id))
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

    let (r, g, b);
    if id < 128 {
        r = BIOME_COLORS[id][0];
        g = BIOME_COLORS[id][1];
        b = BIOME_COLORS[id][2];
    } else {
        r = BIOME_COLORS[id][0].saturating_add(40);
        g = BIOME_COLORS[id][1].saturating_add(40);
        b = BIOME_COLORS[id][2].saturating_add(40);
    }

    [r, g, b, 255]
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
        let mut m = Map::new(area);
        for z in 0..area.h {
            for x in 0..area.w {
                let rx = ((area.x as u64).wrapping_add(x) % 4) as usize;
                let rz = ((area.z as u64).wrapping_add(z) % 4) as usize;
                m.a[(x as usize, z as usize)] = colors[rz * 4 + rx];
            }
        }

        m
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let area = pmap.area();

        self.get_map(area)
    }
}

pub struct TestMapXhz;

impl GetMap for TestMapXhz {
    fn get_map(&self, area: Area) -> Map {
        let mut m = Map::new(area);
        for z in 0..area.h {
            for x in 0..area.w {
                m.a[(x as usize, z as usize)] = ((area.x.wrapping_add(x as i64)).wrapping_mul(area.h as i64) + (area.z.wrapping_add(z as i64))) as i32;
            }
        }

        m
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let area = pmap.area();

        self.get_map(area)
    }
}

// A map which applies a function to its parent map
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

        for z in 0..p_h - 1 {
            let mut v00 = pmap.a[(0, z)];
            let mut v01 = pmap.a[(0, z+1)];

            for x in 0..p_w - 1 {
                let v10 = pmap.a[(x+1, z)]; //& 0xFF;
                let v11 = pmap.a[(x+1, z+1)]; //& 0xFF;

                // Missed optimization (not present in Java):
                // if v00 == v01 == v10 == v11,
                // buf will always be set to the same value, so skip
                // all the calculations
                // Benchmark result: x10 speedup when pmap is all zeros
                if v00 == v01 && v00 == v10 && v00 == v11 {
                    for j in 0..4 {
                        for i in 0..4 {
                            let x = x as usize;
                            let z = z as usize;
                            let idx = ((x << 2) + i, (z << 2) + j);
                            m.a[idx] = v00;
                        }
                    }

                    v00 = v10;
                    v01 = v11;
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

                v00 = v10;
                v01 = v11;
            }
        }

        m
    }
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
        let mut m = Map::new(area);
        let mut r = McRng::new(self.base_seed, self.world_seed);

        for z in 0..area.h {
            for x in 0..area.w {
                let chunk_x = x as i64 + area.x;
                let chunk_z = z as i64 + area.z;
                r.set_chunk_seed(chunk_x, chunk_z);

                m.a[(x as usize, z as usize)] = if r.next_int_n(10) == 0 { 1 } else { 0 };
            }
        }

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
    pub fuzzy: bool, // true when parent is MapIsland
    pub bug_world_seed_not_set: bool, // true if this layer is parent2 of MapHills
}

impl MapZoom {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None, fuzzy: false, bug_world_seed_not_set: false }
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

        for z in 0..p_h - 1 {
            let mut a = pmap.a[(0, z+0)];
            let mut b = pmap.a[(0, z+1)];
            for x in 0..p_w - 1 {
                let a1 = pmap.a[(x+1, z+0)];
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

                    a = a1;
                    b = b1;
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
                    r.select_mode_or_random(a, a1, b, b1)
                };

                a = a1;
                b = b1;
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

        for z in 0..p_h - 1 {
            let mut a = pmap.a[(0, z+0)];
            let mut b = pmap.a[(0, z+1)];
            for x in 0..p_w - 1 {
                let a1 = pmap.a[(x+1, z+0)];
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

                    a = a1;
                    b = b1;
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

                a = a1;
                b = b1;
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
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
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
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
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
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
                let v11 = pmap.a[(x+1, z+1)];

                m.a[(x, z)] = if v11 == 0 {
                    v11
                } else {
                    let chunk_x = x as i64 + area.x;
                    let chunk_z = z as i64 + area.z;
                    r.set_chunk_seed(chunk_x, chunk_z);
                    let r = r.next_int_n(6);

                    if r == 0 {
                        4
                    } else if r <= 1 {
                        3
                    } else {
                        1
                    }
                }
            }
        }

        m
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
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
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
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
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
            let parea = Area {
                x: area.x,
                z: area.z,
                w: area.w,
                h: area.h
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has no margin: pmap.w == map.w
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let (p_w, p_h) = pmap.a.dim();
        let mut m = pmap.clone();

        let mut r = McRng::new(self.base_seed, self.world_seed);
        for z in 0..p_h as usize {
            for x in 0..p_w as usize {
                let mut v = pmap.a[(x, z)];
                if v != 0 {
                    let chunk_x = x as i64 + m.x;
                    let chunk_z = z as i64 + m.z;
                    r.set_chunk_seed(chunk_x, chunk_z);
                    if r.next_int_n(13) == 0 {
                        // What does this mean?
                        // if v == 1 and here we set it to 0x101..0xF01
                        // then it won't trigger any v == 1 checks in the future
                        v |= (1 + r.next_int_n(15)) << 8 & 0xf00;
                    }
                }

                m.a[(x, z)] = v;
            }
        }

        m
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
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
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
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
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
            let parea = Area {
                x: area.x,
                z: area.z,
                w: area.w,
                h: area.h
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has no margin: pmap.w == map.w
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        use self::biome_id::*;
        let warmBiomes = [desert, desert, desert, savanna, savanna, plains];
        let lushBiomes = [forest, roofedForest, extremeHills, plains, birchForest, swampland];
        let coldBiomes = [forest, extremeHills, taiga, plains];
        let snowBiomes = [icePlains, icePlains, icePlains, coldTaiga];

        let (p_w, p_h) = pmap.a.dim();
        let mut m = pmap.clone();

        let mut r = McRng::new(self.base_seed, self.world_seed);
        for z in 0..p_h as usize {
            for x in 0..p_w as usize {
                let mut id = pmap.a[(x, z)];
                let has_high_bit = ((id & 0xf00) >> 8) != 0;
                id &= -0xf01;
                if get_biome_type(id) == Ocean || id == mushroomIsland {
                    m.a[(x, z)] = id;
                    continue;
                }

                let chunk_x = x as i64 + m.x;
                let chunk_z = z as i64 + m.z;
                r.set_chunk_seed(chunk_x, chunk_z);
                m.a[(x, z)] = match id {
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
                };
            }
        }

        m
    }
}

fn is_deep_ocean(id: i32) -> bool {
    use biome_id::*;
    match id {
        deepOcean | warmDeepOcean | lukewarmDeepOcean | coldDeepOcean | frozenDeepOcean => true,
        _ => false,
    }
}

fn equal_or_plateau(id1: i32, id2: i32) -> bool {
    use self::biome_id::*;
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
    return get_biome_type(id1) == get_biome_type(id2);
}

fn replace_edge(out: &mut i32, v10: i32, v21: i32, v01: i32, v12: i32, id: i32, base_id: i32, edge_id: i32) -> bool {
    if id != base_id {
        return false;
    }

    if [v10, v21, v01, v12].iter().map(|&x| equal_or_plateau(x, base_id)).all(|x| x) {
        *out = id;
    } else {
        *out = edge_id;
    }
    return true;
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
        use self::biome_id::*;
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
                let v10 = pmap.a[(x+1, z+0)];
                let v21 = pmap.a[(x+2, z+1)];
                let v01 = pmap.a[(x+0, z+1)];
                let v12 = pmap.a[(x+1, z+2)];
                let v11 = pmap.a[(x+1, z+1)];

                if !replace_edge(&mut m.a[(x, z)], v10, v21, v01, v12, v11, mesaPlateau_F, mesa) &&
                !replace_edge(&mut m.a[(x, z)], v10, v21, v01, v12, v11, mesaPlateau, mesa) &&
                !replace_edge(&mut m.a[(x, z)], v10, v21, v01, v12, v11, megaTaiga, taiga)
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
            let parea = Area {
                x: area.x,
                z: area.z,
                w: area.w,
                h: area.h
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has no margin: pmap.w == map.w
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let (p_w, p_h) = pmap.a.dim();
        let mut m = pmap.clone();

        let mut r = McRng::new(self.base_seed, self.world_seed);
        for z in 0..p_h as usize {
            for x in 0..p_w as usize {
                let v = pmap.a[(x, z)];
                m.a[(x, z)] = if v > 0 {
                    let chunk_x = x as i64 + m.x;
                    let chunk_z = z as i64 + m.z;
                    r.set_chunk_seed(chunk_x, chunk_z);
                    r.next_int_n(299999) + 2
                } else {
                    0
                };
            }
        }

        m
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
    pub parent1: Option<Rc<dyn GetMap>>,
    pub parent2: Option<Rc<dyn GetMap>>,
}

impl MapHills {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent1: None, parent2: None }
    }
    pub fn get_map_from_pmap12(&self, pmap1: &Map, pmap2: &Map) -> Map {
        use self::biome_id::*;
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
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
                let chunk_x = x as i64 + m.x;
                let chunk_z = z as i64 + m.z;
                r.set_chunk_seed(chunk_x, chunk_z);
                let a11 = pmap1.a[(x+1, z+1)]; // biome
                let b11 = pmap2.a[(x+1, z+1)]; // river

                let var12 = (b11 - 2) % 29 == 0;

                m.a[(x, z)] = if a11 != 0 && b11 >= 2 && (b11 - 2) % 29 == 1 && a11 < 128 {
                    if biome_exists(a11 + 128) {
                        a11 + 128
                    } else {
                        a11
                    }
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
                        _ => if equal_or_plateau(a11, mesaPlateau_F) {
                            mesa
                        } else if is_deep_ocean(a11) && r.next_int_n(3) == 0 {
                            // TODO: is_deep_ocean was introduced in 1.13
                            if r.next_int_n(2) == 0 { plains } else { forest }
                        } else {
                            a11
                        }
                    };

                    if var12 && hill_id != a11 {
                        hill_id = if biome_exists(hill_id + 128) {
                            hill_id + 128
                        } else {
                            a11
                        };
                    }

                    if hill_id == a11 {
                        a11
                    } else {
                        let a10 = pmap1.a[(x+1, z+0)];
                        let a21 = pmap1.a[(x+2, z+1)];
                        let a01 = pmap1.a[(x+0, z+1)];
                        let a12 = pmap1.a[(x+1, z+2)];
                        let mut equals = 0;
                        if equal_or_plateau(a10, a11) { equals += 1; }
                        if equal_or_plateau(a21, a11) { equals += 1; }
                        if equal_or_plateau(a01, a11) { equals += 1; }
                        if equal_or_plateau(a12, a11) { equals += 1; }

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
    // TODO: this layer does not need the margin?
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        use self::biome_id::*;
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        let mut r = McRng::new(self.base_seed, self.world_seed);
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
                let v11 = pmap.a[(x+1, z+1)];

                let chunk_x = x as i64 + m.x;
                let chunk_z = z as i64 + m.z;
                r.set_chunk_seed(chunk_x, chunk_z);
                m.a[(x, z)] = if r.next_int_n(57) == 0 && v11 == plains {
                    // Sunflower Plains
                    plains + 128
                } else {
                    v11
                };
            }
        }

        m
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
        use self::biome_id::*;
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
            return true;
        }

        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
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
                } else if /* biome < 128 && */ get_biome_type(biome) == Jungle {
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
                            if get_biome_type(v10) == Mesa && get_biome_type(v21) == Mesa && get_biome_type(v01) == Mesa && get_biome_type(v12) == Mesa {
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
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
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
        use self::biome_id::*;
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
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
        use self::biome_id::*;
        let (p_w, p_h) = pmap.a.dim();
        let area = Area {
            x: pmap.x + 1,
            z: pmap.z + 1,
            w: p_w as u64 - 2,
            h: p_h as u64 - 2
        };
        let mut m = Map::new(area);
        for z in 0..area.h as usize {
            for x in 0..area.w as usize {
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
        use self::biome_id::*;
        let (p_w, p_h) = pmap1.a.dim();
        {
            // Check that both maps are of same size and coords
            assert_eq!(pmap1.area(), pmap2.area());
        }
        let mut m = pmap1.clone();
        for z in 0..p_h as usize {
            for x in 0..p_w as usize {
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
        use biome_id::*;
        let mut m = Map::new(area);

        for z in 0..area.h {
            for x in 0..area.w {
                let tmp = self.perlin.get_ocean_temp((x as i64 + area.x) as f64 / 8.0, (z as i64 + area.z) as f64 / 8.0, 0.0);
                m.a[(x as usize, z as usize)] = if tmp > 0.4 {
                    warmOcean
                } else if tmp > 0.2 {
                    lukewarmOcean
                } else if tmp < -0.4 {
                    frozenOcean
                } else if tmp < -0.2 {
                    coldOcean
                } else {
                    ocean
                };
            }
        }

        m
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
        use self::biome_id::*;
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
        for z in 0..p_h as usize {
            'loop_x: for x in 0..p_w as usize {
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
                                continue 'loop_x;
                            }

                            if ocean_id == frozenOcean {
                                m.a[(x, z)] = coldOcean;
                                continue 'loop_x;
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
            let parea = Area {
                x: area.x,
                z: area.z,
                w: area.w,
                h: area.h
            };
            let pmap = parent.get_map(parea);

            let map = self.get_map_from_pmap(&pmap);

            // No need to crop
            map
        } else {
            panic!("Parent not set");
        }
    }

    // pmap has no margin: pmap.w == map.w
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        use biome_id::*;
        let (p_w, p_h) = pmap.a.dim();
        let mut m = pmap.clone();

        let mut r = McRng::new(self.base_seed, self.world_seed);
        for z in 0..p_h as usize {
            for x in 0..p_w as usize {
                let v = pmap.a[(x, z)];
                if v == jungle {
                    let chunk_x = x as i64 + m.x;
                    let chunk_z = z as i64 + m.z;
                    r.set_chunk_seed(chunk_x, chunk_z);
                    if r.next_int_n(10) == 0 {
                        m.a[(x, z)] = bambooJungle;
                    }
                }
            }
        }

        m
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

                for z in 0..p_h - 1 {
                    for x in 0..p_w - 1 {
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

/// We lose some information here :/
/// Returns a tuple (BiomeMap, RiverMap)
fn decompose_map_river_mix(map: &Map) -> (SparseMap, SparseMap) {
    use self::biome_id::*;
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


// TODO: this function must do the reverse of edge detection
pub fn reverse_map_river(m: &Map) -> Map {
    let (w, h) = m.a.dim();
    let (p_w, p_h) = (w - 2, h - 2);
    let (p_w, p_h) = (p_w as u64, p_h as u64);
    let mut pmap = Map::new(Area { x: m.x + 1, z: m.z + 1, w: p_w, h: p_h });

    for z in 0..p_h {
        for x in 0..p_w {
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

    for z in 0..p_h {
        for x in 0..p_w {
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

    for z in 0..p_h {
        for x in 0..p_w {
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
    // Ignore these functions, I decided to shift the map by 2 and make them useless
    fn divide_coord_by_4(x: i64) -> i64 {
        // 0 => 0
        // 1 => 0
        // 2 => 0
        // 3 => 1
        // 4 => 1
        // 5 => 1
        // 6 => 1
        // 7 => 2
        (x + 1) / 4
    }
    fn multiply_coord_by_4(x: i64) -> i64 {
        // 0 => 2
        // 1 => 6
        (x * 4) + 2
    }
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
    for z in 0..p_h as usize {
        for x in 0..p_w as usize {
            let xx = m4(x as i64) as usize;
            let zz = m4(z as i64) as usize;
            //println!("{:?} => {:?}", (x, z), (xx, zz));
            pmap.a[(x, z)] = adjusted_map[(xx, zz)];
        }
    }

    Ok(pmap)
}

fn slice_to_area(mut m: Map, a: Area) -> Map {
    debug!("{:?} vs {:?}", m.area(), a);
    let x_diff = a.x - m.x;
    let z_diff = a.z - m.z;
    m.x += x_diff;
    m.z += z_diff;
    let (x_diff, z_diff) = (x_diff as i32, z_diff as i32);
    let (new_w, new_h) = (a.w as i32 + x_diff, a.h as i32 + z_diff);
    debug!("x_diff: {}, z_diff: {}, new_w: {}, new_h: {}", x_diff, z_diff, new_w, new_h);
    m.a.slice_collapse(s![x_diff..new_w, z_diff..new_h]);
    debug!("{:?} vs {:?}", m.area(), a);
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
    for (x, z) in coords {
        if x as u8 % 4 == 2 && z as u8 % 4 == 2 {
            prevoronoi_coords.push((x, z));
        } else {
            hd_coords.push((x, z));
        }
    }

    // Now, try to build Area from other_coords, and duplicate all the
    // voronoi_coords which are inside this area
    let area = area_from_coords(&hd_coords);
    for &(x, z) in &prevoronoi_coords {
        if area.contains(x, z) {
            hd_coords.push((x, z));
        }
    }

    (prevoronoi_coords, hd_coords)
}

/// River Seed Finder
pub fn river_seed_finder(river_coords_voronoi: &[Point], extra_biomes: &[(i32, i64, i64)]) -> Vec<i64> {
    river_seed_finder_range(river_coords_voronoi, extra_biomes, 0, 1 << 25)
}

pub fn river_seed_finder_26_range(river_coords_voronoi: &[Point], range_lo: u32, range_hi: u32) -> Vec<i64> {
    // This iterator has 2**24 elements
    let iter25 = McRng::similar_biome_seed_iterator_bits(25).skip(range_lo as usize).take((range_hi - range_lo) as usize);
    // prevoronoi_coords are used to find the first 26 bits
    // But we can use all the coords with reverse_map_voronoi_zoom to get the same result
    let area_voronoi = area_from_coords(&river_coords_voronoi);
    let target_map_voronoi = map_with_river_at(&river_coords_voronoi, area_voronoi);
    let target_map_derived = match reverse_map_voronoi_zoom(&target_map_voronoi) {
        Ok(x) => x,
        Err(()) => {
            debug!("Too few rivers, minimum map size is 8x8");
            return vec![];
        },
    };

    let (prevoronoi_coords, _hd_coords) = segregate_coords_prevoronoi_hd(river_coords_voronoi.to_vec());
    // If the area is large, do a few quick 1x1 checks
    // TODO: do it even if the area is not large?
    let check_coords: Vec<_> = [prevoronoi_coords[0], prevoronoi_coords[prevoronoi_coords.len() / 2], prevoronoi_coords[prevoronoi_coords.len() - 1]].into_iter().map(|(x, z)| {
        let x = (x - 2) / 4;
        let z = (z - 2) / 4;
        Area { x, z, w: 1, h: 1, }
    }).collect();

    let target_map = target_map_derived;
    let area = target_map.area();
    let target_score = count_rivers(&target_map);

    debug!("{}", draw_map(&target_map));
    debug!("Target score: {}", target_score);
    let mut candidates_26 = vec![];

    for world_seed in iter25 {
        // Do a quick check for 2 seeds at once: with the bit 25 undefined
        if check_coords.iter().all(|area| {
            let candidate_map = candidate_river_map_bit_25_undefined(*area, world_seed);
            candidate_map.a[(0, 0)] != biome_id::river
        }) {
            // Skip this candidate if none of the check_coords are river
            continue;
        }

        // Check with bit 25 set to 0
        let candidate_map = candidate_river_map(area, world_seed);
        //debug!("{}", draw_map(&candidate_map));

        let and_map = map_river_and(candidate_map, &target_map);
        let candidate_score = count_rivers(&and_map);
        if candidate_score >= target_score * 90 / 100 {
            let similar_biome_seed = McRng::similar_biome_seed(world_seed) & ((1 << 26) - 1);
            debug!("{:08X}: {}", world_seed, candidate_score);
            debug!("{:08X}: {}", similar_biome_seed, candidate_score);
            candidates_26.push(world_seed);
            candidates_26.push(similar_biome_seed);
        }

        // Check with bit 25 set to 1
        // If the area is large enough, we could skip this check if the map
        // with bit 25 set to 0 had very few matches, as the two maps are
        // usually pretty similar at large scales
        let world_seed = world_seed ^ (1 << 25);
        let candidate_map = candidate_river_map(area, world_seed);
        //debug!("{}", draw_map(&candidate_map));

        let and_map = map_river_and(candidate_map, &target_map);
        let candidate_score = count_rivers(&and_map);
        if candidate_score >= target_score * 90 / 100 {
            let similar_biome_seed = McRng::similar_biome_seed(world_seed) & ((1 << 26) - 1);
            debug!("{:08X}: {}", world_seed, candidate_score);
            debug!("{:08X}: {}", similar_biome_seed, candidate_score);
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
pub fn river_seed_finder_range(river_coords_voronoi: &[Point], extra_biomes: &[(i32, i64, i64)], range_lo: u32, range_hi: u32) -> Vec<i64> {
    // prevoronoi_coords are used to find the first 26 bits
    // But we can use all the coords with reverse_map_voronoi_zoom to get the same result
    let area_voronoi = area_from_coords(&river_coords_voronoi);
    let target_map_voronoi = map_with_river_at(&river_coords_voronoi, area_voronoi);
    let target_map_derived = match reverse_map_voronoi_zoom(&target_map_voronoi) {
        Ok(x) => x,
        Err(()) => {
            debug!("Too few rivers, minimum map size is 8x8");
            return vec![];
        },
    };

    let target_map = target_map_derived;
    let area = target_map.area();
    let target_score = count_rivers(&target_map);

    // For the 34-bit voronoi phase we only want to compare hd_coords
    let (_prevoronoi_coords, hd_coords) = segregate_coords_prevoronoi_hd(river_coords_voronoi.to_vec());
    let hd_area = area_from_coords(&hd_coords);
    let target_map_voronoi_hd = map_with_river_at(&hd_coords, hd_area);
    let target_map_derived_hd = match reverse_map_voronoi_zoom(&target_map_voronoi_hd) {
        Ok(x) => x,
        Err(()) => {
            debug!("Too few high resolution river borders!");
            return river_seed_finder_26_range(river_coords_voronoi, range_lo, range_hi);
        },
    };

    let target_map_hd = target_map_derived_hd;
    // Compare resolution of original and reverse-voronoi + voronoi
    let g43 = MapVoronoiZoom::new(10, 1234);
    let target_rv_voronoi = g43.get_map_from_pmap(&target_map_hd);

    let target_map_voronoi_sliced = slice_to_area(target_map_voronoi_hd.clone(), target_rv_voronoi.area());

    // Actually, we only want to compare borders, so use HelperMapRiverAll, which is actually an
    // edge detector
    let target_map_voronoi_sliced = HelperMapRiverAll::new(1, 0).get_map_from_pmap(&target_map_voronoi_sliced);
    let target_score_voronoi_sliced = count_rivers(&target_map_voronoi_sliced);

    // Ok, begin bruteforce!

    let candidates_26 = river_seed_finder_26_range(river_coords_voronoi, range_lo, range_hi);

    debug!("Target voronoi score: {}", target_score_voronoi_sliced);
    // Now use voronoi zoom to bruteforce the remaining (34-26 = 8 bits)
    let candidates_34 = candidates_26.into_iter().flat_map(|x| {
        let mut v = vec![];
        for seed in 0..(1 << (34 - 26)) {
            let world_seed = x | (seed << 26);
            let g43 = MapVoronoiZoom::new(10, world_seed);
            let candidate_voronoi = g43.get_map_from_pmap(&target_map_hd);
            let candidate_voronoi = HelperMapRiverAll::new(1, 0).get_map_from_pmap(&candidate_voronoi);
            //debug!("{}", draw_map(&target_map_voronoi_sliced));
            //debug!("{}", draw_map(&candidate_voronoi));
            let and_map = map_river_and(candidate_voronoi, &target_map_voronoi_sliced);
            let candidate_score = count_rivers(&and_map);
            if candidate_score >= target_score_voronoi_sliced * 90 / 100 {
                debug!("{:09X}: {}", world_seed, candidate_score);
                v.push(world_seed);
            }
        }

        v
    }).collect::<Vec<_>>();
    debug!("{:08X?}", candidates_34);
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
        // Compare only rivers
        //let g41 = generate_up_to_layer(MinecraftVersion::Java1_7, area, world_seed, 41);
        // Compare all biomes (slower)
        let g42 = generate_up_to_layer(MinecraftVersion::Java1_7, area, world_seed, 42);

        let candidate_score = count_rivers_exact(&g42, &target_map);
        if candidate_score >= target_score * 90 / 100 {
            // When most rivers match, try extra biomes
            let mut hits = 0;
            let target = extra_biomes.len() * 90 / 100;
            for (biome, x, z) in extra_biomes.iter().cloned() {
                let area = Area { x, z, w: 1, h: 1 };
                let g43 = generate_up_to_layer(MinecraftVersion::Java1_7, area, world_seed, 43);
                if g43.a[(0, 0)] == biome {
                    hits += 1;
                }
            }
            if hits >= target {
                debug!("{:016X}: {}", world_seed, candidate_score);
                Some(world_seed)
            } else {
                None
            }
        } else {
            None
        }
    }).collect::<Vec<_>>();
    candidates_64.sort_unstable();
    debug!("{:016X?}", candidates_64);
    debug!("64 bit candidates: {}", candidates_64.len());

    candidates_64
}

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

fn count_rivers_exact(a: &Map, b: &Map) -> u32 {
    assert_eq!(a.area(), b.area());
    let area = a.area();
    let mut acc = 0;
    for z in 0..area.h as usize {
        for x in 0..area.w as usize {
            let v11_a = a.a[(x, z)];
            let v11_b = b.a[(x, z)];
            acc += if v11_a == biome_id::river && v11_a == v11_b {
                1
            } else if v11_a == biome_id::river || v11_b == biome_id::river {
                -1
            } else {
                0
            };
        }
    }

    if acc < 0 { 0 } else { acc as u32 }
}

fn all_candidate_river_maps() {
    let area = Area { x: 250, z: 50, w: 20, h: 20 };
    let world_seed = 0x00ABCDEF;
    let target_map = generate_up_to_layer(MinecraftVersion::Java1_7, area, world_seed, 41);
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

pub fn area_from_coords(c: &[(i64, i64)]) -> Area {
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

pub fn map_with_river_at(c: &[(i64, i64)], area: Area) -> Map {
    let mut m = Map::new(area);
    for (x, z) in c {
        m.a[((x - area.x) as usize, (z - area.z) as usize)] = biome_id::river;
    }
    m
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
    use std::fmt::Write;
    let (w, h) = map.a.dim();
    let mut s = String::new();
    write!(s, "MAP: x: {}, z: {}, {}x{}\n", map.x, map.z, w, h).unwrap();
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


pub fn generate_image(version: MinecraftVersion, area: Area, seed: i64) -> Vec<u8> {
    let num_layers = version.num_layers();
    generate_image_up_to_layer(version, area, seed, num_layers)
}

pub fn generate_image_up_to_layer(version: MinecraftVersion, area: Area, seed: i64, layer: u32) -> Vec<u8> {
    let map = generate_up_to_layer(version, area, seed, layer);

    draw_map_image(&map)
}

pub fn generate(version: MinecraftVersion, a: Area, world_seed: i64) -> Map {
    let num_layers = version.num_layers();
    generate_up_to_layer(version, a, world_seed, num_layers)
}

pub fn generate_up_to_layer(version: MinecraftVersion, area: Area, seed: i64, num_layers: u32) -> Map {
    match version {
        MinecraftVersion::Java1_7 => generate_up_to_layer_1_7(area, seed, num_layers),
        MinecraftVersion::Java1_13 => generate_up_to_layer_1_13(area, seed, num_layers),
        MinecraftVersion::Java1_14 => generate_up_to_layer_1_14(area, seed, num_layers),
        _ => {
            error!("Biome generation in version {:?} is not implemented", version);
            panic!("Biome generation in version {:?} is not implemented", version);
        }
    }
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

    TestMapZero.get_map(a)
}

pub fn generate_up_to_layer_1_7(a: Area, world_seed: i64, layer: u32) -> Map {
    if layer >= 200 {
        return generate_up_to_layer_1_7_extra_2(a, world_seed, layer);
    }
    if layer >= 100 {
        return generate_up_to_layer_1_7_extra(a, world_seed, layer);
    }
    let g0 = MapIsland::new(1, world_seed);
    if layer == 0 { return g0.get_map(a); }
    let mut g1 = MapZoom::new(2000, world_seed);
    g1.parent = Some(Rc::new(g0));
    g1.fuzzy = true;
    if layer == 1 { return g1.get_map(a); }
    let mut g2 = MapAddIsland::new(1, world_seed);
    g2.parent = Some(Rc::new(g1));
    if layer == 2 { return g2.get_map(a); }
    let mut g3 = MapZoom::new(2001, world_seed);
    g3.parent = Some(Rc::new(g2));
    if layer == 3 { return g3.get_map(a); }
    let mut g4 = MapAddIsland::new(2, world_seed);
    g4.parent = Some(Rc::new(g3));
    if layer == 4 { return g4.get_map(a); }
    let mut g5 = MapAddIsland::new(50, world_seed);
    g5.parent = Some(Rc::new(g4));
    if layer == 5 { return g5.get_map(a); }
    let mut g6 = MapAddIsland::new(70, world_seed);
    g6.parent = Some(Rc::new(g5));
    if layer == 6 { return g6.get_map(a); }
    let mut g7 = MapRemoveTooMuchOcean::new(2, world_seed);
    g7.parent = Some(Rc::new(g6));
    if layer == 7 { return g7.get_map(a); }
    let mut g8 = MapAddSnow::new(2, world_seed);
    g8.parent = Some(Rc::new(g7));
    if layer == 8 { return g8.get_map(a); }
    let mut g9 = MapAddIsland::new(3, world_seed);
    g9.parent = Some(Rc::new(g8));
    if layer == 9 { return g9.get_map(a); }
    let mut g10 = MapCoolWarm::new(2, world_seed);
    g10.parent = Some(Rc::new(g9));
    if layer == 10 { return g10.get_map(a); }
    let mut g11 = MapHeatIce::new(2, world_seed);
    g11.parent = Some(Rc::new(g10));
    if layer == 11 { return g11.get_map(a); }
    let mut g12 = MapSpecial::new(3, world_seed);
    g12.parent = Some(Rc::new(g11));
    if layer == 12 { return g12.get_map(a); }
    let mut g13 = MapZoom::new(2002, world_seed);
    g13.parent = Some(Rc::new(g12));
    if layer == 13 { return g13.get_map(a); }
    let mut g14 = MapZoom::new(2003, world_seed);
    g14.parent = Some(Rc::new(g13));
    if layer == 14 { return g14.get_map(a); }
    let mut g15 = MapAddIsland::new(4, world_seed);
    g15.parent = Some(Rc::new(g14));
    if layer == 15 { return g15.get_map(a); }
    let mut g16 = MapAddMushroomIsland::new(5, world_seed);
    g16.parent = Some(Rc::new(g15));
    if layer == 16 { return g16.get_map(a); }
    let mut g17 = MapDeepOcean::new(4, world_seed);
    g17.parent = Some(Rc::new(g16));
    let g17 = Rc::new(g17);
    if layer == 17 { return g17.get_map(a); }
    let mut g18 = MapBiome::new(200, world_seed);
    g18.parent = Some(g17.clone());
    if layer == 18 { return g18.get_map(a); }
    let mut g19 = MapZoom::new(1000, world_seed);
    g19.parent = Some(Rc::new(g18));
    if layer == 19 { return g19.get_map(a); }
    let mut g20 = MapZoom::new(1001, world_seed);
    g20.parent = Some(Rc::new(g19));
    if layer == 20 { return g20.get_map(a); }
    let mut g21 = MapBiomeEdge::new(1000, world_seed);
    g21.parent = Some(Rc::new(g20));
    if layer == 21 { return g21.get_map(a); }
    let mut g22 = MapRiverInit::new(100, world_seed);
    g22.parent = Some(g17.clone());
    let g22 = Rc::new(g22);
    if layer == 22 { return g22.get_map(a); }
    // TODO: use some special color palette for MapRiverInit?
    //if layer == 23 { return MapMap { parent: Rc::new(g23), f: pretty_biome_map_hills }.get_map(a); }
    let mut g23 = MapZoom::new(1000, world_seed);
    g23.parent = Some(g22.clone());
    g23.bug_world_seed_not_set = true;
    if layer == 23 { return MapMap { parent: Rc::new(g23), f: pretty_biome_map_hills }.get_map(a); }
    let mut g24 = MapZoom::new(1001, world_seed);
    g24.parent = Some(Rc::new(g23));
    g24.bug_world_seed_not_set = true;
    if layer == 24 { return MapMap { parent: Rc::new(g24), f: pretty_biome_map_hills }.get_map(a); }
    let mut g25 = MapHills::new(1000, world_seed);
    g25.parent1 = Some(Rc::new(g21));
    g25.parent2 = Some(Rc::new(g24));
    if layer == 25 { return g25.get_map(a); }
    let mut g26 = MapRareBiome::new(1001, world_seed);
    g26.parent = Some(Rc::new(g25));
    if layer == 26 { return g26.get_map(a); }
    let mut g27 = MapZoom::new(1000, world_seed);
    g27.parent = Some(Rc::new(g26));
    if layer == 27 { return g27.get_map(a); }
    let mut g28 = MapAddIsland::new(3, world_seed);
    g28.parent = Some(Rc::new(g27));
    if layer == 28 { return g28.get_map(a); }
    let mut g29 = MapZoom::new(1001, world_seed);
    g29.parent = Some(Rc::new(g28));
    if layer == 29 { return g29.get_map(a); }
    let mut g30 = MapShore::new(1000, world_seed);
    g30.parent = Some(Rc::new(g29));
    if layer == 30 { return g30.get_map(a); }
    let mut g31 = MapZoom::new(1002, world_seed);
    g31.parent = Some(Rc::new(g30));
    if layer == 31 { return g31.get_map(a); }
    let mut g32 = MapZoom::new(1003, world_seed);
    g32.parent = Some(Rc::new(g31));
    if layer == 32 { return g32.get_map(a); }
    let mut g33 = MapSmooth::new(1000, world_seed);
    g33.parent = Some(Rc::new(g32));
    if layer == 33 { return g33.get_map(a); }
    let mut g34 = MapZoom::new(1000, world_seed);
    g34.parent = Some(g22.clone());
    if layer == 34 { return MapMap { parent: Rc::new(g34), f: reduce_id }.get_map(a); }
    let mut g35 = MapZoom::new(1001, world_seed);
    g35.parent = Some(Rc::new(g34));
    if layer == 35 { return MapMap { parent: Rc::new(g35), f: reduce_id }.get_map(a); }
    let mut g36 = MapZoom::new(1000, world_seed);
    g36.parent = Some(Rc::new(g35));
    if layer == 36 { return MapMap { parent: Rc::new(g36), f: reduce_id }.get_map(a); }
    let mut g37 = MapZoom::new(1001, world_seed);
    g37.parent = Some(Rc::new(g36));
    if layer == 37 { return MapMap { parent: Rc::new(g37), f: reduce_id }.get_map(a); }
    let mut g38 = MapZoom::new(1002, world_seed);
    g38.parent = Some(Rc::new(g37));
    if layer == 38 { return MapMap { parent: Rc::new(g38), f: reduce_id }.get_map(a); }
    let mut g39 = MapZoom::new(1003, world_seed);
    g39.parent = Some(Rc::new(g38));
    if layer == 39 { return MapMap { parent: Rc::new(g39), f: reduce_id }.get_map(a); }
    let mut g40 = MapRiver::new(1, world_seed);
    g40.parent = Some(Rc::new(g39));
    if layer == 40 { return g40.get_map(a); }
    let mut g41 = MapSmooth::new(1000, world_seed);
    g41.parent = Some(Rc::new(g40));
    if layer == 41 { return g41.get_map(a); }
    let mut g42 = MapRiverMix::new(100, world_seed);
    g42.parent1 = Some(Rc::new(g33));
    g42.parent2 = Some(Rc::new(g41));
    if layer == 42 { return g42.get_map(a); }
    let mut g43 = MapVoronoiZoom::new(10, world_seed);
    g43.parent = Some(Rc::new(g42));

    let m1 = g43.get_map(a);
    m1
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
    let g0 = MapIsland::new(1, world_seed);
    if layer == 0 { return g0.get_map(a); }
    let mut g1 = MapZoom::new(2000, world_seed);
    g1.parent = Some(Rc::new(g0));
    g1.fuzzy = true;
    if layer == 1 { return g1.get_map(a); }
    let mut g2 = MapAddIsland::new(1, world_seed);
    g2.parent = Some(Rc::new(g1));
    if layer == 2 { return g2.get_map(a); }
    let mut g3 = MapZoom::new(2001, world_seed);
    g3.parent = Some(Rc::new(g2));
    if layer == 3 { return g3.get_map(a); }
    let mut g4 = MapAddIsland::new(2, world_seed);
    g4.parent = Some(Rc::new(g3));
    if layer == 4 { return g4.get_map(a); }
    let mut g5 = MapAddIsland::new(50, world_seed);
    g5.parent = Some(Rc::new(g4));
    if layer == 5 { return g5.get_map(a); }
    let mut g6 = MapAddIsland::new(70, world_seed);
    g6.parent = Some(Rc::new(g5));
    if layer == 6 { return g6.get_map(a); }
    let mut g7 = MapRemoveTooMuchOcean::new(2, world_seed);
    g7.parent = Some(Rc::new(g6));
    if layer == 7 { return g7.get_map(a); }
    let mut g8 = MapAddSnow::new(2, world_seed);
    g8.parent = Some(Rc::new(g7));
    if layer == 8 { return g8.get_map(a); }
    let mut g9 = MapAddIsland::new(3, world_seed);
    g9.parent = Some(Rc::new(g8));
    if layer == 9 { return g9.get_map(a); }
    let mut g10 = MapCoolWarm::new(2, world_seed);
    g10.parent = Some(Rc::new(g9));
    if layer == 10 { return g10.get_map(a); }
    let mut g11 = MapHeatIce::new(2, world_seed);
    g11.parent = Some(Rc::new(g10));
    if layer == 11 { return g11.get_map(a); }
    let mut g12 = MapSpecial::new(3, world_seed);
    g12.parent = Some(Rc::new(g11));
    if layer == 12 { return g12.get_map(a); }
    let mut g13 = MapZoom::new(2002, world_seed);
    g13.parent = Some(Rc::new(g12));
    if layer == 13 { return g13.get_map(a); }
    let mut g14 = MapZoom::new(2003, world_seed);
    g14.parent = Some(Rc::new(g13));
    if layer == 14 { return g14.get_map(a); }
    let mut g15 = MapAddIsland::new(4, world_seed);
    g15.parent = Some(Rc::new(g14));
    if layer == 15 { return g15.get_map(a); }
    let mut g16 = MapAddMushroomIsland::new(5, world_seed);
    g16.parent = Some(Rc::new(g15));
    if layer == 16 { return g16.get_map(a); }
    let mut g17 = MapDeepOcean::new(4, world_seed);
    g17.parent = Some(Rc::new(g16));
    let g17 = Rc::new(g17);
    if layer == 17 { return g17.get_map(a); }
    let mut g18 = MapBiome::new(200, world_seed);
    g18.parent = Some(g17.clone());
    if layer == 18 { return g18.get_map(a); }
    let mut g19 = MapZoom::new(1000, world_seed);
    g19.parent = Some(Rc::new(g18));
    if layer == 19 { return g19.get_map(a); }
    let mut g20 = MapZoom::new(1001, world_seed);
    g20.parent = Some(Rc::new(g19));
    if layer == 20 { return g20.get_map(a); }
    let mut g21 = MapBiomeEdge::new(1000, world_seed);
    g21.parent = Some(Rc::new(g20));
    if layer == 21 { return g21.get_map(a); }
    let mut g22 = MapRiverInit::new(100, world_seed);
    g22.parent = Some(g17.clone());
    let g22 = Rc::new(g22);
    if layer == 22 { return g22.get_map(a); }
    // TODO: use some special color palette for MapRiverInit?
    //if layer == 23 { return MapMap { parent: Rc::new(g23), f: pretty_biome_map_hills }.get_map(a); }
    let mut g23 = MapZoom::new(1000, world_seed);
    g23.parent = Some(g22.clone());
    if layer == 23 { return MapMap { parent: Rc::new(g23), f: pretty_biome_map_hills }.get_map(a); }
    let mut g24 = MapZoom::new(1001, world_seed);
    g24.parent = Some(Rc::new(g23));
    if layer == 24 { return MapMap { parent: Rc::new(g24), f: pretty_biome_map_hills }.get_map(a); }
    let mut g25 = MapHills::new(1000, world_seed);
    g25.parent1 = Some(Rc::new(g21));
    g25.parent2 = Some(Rc::new(g24));
    if layer == 25 { return g25.get_map(a); }
    let mut g26 = MapRareBiome::new(1001, world_seed);
    g26.parent = Some(Rc::new(g25));
    if layer == 26 { return g26.get_map(a); }
    let mut g27 = MapZoom::new(1000, world_seed);
    g27.parent = Some(Rc::new(g26));
    if layer == 27 { return g27.get_map(a); }
    let mut g28 = MapAddIsland::new(3, world_seed);
    g28.parent = Some(Rc::new(g27));
    if layer == 28 { return g28.get_map(a); }
    let mut g29 = MapZoom::new(1001, world_seed);
    g29.parent = Some(Rc::new(g28));
    if layer == 29 { return g29.get_map(a); }
    let mut g30 = MapShore::new(1000, world_seed);
    g30.parent = Some(Rc::new(g29));
    if layer == 30 { return g30.get_map(a); }
    let mut g31 = MapZoom::new(1002, world_seed);
    g31.parent = Some(Rc::new(g30));
    if layer == 31 { return g31.get_map(a); }
    let mut g32 = MapZoom::new(1003, world_seed);
    g32.parent = Some(Rc::new(g31));
    if layer == 32 { return g32.get_map(a); }
    let mut g33 = MapSmooth::new(1000, world_seed);
    g33.parent = Some(Rc::new(g32));
    if layer == 33 { return g33.get_map(a); }
    let mut g34 = MapZoom::new(1000, world_seed);
    g34.parent = Some(g22.clone());
    if layer == 34 { return MapMap { parent: Rc::new(g34), f: reduce_id }.get_map(a); }
    let mut g35 = MapZoom::new(1001, world_seed);
    g35.parent = Some(Rc::new(g34));
    if layer == 35 { return MapMap { parent: Rc::new(g35), f: reduce_id }.get_map(a); }
    let mut g36 = MapZoom::new(1000, world_seed);
    g36.parent = Some(Rc::new(g35));
    if layer == 36 { return MapMap { parent: Rc::new(g36), f: reduce_id }.get_map(a); }
    let mut g37 = MapZoom::new(1001, world_seed);
    g37.parent = Some(Rc::new(g36));
    if layer == 37 { return MapMap { parent: Rc::new(g37), f: reduce_id }.get_map(a); }
    let mut g38 = MapZoom::new(1002, world_seed);
    g38.parent = Some(Rc::new(g37));
    if layer == 38 { return MapMap { parent: Rc::new(g38), f: reduce_id }.get_map(a); }
    let mut g39 = MapZoom::new(1003, world_seed);
    g39.parent = Some(Rc::new(g38));
    if layer == 39 { return MapMap { parent: Rc::new(g39), f: reduce_id }.get_map(a); }
    let mut g40 = MapRiver::new(1, world_seed);
    g40.parent = Some(Rc::new(g39));
    if layer == 40 { return g40.get_map(a); }
    let mut g41 = MapSmooth::new(1000, world_seed);
    g41.parent = Some(Rc::new(g40));
    if layer == 41 { return g41.get_map(a); }
    let mut g42 = MapRiverMix::new(100, world_seed);
    g42.parent1 = Some(Rc::new(g33));
    g42.parent2 = Some(Rc::new(g41));
    if layer == 42 { return g42.get_map(a); }

    // 1.13 ocean layers
    let g43 = MapOceanTemp::new(2, world_seed);
    if layer == 43 { return g43.get_map(a); }
    let mut g44 = MapZoom::new(2001, world_seed);
    g44.parent = Some(Rc::new(g43));
    if layer == 44 { return g44.get_map(a); }
    let mut g45 = MapZoom::new(2002, world_seed);
    g45.parent = Some(Rc::new(g44));
    if layer == 45 { return g45.get_map(a); }
    let mut g46 = MapZoom::new(2003, world_seed);
    g46.parent = Some(Rc::new(g45));
    if layer == 46 { return g46.get_map(a); }
    let mut g47 = MapZoom::new(2004, world_seed);
    g47.parent = Some(Rc::new(g46));
    if layer == 47 { return g47.get_map(a); }
    let mut g48 = MapZoom::new(2005, world_seed);
    g48.parent = Some(Rc::new(g47));
    if layer == 48 { return g48.get_map(a); }
    let mut g49 = MapZoom::new(2006, world_seed);
    g49.parent = Some(Rc::new(g48));
    if layer == 49 { return g49.get_map(a); }

    let mut g50 = MapOceanMix::new(100, world_seed);
    g50.parent1 = Some(Rc::new(g42)); // MapRiverMix
    g50.parent2 = Some(Rc::new(g49)); // MapZoom <- MapOceanTemp
    if layer == 50 { return g50.get_map(a); }

    let mut g51 = MapVoronoiZoom::new(10, world_seed);
    g51.parent = Some(Rc::new(g50));

    let m1 = g51.get_map(a);
    m1
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
    let g0 = MapIsland::new(1, world_seed);
    if layer == 0 { return g0.get_map(a); }
    let mut g1 = MapZoom::new(2000, world_seed);
    g1.parent = Some(Rc::new(g0));
    g1.fuzzy = true;
    if layer == 1 { return g1.get_map(a); }
    let mut g2 = MapAddIsland::new(1, world_seed);
    g2.parent = Some(Rc::new(g1));
    if layer == 2 { return g2.get_map(a); }
    let mut g3 = MapZoom::new(2001, world_seed);
    g3.parent = Some(Rc::new(g2));
    if layer == 3 { return g3.get_map(a); }
    let mut g4 = MapAddIsland::new(2, world_seed);
    g4.parent = Some(Rc::new(g3));
    if layer == 4 { return g4.get_map(a); }
    let mut g5 = MapAddIsland::new(50, world_seed);
    g5.parent = Some(Rc::new(g4));
    if layer == 5 { return g5.get_map(a); }
    let mut g6 = MapAddIsland::new(70, world_seed);
    g6.parent = Some(Rc::new(g5));
    if layer == 6 { return g6.get_map(a); }
    let mut g7 = MapRemoveTooMuchOcean::new(2, world_seed);
    g7.parent = Some(Rc::new(g6));
    if layer == 7 { return g7.get_map(a); }
    let mut g8 = MapAddSnow::new(2, world_seed);
    g8.parent = Some(Rc::new(g7));
    if layer == 8 { return g8.get_map(a); }
    let mut g9 = MapAddIsland::new(3, world_seed);
    g9.parent = Some(Rc::new(g8));
    if layer == 9 { return g9.get_map(a); }
    let mut g10 = MapCoolWarm::new(2, world_seed);
    g10.parent = Some(Rc::new(g9));
    if layer == 10 { return g10.get_map(a); }
    let mut g11 = MapHeatIce::new(2, world_seed);
    g11.parent = Some(Rc::new(g10));
    if layer == 11 { return g11.get_map(a); }
    let mut g12 = MapSpecial::new(3, world_seed);
    g12.parent = Some(Rc::new(g11));
    if layer == 12 { return g12.get_map(a); }
    let mut g13 = MapZoom::new(2002, world_seed);
    g13.parent = Some(Rc::new(g12));
    if layer == 13 { return g13.get_map(a); }
    let mut g14 = MapZoom::new(2003, world_seed);
    g14.parent = Some(Rc::new(g13));
    if layer == 14 { return g14.get_map(a); }
    let mut g15 = MapAddIsland::new(4, world_seed);
    g15.parent = Some(Rc::new(g14));
    if layer == 15 { return g15.get_map(a); }
    let mut g16 = MapAddMushroomIsland::new(5, world_seed);
    g16.parent = Some(Rc::new(g15));
    if layer == 16 { return g16.get_map(a); }
    let mut g17 = MapDeepOcean::new(4, world_seed);
    g17.parent = Some(Rc::new(g16));
    let g17 = Rc::new(g17);
    if layer == 17 { return g17.get_map(a); }
    let mut g18 = MapBiome::new(200, world_seed);
    g18.parent = Some(g17.clone());
    //if layer == 18 { return g18.get_map(a); }
    // 1.14: bamboo
    let mut g18b = MapAddBamboo::new(1001, world_seed);
    g18b.parent = Some(Rc::new(g18));
    if layer == 18 { return g18b.get_map(a); }
    let mut g19 = MapZoom::new(1000, world_seed);
    g19.parent = Some(Rc::new(g18b));
    if layer == 19 { return g19.get_map(a); }
    let mut g20 = MapZoom::new(1001, world_seed);
    g20.parent = Some(Rc::new(g19));
    if layer == 20 { return g20.get_map(a); }
    let mut g21 = MapBiomeEdge::new(1000, world_seed);
    g21.parent = Some(Rc::new(g20));
    if layer == 21 { return g21.get_map(a); }
    let mut g22 = MapRiverInit::new(100, world_seed);
    g22.parent = Some(g17.clone());
    let g22 = Rc::new(g22);
    if layer == 22 { return g22.get_map(a); }
    // TODO: use some special color palette for MapRiverInit?
    //if layer == 23 { return MapMap { parent: Rc::new(g23), f: pretty_biome_map_hills }.get_map(a); }
    let mut g23 = MapZoom::new(1000, world_seed);
    g23.parent = Some(g22.clone());
    if layer == 23 { return MapMap { parent: Rc::new(g23), f: pretty_biome_map_hills }.get_map(a); }
    let mut g24 = MapZoom::new(1001, world_seed);
    g24.parent = Some(Rc::new(g23));
    if layer == 24 { return MapMap { parent: Rc::new(g24), f: pretty_biome_map_hills }.get_map(a); }
    let mut g25 = MapHills::new(1000, world_seed);
    g25.parent1 = Some(Rc::new(g21));
    g25.parent2 = Some(Rc::new(g24));
    if layer == 25 { return g25.get_map(a); }
    let mut g26 = MapRareBiome::new(1001, world_seed);
    g26.parent = Some(Rc::new(g25));
    if layer == 26 { return g26.get_map(a); }
    let mut g27 = MapZoom::new(1000, world_seed);
    g27.parent = Some(Rc::new(g26));
    if layer == 27 { return g27.get_map(a); }
    let mut g28 = MapAddIsland::new(3, world_seed);
    g28.parent = Some(Rc::new(g27));
    if layer == 28 { return g28.get_map(a); }
    let mut g29 = MapZoom::new(1001, world_seed);
    g29.parent = Some(Rc::new(g28));
    if layer == 29 { return g29.get_map(a); }
    let mut g30 = MapShore::new(1000, world_seed);
    g30.parent = Some(Rc::new(g29));
    if layer == 30 { return g30.get_map(a); }
    let mut g31 = MapZoom::new(1002, world_seed);
    g31.parent = Some(Rc::new(g30));
    if layer == 31 { return g31.get_map(a); }
    let mut g32 = MapZoom::new(1003, world_seed);
    g32.parent = Some(Rc::new(g31));
    if layer == 32 { return g32.get_map(a); }
    let mut g33 = MapSmooth::new(1000, world_seed);
    g33.parent = Some(Rc::new(g32));
    if layer == 33 { return g33.get_map(a); }
    let mut g34 = MapZoom::new(1000, world_seed);
    g34.parent = Some(g22.clone());
    if layer == 34 { return MapMap { parent: Rc::new(g34), f: reduce_id }.get_map(a); }
    let mut g35 = MapZoom::new(1001, world_seed);
    g35.parent = Some(Rc::new(g34));
    if layer == 35 { return MapMap { parent: Rc::new(g35), f: reduce_id }.get_map(a); }
    let mut g36 = MapZoom::new(1000, world_seed);
    g36.parent = Some(Rc::new(g35));
    if layer == 36 { return MapMap { parent: Rc::new(g36), f: reduce_id }.get_map(a); }
    let mut g37 = MapZoom::new(1001, world_seed);
    g37.parent = Some(Rc::new(g36));
    if layer == 37 { return MapMap { parent: Rc::new(g37), f: reduce_id }.get_map(a); }
    let mut g38 = MapZoom::new(1002, world_seed);
    g38.parent = Some(Rc::new(g37));
    if layer == 38 { return MapMap { parent: Rc::new(g38), f: reduce_id }.get_map(a); }
    let mut g39 = MapZoom::new(1003, world_seed);
    g39.parent = Some(Rc::new(g38));
    if layer == 39 { return MapMap { parent: Rc::new(g39), f: reduce_id }.get_map(a); }
    let mut g40 = MapRiver::new(1, world_seed);
    g40.parent = Some(Rc::new(g39));
    if layer == 40 { return g40.get_map(a); }
    let mut g41 = MapSmooth::new(1000, world_seed);
    g41.parent = Some(Rc::new(g40));
    if layer == 41 { return g41.get_map(a); }
    let mut g42 = MapRiverMix::new(100, world_seed);
    g42.parent1 = Some(Rc::new(g33));
    g42.parent2 = Some(Rc::new(g41));
    if layer == 42 { return g42.get_map(a); }

    // 1.13 ocean layers
    let g43 = MapOceanTemp::new(2, world_seed);
    if layer == 43 { return g43.get_map(a); }
    let mut g44 = MapZoom::new(2001, world_seed);
    g44.parent = Some(Rc::new(g43));
    if layer == 44 { return g44.get_map(a); }
    let mut g45 = MapZoom::new(2002, world_seed);
    g45.parent = Some(Rc::new(g44));
    if layer == 45 { return g45.get_map(a); }
    let mut g46 = MapZoom::new(2003, world_seed);
    g46.parent = Some(Rc::new(g45));
    if layer == 46 { return g46.get_map(a); }
    let mut g47 = MapZoom::new(2004, world_seed);
    g47.parent = Some(Rc::new(g46));
    if layer == 47 { return g47.get_map(a); }
    let mut g48 = MapZoom::new(2005, world_seed);
    g48.parent = Some(Rc::new(g47));
    if layer == 48 { return g48.get_map(a); }
    let mut g49 = MapZoom::new(2006, world_seed);
    g49.parent = Some(Rc::new(g48));
    if layer == 49 { return g49.get_map(a); }

    let mut g50 = MapOceanMix::new(100, world_seed);
    g50.parent1 = Some(Rc::new(g42)); // MapRiverMix
    g50.parent2 = Some(Rc::new(g49)); // MapZoom <- MapOceanTemp
    if layer == 50 { return g50.get_map(a); }

    let mut g51 = MapVoronoiZoom::new(10, world_seed);
    g51.parent = Some(Rc::new(g50));

    let m1 = g51.get_map(a);
    m1
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let m33 = generate_up_to_layer(MinecraftVersion::Java1_7, area, world_seed, 33);

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
        let m = generate_up_to_layer(MinecraftVersion::Java1_7, area, world_seed, 30);

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
        let m = generate_up_to_layer(MinecraftVersion::Java1_7, area, world_seed, 31);

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
        for z in 0..h {
            for x in 0..w {
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
        let m = generate_up_to_layer(MinecraftVersion::Java1_7, area, world_seed, 32);

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
        for z in 0..h {
            for x in 0..w {
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
        let mut gen = MapZoom::new(base_seed, world_seed);
        let island = MapIsland::new(1, world_seed);
        gen.parent = Some(Rc::new(island));
        gen.fuzzy = true;
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

    // Check that all the layers generate the correct area
    #[test]
    fn preserve_area() {
        let world_seed = 9223090561890311698;
        let base_seed = 2000;
        let parent: Option<Rc<dyn GetMap>> = Some(Rc::new(TestMapZero));
        let g0 = MapIsland::new(base_seed, world_seed);
        let mut g1 = MapZoom::new(base_seed, world_seed);
        g1.parent = parent.clone();
        let mut g2 = MapAddIsland::new(base_seed, world_seed);
        g2.parent = parent.clone();
        let mut g3 = MapVoronoiZoom::new(base_seed, world_seed);
        g3.parent = parent.clone();
        let gv: Vec<&dyn GetMap> = vec![&TestMapZero, &TestMapXhz, &g0, &g1, &g2, &g3];
        let mut av = vec![];
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
        for gen in gv {
            for a in &av {
                let map = gen.get_map(*a);
                assert_eq!(map.a.dim(), (a.w as usize, a.h as usize));
                assert_eq!(map.x, a.x);
                assert_eq!(map.z, a.z);
            }
        }
    }

    #[test]
    fn generate_t() {
        let a = Area { x: 0, z: 0, w: 100, h: 100 };
        generate(MinecraftVersion::Java1_7, a, 1234);
    }

    #[test]
    fn reverse_voronoi_small_map() {
        fn rcoords(c: &[(i64, i64)]) -> Result<Map, ()> {
            let area_voronoi = area_from_coords(&c);
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
        // Helper functions
        // TODO: move to rivers.rs
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
            let target_map = generate_up_to_layer(MinecraftVersion::Java1_7, area, world_seed, 41);
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

        let river_coords_voronoi = vec![
        [268, 211], [268, 210], [267, 211], [266, 211], [268, 209], [266, 209], [267, 210], [267, 209], [266, 210], [265, 211], [264, 211], [263, 211], [261, 211], [262, 210], [262, 211], [263, 210], [264, 210], [265, 210], [261, 210], [261, 209], [262, 209], [263, 209], [264, 209], [265, 209], [264, 212], [267, 208], [267, 207], [267, 206], [267, 205], [261, 205], [262, 205], [263, 205], [264, 205], [265, 205], [266, 205], [266, 206], [264, 206], [265, 206], [262, 206], [263, 206], [261, 206], [261, 207], [261, 208], [262, 207], [262, 208], [263, 207], [263, 208], [264, 207], [264, 208], [265, 207], [265, 208], [266, 207], [266, 208], [266, 204], [266, 203], [266, 202], [265, 202], [264, 202], [263, 202], [262, 202], [261, 202], [261, 203], [261, 204], [262, 203], [262, 204], [263, 203], [263, 204], [264, 203], [264, 204], [265, 203], [265, 204], [267, 201], [266, 201], [265, 201], [264, 201], [263, 201], [262, 201], [261, 201], [267, 200], [266, 200], [265, 200], [266, 199], [267, 199], [268, 199], [269, 199], [270, 199], [271, 199], [272, 199], [266, 198], [268, 198], [267, 198], [269, 198], [270, 198], [271, 198], [272, 198], [273, 198], [273, 197], [272, 197], [271, 197], [270, 197], [269, 197], [268, 197], [267, 197], [266, 197], [265, 197], [264, 196], [265, 196], [266, 196], [267, 196], [268, 196], [269, 196], [270, 196], [271, 196], [272, 196], [273, 196], [263, 195], [264, 194], [264, 195], [265, 194], [265, 195], [266, 194], [266, 195], [267, 194], [267, 195], [268, 194], [268, 195], [269, 194], [269, 195], [270, 194], [270, 195], [271, 194], [271, 195], [272, 194], [272, 195], [273, 194], [273, 195], [273, 191], [264, 191], [264, 192], [264, 193], [265, 191], [265, 192], [265, 193], [266, 193], [267, 193], [268, 193], [269, 193], [270, 193], [271, 193], [272, 193], [273, 193], [273, 192], [272, 192], [271, 192], [270, 192], [269, 192], [268, 192], [267, 192], [266, 192], [266, 191], [267, 191], [268, 191], [269, 191], [272, 191], [271, 191], [270, 191], [271, 190], [264, 190], [265, 190], [266, 190], [267, 190], [268, 190], [269, 190], [270, 190], [269, 189], [270, 189], [271, 189], [264, 189], [265, 189], [270, 188], [269, 188], [271, 188], [273, 188], [274, 188], [275, 188], [276, 188], [269, 187], [270, 187], [271, 187], [272, 187], [273, 187], [274, 187], [275, 187], [276, 187], [268, 186], [269, 186], [270, 186], [271, 186], [272, 186], [273, 186], [274, 186], [275, 186], [276, 186], [277, 185], [267, 185], [269, 185], [268, 185], [270, 185], [271, 185], [272, 185], [273, 185], [274, 185], [275, 185], [276, 185], [277, 184], [276, 184], [275, 184], [274, 184], [273, 184], [272, 184], [271, 184], [270, 184], [269, 184], [268, 184], [267, 184], [268, 183], [268, 182], [277, 182], [277, 183], [276, 182], [276, 183], [275, 182], [275, 183], [274, 182], [274, 183], [273, 182], [273, 183], [272, 182], [272, 183], [271, 182], [271, 183], [270, 182], [270, 183], [269, 182], [269, 183], [270, 181], [269, 181], [268, 181], [273, 181], [274, 181], [275, 181], [276, 181], [277, 181], [269, 180], [273, 180], [274, 180], [275, 180], [276, 180], [277, 180], [273, 177], [273, 178], [273, 179], [274, 177], [274, 178], [274, 179], [275, 177], [275, 178], [275, 179], [276, 177], [276, 178], [276, 179], [277, 177], [277, 178], [277, 179], [276, 176], [277, 176], [277, 175], [276, 175], [277, 174], [278, 173], [278, 174], [278, 175], [278, 176], [278, 177], [278, 178], [278, 179], [279, 180], [279, 173], [279, 174], [279, 175], [279, 177], [279, 176], [279, 178], [279, 179], [280, 172], [281, 173], [282, 173], [280, 173], [280, 180], [281, 180], [282, 180], [282, 179], [280, 179], [281, 179], [281, 178], [280, 178], [282, 178], [282, 177], [281, 177], [280, 177], [280, 176], [281, 176], [282, 176], [282, 175], [281, 175], [280, 175], [280, 174], [282, 174], [281, 174], [283, 173], [284, 172], [283, 180], [284, 180], [284, 179], [283, 179], [283, 178], [283, 177], [284, 178], [284, 177], [284, 176], [283, 176], [283, 175], [283, 174], [284, 173], [284, 175], [284, 174], [285, 180], [286, 180], [287, 180], [288, 180], [289, 180], [290, 172], [289, 172], [288, 172], [287, 172], [286, 172], [285, 172], [290, 179], [290, 178], [290, 177], [290, 176], [290, 175], [290, 174], [290, 173], [289, 173], [288, 173], [287, 173], [286, 173], [285, 173], [285, 174], [286, 174], [287, 174], [288, 174], [289, 174], [289, 175], [288, 175], [287, 175], [286, 175], [285, 175], [285, 176], [286, 176], [287, 176], [288, 176], [289, 176], [289, 177], [288, 177], [287, 177], [286, 177], [285, 177], [285, 178], [285, 179], [286, 178], [286, 179], [287, 178], [287, 179], [288, 178], [288, 179], [289, 178], [289, 179], [291, 179], [292, 178], [292, 172], [291, 173], [292, 173], [292, 174], [292, 175], [292, 176], [292, 177], [291, 178], [291, 177], [291, 176], [291, 175], [291, 174], [293, 171], [293, 178], [294, 178], [294, 171], [295, 172], [296, 172], [297, 172], [295, 179], [296, 179], [297, 179], [298, 173], [299, 174], [298, 179], [299, 180], [299, 175], [299, 176], [299, 178], [299, 177], [299, 179], [298, 178], [298, 177], [298, 176], [298, 175], [298, 174], [293, 172], [294, 172], [293, 173], [294, 173], [295, 173], [296, 173], [297, 173], [297, 175], [297, 174], [296, 174], [296, 175], [295, 174], [293, 174], [294, 174], [294, 175], [294, 176], [293, 176], [293, 177], [293, 175], [294, 177], [295, 176], [295, 175], [296, 176], [297, 176], [297, 177], [296, 177], [295, 177], [295, 178], [296, 178], [297, 178], [300, 180], [300, 174], [300, 175], [300, 176], [300, 177], [300, 178], [300, 179], [301, 179], [301, 177], [301, 178], [301, 176], [301, 175], [301, 174], [302, 172], [302, 173], [303, 172], [303, 173], [302, 179], [303, 179], [304, 179], [302, 174], [303, 174], [304, 171], [304, 172], [304, 173], [304, 174], [302, 176], [302, 175], [303, 175], [304, 175], [304, 176], [303, 176], [302, 177], [303, 177], [303, 178], [302, 178], [304, 177], [304, 178], [304, 170], [304, 169], [304, 168], [305, 168], [306, 168], [307, 168], [305, 178], [306, 178], [307, 178], [308, 176], [307, 176], [307, 177], [309, 175], [308, 167], [308, 166], [308, 165], [308, 168], [308, 164], [307, 164], [307, 165], [309, 163], [309, 164], [309, 165], [310, 165], [312, 165], [311, 166], [312, 166], [312, 167], [312, 171], [312, 172], [312, 173], [311, 175], [311, 174], [310, 175], [310, 174], [309, 174], [311, 173], [308, 175], [311, 167], [311, 168], [311, 169], [311, 170], [311, 171], [310, 166], [309, 166], [310, 167], [309, 167], [305, 169], [305, 170], [305, 171], [306, 169], [310, 168], [309, 168], [309, 169], [311, 172], [310, 169], [305, 172], [305, 173], [305, 174], [305, 175], [305, 176], [305, 177], [306, 177], [306, 176], [306, 175], [306, 174], [306, 173], [306, 172], [306, 170], [306, 171], [307, 169], [308, 169], [308, 170], [307, 170], [307, 171], [308, 171], [310, 170], [309, 170], [310, 171], [309, 171], [310, 172], [309, 172], [308, 172], [307, 172], [307, 173], [308, 173], [308, 174], [307, 174], [307, 175], [309, 173], [310, 173], [312, 160], [312, 159], [313, 159], [313, 160], [313, 165], [313, 164], [313, 163], [313, 162], [313, 161], [313, 167], [314, 167], [315, 167], [313, 166], [314, 166], [315, 166], [316, 166], [316, 165], [316, 164], [317, 163], [316, 163], [315, 163], [314, 163], [314, 164], [315, 164], [315, 165], [314, 165], [318, 162], [319, 162], [320, 161], [320, 160], [316, 159], [317, 159], [318, 159], [319, 159], [314, 160], [315, 160], [316, 160], [317, 160], [318, 160], [319, 160], [319, 161], [317, 161], [318, 161], [317, 162], [316, 161], [315, 161], [314, 161], [314, 162], [315, 162], [316, 162], [320, 159], [316, 158], [316, 157], [316, 156], [316, 153], [316, 154], [317, 155], [317, 153], [317, 154], [317, 156], [317, 157], [317, 158], [318, 153], [318, 154], [319, 154], [320, 154], [321, 154], [322, 154], [322, 153], [323, 153], [324, 153], [324, 152], [324, 151], [325, 152], [326, 152], [327, 152], [327, 153], [327, 154], [328, 153], [328, 154], [327, 155], [327, 156], [327, 157], [328, 157], [328, 158], [327, 158], [327, 159], [328, 159], [328, 160], [328, 161], [328, 162], [327, 161], [326, 160], [327, 160], [325, 160], [321, 159], [322, 159], [323, 159], [324, 159], [325, 159], [326, 159], [318, 158], [318, 157], [318, 156], [318, 155], [319, 155], [320, 155], [321, 155], [322, 155], [323, 154], [324, 154], [325, 153], [326, 153], [326, 154], [325, 154], [325, 155], [326, 155], [326, 156], [325, 156], [325, 157], [326, 158], [326, 157], [325, 158], [324, 157], [324, 158], [324, 156], [324, 155], [323, 155], [323, 156], [322, 156], [321, 156], [320, 156], [319, 156], [319, 157], [320, 157], [320, 158], [319, 158], [322, 158], [321, 157], [321, 158], [322, 157], [323, 158], [323, 157], [329, 158], [329, 159], [329, 160], [329, 161], [329, 162], [329, 163], [330, 163], [331, 163], [332, 164], [333, 164], [334, 164], [335, 164], [332, 163], [333, 163], [334, 163], [335, 163], [336, 163], [335, 162], [334, 161], [333, 160], [334, 159], [335, 158], [336, 159], [337, 160], [338, 160], [339, 160], [340, 159], [336, 158], [334, 157], [334, 158], [337, 158], [337, 159], [338, 156], [339, 156], [340, 156], [340, 157], [339, 157], [338, 157], [338, 158], [339, 158], [340, 158], [339, 159], [338, 159], [333, 157], [332, 157], [331, 157], [330, 157], [333, 156], [330, 158], [331, 158], [333, 158], [332, 158], [333, 159], [332, 159], [330, 159], [331, 159], [330, 160], [331, 160], [332, 160], [330, 161], [330, 162], [331, 161], [331, 162], [332, 161], [332, 162], [333, 161], [333, 162], [334, 162], [338, 152], [338, 153], [338, 154], [338, 155], [337, 155], [337, 154], [336, 154], [335, 154], [334, 152], [334, 153], [335, 152], [335, 153], [336, 152], [336, 153], [337, 152], [337, 153], [339, 155], [340, 155], [340, 154], [339, 154], [339, 153], [340, 153], [339, 152], [339, 151], [339, 150], [340, 150], [340, 151], [340, 152], [340, 149], [341, 149], [342, 149], [343, 149], [343, 148], [344, 148], [345, 148], [345, 147], [341, 159], [341, 160], [342, 160], [343, 160], [343, 159], [344, 158], [344, 159], [345, 157], [344, 157], [346, 156], [345, 156], [346, 155], [347, 155], [347, 153], [346, 152], [346, 153], [347, 154], [346, 154], [346, 151], [346, 150], [346, 149], [346, 148], [346, 147], [345, 146], [345, 145], [345, 144], [346, 144], [346, 145], [346, 146], [347, 144], [347, 145], [347, 146], [347, 147], [347, 148], [345, 149], [344, 149], [341, 150], [342, 150], [343, 150], [344, 150], [345, 150], [345, 151], [344, 151], [343, 151], [342, 151], [341, 151], [341, 152], [342, 152], [343, 152], [344, 152], [345, 152], [345, 153], [344, 153], [343, 153], [342, 153], [341, 154], [341, 153], [342, 154], [344, 154], [343, 154], [345, 154], [345, 155], [344, 155], [343, 155], [342, 155], [341, 155], [341, 156], [342, 156], [343, 156], [344, 156], [343, 157], [342, 157], [341, 157], [341, 158], [342, 158], [342, 159], [343, 158], [348, 147], [349, 147], [350, 148], [351, 149], [352, 149], [353, 149], [354, 148], [355, 147], [353, 147], [354, 147], [353, 148], [354, 146], [355, 144], [354, 144], [354, 145], [348, 143], [348, 142], [348, 141], [348, 140], [349, 140], [349, 141], [349, 142], [349, 143], [348, 144], [349, 144], [348, 145], [348, 146], [349, 146], [352, 141], [351, 141], [350, 141], [353, 141], [353, 142], [353, 143], [352, 142], [351, 142], [350, 142], [351, 143], [350, 143], [350, 144], [352, 143], [353, 144], [352, 144], [351, 144], [351, 145], [349, 145], [350, 145], [350, 146], [352, 145], [353, 145], [353, 146], [352, 146], [351, 146], [350, 147], [351, 147], [352, 147], [352, 148], [351, 148], [354, 140], [355, 140], [355, 141], [354, 141], [354, 142], [355, 142], [355, 143], [354, 143], [356, 143], [357, 143], [358, 143], [360, 143], [359, 143], [360, 139], [360, 140], [360, 141], [360, 142], [353, 139], [354, 139], [356, 139], [355, 139], [357, 139], [358, 139], [359, 139], [359, 140], [358, 140], [357, 140], [356, 140], [356, 141], [357, 141], [358, 141], [359, 141], [359, 142], [357, 142], [358, 142], [356, 142], [353, 138], [353, 137], [353, 136], [352, 136], [359, 138], [359, 137], [359, 136], [359, 135], [360, 135], [355, 136], [354, 136], [356, 135], [356, 136], [354, 137], [354, 138], [355, 137], [355, 138], [356, 137], [356, 138], [357, 137], [357, 138], [358, 138], [358, 137], [358, 136], [357, 136], [357, 135], [358, 135], [356, 133], [356, 134], [360, 133], [360, 134], [359, 134], [359, 133], [358, 133], [357, 133], [357, 134], [358, 134], [361, 134], [362, 134], [363, 134], [364, 134], [365, 133], [364, 133], [363, 133], [362, 133], [361, 133], [361, 132], [365, 132], [364, 132], [363, 132], [362, 132], [366, 131], [367, 131], [368, 131], [369, 131], [369, 132], [368, 132], [361, 131], [362, 131], [363, 131], [364, 131], [365, 131], [360, 130], [361, 130], [362, 130], [363, 130], [364, 130], [365, 130], [366, 130], [367, 130], [368, 130], [369, 130], [360, 129], [359, 128], [369, 128], [369, 129], [368, 128], [367, 128], [366, 128], [365, 128], [364, 128], [363, 128], [362, 128], [360, 128], [361, 128], [361, 129], [362, 129], [363, 129], [364, 129], [365, 129], [366, 129], [367, 129], [368, 129], [360, 127], [361, 126], [361, 125], [362, 125], [363, 125], [363, 124], [363, 123], [361, 127], [362, 126], [362, 127], [363, 126], [363, 127], [364, 124], [365, 124], [366, 124], [367, 123], [366, 123], [364, 123], [365, 123], [366, 125], [368, 127], [367, 126], [367, 127], [366, 126], [365, 125], [364, 125], [364, 126], [365, 126], [364, 127], [365, 127], [366, 127], [367, 122], [368, 122], [369, 122], [370, 122], [364, 122], [365, 122], [366, 122], [364, 121], [364, 120], [365, 119], [365, 120], [365, 121], [365, 118], [365, 117], [365, 116], [366, 116], [366, 117], [366, 118], [366, 119], [366, 120], [366, 121], [371, 121], [371, 120], [370, 120], [370, 121], [371, 119], [372, 119], [373, 119], [374, 119], [374, 120], [375, 121], [376, 121], [377, 120], [376, 120], [375, 120], [375, 119], [376, 119], [376, 118], [375, 118], [375, 117], [376, 116], [377, 116], [378, 116], [379, 116], [380, 116], [375, 116], [367, 116], [368, 116], [369, 116], [370, 116], [371, 116], [372, 116], [373, 116], [374, 116], [374, 118], [373, 117], [374, 117], [373, 118], [372, 117], [372, 118], [371, 117], [370, 117], [368, 117], [367, 117], [369, 117], [371, 118], [370, 118], [370, 119], [369, 118], [368, 118], [367, 118], [368, 119], [369, 119], [369, 120], [368, 120], [367, 119], [367, 121], [367, 120], [368, 121], [369, 121], [367, 115], [368, 115], [369, 114], [369, 113], [369, 115], [370, 113], [370, 114], [370, 115], [372, 113], [371, 113], [371, 114], [372, 114], [372, 115], [371, 115], [373, 115], [373, 114], [374, 114], [375, 114], [376, 113], [376, 114], [376, 115], [375, 115], [374, 115], [377, 113], [378, 113], [379, 113], [379, 114], [379, 115], [378, 114], [377, 114], [377, 115], [378, 115], [380, 115], [376, 112], [377, 112], [378, 112], [379, 112], [380, 112], [381, 112], [376, 111], [375, 110], [375, 109], [375, 108], [376, 107], [377, 106], [377, 105], [378, 104], [377, 103], [376, 102], [376, 101], [375, 100], [374, 100], [373, 100], [373, 99], [373, 98], [372, 97], [372, 96], [372, 95], [373, 94], [372, 93], [371, 92], [370, 91], [365, 91], [366, 91], [367, 91], [368, 91], [369, 91], [364, 90], [365, 90], [366, 90], [367, 90], [368, 90], [369, 90], [374, 90], [373, 90], [372, 90], [371, 90], [370, 90], [375, 91], [374, 91], [373, 91], [372, 91], [371, 91], [376, 92], [377, 92], [378, 92], [379, 92], [380, 93], [380, 94], [380, 95], [380, 96], [380, 99], [380, 100], [381, 100], [379, 96], [379, 97], [379, 98], [379, 99], [381, 101], [382, 101], [382, 111], [382, 110], [382, 109], [382, 108], [382, 107], [382, 106], [382, 105], [382, 104], [382, 103], [382, 102], [383, 104], [383, 111], [384, 112], [384, 111], [383, 110], [383, 109], [383, 108], [383, 107], [383, 106], [383, 105], [384, 105], [384, 106], [384, 107], [384, 108], [384, 109], [384, 110], [386, 105], [385, 105], [386, 112], [385, 112], [386, 111], [386, 110], [386, 109], [386, 108], [386, 107], [386, 106], [385, 106], [385, 107], [385, 108], [385, 109], [385, 110], [385, 111], [391, 112], [390, 112], [389, 112], [388, 112], [387, 112], [391, 105], [391, 106], [392, 105], [391, 107], [391, 108], [391, 109], [391, 110], [391, 111], [394, 107], [394, 100], [394, 101], [394, 102], [394, 103], [394, 104], [394, 105], [394, 106], [393, 100], [392, 101], [392, 107], [393, 107], [393, 106], [392, 106], [393, 105], [392, 104], [392, 102], [393, 101], [393, 102], [392, 103], [393, 103], [393, 104], [390, 104], [391, 104], [389, 104], [387, 104], [387, 105], [388, 105], [390, 105], [389, 105], [387, 106], [388, 106], [389, 106], [390, 106], [390, 107], [389, 107], [387, 107], [388, 107], [387, 108], [388, 108], [389, 108], [390, 108], [390, 109], [389, 109], [388, 109], [387, 109], [388, 110], [389, 110], [390, 110], [390, 111], [389, 111], [387, 110], [387, 111], [388, 111], [372, 92], [373, 92], [374, 92], [375, 92], [373, 93], [374, 93], [375, 93], [376, 93], [377, 93], [378, 93], [379, 93], [379, 94], [378, 94], [377, 94], [376, 94], [375, 94], [374, 94], [373, 95], [374, 95], [373, 96], [373, 97], [374, 96], [374, 97], [374, 98], [374, 99], [375, 99], [376, 100], [379, 95], [378, 95], [377, 95], [376, 95], [375, 95], [375, 96], [376, 96], [377, 96], [378, 96], [378, 97], [377, 97], [376, 97], [375, 97], [375, 98], [376, 98], [377, 98], [378, 98], [376, 99], [377, 99], [378, 99], [377, 100], [378, 100], [379, 100], [377, 101], [378, 101], [377, 102], [378, 102], [379, 101], [380, 101], [381, 102], [378, 103], [379, 102], [380, 102], [381, 103], [376, 110], [377, 109], [379, 107], [376, 108], [378, 106], [376, 109], [378, 105], [381, 111], [380, 110], [380, 111], [381, 104], [380, 104], [380, 103], [379, 103], [379, 104], [379, 105], [379, 106], [380, 105], [381, 105], [377, 110], [377, 111], [378, 110], [379, 110], [379, 111], [378, 111], [377, 108], [377, 107], [378, 107], [378, 108], [379, 108], [380, 106], [381, 106], [381, 107], [380, 107], [380, 108], [381, 108], [381, 109], [381, 110], [380, 109], [379, 109], [378, 109]
        ];
        let river_coords_voronoi = river_coords_voronoi.into_iter().map(|x| (x[0], x[1])).collect::<Vec<_>>();
        let area_voronoi = area_from_coords(&river_coords_voronoi);
        let target_map_voronoi = map_with_river_at(&river_coords_voronoi, area_voronoi);
        let target_map_derived = reverse_map_voronoi_zoom(&target_map_voronoi).unwrap();
        let target_map = target_map_derived;
        println!("{}", draw_map(&target_map));

        let river_coords_rv_expected = vec![
            [65, 51], [66, 51], [65, 50], [66, 48], [67, 48], [67, 47], [66, 50], [66, 49], [67, 49], [66, 47], [67, 45], [68, 45], [68, 46], [67, 46], [68, 44], [69, 43], [69, 44], [70, 44], [70, 43], [71, 43], [71, 44], [72, 43], [72, 44], [73, 43], [73, 44], [74, 43], [74, 44], [75, 43], [76, 43], [75, 44], [77, 43], [76, 44], [76, 42], [77, 42], [77, 41], [78, 41], [78, 40], [79, 40], [79, 39], [79, 38], [80, 38], [80, 39], [81, 39], [81, 38], [82, 39], [83, 39], [83, 40], [82, 40], [84, 39], [84, 38], [85, 38], [85, 39], [85, 37], [86, 37], [86, 38], [86, 36], [87, 36], [88, 36], [87, 35], [88, 35], [89, 35], [88, 34], [89, 34], [89, 33], [90, 33], [90, 31], [91, 31], [91, 32], [90, 32], [91, 29], [91, 30], [92, 29], [92, 30], [93, 29], [92, 28], [93, 28], [94, 28], [94, 27], [95, 27], [96, 27], [97, 27], [97, 26], [96, 26], [95, 26], [94, 26], [94, 25], [95, 25], [93, 23], [93, 24], [94, 23], [94, 24], [93, 22], [92, 22], [91, 22], [65, 52], [66, 52], [98, 26], [98, 25]
        ];
        let river_coords_rv_expected  = river_coords_rv_expected.into_iter().map(|x| (x[0], x[1])).collect::<Vec<_>>();
        let area_rv = area_from_coords(&river_coords_rv_expected);
        let expected_rv_map = map_with_river_at(&river_coords_rv_expected, area_rv);
        println!("{}", draw_map(&expected_rv_map));
        assert_eq!(target_map, expected_rv_map);
    }
}

#[allow(non_upper_case_globals)]
pub mod biome_id {
pub type BiomeID = i32;
pub const bambooJungleHills: BiomeID = 169;
pub const bambooJungle: BiomeID = 168;
pub const BIOME_NUM: BiomeID = 51;
pub const frozenDeepOcean: BiomeID = 50;
// 40-49
pub const coldDeepOcean: BiomeID = 49;
pub const lukewarmDeepOcean: BiomeID = 48;
pub const warmDeepOcean: BiomeID = 47;
pub const coldOcean: BiomeID = 46;
pub const lukewarmOcean: BiomeID = 45;
pub const warmOcean: BiomeID = 44;
pub const skyIslandBarren: BiomeID = 43;
pub const skyIslandHigh: BiomeID = 42;
pub const skyIslandMedium: BiomeID = 41;
// 1.13
pub const skyIslandLow: BiomeID = 40;
// 30-39
pub const mesaPlateau: BiomeID = 39;
pub const mesaPlateau_F: BiomeID = 38;
pub const mesa: BiomeID = 37;
pub const savannaPlateau: BiomeID = 36;
pub const savanna: BiomeID = 35;
pub const extremeHillsPlus: BiomeID = 34;
pub const megaTaigaHills: BiomeID = 33;
pub const megaTaiga: BiomeID = 32;
pub const coldTaigaHills: BiomeID = 31;
pub const coldTaiga: BiomeID = 30;
// 20-29
pub const roofedForest: BiomeID = 29;
pub const birchForestHills: BiomeID = 28;
pub const birchForest: BiomeID = 27;
pub const coldBeach: BiomeID = 26;
pub const stoneBeach: BiomeID = 25;
pub const deepOcean: BiomeID = 24;
pub const jungleEdge: BiomeID = 23;
pub const jungleHills: BiomeID = 22;
pub const jungle: BiomeID = 21;
pub const extremeHillsEdge: BiomeID = 20;
// 10-19
pub const taigaHills: BiomeID = 19;
pub const forestHills: BiomeID = 18;
pub const desertHills: BiomeID = 17;
pub const beach: BiomeID = 16;
pub const mushroomIslandShore: BiomeID = 15;
pub const mushroomIsland: BiomeID = 14;
pub const iceMountains: BiomeID = 13;
pub const icePlains: BiomeID = 12;
pub const frozenRiver: BiomeID = 11;
pub const frozenOcean: BiomeID = 10;
// 0-9
pub const sky: BiomeID = 9;
pub const hell: BiomeID = 8;
pub const river: BiomeID = 7;
pub const swampland: BiomeID = 6;
pub const taiga: BiomeID = 5;
pub const forest: BiomeID = 4;
pub const extremeHills: BiomeID = 3;
pub const desert: BiomeID = 2;
pub const plains: BiomeID = 1;
pub const ocean: BiomeID = 0;
pub const none: BiomeID = -1;
pub type BiomeType = i32;
pub const BTYPE_NUM: BiomeType = 17;
pub const Mesa: BiomeType = 16;
pub const Savanna: BiomeType = 15;
pub const StoneBeach: BiomeType = 14;
pub const Jungle: BiomeType = 13;
pub const Beach: BiomeType = 12;
pub const MushroomIsland: BiomeType = 11;
pub const Snow: BiomeType = 10;
pub const Sky: BiomeType = 9;
pub const Hell: BiomeType = 8;
pub const River: BiomeType = 7;
pub const Swamp: BiomeType = 6;
pub const Taiga: BiomeType = 5;
pub const Forest: BiomeType = 4;
pub const Hills: BiomeType = 3;
pub const Desert: BiomeType = 2;
pub const Plains: BiomeType = 1;
pub const Ocean: BiomeType = 0;
pub type BiomeTempCategory = i32;
pub const Unknown: BiomeTempCategory = 5;
pub const Freezing: BiomeTempCategory = 4;
pub const Cold: BiomeTempCategory = 3;
pub const Lush: BiomeTempCategory = 2;
pub const Warm: BiomeTempCategory = 1;
pub const Oceanic: BiomeTempCategory = 0;
}

// TODO: I changed 252 to pure green to help with debugging
pub const UNKNOWN_BIOME_ID: i32 = 252;
pub static BIOME_COLORS: [[u8; 3]; 256] =
[[0, 0, 112], [141, 179, 96], [250, 148, 24], [96, 96, 96], [5, 102, 33], [11, 102, 89], [7, 249, 178], [0, 0, 255], [255, 0, 0], [128, 128, 255], [112, 112, 214], [160, 160, 255], [255, 255, 255], [160, 160, 160], [255, 0, 255], [160, 0, 255], [250, 222, 85], [210, 95, 18], [34, 85, 28], [22, 57, 51], [114, 120, 154], [83, 123, 9], [44, 66, 5], [98, 139, 23], [0, 0, 48], [162, 162, 132], [250, 240, 192], [48, 116, 68], [31, 95, 50], [64, 81, 26], [49, 85, 74], [36, 63, 54], [89, 102, 81], [69, 79, 62], [80, 112, 80], [189, 178, 95], [167, 157, 100], [217, 69, 21], [176, 151, 101], [202, 140, 101], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 172], [0, 0, 144], [32, 32, 112], [0, 0, 80], [0, 0, 64], [32, 32, 56], [64, 64, 144], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 112], [141, 179, 96], [250, 148, 24], [96, 96, 96], [5, 102, 33], [11, 102, 89], [7, 249, 178], [0, 0, 255], [255, 0, 0], [128, 128, 255], [144, 144, 160], [160, 160, 255], [140, 180, 180], [160, 160, 160], [255, 0, 255], [160, 0, 255], [250, 222, 85], [210, 95, 18], [34, 85, 28], [22, 57, 51], [114, 120, 154], [83, 123, 9], [44, 66, 5], [98, 139, 23], [0, 0, 48], [162, 162, 132], [250, 240, 192], [48, 116, 68], [31, 95, 50], [64, 81, 26], [49, 85, 74], [36, 63, 54], [89, 102, 81], [69, 79, 62], [80, 112, 80], [189, 178, 95], [167, 157, 100], [217, 69, 21], [176, 151, 101], [202, 140, 101], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 255, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]]
;

pub static BIOME_INFO: [Biome; 256] = 
[Biome { id: 0, type_0: 0, height: -1.0, temp: 0.5, tempCat: 0 }, Biome { id: 1, type_0: 1, height: 0.10000000149011612, temp: 0.800000011920929, tempCat: 2 }, Biome { id: 2, type_0: 2, height: 0.125, temp: 2.0, tempCat: 1 }, Biome { id: 3, type_0: 3, height: 1.0, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 4, type_0: 4, height: 0.10000000149011612, temp: 0.699999988079071, tempCat: 2 }, Biome { id: 5, type_0: 5, height: 0.20000000298023224, temp: 0.25, tempCat: 2 }, Biome { id: 6, type_0: 6, height: -0.20000000298023224, temp: 0.800000011920929, tempCat: 2 }, Biome { id: 7, type_0: 7, height: -0.5, temp: 0.5, tempCat: 2 }, Biome { id: 8, type_0: 8, height: 0.10000000149011612, temp: 2.0, tempCat: 1 }, Biome { id: 9, type_0: 9, height: 0.10000000149011612, temp: 0.5, tempCat: 2 }, Biome { id: 10, type_0: 0, height: -1.0, temp: 0.0, tempCat: 0 }, Biome { id: 11, type_0: 7, height: -0.5, temp: 0.0, tempCat: 3 }, Biome { id: 12, type_0: 10, height: 0.125, temp: 0.0, tempCat: 3 }, Biome { id: 13, type_0: 10, height: 0.44999998807907104, temp: 0.0, tempCat: 3 }, Biome { id: 14, type_0: 11, height: 0.20000000298023224, temp: 0.8999999761581421, tempCat: 2 }, Biome { id: 15, type_0: 11, height: 0.0, temp: 0.8999999761581421, tempCat: 2 }, Biome { id: 16, type_0: 12, height: 0.0, temp: 0.800000011920929, tempCat: 2 }, Biome { id: 17, type_0: 2, height: 0.44999998807907104, temp: 2.0, tempCat: 1 }, Biome { id: 18, type_0: 4, height: 0.44999998807907104, temp: 0.699999988079071, tempCat: 2 }, Biome { id: 19, type_0: 5, height: 0.44999998807907104, temp: 0.25, tempCat: 2 }, Biome { id: 20, type_0: 3, height: 1.0, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 21, type_0: 13, height: 0.10000000149011612, temp: 0.949999988079071, tempCat: 2 }, Biome { id: 22, type_0: 13, height: 0.44999998807907104, temp: 0.949999988079071, tempCat: 2 }, Biome { id: 23, type_0: 13, height: 0.10000000149011612, temp: 0.949999988079071, tempCat: 2 }, Biome { id: 24, type_0: 0, height: -1.7999999523162842, temp: 0.5, tempCat: 0 }, Biome { id: 25, type_0: 14, height: 0.10000000149011612, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 26, type_0: 12, height: 0.0, temp: 0.05000000074505806, tempCat: 3 }, Biome { id: 27, type_0: 4, height: 0.10000000149011612, temp: 0.6000000238418579, tempCat: 2 }, Biome { id: 28, type_0: 4, height: 0.44999998807907104, temp: 0.6000000238418579, tempCat: 2 }, Biome { id: 29, type_0: 4, height: 0.10000000149011612, temp: 0.699999988079071, tempCat: 2 }, Biome { id: 30, type_0: 5, height: 0.20000000298023224, temp: -0.5, tempCat: 3 }, Biome { id: 31, type_0: 5, height: 0.44999998807907104, temp: -0.5, tempCat: 3 }, Biome { id: 32, type_0: 5, height: 0.20000000298023224, temp: 0.30000001192092896, tempCat: 2 }, Biome { id: 33, type_0: 5, height: 0.44999998807907104, temp: 0.30000001192092896, tempCat: 2 }, Biome { id: 34, type_0: 3, height: 1.0, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 35, type_0: 15, height: 0.125, temp: 1.2000000476837158, tempCat: 1 }, Biome { id: 36, type_0: 15, height: 1.5, temp: 1.0, tempCat: 1 }, Biome { id: 37, type_0: 16, height: 0.10000000149011612, temp: 2.0, tempCat: 1 }, Biome { id: 38, type_0: 16, height: 1.5, temp: 2.0, tempCat: 1 }, Biome { id: 39, type_0: 16, height: 1.5, temp: 2.0, tempCat: 1 }, Biome { id: 40, type_0: 9, height: 0.0, temp: 0.0, tempCat: 2 }, Biome { id: 41, type_0: 9, height: 0.0, temp: 0.0, tempCat: 2 }, Biome { id: 42, type_0: 9, height: 0.0, temp: 0.0, tempCat: 2 }, Biome { id: 43, type_0: 9, height: 0.0, temp: 0.0, tempCat: 2 }, Biome { id: 44, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 45, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 46, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 47, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 48, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 49, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 50, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 129, type_0: 1, height: 0.10000000149011612, temp: 0.800000011920929, tempCat: 2 }, Biome { id: 130, type_0: 2, height: 0.125, temp: 2.0, tempCat: 1 }, Biome { id: 131, type_0: 3, height: 1.0, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 132, type_0: 4, height: 0.10000000149011612, temp: 0.699999988079071, tempCat: 2 }, Biome { id: 133, type_0: 5, height: 0.20000000298023224, temp: 0.25, tempCat: 2 }, Biome { id: 134, type_0: 6, height: -0.20000000298023224, temp: 0.800000011920929, tempCat: 2 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 140, type_0: 10, height: 0.125, temp: 0.0, tempCat: 3 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 149, type_0: 13, height: 0.10000000149011612, temp: 0.949999988079071, tempCat: 2 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 151, type_0: 13, height: 0.10000000149011612, temp: 0.949999988079071, tempCat: 2 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 155, type_0: 4, height: 0.10000000149011612, temp: 0.6000000238418579, tempCat: 2 }, Biome { id: 156, type_0: 4, height: 0.44999998807907104, temp: 0.6000000238418579, tempCat: 2 }, Biome { id: 157, type_0: 4, height: 0.10000000149011612, temp: 0.699999988079071, tempCat: 2 }, Biome { id: 158, type_0: 5, height: 0.20000000298023224, temp: -0.5, tempCat: 3 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 160, type_0: 5, height: 0.20000000298023224, temp: 0.30000001192092896, tempCat: 2 }, Biome { id: 161, type_0: 5, height: 0.44999998807907104, temp: 0.30000001192092896, tempCat: 2 }, Biome { id: 162, type_0: 3, height: 1.0, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 163, type_0: 15, height: 0.125, temp: 1.2000000476837158, tempCat: 1 }, Biome { id: 164, type_0: 15, height: 1.5, temp: 1.0, tempCat: 1 }, Biome { id: 165, type_0: 16, height: 0.10000000149011612, temp: 2.0, tempCat: 1 }, Biome { id: 166, type_0: 16, height: 1.5, temp: 2.0, tempCat: 1 }, Biome { id: 167, type_0: 16, height: 1.5, temp: 2.0, tempCat: 1 }, Biome { id: 168, type_0: 13, height: 0.10000000149011612, temp: 0.949999988079071, tempCat: 2 }, Biome { id: 169, type_0: 13, height: 0.44999998807907104, temp: 0.949999988079071, tempCat: 2 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }]
;
