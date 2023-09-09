#![no_main]
use libfuzzer_sys::fuzz_target;
use libfuzzer_sys::arbitrary;
use arbitrary::Arbitrary;
use slime_seed_finder::anvil::find_multi_spawners;
use std::collections::HashMap;

#[derive(Debug, Arbitrary)]
struct FuzzData {
    pos: (i32, i32, i32),
    offsets: Vec<(i8, i8, i8)>,
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

/// Return the distance between 2 3D points, squared.
/// This is useful because comparing distances can be done faster if comparing the distances
/// squared, as we avoid sqrt operations.
fn distance3dsquared(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> f64 {
    let x = (a.0 as f64 - b.0 as f64);
    let y = (a.1 as f64 - b.1 as f64);
    let z = (a.2 as f64 - b.2 as f64);

    x * x + y * y + z * z
}

fuzz_target!(|data: FuzzData| {
    pretty_env_logger::try_init();
    log::debug!("{:?}", data);

    if data.offsets.len() >= 100 {
        return;
    }
    if !all_unique(data.offsets.iter()) {
        return;
    }
    let all_dungeons: Vec<_> = data.offsets.iter().map(|(dx, dy, dz)| {
        (((data.pos.0 as i64).saturating_add(*dx as i64), (data.pos.1 as i64).saturating_add(*dy as i64), (data.pos.2 as i64).saturating_add(*dz as i64)), "".to_string())
    }).collect();

    // Ensure that distance between initial pos and all dungeons is less than activation radius
    // (we want all dungeons to be active from pos)
    let max_distance_squared = (16) * (16);
    for d in all_dungeons.iter() {
        if distance3dsquared(&(data.pos.0 as i64, data.pos.1 as i64, data.pos.2 as i64), &d.0) >= max_distance_squared as f64 {
            return;
        }
    }

    log::debug!("all_dungeons: {:?}", all_dungeons);
    let out = find_multi_spawners(all_dungeons);
    log::debug!("out: {:?}", out);

    if data.offsets.len() <= 1 {
        assert_eq!(out, vec![]);
        return;
    }

    assert_eq!(out.len(), 1);
    assert_eq!(out[0].spawners.len(), data.offsets.len());
});
