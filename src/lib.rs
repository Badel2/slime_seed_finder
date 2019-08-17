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
pub mod noise_generator;

use crate::slime::is_slime_chunk;
use crate::chunk::Chunk;

pub fn generate_slime_chunks_and_not(seed: i64, limit_yes: usize, limit_no: usize) -> (Vec<Chunk>, Vec<Chunk>) {
    let mut vy = Vec::with_capacity(limit_yes);
    let mut vn = Vec::with_capacity(limit_no);
    for x in 0.. { // yeah just go on forever
        for z in -99..100 {
            let c = Chunk::new(x, z);
            if is_slime_chunk(seed as u64, &c) {
                if vy.len() < limit_yes {
                    vy.push(c);
                }
            } else {
                if vn.len() < limit_no {
                    vn.push(c);
                }
            }
            if vy.len() == limit_yes && vn.len() == limit_no {
                return (vy, vn);
            }
        }
    }

    // unreachable
    (vec![], vec![])
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

