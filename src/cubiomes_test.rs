use biome_layers::*;
// TODO: Array2[(x, z)] is a nice syntax, but the fastest dimension to iterate
// is the z dimension, but in the Java code it is the x dimension, as the arrays
// are defined as (z * w + x).
use cubiomes_rs::{biome_util, layers, generator};
use cubiomes_rs::generator::{allocCache, applySeed, freeGenerator, genArea, setupGeneratorMC17};
//use cubiomes_rs::rendermaplayers::getMapForLayerIdx;
use libc;

type LayerFn = unsafe extern "C" fn(l: *mut layers::Layer, out: *mut libc::c_int, x: libc::c_int, z: libc::c_int, w: libc::c_int, h: libc::c_int);

#[allow(non_snake_case)]
#[no_mangle]
pub unsafe extern "C" fn getMapForLayerIdx(
    idx: libc::c_int,
    worldSeed: i64,
    areaX: libc::c_int,
    areaZ: libc::c_int,
    areaWidth: libc::c_int,
    areaHeight: libc::c_int,
) -> *mut libc::c_int {
    layers::initBiomes();
    let mut g = setupGeneratorMC17();
    applySeed(&mut g, worldSeed);
    let l = &mut *g.layers.offset(idx as isize) as *mut generator::Layer;
    println!("{:#?}", *l);
    let cache: *mut libc::c_int = allocCache(l, areaWidth, areaHeight);
    genArea(l, cache, areaX, areaZ, areaWidth, areaHeight);
    freeGenerator(g);
    return cache;
}

pub unsafe fn biome_colors() -> [[u8; 3]; 256] {
    let mut biome: [[u8; 3]; 256] = [[0; 3]; 256];
    biome_util::initBiomeColours(biome.as_mut_ptr());
    biome
}

pub unsafe fn biome_info() -> Vec<Biome> {
    layers::initBiomes();
    layers::biomes.iter().map(|x| {
        Biome {
            id: x.id,
            type_0: x.type_0,
            height: x.height,
            temp: x.temp,
            tempCat: x.tempCat,
        }
    }).collect()
}

pub fn call_layer(idx: usize, world_seed: i64, a: Area) -> Map {
    unsafe {
        let out = getMapForLayerIdx(idx as i32, world_seed, a.x as i32, a.z as i32, a.w as i32, a.h as i32);
        let mut map = Map::new(a);
        let (w, h) = (a.w as usize, a.h as usize);
        for z in 0..h {
            for x in 0..w {
                map.a[(x, z)] = *out.offset((z * w + x) as isize);
            }
        }
        libc::free(out as *mut libc::c_void);
        
        map
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CubiomesLayer {
    idx: usize,
    world_seed: i64,
}

impl CubiomesLayer {
    fn new(idx: usize, world_seed: i64) -> Self {
        Self { idx, world_seed }
    }
}

impl GetMap for CubiomesLayer {
    fn get_map(&self, area: Area) -> Map {
        call_layer(self.idx, self.world_seed, area)
    }
    fn get_map_from_pmap(&self, pmap: &Map) -> Map {
        unimplemented!("This layer must generate the pmap anyway");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mc_rng::McRng;
    use std::rc::Rc;

    #[test]
    fn map_l00_island() {
        let base_seed = 1;
        let world_seed = 9223090561890311698;
        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let gen = MapIsland::new(base_seed, world_seed);
        let m1 = gen.get_map(a);
        let m2 = call_layer(0, world_seed, a);
        assert_eq!(m1, m2);
    }

    #[test]
    fn map_l01_zoom() {
        let world_seed = 9223090561890311698;
        let base_seed = 2000;
        let mut gen = MapZoom::new(base_seed, world_seed);
        let island = MapIsland::new(1, world_seed);
        gen.parent = Some(Rc::new(island));
        gen.fuzzy = true;
        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = gen.get_map(a);
        let m2 = call_layer(1, world_seed, a);
        assert_eq!(m1, m2);
    }

    #[test]
    fn map_all_layers() {
        let world_seed = 9223090561890311698;
        let g0 = MapIsland::new(1, world_seed);

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g0.get_map(a);
        let m2 = call_layer(0, world_seed, a);
        assert_eq!(m1, m2);

        let mut g1 = MapZoom::new(2000, world_seed);
        g1.parent = Some(Rc::new(g0));
        g1.fuzzy = true;

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g1.get_map(a);
        let m2 = call_layer(1, world_seed, a);
        assert_eq!(m1, m2);

        let mut g2 = MapAddIsland::new(1, world_seed);
        g2.parent = Some(Rc::new(g1));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g2.get_map(a);
        let m2 = call_layer(2, world_seed, a);
        assert_eq!(m1, m2);

        let mut g3 = MapZoom::new(2001, world_seed);
        g3.parent = Some(Rc::new(g2));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g3.get_map(a);
        let m2 = call_layer(3, world_seed, a);
        assert_eq!(m1, m2);

        let mut g4 = MapAddIsland::new(2, world_seed);
        g4.parent = Some(Rc::new(g3));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g4.get_map(a);
        let m2 = call_layer(4, world_seed, a);
        assert_eq!(m1, m2);

        let mut g5 = MapAddIsland::new(50, world_seed);
        g5.parent = Some(Rc::new(g4));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g5.get_map(a);
        let m2 = call_layer(5, world_seed, a);
        assert_eq!(m1, m2);

        let mut g6 = MapAddIsland::new(70, world_seed);
        g6.parent = Some(Rc::new(g5));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g6.get_map(a);
        let m2 = call_layer(6, world_seed, a);
        assert_eq!(m1, m2);

        let mut g7 = MapRemoveTooMuchOcean::new(2, world_seed);
        g7.parent = Some(Rc::new(g6));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g7.get_map(a);
        let m2 = call_layer(7, world_seed, a);
        assert_eq!(m1, m2);

        let mut g8 = MapAddSnow::new(2, world_seed);
        g8.parent = Some(Rc::new(g7));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g8.get_map(a);
        let m2 = call_layer(8, world_seed, a);
        assert_eq!(m1, m2);

        let mut g9 = MapAddIsland::new(3, world_seed);
        g9.parent = Some(Rc::new(g8));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g9.get_map(a);
        let m2 = call_layer(9, world_seed, a);
        assert_eq!(m1, m2);

        let mut g10 = MapCoolWarm::new(2, world_seed);
        g10.parent = Some(Rc::new(g9));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g10.get_map(a);
        let m2 = call_layer(10, world_seed, a);
        assert_eq!(m1, m2);

        let mut g11 = MapHeatIce::new(2, world_seed);
        g11.parent = Some(Rc::new(g10));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g11.get_map(a);
        let m2 = call_layer(11, world_seed, a);
        assert_eq!(m1, m2);

        let mut g12 = MapSpecial::new(3, world_seed);
        g12.parent = Some(Rc::new(g11));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g12.get_map(a);
        let m2 = call_layer(12, world_seed, a);
        assert_eq!(m1, m2);

        let mut g13 = MapZoom::new(2002, world_seed);
        g13.parent = Some(Rc::new(g12));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g13.get_map(a);
        let m2 = call_layer(13, world_seed, a);
        assert_eq!(m1, m2);

        let mut g14 = MapZoom::new(2003, world_seed);
        g14.parent = Some(Rc::new(g13));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g14.get_map(a);
        let m2 = call_layer(14, world_seed, a);
        assert_eq!(m1, m2);

        let mut g15 = MapAddIsland::new(4, world_seed);
        g15.parent = Some(Rc::new(g14));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g15.get_map(a);
        let m2 = call_layer(15, world_seed, a);
        assert_eq!(m1, m2);

        let mut g16 = MapAddMushroomIsland::new(5, world_seed);
        g16.parent = Some(Rc::new(g15));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g16.get_map(a);
        let m2 = call_layer(16, world_seed, a);
        assert_eq!(m1, m2);

        let mut g17 = MapDeepOcean::new(4, world_seed);
        g17.parent = Some(Rc::new(g16));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g17.get_map(a);
        let m2 = call_layer(17, world_seed, a);
        assert_eq!(m1, m2);

        let g17 = Rc::new(g17);
        let mut g18 = MapBiome::new(200, world_seed);
        g18.parent = Some(g17.clone());

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g18.get_map(a);
        let m2 = call_layer(18, world_seed, a);
        assert_eq!(m1, m2);

        let mut g19 = MapZoom::new(1000, world_seed);
        g19.parent = Some(Rc::new(g18));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g19.get_map(a);
        let m2 = call_layer(19, world_seed, a);
        assert_eq!(m1, m2);

        let mut g20 = MapZoom::new(1001, world_seed);
        g20.parent = Some(Rc::new(g19));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g20.get_map(a);
        let m2 = call_layer(20, world_seed, a);
        assert_eq!(m1, m2);

        let mut g21 = MapBiomeEdge::new(1000, world_seed);
        g21.parent = Some(Rc::new(g20));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g21.get_map(a);
        let m2 = call_layer(21, world_seed, a);
        assert_eq!(m1, m2);

        let mut g22 = MapRiverInit::new(100, world_seed);
        g22.parent = Some(g17.clone());

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g22.get_map(a);
        let m2 = call_layer(22, world_seed, a);
        assert_eq!(m1, m2);

        let g22 = Rc::new(g22);
        let mut g23 = MapZoom::new(1000, world_seed);
        g23.parent = Some(g22.clone());

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g23.get_map(a);
        let m2 = call_layer(23, world_seed, a);
        // TODO: This fails for some unknown reason, so instead of trying to fix it,
        // we just continue the test by adding a "virtual" parent to g24
        assert_eq!(m1, m2);

        let mut g24 = MapZoom::new(1001, world_seed);
        g24.parent = Some(Rc::new(g23));
        //g24.parent = Some(Rc::new(CubiomesLayer::new(23, world_seed)));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g24.get_map(a);
        let m2 = call_layer(24, world_seed, a);
        // TODO: This fails for some unknown reason, so instead of trying to fix it,
        // we just continue the test by adding a "virtual" parent to g25
        assert_eq!(m1, m2);

        let mut g25 = MapHills::new(1000, world_seed);
        g25.parent1 = Some(Rc::new(g21));
        g25.parent2 = Some(Rc::new(g24));
        //g25.parent2 = Some(Rc::new(CubiomesLayer::new(24, world_seed)));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g25.get_map(a);
        let m2 = call_layer(25, world_seed, a);
        // TODO: This fails for some unknown reason, so instead of trying to fix it,
        // we just continue the test by adding a "virtual" parent to g26
        assert_eq!(m1, m2);

        let mut g26 = MapRareBiome::new(1001, world_seed);
        g26.parent = Some(Rc::new(g25));
        //g26.parent = Some(Rc::new(CubiomesLayer::new(25, world_seed)));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g26.get_map(a);
        let m2 = call_layer(26, world_seed, a);
        assert_eq!(m1, m2);

        let mut g27 = MapZoom::new(1000, world_seed);
        g27.parent = Some(Rc::new(g26));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g27.get_map(a);
        let m2 = call_layer(27, world_seed, a);
        assert_eq!(m1, m2);

        let mut g28 = MapAddIsland::new(3, world_seed);
        g28.parent = Some(Rc::new(g27));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g28.get_map(a);
        let m2 = call_layer(28, world_seed, a);
        assert_eq!(m1, m2);

        let mut g29 = MapZoom::new(1001, world_seed);
        g29.parent = Some(Rc::new(g28));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g29.get_map(a);
        let m2 = call_layer(29, world_seed, a);
        assert_eq!(m1, m2);

        let mut g30 = MapShore::new(1000, world_seed);
        g30.parent = Some(Rc::new(g29));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g30.get_map(a);
        let m2 = call_layer(30, world_seed, a);
        assert_eq!(m1, m2);

        let mut g31 = MapZoom::new(1002, world_seed);
        g31.parent = Some(Rc::new(g30));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g31.get_map(a);
        let m2 = call_layer(31, world_seed, a);
        assert_eq!(m1, m2);

        let mut g32 = MapZoom::new(1003, world_seed);
        g32.parent = Some(Rc::new(g31));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g32.get_map(a);
        let m2 = call_layer(32, world_seed, a);
        assert_eq!(m1, m2);

        let mut g33 = MapSmooth::new(1000, world_seed);
        g33.parent = Some(Rc::new(g32));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g33.get_map(a);
        let m2 = call_layer(33, world_seed, a);
        assert_eq!(m1, m2);

        let mut g34 = MapZoom::new(1000, world_seed);
        g34.parent = Some(g22.clone());

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g34.get_map(a);
        let m2 = call_layer(34, world_seed, a);
        assert_eq!(m1, m2);

        let mut g35 = MapZoom::new(1001, world_seed);
        g35.parent = Some(Rc::new(g34));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g35.get_map(a);
        let m2 = call_layer(35, world_seed, a);
        assert_eq!(m1, m2);

        let mut g36 = MapZoom::new(1000, world_seed);
        g36.parent = Some(Rc::new(g35));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g36.get_map(a);
        let m2 = call_layer(36, world_seed, a);
        assert_eq!(m1, m2);

        let mut g37 = MapZoom::new(1001, world_seed);
        g37.parent = Some(Rc::new(g36));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g37.get_map(a);
        let m2 = call_layer(37, world_seed, a);
        assert_eq!(m1, m2);

        let mut g38 = MapZoom::new(1002, world_seed);
        g38.parent = Some(Rc::new(g37));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g38.get_map(a);
        let m2 = call_layer(38, world_seed, a);
        assert_eq!(m1, m2);

        let mut g39 = MapZoom::new(1003, world_seed);
        g39.parent = Some(Rc::new(g38));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g39.get_map(a);
        let m2 = call_layer(39, world_seed, a);
        assert_eq!(m1, m2);

        let mut g40 = MapRiver::new(1, world_seed);
        g40.parent = Some(Rc::new(g39));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g40.get_map(a);
        let m2 = call_layer(40, world_seed, a);
        assert_eq!(m1, m2);

        let mut g41 = MapSmooth::new(1000, world_seed);
        g41.parent = Some(Rc::new(g40));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g41.get_map(a);
        let m2 = call_layer(41, world_seed, a);
        assert_eq!(m1, m2);

        let mut g42 = MapRiverMix::new(100, world_seed);
        g42.parent1 = Some(Rc::new(g33));
        g42.parent2 = Some(Rc::new(g41));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g42.get_map(a);
        let m2 = call_layer(42, world_seed, a);
        assert_eq!(m1, m2);

        let mut g43 = MapVoronoiZoom::new(10, world_seed);
        g43.parent = Some(Rc::new(g42));

        let a = Area { x: -20, z: -10, w: 1000, h: 1000 };
        let m1 = g43.get_map(a);
        let m2 = call_layer(43, world_seed, a);
        assert_eq!(m1, m2);
    }

    #[test]
    fn rng_set_chunk_seed() {
        use std::ptr;
        let base_seed = 1;
        let world_seed = 1234;
        let chunk_x = 0;
        let chunk_z = 0;
        let mut r1 = McRng::new(base_seed, world_seed);

        let mut l = generator::Layer {
            baseSeed: 0,
            worldSeed: 0,
            chunkSeed: 0,
            getMap: None,
            oceanRnd: ptr::null_mut(),
            p: ptr::null_mut(),
            p2: ptr::null_mut(),
            scale: 1,
        };
        unsafe {
            generator::setBaseSeed(&mut l, base_seed);
        }
        assert_eq!(r1.base_seed(), l.baseSeed);

        unsafe {
            layers::setWorldSeed(&mut l as *mut _ as *mut layers::Layer, world_seed);
        }
        assert_eq!(r1.base_seed(), l.baseSeed);
        assert_eq!(r1.world_seed(), l.worldSeed);

        r1.set_chunk_seed(chunk_x, chunk_z);
        unsafe {
            generator::setChunkSeed(&mut l, chunk_x, chunk_z);
        }
        assert_eq!(r1.base_seed(), l.baseSeed);
        assert_eq!(r1.world_seed(), l.worldSeed);
        assert_eq!(r1.chunk_seed(), l.chunkSeed);

        unsafe {
            for i in 0..100 {
                assert_eq!((i, r1.next_int_n(10)), (i, generator::mcNextInt(&mut l, 10)));
            }
        }
    }

    #[test]
    fn biome_colors_t() {
        use biome_layers::BIOME_COLORS;
        let biome_colors = unsafe { biome_colors() };
        println!("{:?}", &biome_colors[..]);
        assert_eq!(&biome_colors[..], &BIOME_COLORS[..]);
    }

    #[test]
    fn biome_info_t() {
        use biome_layers::BIOME_INFO;
        let biome_info = unsafe { biome_info() };
        println!("{:?}", &biome_info[..]);
        assert_eq!(&biome_info[..], &BIOME_INFO[..]);
    }
}
