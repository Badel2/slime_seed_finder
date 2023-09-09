#![no_main]
use libfuzzer_sys::fuzz_target;
use libfuzzer_sys::arbitrary;
use arbitrary::Arbitrary;
use slime_seed_finder::population::chunk_population_seed_to_world_seed;
use slime_seed_finder::population::world_seed_to_chunk_population_seed_1_13;
use slime_seed_finder::population::world_seed_to_chunk_population_seed;
use slime_seed_finder::population::round_to_odd;
use slime_seed_finder::java_rng::JavaRng;
use std::collections::HashMap;

#[derive(Debug, Arbitrary)]
struct FuzzData {
    version: u8,
    world_seed: i64,
    chunk_x: i32,
    chunk_z: i32,
}

pub fn world_seed_to_chunk_population_seed_high_bits_undefined(world_seed: i64, chunk_x: i32, chunk_z: i32) -> Vec<u64> {
    let mut r = JavaRng::with_seed(world_seed as u64);
    let rm = r.next_long() as i64;
    let rn = r.next_long() as i64;
    let mut r = JavaRng::with_seed(world_seed as u64);

    let m = round_to_odd(r.next_long() as i64);
    let n = round_to_odd(r.next_long() as i64);
    //println!("world_seed={world_seed:064b}, rm={rm:064b}, m={m:064b}, rn={rn:064b}, n={n:064b}");

    // (x * m + z * n) ^ world_seed
    let first = (((chunk_x as i64)
        .wrapping_mul(m)
        .wrapping_add((chunk_z as i64).wrapping_mul(n)))
        ^ world_seed) as u64;

    let rm_is_even = (rm & 1) == 0;
    let rn_is_even = (rn & 1) == 0;
    // Possible return len: 1, 2, 4
    let mut num_candidates = 1;
    if !rm_is_even {
        num_candidates = num_candidates << 1;
    }
    if !rn_is_even {
        num_candidates = num_candidates << 1;
    }
    let mut v = Vec::with_capacity(num_candidates);

    for dm in [-2, 0, 2] {
        if dm != 0 && rm_is_even {
            continue;
        }
        let rm_is_negative = rm < 0;
        if rm_is_negative {
            if dm == 2 {
                continue;
            }
        } else {
            if dm == -2 {
                continue;
            }
        }
        for dn in [-2, 0, 2] {
            if dn != 0 && rn_is_even {
                continue;
            }
            let rn_is_negative = rn < 0;
            if rn_is_negative {
                if dn == 2 {
                    continue;
                }
            } else {
                if dn == -2 {
                    continue;
                }
            }
            let diff = (chunk_x as i64).wrapping_mul(dm).wrapping_add((chunk_z as i64).wrapping_mul(dn));
            let next = ((first as i64 ^ world_seed).wrapping_add(diff)) ^ world_seed;
            v.push(next as u64);
        }
    }

    v
}

fuzz_target!(|data: FuzzData| {
    if data.version != 0 {
        return;
    }
    let chunk_x = data.chunk_x;
    let chunk_z = data.chunk_z;
    let world_seed = data.world_seed;
    let cs = world_seed_to_chunk_population_seed(world_seed, chunk_x, chunk_z);

    let first_step_bits = 32;
    let known_bits = 17;
    let mask_bits = first_step_bits - known_bits;
    let mask_in = (1 << first_step_bits) - 1;
    let mask_out = (1 << mask_bits) - 1;

    /*
    let cs1 = world_seed_to_chunk_population_seed(world_seed & mask_in, chunk_x, chunk_z);
    assert_eq!(cs & mask_out, cs1 & mask_out);
    */
    let mut cs1 = world_seed_to_chunk_population_seed_high_bits_undefined(world_seed & mask_in, chunk_x, chunk_z);
    for cc in &mut cs1 {
        *cc = *cc & mask_out;
    }
    assert!(cs1.contains(&(cs & mask_out)), "{:08X} not in {:08X?}", cs & mask_out, cs1);

    /*
    // Assert that there is only one possibility if we use 33 bits
    let mask_in2 = (1 << (first_step_bits + 1)) - 1;
    let mask_out2 = (1 << (mask_bits + 1)) - 1;
    let mut cs2a = world_seed_to_chunk_population_seed_high_bits_undefined(world_seed & mask_in2, chunk_x, chunk_z);
    let mut ws = world_seed ^ (1 << 32);
    let mut cs2b = world_seed_to_chunk_population_seed_high_bits_undefined(ws & mask_in2, chunk_x, chunk_z);

    assert_eq!(cs2a.len(), cs2b.len());
    
    for i in 0..cs2a.len() {
        for j in 0..cs2b.len() {
            assert_ne!(cs2a[i] & mask_out2, cs2b[j] & mask_out2);
        }
    }
    */

    // Assert that this is the only 48-bit world seed that produces this chunk seed
    let mut hist: HashMap<u64, u32> = HashMap::new();
    for high_bits in 0..(1 << 16) {
        let mask_48 = (1u64 << 48) - 1;
        let ws = (world_seed & mask_in) | (high_bits << first_step_bits);
        let cs2 = world_seed_to_chunk_population_seed_high_bits_undefined(ws & (mask_48 as i64), chunk_x, chunk_z);
        for c in cs2 {
            *hist.entry(c & mask_48).or_default() += 1;
        }
    }

    let max_candidates = 256;
    let mut hist: Vec<_> = hist.into_iter().filter_map(|(cand, count)| if count <= max_candidates { None } else { Some((cand, count)) }).collect();
    hist.sort_by_key(|(cand, count)| (*count, !*cand));
    hist.reverse();
    assert_eq!(hist, vec![]);
});
