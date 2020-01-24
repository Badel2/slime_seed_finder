#![feature(test)]



extern crate test;

use slime_seed_finder::biome_layers::*;
use ndarray::Array2;
use self::test::Bencher;

// DIM: Base map size for the benchmarks.
// To ensure a useful comparison between layers, the layers which increase
// the scale x2 must use DIM2, and voronoi zoom (x4) must use DIM4.
// All other layers which preserve dimensions must use DIM.
const DIM4: (usize, usize) = (30, 30);
const DIM2: (usize, usize) = (DIM4.0 * 2, DIM4.1 * 2);
const DIM: (usize, usize) = (DIM4.0 * 4, DIM4.1 * 4);

#[bench]
fn map_voronoi_zoom_xhz(b: &mut Bencher) {
    let base_seed = 10;
    let world_seed = 1234;
    let voronoi_zoom = MapVoronoiZoom::new(base_seed, world_seed);
    let (w, h) = DIM4;
    let mut a = Array2::zeros((w, h));
    for z in 0..h {
        for x in 0..w {
            a[(x, z)] = (x * h + z) as i32;
        }
    }
        
    let mut m = Map::new(Area { x: 0, z: 0, w: 0, h: 0 });
    m.a = a;

    b.iter(|| {
        voronoi_zoom.get_map_from_pmap(&m)
    });
}

#[bench]
fn map_voronoi_zoom_zeros(b: &mut Bencher) {
    let base_seed = 10;
    let world_seed = 1234;
    let voronoi_zoom = MapVoronoiZoom::new(base_seed, world_seed);
    let (w, h) = DIM4;
    let a = Array2::zeros((w, h));
    let mut m = Map::new(Area { x: 0, z: 0, w: 0, h: 0 });
    m.a = a;
        
    b.iter(|| {
        voronoi_zoom.get_map_from_pmap(&m)
    });
}

#[bench]
fn map_island(b: &mut Bencher) {
    let base_seed = 1;
    let world_seed = 1234;
    let gen = MapIsland::new(base_seed, world_seed);
    let (w, h) = DIM;
    let area = Area { x: 0, z: 0, w: w as u64, h: h as u64 };
    b.iter(|| {
        gen.get_map(area)
    });
}

#[bench]
fn map_zoom_fuzzy_zeros(b: &mut Bencher) {
    let (w, h) = DIM2;
    let a = Array2::zeros((w, h));
    let mut m = Map::new(Area { x: 0, z: 0, w: 0, h: 0 });
    m.a = a;
    bench_map_zoom_fuzzy(b, &m);
}

#[bench]
fn map_zoom_fuzzy_xhz(b: &mut Bencher) {
    let (w, h) = DIM2;
    let mut a = Array2::zeros((w, h));
    for z in 0..h {
        for x in 0..w {
            a[(x, z)] = (x * h + z) as i32;
        }
    }
    let mut m = Map::new(Area { x: 0, z: 0, w: 0, h: 0 });
    m.a = a;
    bench_map_zoom_fuzzy(b, &m);
}

#[bench]
fn map_zoom_zeros(b: &mut Bencher) {
    let (w, h) = DIM2;
    let a = Array2::zeros((w, h));
    let mut m = Map::new(Area { x: 0, z: 0, w: 0, h: 0 });
    m.a = a;
    bench_map_zoom(b, &m);
}

#[bench]
fn map_zoom_xhz(b: &mut Bencher) {
    let (w, h) = DIM2;
    let mut a = Array2::zeros((w, h));
    for z in 0..h {
        for x in 0..w {
            a[(x, z)] = (x * h + z) as i32;
        }
    }
    let mut m = Map::new(Area { x: 0, z: 0, w: 0, h: 0 });
    m.a = a;
    bench_map_zoom(b, &m);
}

// This is a real world benchmark: fuzzy zoom is only used after MapIsland
#[bench]
fn map_zoom_fuzzy_island(b: &mut Bencher) {
    let base_seed = 1;
    let world_seed = 1234;
    let island = MapIsland::new(base_seed, world_seed);
    let (w, h) = DIM2;
    let area = Area { x: 0, z: 0, w: w as u64, h: h as u64 };
    let pmap = island.get_map(area);
    bench_map_zoom_fuzzy(b, &pmap);
}

fn bench_map_zoom(b: &mut Bencher, pmap: &Map) {
    let base_seed = 2000;
    let world_seed = 1234;
    let gen = MapZoom::new(base_seed, world_seed);
    b.iter(|| {
        gen.get_map_from_pmap(&pmap)
    });
}

fn bench_map_zoom_fuzzy(b: &mut Bencher, pmap: &Map) {
    let base_seed = 2000;
    let world_seed = 1234;
    let gen = MapZoomFuzzy::new(base_seed, world_seed);
    b.iter(|| {
        gen.get_map_from_pmap(&pmap)
    });

}

#[bench]
fn map_add_island_zeros(b: &mut Bencher) {
    let (w, h) = DIM;
    let a = Array2::zeros((w, h));
    let mut m = Map::new(Area { x: 0, z: 0, w: 0, h: 0 });
    m.a = a;

    let base_seed = 1;
    let world_seed = 1234;
    let gen = MapAddIsland::new(base_seed, world_seed);
    b.iter(|| {
        gen.get_map_from_pmap(&m)
    });
}

#[bench]
fn map_add_island_xhz(b: &mut Bencher) {
    let (w, h) = DIM;
    let mut a = Array2::zeros((w, h));
    for z in 0..h {
        for x in 0..w {
            a[(x, z)] = (x * h + z) as i32;
        }
    }
    let mut m = Map::new(Area { x: 0, z: 0, w: 0, h: 0 });
    m.a = a;

    let base_seed = 1;
    let world_seed = 1234;
    let gen = MapAddIsland::new(base_seed, world_seed);
    b.iter(|| {
        gen.get_map_from_pmap(&m)
    });
}
