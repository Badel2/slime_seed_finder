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

fn integer_counter_step(limits: (u64, u64, u64), step: u64) -> Vec<(u64, u64, u64)> {
    let mut res = vec![];
    let seed = (0, 100, 10000);
    let increment = |x| x + 1;
    let gen = GenPairs3L::new_one_step(seed, increment, limits, step);

    for x in gen {
        res.push(x);
    }

    res
}

fn integer_counter_all_steps(limits: (u64, u64, u64)) -> Vec<(u64, u64, u64)> {
    let mut res = vec![];
    let max_limit = std::cmp::max(std::cmp::max(limits.0, limits.1), limits.2);
    for step in 0..max_limit {
        res.extend(integer_counter_step(limits, step));
    }

    res
}

fuzz_target!(|data: &[u8]| {
    if data.len() < 3 {
        return;
    }
    let limits = (data[0] as u64, data[1] as u64, data[2] as u64);
    let res1 = integer_counter(limits);
    let res2 = integer_counter_all_steps(limits);
    assert_eq!(res1, res2);
});
