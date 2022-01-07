use crate::biome_layers::{Area, Map};
use crate::chunk::Chunk;
use crate::java_rng::JavaRng;
use log::info;
use std::cmp::min;
use std::num::Wrapping;

pub struct SlimeChunks {
    slime_chunks: Vec<Chunk>,
    slime_data: Vec<u64>,
    no_slime_chunks: Vec<Chunk>,
    no_slime_data: Vec<u64>,
    max_errors: usize,
    max_no_errors: usize,
    low_18_candidates: Vec<u32>,
}

impl SlimeChunks {
    pub fn new(
        slime_chunks: &[Chunk],
        max_errors: usize,
        no_slime_chunks: &[Chunk],
        max_no_errors: usize,
    ) -> SlimeChunks {
        let slime_data: Vec<u64> = slime_chunks
            .into_iter()
            .map(|c| calculate_slime_data(c))
            .collect();
        let no_slime_data: Vec<u64> = no_slime_chunks
            .into_iter()
            .map(|c| calculate_slime_data(c))
            .collect();

        // low_18_candidates is sorted, this is important
        let low_18_candidates = slime_candidates_18(&slime_data, max_errors);

        let slime_chunks = slime_chunks.to_vec();
        let no_slime_chunks = no_slime_chunks.to_vec();

        SlimeChunks {
            slime_chunks,
            slime_data,
            no_slime_chunks,
            no_slime_data,
            max_errors,
            max_no_errors,
            low_18_candidates,
        }
    }

    pub fn find_seed(&self) -> Vec<u64> {
        self.find_seed_range(0, 1 << (48 - 18))
    }

    // Use this to implement multithreading later
    // Range units are multiples of 2^18: lo=0, count=1 will search a 2^18 seed
    // range (runtime depends on number of candidates).
    // The maximum value for `lo = 0` is 2^30, or `count = 1 << 30`.
    pub fn find_seed_range(&self, lo: u32, count: u32) -> Vec<u64> {
        let hi = min(lo + count, 1 << 30);
        let mut v = vec![];

        for &l in &self.low_18_candidates {
            for seed in lo..hi {
                let seed = ((seed as u64) << 18) | (l as u64);

                if self.try_seed_skip_18(seed) {
                    info!("Found seed: {:012X}", seed);
                    v.push(seed);
                }
            }
        }

        v
    }

    // true if the seeds meets the requirements, skip the low18 check
    pub fn try_seed_skip_18(&self, seed: u64) -> bool {
        let mut errors = 0;
        for &x in &self.slime_data {
            let mut r = rng_with_slime_data(seed, x);
            let mod_ten = r.next_int_n(10);
            if mod_ten != 0 {
                errors += 1;
                if errors > self.max_errors {
                    return false;
                }
            }
        }

        let mut no_errors = 0;
        for &x in &self.no_slime_data {
            let mut r = rng_with_slime_data(seed, x);
            let mod_ten = r.next_int_n(10);
            if mod_ten == 0 {
                no_errors += 1;
                if no_errors > self.max_no_errors {
                    return false;
                }
            }
        }

        true
    }

    // true if the seeds meets the requirements
    pub fn try_seed(&self, seed: u64) -> bool {
        // Check the low 18 bits first
        const MASK18: u64 = (1 << 18) - 1;
        let low18 = (seed & MASK18) as u32;
        if let Err(_) = self.low_18_candidates.binary_search(&low18) {
            return false;
        }

        self.try_seed_skip_18(seed)
    }

    pub fn num_low_18_candidates(&self) -> usize {
        self.low_18_candidates.len()
    }

    // Assumes the low 18 bits were already checked
    // Use this only if the input iterator has less than 2^30 candidates
    pub fn bruteforce_iter_u48<'a, I>(&'a self, iter_u48: I) -> impl Iterator<Item = u64> + 'a
    where
        I: IntoIterator<Item = u64> + 'a,
    {
        iter_u48
            .into_iter()
            .filter(move |&seed| self.try_seed_skip_18(seed))
    }
}

mod slime_const {
    use std::num::Wrapping;
    pub const A: Wrapping<i32> = Wrapping(0x4c1906);
    pub const B: Wrapping<i32> = Wrapping(0x5ac0db);
    pub const C: Wrapping<i64> = Wrapping(0x4307a7);
    pub const D: Wrapping<i32> = Wrapping(0x5f24f);
    pub const E: u64 = 0x3ad8025f;
}

// This is the slime chunk algorithm
// See discussion about optimizations in the test section:
// constant_z()
fn calculate_slime_data(c: &Chunk) -> u64 {
    let x = Wrapping(c.x as i32);
    let z = Wrapping(c.z as i32);
    let a = Wrapping((x * x * slime_const::A).0 as i64);
    let b = Wrapping((x * slime_const::B).0 as i64);
    let c = Wrapping((z * z).0 as i64) * slime_const::C;
    let d = Wrapping((z * slime_const::D).0 as i64);

    (a + b + c + d).0 as u64
}

pub fn is_slime_chunk(seed: u64, c: &Chunk) -> bool {
    let x = calculate_slime_data(c);
    is_slime_data(seed, x)
}

fn is_slime_data(seed: u64, x: u64) -> bool {
    let mut r = rng_with_slime_data(seed, x);
    r.next_int_n(10) == 0
}

fn rng_with_slime_data(seed: u64, x: u64) -> JavaRng {
    // new Random(seed + x ^ e)
    let s = seed.wrapping_add(x) ^ slime_const::E;
    JavaRng::with_seed(s)
}

/// Find all the 18-bit seeds that could generate these slime chunks
pub fn slime_candidates_18(slimedata: &[u64], max_errors: usize) -> Vec<u32> {
    slime_candidates_18_iter(0..(1u32 << 18), slimedata, max_errors).collect()
}

/// Parallel version of slime_candidates_18. iter_u18 should be an iterator over all the possible
/// 18-bit integers, or a subset of them.
pub fn slime_candidates_18_iter<'a, I>(
    iter_u18: I,
    slimedata: &'a [u64],
    max_errors: usize,
) -> impl Iterator<Item = u32> + 'a
where
    I: IntoIterator<Item = u32> + 'a,
{
    iter_u18.into_iter().filter(move |&seed| {
        let mut errors = 0;
        for &x in slimedata {
            let mut r = rng_with_slime_data(seed as u64, x);
            // TODO: make sure r doesn't trigger the modulo bias check
            let ten = r.next_int_n(10);

            // Since the parity of "ten" depends only on the last 18 bits of
            // the seed, when ten % 2 == 1, then ten % 10 cannot be 0,
            // independently of the other bits.
            // So we discard this combination
            if ten % 2 != 0 {
                errors += 1;
                if errors > max_errors {
                    return false;
                }
            }
        }

        // This seed wasn't discarted, so we keep it
        true
    })
}

/// Find all the seeds that have this slime chunks. Params:
/// * slimechunks: list of slime chunks
/// * max_errors: error margin, 0 means you're 100% sure that the slime chunks are correct
/// * noslimechunks: list of chunks that aren't slime chunks
/// * max_noerrors: error margin but for noslimechunks, I'm bad at naming stuff
pub fn seed_from_slime_chunks(
    slimechunks: &[Chunk],
    max_errors: usize,
    noslimechunks: &[Chunk],
    max_noerrors: usize,
) -> Vec<u64> {
    let sc = SlimeChunks::new(slimechunks, max_errors, noslimechunks, max_noerrors);

    info!("Found {} 18-bit candidates", sc.low_18_candidates.len());

    sc.find_seed()
}

/// Given a list of seed candidates, try all of these
pub fn seed_from_slime_chunks_and_candidates(
    slimechunks: &[Chunk],
    max_errors: usize,
    noslimechunks: &[Chunk],
    max_noerrors: usize,
    candidates: Vec<u64>,
) -> Vec<u64> {
    let sc = SlimeChunks::new(slimechunks, max_errors, noslimechunks, max_noerrors);

    candidates
        .into_iter()
        .filter(|&seed| sc.try_seed(seed))
        .collect()
}

/// Generate a Map where slime chunks are set to 1 and non slime chunks are set to 0
pub fn gen_map_from_seed(area: Area, seed: u64) -> Map {
    let mut m = Map::new(area);
    for i in 0..area.w as usize {
        for j in 0..area.h as usize {
            m.a[(i, j)] = is_slime_chunk(
                seed,
                &Chunk::new(area.x as i32 + i as i32, area.z as i32 + j as i32),
            ) as i32;
        }
    }

    m
}

/// Generate a list of slime chunks and not slime chunks using the given seed
pub fn generate_slime_chunks_and_not(
    seed: i64,
    limit_yes: usize,
    limit_no: usize,
) -> (Vec<Chunk>, Vec<Chunk>) {
    let mut vy = Vec::with_capacity(limit_yes);
    let mut vn = Vec::with_capacity(limit_no);
    for x in 0.. {
        // yeah just go on forever
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
    use super::*;

    #[test]
    fn slime_data() {
        // This was compared against java so it should be ok
        // (assuming I got the right algorythm)
        let e = slime_const::E;

        let c = Chunk::new(0, 0);
        let a = calculate_slime_data(&c);
        assert_eq!(a ^ e, 987234911);
        let c = Chunk::new(1, 1);
        let a = calculate_slime_data(&c);
        assert_eq!(a ^ e, 976736648);
        let c = Chunk::new(0, 1_000);
        let a = calculate_slime_data(&c);
        assert_eq!(a ^ e, 4393087172103);
        let c = Chunk::new(1_000, 0);
        let a = calculate_slime_data(&c);
        assert_eq!(a ^ e, 2978817703);
        let c = Chunk::new(1_000, 1_000);
        let a = calculate_slime_data(&c);
        assert_eq!(a ^ e, 4395174175503);
        let c = Chunk::new(1_000_000, 1_000_000);
        let a = calculate_slime_data(&c);
        assert_eq!((a ^ e) as i64, -3195288407282465);
    }

    #[test]
    fn slime_chunks() {
        let s = 0xbade12;
        let mut chunks = vec![];
        chunks.push(Chunk::new(0, -2));
        chunks.push(Chunk::new(1, 0));
        chunks.push(Chunk::new(1, 1));
        chunks.push(Chunk::new(2, 2));
        chunks.push(Chunk::new(-4, 1));
        chunks.push(Chunk::new(-5, 2));
        chunks.push(Chunk::new(-5, -3));
        chunks.push(Chunk::new(-5, -2));
        chunks.push(Chunk::new(4, -2));
        chunks.push(Chunk::new(5, 0));
        chunks.push(Chunk::new(1, -5));
        chunks.push(Chunk::new(3, -5));
        chunks.push(Chunk::new(5, 4));
        chunks.push(Chunk::new(7, 4));
        chunks.push(Chunk::new(8, 5));
        chunks.push(Chunk::new(8, 6));
        chunks.push(Chunk::new(11, 7));
        chunks.push(Chunk::new(12, 7));

        for c in &chunks {
            if !is_slime_chunk(s, c) {
                panic!("{:?} is not a slime chunk", c);
            }
        }
    }

    // Wtf, the z is the only variable that affects
    // the high bits [64, 32] of the slimedata
    // ^ Incorrect, if the result of (a+b+d) is negative, instead of
    // (c >> 32) we have ((c >> 32) - 1): 0x0001LLLLLLLL into 0x0000LLLLLLLL
    // Of course, only 48 bits are used
    // BUT if z is small, z * z * C only uses 32 bits anyway, up to
    // z = 22 (uses 31 bits)
    // z = 31 (uses 32 bits)
    // z = 44 (uses 33 bits)
    // z = 8004 (uses 48 bits)
    // z = floor(sqrt(2^nbits / slime_const::C));
    // Anyway, we can just check if the higher n bits are equal, then what?
    // The higher n bits of the rng_with_slime_data will be the same, but the
    // call to next() will set them to different values.
    #[test]
    fn constant_z() {
        // Is there any possible optimization if the slime chunks are
        // on the z axis?
        let bits_47_33 = 0xFFFE00000000;
        let seed = 1;
        let z = 0;
        let mut the_same = None;
        for x in 0..10 {
            let c = Chunk::new(x, z);
            let a = calculate_slime_data(&c);
            let r = rng_with_slime_data(seed, a);
            let hbits = r.get_seed() & bits_47_33;
            // For z=0, the high bits can be 0x0000 or 0xFFFE, we want
            // to consider both cases valid so we set 0xFFFE to 0
            let hbits = if hbits == bits_47_33 { 0 } else { hbits };

            if the_same.is_none() {
                the_same = Some(hbits);
            }
            assert_eq!(hbits, the_same.unwrap());
            //r.next(31);
            //println!("{:012X}", r.get_raw_seed());
        }
    }
}
