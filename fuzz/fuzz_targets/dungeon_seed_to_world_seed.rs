#![no_main]
use libfuzzer_sys::fuzz_target;
use libfuzzer_sys::arbitrary;
use arbitrary::Arbitrary;
use slime_seed_finder::population::chunk_population_seed_to_world_seed;
use slime_seed_finder::population::world_seed_to_chunk_population_seed_1_13;
use slime_seed_finder::population::world_seed_to_chunk_population_seed;

#[derive(Debug, Arbitrary)]
struct FuzzData {
    world_seed: i64,
    chunks: [(u8, i32, i32, u16); 3],
}

// Returns true if all the elements of a slice are different.
// Do not use with large slices.
fn all_unique<T: PartialEq, I: Iterator<Item = T> + Clone>(a: I) -> bool {
    let a1 = a.clone();
    for (i, x) in a1.enumerate() {
        let a2 = a.clone();
        for y in a2.skip(i + 1) {
            if x == y {
                return false;
            }
        }
    }

    true
}

// TODO: this is very slow on some inputs like
// FuzzData { world_seed: 13056, chunks: [(0, 4137, 0, 47), (0, 0, 46080, 4), (0, 65535, 0, 33)] }

fuzz_target!(|data: FuzzData| {
    if !all_unique(data.chunks.iter().map(|(version, chunk_x, chunk_z, rng_steps)| (chunk_x, chunk_z))) {
        return;
    }

    println!("{:?}", data);

    let mut chunk_seeds = vec![];

    for (version, chunk_x, chunk_z, rng_steps) in data.chunks {
        // Start small
        if rng_steps < 3 || rng_steps > 100 {
            return;
        }
        if version == 0 {
            let cs = world_seed_to_chunk_population_seed(data.world_seed, chunk_x, chunk_z);
            chunk_seeds.push(cs);
        } else if version == 1 {
            let cs = world_seed_to_chunk_population_seed_1_13(data.world_seed, chunk_x, chunk_z);
            chunk_seeds.push(cs);
        } else {
            return;
        }
    }

    let a0 = (chunk_seeds[0], data.chunks[0].1, data.chunks[0].2);
    let a1 = (chunk_seeds[1], data.chunks[1].1, data.chunks[1].2);
    let a2 = (chunk_seeds[2], data.chunks[2].1, data.chunks[2].2);

    let world_seeds = chunk_population_seed_to_world_seed(
        a0, a1, a2
    );

    assert!(world_seeds.contains(&data.world_seed), "{:?}", world_seeds);
    assert_eq!(world_seeds.len(), 1);
});
