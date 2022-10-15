#![allow(unused_labels)]

#[macro_use]
extern crate ndarray;

#[rustfmt::skip]
pub mod anvil;
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[rustfmt::skip]
pub mod biome_info;
pub mod biome_info_118;
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[rustfmt::skip]
pub mod biome_layers;
pub mod chunk;
pub mod climate;
pub mod fastanvil_ext;
pub mod gen_pairs3;
pub mod java_rng;
#[rustfmt::skip]
pub mod mc_rng;
pub mod noise_generator;
pub mod population;
#[rustfmt::skip]
pub mod seed_info;
pub mod slime;
pub mod spline;
pub mod strict_parse_int;
pub mod structures;
pub mod voronoi;
pub mod weak_alloc;
pub mod xoroshiro128plusplus;
pub mod zip_ext;
