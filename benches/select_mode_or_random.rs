//! This is an experiment to see if rewriting select_mode_or_random in a
//! different way improves performance.
//!
//! This is a sample run of the benchmarks after commit f50f05c
//!
//! The version used in actual code ([McRng::select_mode_or_random]) is the one
//! named "_the_one_used".
//!
//! ```norun
//! test select_mode_or_random_all_arith             ... bench:       1,074 ns/iter (+/- 51)
//! test select_mode_or_random_all_arith2            ... bench:       1,154 ns/iter (+/- 166)
//! test select_mode_or_random_all_arith3            ... bench:       1,139 ns/iter (+/- 70)
//! test select_mode_or_random_all_arith_lut         ... bench:       1,271 ns/iter (+/- 203)
//! test select_mode_or_random_all_arith_lut2        ... bench:       1,309 ns/iter (+/- 105)
//! test select_mode_or_random_all_arraymap          ... bench:       4,431 ns/iter (+/- 345)
//! test select_mode_or_random_all_arraymap_unrolled ... bench:       1,577 ns/iter (+/- 96)
//! test select_mode_or_random_all_branchless        ... bench:       1,372 ns/iter (+/- 95)
//! test select_mode_or_random_all_btreemap          ... bench:      35,900 ns/iter (+/- 1,092)
//! test select_mode_or_random_all_hashmap           ... bench:      32,347 ns/iter (+/- 707)
//! test select_mode_or_random_all_nested            ... bench:       1,495 ns/iter (+/- 92)
//! test select_mode_or_random_all_nested2           ... bench:       1,921 ns/iter (+/- 123)
//! test select_mode_or_random_all_original          ... bench:       1,389 ns/iter (+/- 64)
//! test select_mode_or_random_all_the_one_used      ... bench:       1,283 ns/iter (+/- 24)
//! test select_mode_or_random_all_vecmap            ... bench:      10,338 ns/iter (+/- 121)
//! ```

#![feature(test)]
extern crate test;

use self::test::Bencher;
use slime_seed_finder::mc_rng::McRng;

fn select_mode_or_random_original(r: &mut McRng, a: i32, a1: i32, b: i32, b1: i32) -> i32 {
    if a1 == b && b == b1 {
        a1
    } else if a == a1 && a == b {
        a
    } else if a == a1 && a == b1 {
        a
    } else if a == b && a == b1 {
        a
    } else if a == a1 && b != b1 {
        a
    } else if a == b && a1 != b1 {
        a
    } else if a == b1 && a1 != b {
        a
    } else if a1 == b && a != b1 {
        a1
    } else if a1 == b1 && a != b {
        a1
    } else if b == b1 && a != a1 {
        b
    } else {
        r.choose4(a, a1, b, b1)
    }
}

fn select_mode_or_random_nested(r: &mut McRng, a: i32, a1: i32, b: i32, b1: i32) -> i32 {
    if a == a1 {
        if a == b {
            a
        } else if b == b1 {
            r.choose4(a, a1, b, b1)
        } else {
            a
        }
    } else if a == b {
        if a == b1 {
            a
        } else if a1 == b1 {
            r.choose4(a, a1, b, b1)
        } else {
            a
        }
    } else if a == b1 {
        if a1 == b {
            r.choose4(a, a1, b, b1)
        } else {
            a
        }
    } else if a1 == b {
        a1
    } else if a1 == b1 {
        a1
    } else if b == b1 {
        b
    } else {
        r.choose4(a, a1, b, b1)
    }
}

fn select_mode_or_random_nested2(r: &mut McRng, a: i32, a1: i32, b: i32, b1: i32) -> i32 {
    if b != b1 {
        if a == a1 {
            return a;
        }
        let ca = a == b || a == b1;
        let ca1 = a1 == b || a1 == b1;
        if ca == ca1 {
            r.choose4(a, a1, b, b1)
        } else if ca1 {
            a1
        } else {
            a
        }
    } else {
        if a == b {
            a
        } else if a == a1 {
            r.choose4(a, a1, b, b1)
        } else {
            b
        }
    }
}

fn select_mode_or_random_arith(r: &mut McRng, a: i32, a1: i32, b: i32, b1: i32) -> i32 {
    let ca = u8::from(a == a1) + u8::from(a == b) + u8::from(a == b1);
    // ca1 doesnt need +, | is enough but slower
    let ca1 = u8::from(a1 == b) + u8::from(a1 == b1);
    let cb = u8::from(b == b1);

    if ca > ca1 && ca > cb {
        a
    } else if ca1 > ca {
        a1
    } else if cb > ca {
        b
    } else {
        r.choose4(a, a1, b, b1)
    }
}

fn select_mode_or_random_arith2(r: &mut McRng, a: i32, a1: i32, b: i32, b1: i32) -> i32 {
    let ca = i8::from(a == a1) + i8::from(a == b) + i8::from(a == b1);
    if ca > 1 {
        return a;
    }
    let ca1 = i8::from(a1 == b) + i8::from(a1 == b1) - ca;
    let cb = i8::from(b == b1) - ca;

    if ca1 < 0 && cb < 0 {
        a
    } else if ca1 > 0 {
        a1
    } else if cb > 0 {
        b
    } else {
        r.choose4(a, a1, b, b1)
    }
}

fn select_mode_or_random_arith3(r: &mut McRng, a: i32, a1: i32, b: i32, b1: i32) -> i32 {
    // If ca == 1, the result can only be "a" or random
    // There are only 10 possible combinations of (ca, ca1, cb):
    /*
    (0, 0, 0) => r
    (0, 0, 1) => b
    (0, 1, 0) => a1
    (0, 2, 1) => a1, b
    (1, 0, 0) => a
    (1, 0, 1) => r
    (1, 1, 0) => r
    (2, 0, 1) => a, b
    (2, 1, 0) => a, a1
    (3, 2, 1) => a, a1, b, r
    */
    /*
    (0, X, 1) => b
    (0, 1, 0) => a1
    (0, 0, 0) => r
    (1, 0, 1) => r
    (1, 1, 0) => r
    (1, 0, 0) => a
    (2, 0, 1) => a, b
    (2, 1, 0) => a, a1
    (3, 2, 1) => a, a1, b, r
    */
    let ca = u8::from(a == a1) + u8::from(a == b) + u8::from(a == b1);
    if ca >= 2 {
        return a;
    }
    let ca1 = a1 == b || a1 == b1;
    let cb = b == b1;

    match (ca != 0, ca1, cb) {
        (false, _, true) => b,
        (true, false, false) => a,
        (false, true, false) => a1,
        _ => r.choose4(a, a1, b, b1),
    }
}

fn select_mode_or_random_arith_lut(r: &mut McRng, a: i32, a1: i32, b: i32, b1: i32) -> i32 {
    let ca = u8::from(a == a1) + u8::from(a == b) + u8::from(a == b1);
    let ca1 = u8::from(a1 == b) + u8::from(a1 == b1);
    let cb = u8::from(b == b1);

    let idx = ca | (ca1 << 2) | (cb << 4);

    let lut: u64 = 0b01_00_00_00_00_00_00_11_10_00_00_00_00_00_00_00_00_00_00_11_01_00_00_00_11;

    let lut_get = |i| ((lut >> (2 * i)) & 0x3) as u8;

    match lut_get(idx) {
        0 => a,
        1 => a1,
        2 => b,
        _ => r.choose4(a, a1, b, b1),
    }
}

fn select_mode_or_random_arith_lut2(r: &mut McRng, a: i32, a1: i32, b: i32, b1: i32) -> i32 {
    let ca = u8::from(a == a1) + u8::from(a == b) + u8::from(a == b1);
    let ca1 = u8::from(a1 == b) + u8::from(a1 == b1);
    let cb = u8::from(b == b1);

    let idx = ca | (ca1 << 2) | (cb << 4);

    match idx {
        0 | 5 | 17 => r.choose4(a, a1, b, b1),
        16 => b,
        4 | 24 => a1,
        _ => a,
    }
}

fn select_mode_or_random_branchless_cc(
    a0a1: bool,
    a0b0: bool,
    a0b1: bool,
    a1b0: bool,
    a1b1: bool,
    b0b1: bool,
) -> u8 {
    let idx = u8::from(a0a1)
        | (u8::from(a0b0) << 1)
        | (u8::from(a0b1) << 2)
        | (u8::from(a1b0) << 3)
        | (u8::from(a1b1) << 4)
        | (u8::from(b0b1) << 5);

    let lut: u128 = 0x100000000000E0000003103010003;

    let lut_get = |i| ((lut >> (2 * i)) & 0x3) as u8;

    lut_get(idx)
}

fn select_mode_or_random_branchless(r: &mut McRng, a: i32, a1: i32, b: i32, b1: i32) -> i32 {
    let i =
        select_mode_or_random_branchless_cc(a == a1, a == b, a == b1, a1 == b, a1 == b1, b == b1);
    // TODO: replace match to avoid branch?
    // No, we don't want to always call choose4
    // And this is slightly slower because of bounds checking:
    // the compiler doesn't know that lut_get always returns a safe index
    //[a, a1, b, self.choose4(a, a1, b, b1)][usize::from(i)]
    match i {
        0 => a,
        1 => a1,
        2 => b,
        _ => r.choose4(a, a1, b, b1),
    }
}

fn select_mode_or_random_hashmap(r: &mut McRng, a: i32, a1: i32, b: i32, b1: i32) -> i32 {
    use std::collections::HashMap;
    let mut h: HashMap<i32, u8> = HashMap::with_capacity(4);
    let mut incr = |k| {
        *h.entry(k).or_default() += 1;
    };

    incr(a);
    incr(a1);
    incr(b);
    incr(b1);

    let mut max_v = 0;
    let mut max_k = None;
    for (k, v) in h {
        if v == max_v {
            max_k = None;
        }
        if v > max_v {
            max_v = v;
            max_k = Some(k);
        }
    }

    max_k.unwrap_or_else(|| r.choose4(a, a1, b, b1))
}

fn select_mode_or_random_btreemap(r: &mut McRng, a: i32, a1: i32, b: i32, b1: i32) -> i32 {
    use std::collections::BTreeMap;
    let mut h: BTreeMap<i32, u8> = BTreeMap::new();
    let mut incr = |k| {
        *h.entry(k).or_default() += 1;
    };

    incr(a);
    incr(a1);
    incr(b);
    incr(b1);

    let mut max_v = 0;
    let mut max_k = None;
    for (k, v) in h {
        if v == max_v {
            max_k = None;
        }
        if v > max_v {
            max_v = v;
            max_k = Some(k);
        }
    }

    max_k.unwrap_or_else(|| r.choose4(a, a1, b, b1))
}

fn select_mode_or_random_vecmap(r: &mut McRng, a: i32, a1: i32, b: i32, b1: i32) -> i32 {
    let mut h: Vec<(i32, u8)> = Vec::with_capacity(4);
    let mut incr = |k| match h.iter().position(|x| x.0 == k) {
        Some(i) => h[i].1 += 1,
        None => h.push((k, 1)),
    };

    incr(a);
    incr(a1);
    incr(b);
    incr(b1);

    let mut max_v = 0;
    let mut max_k = None;
    for (k, v) in h {
        if v == max_v {
            max_k = None;
        }
        if v > max_v {
            max_v = v;
            max_k = Some(k);
        }
    }

    max_k.unwrap_or_else(|| r.choose4(a, a1, b, b1))
}

fn select_mode_or_random_arraymap(r: &mut McRng, a: i32, a1: i32, b: i32, b1: i32) -> i32 {
    let mut h: [(i32, u8); 4] = Default::default();
    let mut h_len = 0;
    let mut incr = |k| match h[..h_len].iter().position(|x| x.0 == k) {
        Some(i) => h[i].1 += 1,
        None => {
            h[h_len] = (k, 1);
            h_len += 1;
        }
    };

    incr(a);
    incr(a1);
    incr(b);
    incr(b1);

    let mut max_v = 0;
    let mut max_k = None;
    for &(k, v) in h.iter() {
        if v == max_v {
            max_k = None;
        }
        if v > max_v {
            max_v = v;
            max_k = Some(k);
        }
    }

    max_k.unwrap_or_else(|| r.choose4(a, a1, b, b1))
}

fn select_mode_or_random_arraymap_unrolled(r: &mut McRng, a: i32, a1: i32, b: i32, b1: i32) -> i32 {
    let mut ks: [i32; 4] = [a, a1, b, b1];
    let mut kv: [u8; 4] = [0, 0, 0, 0];

    if a == a1 {
        kv[0] += 1;
    }

    if a == b {
        kv[0] += 1;
    } else if a1 == b {
        kv[1] += 1;
    }

    if a == b1 {
        kv[0] += 1;
    } else if a1 == b1 {
        kv[1] += 1;
    } else if b == b1 {
        kv[2] += 1;
    }

    let mut max_v = 0;
    let mut max_k = Some(a);

    for i in 0..4 {
        let v = kv[i];
        let k = ks[i];
        if v == max_v {
            max_k = None;
        }
        if v > max_v {
            max_v = v;
            max_k = Some(k);
        }
    }

    max_k.unwrap_or_else(|| r.choose4(a, a1, b, b1))
}

// This is a test, not sure how to run tests in the benches folder
#[bench]
fn test_select_mode_or_random_equivalent(b: &mut Bencher) {
    let base_seed = 1000;
    let world_seed = 1234;
    b.iter(|| {
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        let mut mc0 = McRng::new(base_seed, world_seed);
                        let mut mc1 = McRng::new(base_seed, world_seed);
                        assert_eq!(
                            mc0.select_mode_or_random(a, b, c, d),
                            select_mode_or_random_original(&mut mc1, a, b, c, d),
                            "{:?}",
                            (a, b, c, d)
                        );
                        assert_eq!(mc0.chunk_seed(), mc1.chunk_seed());
                    }
                }
            }
        }
    });
}

#[bench]
fn select_mode_or_random_all_the_one_used(b: &mut Bencher) {
    let base_seed = 1000;
    let world_seed = 1234;
    b.iter(|| {
        let mut mc = McRng::new(base_seed, world_seed);
        let mut r = Vec::with_capacity(4 * 4 * 4 * 4);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        r.push(mc.select_mode_or_random(a, b, c, d));
                    }
                }
            }
        }
        r
    });
}

#[bench]
fn select_mode_or_random_all_original(b: &mut Bencher) {
    let base_seed = 1000;
    let world_seed = 1234;
    b.iter(|| {
        let mut mc = McRng::new(base_seed, world_seed);
        let mut r = Vec::with_capacity(4 * 4 * 4 * 4);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        r.push(select_mode_or_random_original(&mut mc, a, b, c, d));
                    }
                }
            }
        }
        r
    });
}

#[bench]
fn select_mode_or_random_all_nested(b: &mut Bencher) {
    let base_seed = 1000;
    let world_seed = 1234;
    b.iter(|| {
        let mut mc = McRng::new(base_seed, world_seed);
        let mut r = Vec::with_capacity(4 * 4 * 4 * 4);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        r.push(select_mode_or_random_nested(&mut mc, a, b, c, d));
                    }
                }
            }
        }
        r
    });
}

#[bench]
fn select_mode_or_random_all_nested2(b: &mut Bencher) {
    let base_seed = 1000;
    let world_seed = 1234;
    b.iter(|| {
        let mut mc = McRng::new(base_seed, world_seed);
        let mut r = Vec::with_capacity(4 * 4 * 4 * 4);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        r.push(select_mode_or_random_nested2(&mut mc, a, b, c, d));
                    }
                }
            }
        }
        r
    });
}

#[bench]
fn select_mode_or_random_all_arith(b: &mut Bencher) {
    let base_seed = 1000;
    let world_seed = 1234;
    b.iter(|| {
        let mut mc = McRng::new(base_seed, world_seed);
        let mut r = Vec::with_capacity(4 * 4 * 4 * 4);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        r.push(select_mode_or_random_arith(&mut mc, a, b, c, d));
                    }
                }
            }
        }
        r
    });
}

#[bench]
fn select_mode_or_random_all_arith2(b: &mut Bencher) {
    let base_seed = 1000;
    let world_seed = 1234;
    b.iter(|| {
        let mut mc = McRng::new(base_seed, world_seed);
        let mut r = Vec::with_capacity(4 * 4 * 4 * 4);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        r.push(select_mode_or_random_arith2(&mut mc, a, b, c, d));
                    }
                }
            }
        }
        r
    });
}

#[bench]
fn select_mode_or_random_all_arith3(b: &mut Bencher) {
    let base_seed = 1000;
    let world_seed = 1234;
    b.iter(|| {
        let mut mc = McRng::new(base_seed, world_seed);
        let mut r = Vec::with_capacity(4 * 4 * 4 * 4);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        r.push(select_mode_or_random_arith3(&mut mc, a, b, c, d));
                    }
                }
            }
        }
        r
    });
}

#[bench]
fn select_mode_or_random_all_arith_lut(b: &mut Bencher) {
    let base_seed = 1000;
    let world_seed = 1234;
    b.iter(|| {
        let mut mc = McRng::new(base_seed, world_seed);
        let mut r = Vec::with_capacity(4 * 4 * 4 * 4);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        r.push(select_mode_or_random_arith_lut(&mut mc, a, b, c, d));
                    }
                }
            }
        }
        r
    });
}

#[bench]
fn select_mode_or_random_all_arith_lut2(b: &mut Bencher) {
    let base_seed = 1000;
    let world_seed = 1234;
    b.iter(|| {
        let mut mc = McRng::new(base_seed, world_seed);
        let mut r = Vec::with_capacity(4 * 4 * 4 * 4);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        r.push(select_mode_or_random_arith_lut2(&mut mc, a, b, c, d));
                    }
                }
            }
        }
        r
    });
}

#[bench]
fn select_mode_or_random_all_branchless(b: &mut Bencher) {
    let base_seed = 1000;
    let world_seed = 1234;
    b.iter(|| {
        let mut mc = McRng::new(base_seed, world_seed);
        let mut r = Vec::with_capacity(4 * 4 * 4 * 4);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        r.push(select_mode_or_random_branchless(&mut mc, a, b, c, d));
                    }
                }
            }
        }
        r
    });
}

#[bench]
fn select_mode_or_random_all_hashmap(b: &mut Bencher) {
    let base_seed = 1000;
    let world_seed = 1234;
    b.iter(|| {
        let mut mc = McRng::new(base_seed, world_seed);
        let mut r = Vec::with_capacity(4 * 4 * 4 * 4);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        r.push(select_mode_or_random_hashmap(&mut mc, a, b, c, d));
                    }
                }
            }
        }
        r
    });
}

#[bench]
fn select_mode_or_random_all_btreemap(b: &mut Bencher) {
    let base_seed = 1000;
    let world_seed = 1234;
    b.iter(|| {
        let mut mc = McRng::new(base_seed, world_seed);
        let mut r = Vec::with_capacity(4 * 4 * 4 * 4);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        r.push(select_mode_or_random_btreemap(&mut mc, a, b, c, d));
                    }
                }
            }
        }
        r
    });
}

#[bench]
fn select_mode_or_random_all_vecmap(b: &mut Bencher) {
    let base_seed = 1000;
    let world_seed = 1234;
    b.iter(|| {
        let mut mc = McRng::new(base_seed, world_seed);
        let mut r = Vec::with_capacity(4 * 4 * 4 * 4);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        r.push(select_mode_or_random_vecmap(&mut mc, a, b, c, d));
                    }
                }
            }
        }
        r
    });
}

#[bench]
fn select_mode_or_random_all_arraymap(b: &mut Bencher) {
    let base_seed = 1000;
    let world_seed = 1234;
    b.iter(|| {
        let mut mc = McRng::new(base_seed, world_seed);
        let mut r = Vec::with_capacity(4 * 4 * 4 * 4);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        r.push(select_mode_or_random_arraymap(&mut mc, a, b, c, d));
                    }
                }
            }
        }
        r
    });
}

#[bench]
fn select_mode_or_random_all_arraymap_unrolled(b: &mut Bencher) {
    let base_seed = 1000;
    let world_seed = 1234;
    b.iter(|| {
        let mut mc = McRng::new(base_seed, world_seed);
        let mut r = Vec::with_capacity(4 * 4 * 4 * 4);
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    for d in 0..4 {
                        r.push(select_mode_or_random_arraymap_unrolled(&mut mc, a, b, c, d));
                    }
                }
            }
        }
        r
    });
}
