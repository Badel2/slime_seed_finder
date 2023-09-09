#![no_main]
use libfuzzer_sys::fuzz_target;
use libfuzzer_sys::arbitrary;
use arbitrary::Arbitrary;
use slime_seed_finder::population::chunk_population_seed_to_world_seed;
use slime_seed_finder::population::world_seed_to_chunk_population_seed_1_13;
use slime_seed_finder::population::world_seed_to_chunk_population_seed;
use std::collections::HashMap;

#[derive(Debug, Arbitrary)]
struct FuzzData {
    version: u8,
    world_seed: i64,
    chunk_x: i32,
    chunk_z: i32,
}

fuzz_target!(|data: FuzzData| {
    if data.version != 0 {
        return;
    }
    let mut bits_in_common = HashMap::new();
    let chunk_x = data.chunk_x;
    let chunk_z = data.chunk_z;
    let world_seed = data.world_seed;
    let cs = world_seed_to_chunk_population_seed(world_seed, chunk_x, chunk_z);

    let mut ws = data.world_seed;
    for b in (0..=63).rev() {
        // Flip bit
        ws ^= (1 << b);
        // Calculate chunk seed again
        let cs1 = world_seed_to_chunk_population_seed(ws, chunk_x, chunk_z);
        bits_in_common.insert(b, cs1);
    }

    let mut bits_in_common: Vec<_> = bits_in_common.into_iter().collect();
    bits_in_common.sort();

    for (b, cs1) in bits_in_common.iter().rev() {
        let b = *b;
        let cs1 = *cs1;
        // Get number of unchanged trailing bits
        let common = (cs ^ cs1).trailing_zeros();
        if b >= 48 {
            assert_eq!(b, common);
        } else {
            //assert!(b.saturating_sub(17) <= common, "b: {}, common: {}", b, common);
            if b.saturating_sub(17) > common {
                // The difference in m and n must be at most +/-2
                // x*m + z*n
                // x*-2 + z*-2
                // x*0 + z*-2
                // x*+2 + z*-2
                // x*-2 + z*0
                // x*0 + z*0
                // x*+2 + z*0
                // x*-2 + z*+2
                // x*0 + z*+2
                // x*+2 + z*+2
                let mut possible_diffs = vec![];
                for m in [-2, 0, 2] {
                    for n in [-2, 0, 2] {
                        if m == 0 && n == 0 {
                            continue;
                        }
                        possible_diffs.push((chunk_x as i64).wrapping_mul(m).wrapping_add((chunk_z as i64).wrapping_mul(n)));
                    }
                }
                let expected_b = b.saturating_sub(17);
                let mask = (1 << expected_b) - 1;
                let diff = ((cs ^ world_seed as u64) & mask).wrapping_sub((cs1 ^ world_seed as u64) & mask);

                for d in &mut possible_diffs {
                    *d = *d & (mask as i64);
                }
                let diff_i64 = (diff as i64) & (mask as i64);
                assert!(possible_diffs.contains(&diff_i64), "Unexpected diff: {:08X} not in {:08X?}", diff, possible_diffs);
                // Hit!
                // Possible positions: all of them, but not sure if it depends on the world seed,
                // on chunk coordinates, or on both
                //println!("Hit with pos {}", possible_diffs.iter().position(|x| *x == diff_i64).unwrap());
            } else {
                // At least b-17 common bits, good
            }
        }
    }

    //panic!("bits in common: {:?}", bits_in_common);
});
