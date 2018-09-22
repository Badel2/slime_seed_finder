use biome_layers::*;
// TODO: Array2[(x, z)] is a nice syntax, but the fastest dimension to iterate
// is the z dimension, but in the Java code it is the x dimension, as the arrays
// are defined as (z * w + x).
use cubiomes_rs::layers;
use cubiomes_rs::generator;
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

#[cfg(test)]
mod tests {
    use super::*;
    use mc_rng::McRng;
    use std::rc::Rc;

    #[test]
    fn map_l00_island() {
        let base_seed = 1;
        let world_seed = 9223090561890311698;
        let a = Area { x: -20, z: -10, w: 100, h: 100 };
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
        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = gen.get_map(a);
        let m2 = call_layer(1, world_seed, a);
        assert_eq!(m1, m2);
    }

    #[test]
    fn map_all_layers() {
        let world_seed = 9223090561890311698;
        let g0 = MapIsland::new(1, world_seed);

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g0.get_map(a);
        let m2 = call_layer(0, world_seed, a);
        assert_eq!(m1, m2);

        let mut g1 = MapZoom::new(2000, world_seed);
        g1.parent = Some(Rc::new(g0));
        g1.fuzzy = true;

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g1.get_map(a);
        let m2 = call_layer(1, world_seed, a);
        assert_eq!(m1, m2);

        let mut g2 = MapAddIsland::new(1, world_seed);
        g2.parent = Some(Rc::new(g1));

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g2.get_map(a);
        let m2 = call_layer(2, world_seed, a);
        assert_eq!(m1, m2);

        let mut g3 = MapZoom::new(2001, world_seed);
        g3.parent = Some(Rc::new(g2));

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g3.get_map(a);
        let m2 = call_layer(3, world_seed, a);
        assert_eq!(m1, m2);

        let mut g4 = MapAddIsland::new(2, world_seed);
        g4.parent = Some(Rc::new(g3));

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g4.get_map(a);
        let m2 = call_layer(4, world_seed, a);
        assert_eq!(m1, m2);

        let mut g5 = MapAddIsland::new(50, world_seed);
        g5.parent = Some(Rc::new(g4));

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g5.get_map(a);
        let m2 = call_layer(5, world_seed, a);
        assert_eq!(m1, m2);

        let mut g6 = MapAddIsland::new(70, world_seed);
        g6.parent = Some(Rc::new(g5));

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g6.get_map(a);
        let m2 = call_layer(6, world_seed, a);
        assert_eq!(m1, m2);

        let mut g7 = MapRemoveTooMuchOcean::new(2, world_seed);
        g7.parent = Some(Rc::new(g6));

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g7.get_map(a);
        let m2 = call_layer(7, world_seed, a);
        assert_eq!(m1, m2);

        let mut g8 = MapAddSnow::new(2, world_seed);
        g8.parent = Some(Rc::new(g7));

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g8.get_map(a);
        let m2 = call_layer(8, world_seed, a);
        assert_eq!(m1, m2);

        let mut g9 = MapAddIsland::new(3, world_seed);
        g9.parent = Some(Rc::new(g8));

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g9.get_map(a);
        let m2 = call_layer(9, world_seed, a);
        assert_eq!(m1, m2);

        let mut g10 = MapCoolWarm::new(2, world_seed);
        g10.parent = Some(Rc::new(g9));

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g10.get_map(a);
        let m2 = call_layer(10, world_seed, a);
        assert_eq!(m1, m2);

        let mut g11 = MapHeatIce::new(2, world_seed);
        g11.parent = Some(Rc::new(g10));

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g11.get_map(a);
        let m2 = call_layer(11, world_seed, a);
        assert_eq!(m1, m2);

        let mut g12 = MapSpecial::new(3, world_seed);
        g12.parent = Some(Rc::new(g11));

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g12.get_map(a);
        let m2 = call_layer(12, world_seed, a);
        assert_eq!(m1, m2);

        let mut g13 = MapZoom::new(2002, world_seed);
        g13.parent = Some(Rc::new(g12));

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g13.get_map(a);
        let m2 = call_layer(13, world_seed, a);
        assert_eq!(m1, m2);

        let mut g14 = MapZoom::new(2003, world_seed);
        g14.parent = Some(Rc::new(g13));

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g14.get_map(a);
        let m2 = call_layer(14, world_seed, a);
        assert_eq!(m1, m2);

        let mut g15 = MapAddIsland::new(4, world_seed);
        g15.parent = Some(Rc::new(g14));

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g15.get_map(a);
        let m2 = call_layer(15, world_seed, a);
        assert_eq!(m1, m2);

        let mut g16 = MapAddMushroomIsland::new(5, world_seed);
        g16.parent = Some(Rc::new(g15));

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g16.get_map(a);
        let m2 = call_layer(16, world_seed, a);
        assert_eq!(m1, m2);

        let mut g17 = MapDeepOcean::new(4, world_seed);
        g17.parent = Some(Rc::new(g16));

        let a = Area { x: -20, z: -10, w: 100, h: 100 };
        let m1 = g17.get_map(a);
        let m2 = call_layer(17, world_seed, a);
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
}
