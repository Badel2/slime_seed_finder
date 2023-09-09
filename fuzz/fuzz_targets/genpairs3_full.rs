#![no_main]
use libfuzzer_sys::fuzz_target;
use slime_seed_finder::gen_pairs3::GenPairs3L;

fn integer_counter(limits: (u64, u64, u64)) -> Vec<(u64, u64, u64)> {
    let mut res = vec![];
    let seed = (0, 100, 10000);
    let increment = |x| x + 1;
    let gen = GenPairs3L::new(seed, increment, limits);

    for x in gen {
        res.push(x);
    }

    res
}

fn naive_all_pairs3(limits: (u64, u64, u64)) -> Vec<(u64, u64, u64)> {
    let mut res = vec![];

    for i in 0..limits.0 {
        for j in 0..limits.1 {
            for k in 0..limits.2 {
                res.push((0 + i, 100 + j, 10000 + k));
            }
        }
    }

    res
}

fuzz_target!(|data: &[u8]| {
    if data.len() < 3 {
        return;
    }
    let limits = (data[0] as u64, data[1] as u64, data[2] as u64);
    let mut res1 = integer_counter(limits);
    let res2 = naive_all_pairs3(limits);
    res1.sort();
    assert_eq!(res1, res2);
});
