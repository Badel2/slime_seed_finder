use mc_rng::McRng;
// TODO: Array2[(x, z)] is a nice syntax, but the fastest dimension to iterate
// is the z dimension, but in the Java code it is the x dimension, as the arrays
// are defined as (z * w + x).
use ndarray::Array2;
use std::rc::Rc;

// The different Map* layers are copied from
// https://github.com/Cubitect/cubiomes
// since it's easier to translate C to Rust than Java to Rust.

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct Area {
    pub x: i64,
    pub z: i64,
    pub w: u64,
    pub h: u64,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
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

pub struct TestMapXhz;

impl GetMap for TestMapXhz {
    fn get_map(&self, area: Area) -> Map {
        let mut m = Map::new(area);
        for z in 0..area.h {
            for x in 0..area.w {
                // TODO: Wrapping arithmetic here
                m.a[(x as usize, z as usize)] = ((area.x + x as i64) * area.h as i64 + (area.z + z as i64)) as i32;
            }
        }

        m
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        let area = pmap.area();

        self.get_map(area)
    }
}

pub struct MapVoronoiZoom {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<GetMap>>,
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
            map.a.slice_inplace(s![
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

pub struct MapZoom {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<GetMap>>,
    pub fuzzy: bool, // true when parent is MapIsland
}

impl MapZoom {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        Self { base_seed, world_seed, parent: None, fuzzy: false }
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
            map.a.slice_inplace(s![
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

pub struct MapAddIsland {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<GetMap>>,
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

            let mut map = self.get_map_from_pmap(&pmap);

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
    pub parent: Option<Rc<GetMap>>,
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

            let mut map = self.get_map_from_pmap(&pmap);

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
    pub parent: Option<Rc<GetMap>>,
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

            let mut map = self.get_map_from_pmap(&pmap);

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
    pub parent: Option<Rc<GetMap>>,
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

            let mut map = self.get_map_from_pmap(&pmap);

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
    pub parent: Option<Rc<GetMap>>,
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

            let mut map = self.get_map_from_pmap(&pmap);

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
    pub parent: Option<Rc<GetMap>>,
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

            let mut map = self.get_map_from_pmap(&pmap);

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
    pub parent: Option<Rc<GetMap>>,
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

            let mut map = self.get_map_from_pmap(&pmap);

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
    pub parent: Option<Rc<GetMap>>,
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

            let mut map = self.get_map_from_pmap(&pmap);

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
    pub parent: Option<Rc<GetMap>>,
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

            let mut map = self.get_map_from_pmap(&pmap);

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
    pub parent: Option<Rc<GetMap>>,
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

            let mut map = self.get_map_from_pmap(&pmap);

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
                let mut v11 = pmap.a[(x+1, z+1)];

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
                                if v10 != jungle && v12 != jungle && v21 != jungle && v01 != jungle {
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
    pub parent: Option<Rc<GetMap>>,
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

            let mut map = self.get_map_from_pmap(&pmap);

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
                m.a[(x, z)] = if v > 0 {
                    let chunk_x = x as i64 + m.x;
                    let chunk_z = z as i64 + m.z;
                    r.set_chunk_seed(chunk_x, chunk_z);
                    r.next_int_n(299999) + 2
                } else {
                    // TODO: is this branch ever executed?
                    0
                };
            }
        }

        m
    }
}

pub struct MapHills {
    base_seed: i64,
    world_seed: i64,
    pub parent1: Option<Rc<GetMap>>,
    pub parent2: Option<Rc<GetMap>>,
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
                        ocean => deepOcean,
                        extremeHills => extremeHillsPlus,
                        savanna => savannaPlateau,
                        _ => if equal_or_plateau(a11, mesaPlateau_F) {
                            mesa
                        } else {
                            if r.next_int_n(2) == 0 { plains } else { forest }
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

            let mut map = self.get_map_from_pmap12(&pmap1, &pmap2);

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
    pub parent: Option<Rc<GetMap>>,
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

            let mut map = self.get_map_from_pmap(&pmap);

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

pub struct MapShore {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<GetMap>>,
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

            let mut map = self.get_map_from_pmap(&pmap);

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
                let mut v11 = pmap.a[(x+1, z+1)];

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
    pub parent: Option<Rc<GetMap>>,
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

            let mut map = self.get_map_from_pmap(&pmap);

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

fn reduce_id(id: i32) -> i32 {
    if id >= 2 {
        2 + (id & 1)
    } else {
        id
    }
}

pub struct MapRiver {
    base_seed: i64,
    world_seed: i64,
    pub parent: Option<Rc<GetMap>>,
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

            let mut map = self.get_map_from_pmap(&pmap);

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

pub struct MapRiverMix {
    base_seed: i64,
    world_seed: i64,
    pub parent1: Option<Rc<GetMap>>,
    pub parent2: Option<Rc<GetMap>>,
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

            let mut map = self.get_map_from_pmap12(&pmap1, &pmap2);

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

pub struct MapSkip {
    zoom_factor: u8,
    pub parent: Option<Rc<GetMap>>,
}

impl MapSkip {
    /// Zoom factor: n in 2^n
    /// 0: same as parent
    /// 1: 2x zoom in each direction
    /// 2: 4x zoom in each direction
    pub fn new(parent: Rc<GetMap>, zoom_factor: u8) -> Self {
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
                    map.a.slice_inplace(s![
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


// Works 99.9 % of the time*
// p = 0.9992 for each tile
// The probability of having at least one error in a 30x30 area is 50%
pub fn reverse_map_voronoi_zoom(buf: &Array2<i32>, _p_x: i64, _p_z: i64, _world_seed: i64) -> Array2<i32> {
    let (w, h) = buf.dim();
    let (p_w, p_h) = (w >> 2, h >> 2);
    let mut pmap = Array2::zeros((p_w, p_h));

    for z in 0..p_h {
        for x in 0..p_w {
            pmap[(x, z)] = buf[(x << 2, z << 2)];
        }
    }

    pmap
}

fn draw_map(map: &Map) -> String {
    let (w, h) = map.a.dim();
    let mut s = String::new();
    s.push_str("TITLE\n");
    for z in 0..h {
        for x in 0..w {
            //let c = if map.a[(x, z)] != 0 { "#" } else { "_" };
            let c = match map.a[(x, z)] {
                0 => "_",
                1 => "#",
                2 => "2",
                3 => "3",
                _ => "?",
            };
            s.push_str(c);
        }
        s.push_str("\n");
    }

    s
}

pub fn generate_image(area: Area, seed: i64) -> Vec<u8> {
    let map = generate(area, seed);
    let (w, h) = map.a.dim();
    let mut v = vec![0; w*h*4];
    for x in 0..w {
        for z in 0..h {
            let color = biome_to_color(map.a[(x, z)]);
            let i = z * h + x;
            v[i*4+0] = color[0];
            v[i*4+1] = color[1];
            v[i*4+2] = color[2];
            v[i*4+3] = color[3];
        }
    }

    v
}

pub fn generate_image_up_to_layer(area: Area, seed: i64, layer: u32) -> Vec<u8> {
    let map = generate_up_to_layer(area, seed, layer);
    let (w, h) = map.a.dim();
    let mut v = vec![0; w*h*4];
    for x in 0..w {
        for z in 0..h {
            let color = biome_to_color(map.a[(x, z)]);
            let i = z * h + x;
            v[i*4+0] = color[0];
            v[i*4+1] = color[1];
            v[i*4+2] = color[2];
            v[i*4+3] = color[3];
        }
    }

    v
}

pub fn generate(a: Area, world_seed: i64) -> Map {
    let g0 = MapIsland::new(1, world_seed);
    let mut g1 = MapZoom::new(2000, world_seed);
    g1.parent = Some(Rc::new(g0));
    g1.fuzzy = true;
    let mut g2 = MapAddIsland::new(1, world_seed);
    g2.parent = Some(Rc::new(g1));
    let mut g3 = MapZoom::new(2001, world_seed);
    g3.parent = Some(Rc::new(g2));
    let mut g4 = MapAddIsland::new(2, world_seed);
    g4.parent = Some(Rc::new(g3));
    let mut g5 = MapAddIsland::new(50, world_seed);
    g5.parent = Some(Rc::new(g4));
    let mut g6 = MapAddIsland::new(70, world_seed);
    g6.parent = Some(Rc::new(g5));
    let mut g7 = MapRemoveTooMuchOcean::new(2, world_seed);
    g7.parent = Some(Rc::new(g6));
    let mut g8 = MapAddSnow::new(2, world_seed);
    g8.parent = Some(Rc::new(g7));
    let mut g9 = MapAddIsland::new(3, world_seed);
    g9.parent = Some(Rc::new(g8));
    let mut g10 = MapCoolWarm::new(2, world_seed);
    g10.parent = Some(Rc::new(g9));
    let mut g11 = MapHeatIce::new(2, world_seed);
    g11.parent = Some(Rc::new(g10));
    let mut g12 = MapSpecial::new(3, world_seed);
    g12.parent = Some(Rc::new(g11));
    let mut g13 = MapZoom::new(2002, world_seed);
    g13.parent = Some(Rc::new(g12));
    let mut g14 = MapZoom::new(2003, world_seed);
    g14.parent = Some(Rc::new(g13));
    let mut g15 = MapAddIsland::new(4, world_seed);
    g15.parent = Some(Rc::new(g14));
    let mut g16 = MapAddMushroomIsland::new(5, world_seed);
    g16.parent = Some(Rc::new(g15));
    let mut g17 = MapDeepOcean::new(4, world_seed);
    g17.parent = Some(Rc::new(g16));
    let g17 = Rc::new(g17);
    let mut g18 = MapBiome::new(200, world_seed);
    g18.parent = Some(g17.clone());
    let mut g19 = MapZoom::new(1000, world_seed);
    g19.parent = Some(Rc::new(g18));
    let mut g20 = MapZoom::new(1001, world_seed);
    g20.parent = Some(Rc::new(g19));
    let mut g21 = MapBiomeEdge::new(1000, world_seed);
    g21.parent = Some(Rc::new(g20));
    let mut g22 = MapRiverInit::new(100, world_seed);
    g22.parent = Some(g17.clone());
    let g22 = Rc::new(g22);
    let mut g23 = MapZoom::new(1000, world_seed);
    g23.parent = Some(g22.clone());
    let mut g24 = MapZoom::new(1001, world_seed);
    g24.parent = Some(Rc::new(g23));
    let mut g25 = MapHills::new(1000, world_seed);
    g25.parent1 = Some(Rc::new(g21));
    g25.parent2 = Some(Rc::new(g24));
    let mut g26 = MapRareBiome::new(1001, world_seed);
    g26.parent = Some(Rc::new(g25));
    let mut g27 = MapZoom::new(1000, world_seed);
    g27.parent = Some(Rc::new(g26));
    let mut g28 = MapAddIsland::new(3, world_seed);
    g28.parent = Some(Rc::new(g27));
    let mut g29 = MapZoom::new(1001, world_seed);
    g29.parent = Some(Rc::new(g28));
    let mut g30 = MapShore::new(1000, world_seed);
    g30.parent = Some(Rc::new(g29));
    let mut g31 = MapZoom::new(1002, world_seed);
    g31.parent = Some(Rc::new(g30));
    let mut g32 = MapZoom::new(1003, world_seed);
    g32.parent = Some(Rc::new(g31));
    let mut g33 = MapSmooth::new(1000, world_seed);
    g33.parent = Some(Rc::new(g32));
    let mut g34 = MapZoom::new(1000, world_seed);
    g34.parent = Some(g22.clone());
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
    let mut g40 = MapRiver::new(1, world_seed);
    g40.parent = Some(Rc::new(g39));
    let mut g41 = MapSmooth::new(1000, world_seed);
    g41.parent = Some(Rc::new(g40));
    let mut g42 = MapRiverMix::new(100, world_seed);
    g42.parent1 = Some(Rc::new(g33));
    g42.parent2 = Some(Rc::new(g41));
    let mut g43 = MapVoronoiZoom::new(10, world_seed);
    g43.parent = Some(Rc::new(g42));

    let m1 = g43.get_map(a);
    m1
}

pub fn generate_up_to_layer(a: Area, world_seed: i64, layer: u32) -> Map {
    let g0 = MapIsland::new(1, world_seed);
    if layer == 0 { return MapSkip::new(Rc::new(g0), 0).get_map(a); }
    let mut g1 = MapZoom::new(2000, world_seed);
    g1.parent = Some(Rc::new(g0));
    g1.fuzzy = true;
    if layer == 1 { return MapSkip::new(Rc::new(g1), 0).get_map(a); }
    let mut g2 = MapAddIsland::new(1, world_seed);
    g2.parent = Some(Rc::new(g1));
    if layer == 2 { return MapSkip::new(Rc::new(g2), 0).get_map(a); }
    let mut g3 = MapZoom::new(2001, world_seed);
    g3.parent = Some(Rc::new(g2));
    if layer == 3 { return MapSkip::new(Rc::new(g3), 0).get_map(a); }
    let mut g4 = MapAddIsland::new(2, world_seed);
    g4.parent = Some(Rc::new(g3));
    if layer == 4 { return MapSkip::new(Rc::new(g4), 0).get_map(a); }
    let mut g5 = MapAddIsland::new(50, world_seed);
    g5.parent = Some(Rc::new(g4));
    if layer == 5 { return MapSkip::new(Rc::new(g5), 0).get_map(a); }
    let mut g6 = MapAddIsland::new(70, world_seed);
    g6.parent = Some(Rc::new(g5));
    if layer == 6 { return MapSkip::new(Rc::new(g6), 0).get_map(a); }
    let mut g7 = MapRemoveTooMuchOcean::new(2, world_seed);
    g7.parent = Some(Rc::new(g6));
    if layer == 7 { return MapSkip::new(Rc::new(g7), 0).get_map(a); }
    let mut g8 = MapAddSnow::new(2, world_seed);
    g8.parent = Some(Rc::new(g7));
    if layer == 8 { return MapSkip::new(Rc::new(g8), 0).get_map(a); }
    let mut g9 = MapAddIsland::new(3, world_seed);
    g9.parent = Some(Rc::new(g8));
    if layer == 9 { return MapSkip::new(Rc::new(g9), 0).get_map(a); }
    let mut g10 = MapCoolWarm::new(2, world_seed);
    g10.parent = Some(Rc::new(g9));
    if layer == 10 { return MapSkip::new(Rc::new(g10), 0).get_map(a); }
    let mut g11 = MapHeatIce::new(2, world_seed);
    g11.parent = Some(Rc::new(g10));
    if layer == 11 { return MapSkip::new(Rc::new(g11), 0).get_map(a); }
    let mut g12 = MapSpecial::new(3, world_seed);
    g12.parent = Some(Rc::new(g11));
    if layer == 12 { return MapSkip::new(Rc::new(g12), 0).get_map(a); }
    let mut g13 = MapZoom::new(2002, world_seed);
    g13.parent = Some(Rc::new(g12));
    if layer == 13 { return MapSkip::new(Rc::new(g13), 0).get_map(a); }
    let mut g14 = MapZoom::new(2003, world_seed);
    g14.parent = Some(Rc::new(g13));
    if layer == 14 { return MapSkip::new(Rc::new(g14), 8).get_map(a); }
    let mut g15 = MapAddIsland::new(4, world_seed);
    g15.parent = Some(Rc::new(g14));
    if layer == 15 { return MapSkip::new(Rc::new(g15), 8).get_map(a); }
    let mut g16 = MapAddMushroomIsland::new(5, world_seed);
    g16.parent = Some(Rc::new(g15));
    if layer == 16 { return MapSkip::new(Rc::new(g16), 8).get_map(a); }
    let mut g17 = MapDeepOcean::new(4, world_seed);
    g17.parent = Some(Rc::new(g16));
    let g17 = Rc::new(g17);
    //if layer == 17 { return MapSkip::new(Rc::clone(&g17), 0).get_map(a); }
    let mut g18 = MapBiome::new(200, world_seed);
    g18.parent = Some(g17.clone());
    if layer == 18 { return MapSkip::new(Rc::new(g18), 8).get_map(a); }
    let mut g19 = MapZoom::new(1000, world_seed);
    g19.parent = Some(Rc::new(g18));
    if layer == 19 { return MapSkip::new(Rc::new(g19), 8).get_map(a); }
    let mut g20 = MapZoom::new(1001, world_seed);
    g20.parent = Some(Rc::new(g19));
    if layer == 20 { return MapSkip::new(Rc::new(g20), 7).get_map(a); }
    let mut g21 = MapBiomeEdge::new(1000, world_seed);
    g21.parent = Some(Rc::new(g20));
    if layer == 21 { return MapSkip::new(Rc::new(g21), 7).get_map(a); }
    let mut g22 = MapRiverInit::new(100, world_seed);
    g22.parent = Some(g17.clone());
    let g22 = Rc::new(g22);
    //if layer == 22 { return MapSkip::new(Rc::clone(&g22), 0).get_map(a); }
    let mut g23 = MapZoom::new(1000, world_seed);
    g23.parent = Some(g22.clone());
    if layer == 23 { return MapSkip::new(Rc::new(g23), 8).get_map(a); }
    let mut g24 = MapZoom::new(1001, world_seed);
    g24.parent = Some(Rc::new(g23));
    if layer == 24 { return MapSkip::new(Rc::new(g24), 7).get_map(a); }
    let mut g25 = MapHills::new(1000, world_seed);
    g25.parent1 = Some(Rc::new(g21));
    g25.parent2 = Some(Rc::new(g24));
    if layer == 25 { return MapSkip::new(Rc::new(g25), 6).get_map(a); }
    let mut g26 = MapRareBiome::new(1001, world_seed);
    g26.parent = Some(Rc::new(g25));
    if layer == 26 { return MapSkip::new(Rc::new(g26), 6).get_map(a); }
    let mut g27 = MapZoom::new(1000, world_seed);
    g27.parent = Some(Rc::new(g26));
    if layer == 27 { return MapSkip::new(Rc::new(g27), 5).get_map(a); }
    let mut g28 = MapAddIsland::new(3, world_seed);
    g28.parent = Some(Rc::new(g27));
    if layer == 28 { return MapSkip::new(Rc::new(g28), 5).get_map(a); }
    let mut g29 = MapZoom::new(1001, world_seed);
    g29.parent = Some(Rc::new(g28));
    if layer == 29 { return MapSkip::new(Rc::new(g29), 4).get_map(a); }
    let mut g30 = MapShore::new(1000, world_seed);
    g30.parent = Some(Rc::new(g29));
    if layer == 30 { return MapSkip::new(Rc::new(g30), 4).get_map(a); }
    let mut g31 = MapZoom::new(1002, world_seed);
    g31.parent = Some(Rc::new(g30));
    if layer == 31 { return MapSkip::new(Rc::new(g31), 3).get_map(a); }
    let mut g32 = MapZoom::new(1003, world_seed);
    g32.parent = Some(Rc::new(g31));
    if layer == 32 { return MapSkip::new(Rc::new(g32), 2).get_map(a); }
    let mut g33 = MapSmooth::new(1000, world_seed);
    g33.parent = Some(Rc::new(g32));
    if layer == 33 { return MapSkip::new(Rc::new(g33), 2).get_map(a); }
    let mut g34 = MapZoom::new(1000, world_seed);
    g34.parent = Some(g22.clone());
    if layer == 34 { return MapSkip::new(Rc::new(g34), 7).get_map(a); }
    let mut g35 = MapZoom::new(1001, world_seed);
    g35.parent = Some(Rc::new(g34));
    if layer == 35 { return MapSkip::new(Rc::new(g35), 6).get_map(a); }
    let mut g36 = MapZoom::new(1000, world_seed);
    g36.parent = Some(Rc::new(g35));
    if layer == 36 { return MapSkip::new(Rc::new(g36), 5).get_map(a); }
    let mut g37 = MapZoom::new(1001, world_seed);
    g37.parent = Some(Rc::new(g36));
    if layer == 37 { return MapSkip::new(Rc::new(g37), 4).get_map(a); }
    let mut g38 = MapZoom::new(1002, world_seed);
    g38.parent = Some(Rc::new(g37));
    if layer == 38 { return MapSkip::new(Rc::new(g38), 3).get_map(a); }
    let mut g39 = MapZoom::new(1003, world_seed);
    g39.parent = Some(Rc::new(g38));
    if layer == 39 { return MapSkip::new(Rc::new(g39), 2).get_map(a); }
    let mut g40 = MapRiver::new(1, world_seed);
    g40.parent = Some(Rc::new(g39));
    if layer == 40 { return MapSkip::new(Rc::new(g40), 2).get_map(a); }
    let mut g41 = MapSmooth::new(1000, world_seed);
    g41.parent = Some(Rc::new(g40));
    if layer == 41 { return MapSkip::new(Rc::new(g41), 2).get_map(a); }
    let mut g42 = MapRiverMix::new(100, world_seed);
    g42.parent1 = Some(Rc::new(g33));
    g42.parent2 = Some(Rc::new(g41));
    if layer == 42 { return MapSkip::new(Rc::new(g42), 2).get_map(a); }
    let mut g43 = MapVoronoiZoom::new(10, world_seed);
    g43.parent = Some(Rc::new(g42));

    let m1 = g43.get_map(a);
    m1
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let a_r = reverse_map_voronoi_zoom(&b.a, 0, 0, 0);
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
        let parent: Option<Rc<GetMap>> = Some(Rc::new(TestMapZero));
        let g0 = MapIsland::new(base_seed, world_seed);
        let mut g1 = MapZoom::new(base_seed, world_seed);
        g1.parent = parent.clone();
        let mut g2 = MapAddIsland::new(base_seed, world_seed);
        g2.parent = parent.clone();
        let mut g3 = MapVoronoiZoom::new(base_seed, world_seed);
        g3.parent = parent.clone();
        let gv: Vec<&GetMap> = vec![&TestMapZero, &TestMapXhz, &g0, &g1, &g2, &g3];
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
        generate(a, 1234);
    }
}

#[allow(non_upper_case_globals)]
mod biome_id {
pub type BiomeID = i32;
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

pub static BIOME_COLORS: [[u8; 3]; 256] =
[[0, 0, 112], [141, 179, 96], [250, 148, 24], [96, 96, 96], [5, 102, 33], [11, 102, 89], [7, 249, 178], [0, 0, 255], [255, 0, 0], [128, 128, 255], [112, 112, 214], [160, 160, 255], [255, 255, 255], [160, 160, 160], [255, 0, 255], [160, 0, 255], [250, 222, 85], [210, 95, 18], [34, 85, 28], [22, 57, 51], [114, 120, 154], [83, 123, 9], [44, 66, 5], [98, 139, 23], [0, 0, 48], [162, 162, 132], [250, 240, 192], [48, 116, 68], [31, 95, 50], [64, 81, 26], [49, 85, 74], [36, 63, 54], [89, 102, 81], [69, 79, 62], [80, 112, 80], [189, 178, 95], [167, 157, 100], [217, 69, 21], [176, 151, 101], [202, 140, 101], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 172], [0, 0, 144], [32, 32, 112], [0, 0, 80], [0, 0, 64], [32, 32, 56], [64, 64, 144], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 112], [141, 179, 96], [250, 148, 24], [96, 96, 96], [5, 102, 33], [11, 102, 89], [7, 249, 178], [0, 0, 255], [255, 0, 0], [128, 128, 255], [144, 144, 160], [160, 160, 255], [140, 180, 180], [160, 160, 160], [255, 0, 255], [160, 0, 255], [250, 222, 85], [210, 95, 18], [34, 85, 28], [22, 57, 51], [114, 120, 154], [83, 123, 9], [44, 66, 5], [98, 139, 23], [0, 0, 48], [162, 162, 132], [250, 240, 192], [48, 116, 68], [31, 95, 50], [64, 81, 26], [49, 85, 74], [36, 63, 54], [89, 102, 81], [69, 79, 62], [80, 112, 80], [189, 178, 95], [167, 157, 100], [217, 69, 21], [176, 151, 101], [202, 140, 101], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0], [0, 0, 0]]
;

pub static BIOME_INFO: [Biome; 256] = 
[Biome { id: 0, type_0: 0, height: -1.0, temp: 0.5, tempCat: 0 }, Biome { id: 1, type_0: 1, height: 0.10000000149011612, temp: 0.800000011920929, tempCat: 2 }, Biome { id: 2, type_0: 2, height: 0.125, temp: 2.0, tempCat: 1 }, Biome { id: 3, type_0: 3, height: 1.0, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 4, type_0: 4, height: 0.10000000149011612, temp: 0.699999988079071, tempCat: 2 }, Biome { id: 5, type_0: 5, height: 0.20000000298023224, temp: 0.25, tempCat: 2 }, Biome { id: 6, type_0: 6, height: -0.20000000298023224, temp: 0.800000011920929, tempCat: 2 }, Biome { id: 7, type_0: 7, height: -0.5, temp: 0.5, tempCat: 2 }, Biome { id: 8, type_0: 8, height: 0.10000000149011612, temp: 2.0, tempCat: 1 }, Biome { id: 9, type_0: 9, height: 0.10000000149011612, temp: 0.5, tempCat: 2 }, Biome { id: 10, type_0: 0, height: -1.0, temp: 0.0, tempCat: 0 }, Biome { id: 11, type_0: 7, height: -0.5, temp: 0.0, tempCat: 3 }, Biome { id: 12, type_0: 10, height: 0.125, temp: 0.0, tempCat: 3 }, Biome { id: 13, type_0: 10, height: 0.44999998807907104, temp: 0.0, tempCat: 3 }, Biome { id: 14, type_0: 11, height: 0.20000000298023224, temp: 0.8999999761581421, tempCat: 2 }, Biome { id: 15, type_0: 11, height: 0.0, temp: 0.8999999761581421, tempCat: 2 }, Biome { id: 16, type_0: 12, height: 0.0, temp: 0.800000011920929, tempCat: 2 }, Biome { id: 17, type_0: 2, height: 0.44999998807907104, temp: 2.0, tempCat: 1 }, Biome { id: 18, type_0: 4, height: 0.44999998807907104, temp: 0.699999988079071, tempCat: 2 }, Biome { id: 19, type_0: 5, height: 0.44999998807907104, temp: 0.25, tempCat: 2 }, Biome { id: 20, type_0: 3, height: 1.0, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 21, type_0: 13, height: 0.10000000149011612, temp: 0.949999988079071, tempCat: 2 }, Biome { id: 22, type_0: 13, height: 0.44999998807907104, temp: 0.949999988079071, tempCat: 2 }, Biome { id: 23, type_0: 13, height: 0.10000000149011612, temp: 0.949999988079071, tempCat: 2 }, Biome { id: 24, type_0: 0, height: -1.7999999523162842, temp: 0.5, tempCat: 0 }, Biome { id: 25, type_0: 14, height: 0.10000000149011612, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 26, type_0: 12, height: 0.0, temp: 0.05000000074505806, tempCat: 3 }, Biome { id: 27, type_0: 4, height: 0.10000000149011612, temp: 0.6000000238418579, tempCat: 2 }, Biome { id: 28, type_0: 4, height: 0.44999998807907104, temp: 0.6000000238418579, tempCat: 2 }, Biome { id: 29, type_0: 4, height: 0.10000000149011612, temp: 0.699999988079071, tempCat: 2 }, Biome { id: 30, type_0: 5, height: 0.20000000298023224, temp: -0.5, tempCat: 3 }, Biome { id: 31, type_0: 5, height: 0.44999998807907104, temp: -0.5, tempCat: 3 }, Biome { id: 32, type_0: 5, height: 0.20000000298023224, temp: 0.30000001192092896, tempCat: 2 }, Biome { id: 33, type_0: 5, height: 0.44999998807907104, temp: 0.30000001192092896, tempCat: 2 }, Biome { id: 34, type_0: 3, height: 1.0, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 35, type_0: 15, height: 0.125, temp: 1.2000000476837158, tempCat: 1 }, Biome { id: 36, type_0: 15, height: 1.5, temp: 1.0, tempCat: 1 }, Biome { id: 37, type_0: 16, height: 0.10000000149011612, temp: 2.0, tempCat: 1 }, Biome { id: 38, type_0: 16, height: 1.5, temp: 2.0, tempCat: 1 }, Biome { id: 39, type_0: 16, height: 1.5, temp: 2.0, tempCat: 1 }, Biome { id: 40, type_0: 9, height: 0.0, temp: 0.0, tempCat: 2 }, Biome { id: 41, type_0: 9, height: 0.0, temp: 0.0, tempCat: 2 }, Biome { id: 42, type_0: 9, height: 0.0, temp: 0.0, tempCat: 2 }, Biome { id: 43, type_0: 9, height: 0.0, temp: 0.0, tempCat: 2 }, Biome { id: 44, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 45, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 46, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 47, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 48, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 49, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 50, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 129, type_0: 1, height: 0.10000000149011612, temp: 0.800000011920929, tempCat: 2 }, Biome { id: 130, type_0: 2, height: 0.125, temp: 2.0, tempCat: 1 }, Biome { id: 131, type_0: 3, height: 1.0, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 132, type_0: 4, height: 0.10000000149011612, temp: 0.699999988079071, tempCat: 2 }, Biome { id: 133, type_0: 5, height: 0.20000000298023224, temp: 0.25, tempCat: 2 }, Biome { id: 134, type_0: 6, height: -0.20000000298023224, temp: 0.800000011920929, tempCat: 2 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 140, type_0: 10, height: 0.125, temp: 0.0, tempCat: 3 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 149, type_0: 13, height: 0.10000000149011612, temp: 0.949999988079071, tempCat: 2 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 151, type_0: 13, height: 0.10000000149011612, temp: 0.949999988079071, tempCat: 2 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 155, type_0: 4, height: 0.10000000149011612, temp: 0.6000000238418579, tempCat: 2 }, Biome { id: 156, type_0: 4, height: 0.44999998807907104, temp: 0.6000000238418579, tempCat: 2 }, Biome { id: 157, type_0: 4, height: 0.10000000149011612, temp: 0.699999988079071, tempCat: 2 }, Biome { id: 158, type_0: 5, height: 0.20000000298023224, temp: -0.5, tempCat: 3 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: 160, type_0: 5, height: 0.20000000298023224, temp: 0.30000001192092896, tempCat: 2 }, Biome { id: 161, type_0: 5, height: 0.44999998807907104, temp: 0.30000001192092896, tempCat: 2 }, Biome { id: 162, type_0: 3, height: 1.0, temp: 0.20000000298023224, tempCat: 2 }, Biome { id: 163, type_0: 15, height: 0.125, temp: 1.2000000476837158, tempCat: 1 }, Biome { id: 164, type_0: 15, height: 1.5, temp: 1.0, tempCat: 1 }, Biome { id: 165, type_0: 16, height: 0.10000000149011612, temp: 2.0, tempCat: 1 }, Biome { id: 166, type_0: 16, height: 1.5, temp: 2.0, tempCat: 1 }, Biome { id: 167, type_0: 16, height: 1.5, temp: 2.0, tempCat: 1 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }, Biome { id: -1, type_0: 0, height: 0.0, temp: 0.0, tempCat: 0 }]
;
