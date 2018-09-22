#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(unused_mut)]
#![allow(unused_unsafe)]
#![allow(unused_assignments)]
#![allow(unused_variables)]

//! This module was generated using the [C2Rust][1] tool and the
//! C code from [Cubiomes][2].
//!
//! That code (with some minor modifications) was directly converted to unsafe Rust.
//! As such, the resulting code uses pointers, calls malloc and free, and is
//! sematically equivalent to the C code, with all bugs included.
//! Unfortunately the [C2Rust][1] tool is still in its early phases, so the
//! generated code has some flaws. The major flaw is that the code is
//! duplicated instead of using `use` statements, so for example there is one
//! `Layer` struct in each module, and they can only be converted from one to
//! another using unsafe pointer transmute: `l as *mut _ as *mut other_mod::Layer`.
//! Therefore, this module is mainly used for testing the equivalence between
//! the pure Rust implementation from the biome_layers module and the [Cubiomes][2]
//! implementation, and will be dropped once the pure Rust code achives similar
//! functionality and performance.
//!
//! [1]: https://github.com/immunant/c2rust
//! [2]: https://github.com/Cubitect/cubiomes

pub mod biome_util;

pub mod finders;

pub mod generator;

pub mod layers;

pub mod rendermaplayers;

