#![allow(dead_code)]
#![cfg_attr(feature = "cubiomes_rs", feature(extern_types))]
#[macro_use]
extern crate ndarray;
#[cfg(feature = "cubiomes_rs")]
extern crate libc;

pub mod java_rng;
pub mod chunk;
pub mod slime;
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod biome_layers;
pub mod mc_rng;
#[cfg(feature = "cubiomes_rs")]
pub mod cubiomes_test;
#[cfg(feature = "cubiomes_rs")]
pub mod cubiomes_rs;
pub mod voronoi;
pub mod seed_info;
pub use crate::java_rng::Rng;
pub use crate::chunk::Chunk;
pub use crate::slime::is_slime_chunk;
pub use crate::slime::seed_from_slime_chunks;

pub fn generate_slime_chunks(seed: i64, limit: usize) -> Vec<Chunk> {
    generate_slime_chunks_or_not(true, seed, limit)
}

pub fn generate_no_slime_chunks(seed: i64, limit: usize) -> Vec<Chunk> {
    generate_slime_chunks_or_not(false, seed, limit)
}

pub fn generate_slime_chunks_or_not(slime: bool, seed: i64, limit: usize) -> Vec<Chunk> {
    let mut v = Vec::with_capacity(limit);
    for x in 0.. { // yeah just go on forever
        for z in -99..100 {
            let c = Chunk::new(x, z);
            if is_slime_chunk(seed as u64, &c) ^ (!slime) {
                v.push(c);
                if v.len() >= limit {
                    return v;
                }
            }
        }
    }

    // unreachable
    vec![]
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

