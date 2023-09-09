#![no_main]
use libfuzzer_sys::fuzz_target;
use libfuzzer_sys::arbitrary;
use arbitrary::Arbitrary;
use slime_seed_finder::anvil::find_multi_spawners;
use std::collections::HashMap;

#[derive(Debug, Arbitrary)]
struct FuzzData {
    spawners: Vec<(i64, i64, i64)>,
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

fuzz_target!(|data: FuzzData| {
    if data.spawners.len() >= 100 {
        return;
    }
    if !all_unique(data.spawners.iter()) {
        return;
    }
    let all_dungeons = data.spawners.iter().map(|x| {
        (*x, "".to_string())
    }).collect();
    let out = find_multi_spawners(all_dungeons);
    // Checks that find_multi_spawners never panics
});
