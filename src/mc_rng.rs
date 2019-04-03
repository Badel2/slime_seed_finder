use std::num::Wrapping;

// The constants used by the Linear Congruential Generator
pub mod mc_lcg_const {
    pub const A: i64 = 6364136223846793005;
    pub const C: i64 = 1442695040888963407;
}

fn mask_up_to_bit(i: u8) -> i64 {
    (!0u64 >> (63 - i)) as i64
}

// This appears to be a QCG with a variable parameter k
// http://statmath.wu.ac.at/prng/doc/prng.html#QCG
// s = A*s*s + C*s + k (mod 2^64)
// The constants A and C are from Knuth's MMIX PRNG
#[derive(Copy, Clone, Debug, Default)]
pub struct McRng {
    base_seed: i64, // known
    world_seed: i64, // unknown
    chunk_seed: i64, // depends on world_seed
}

impl McRng {
    pub fn new(base_seed: i64, world_seed: i64) -> Self {
        let mut r: Self = Default::default();
        r.set_base_seed(base_seed);
        r.set_world_seed(world_seed);

        r
    }
    pub fn set_base_seed(&mut self, base_seed: i64) {
        let seed = base_seed as i64;
        self.base_seed = seed;
        self.base_seed = Self::next_state(self.base_seed, seed);
        self.base_seed = Self::next_state(self.base_seed, seed);
        self.base_seed = Self::next_state(self.base_seed, seed);
        self.world_seed = 0;
        self.chunk_seed = 0;
    }
    pub fn set_world_seed(&mut self, world_seed: i64) {
        self.world_seed = world_seed as i64;
        self.world_seed = Self::next_state(self.world_seed, self.base_seed);
        self.world_seed = Self::next_state(self.world_seed, self.base_seed);
        self.world_seed = Self::next_state(self.world_seed, self.base_seed);
    }
    pub fn set_chunk_seed(&mut self, chunk_x: i64, chunk_z: i64) {
        self.chunk_seed = self.world_seed;
        self.chunk_seed = Self::next_state(self.chunk_seed, chunk_x as i64);
        self.chunk_seed = Self::next_state(self.chunk_seed, chunk_z as i64);
        self.chunk_seed = Self::next_state(self.chunk_seed, chunk_x as i64);
        self.chunk_seed = Self::next_state(self.chunk_seed, chunk_z as i64);
    }
    pub fn base_seed(&self) -> i64 {
        self.base_seed as i64
    }
    pub fn world_seed(&self) -> i64 {
        self.world_seed as i64
    }
    pub fn chunk_seed(&self) -> i64 {
        self.chunk_seed as i64
    }
    // s *= s * A + C; s += k;
    // A*s*s + C*s + k = s
    // Is it possible that this operation will leave the state unchanged?
    // Yes (see state_unchanged test), but how would that be useful?
    pub fn next_state(s: i64, k: i64) -> i64
    {
        let s: Wrapping<i64> = Wrapping(s);
        let k: Wrapping<i64> = Wrapping(k);

        (s * (s * Wrapping(mc_lcg_const::A) + Wrapping(mc_lcg_const::C)) + k).0
    }
    pub fn next_int_n(&mut self, n: i32) -> i32 {
        let mut ret = ((self.chunk_seed >> 24) % (n as i64)) as i32;

        // Java % is not the same as C %, this is needed in Java because we
        // do not want negative results:
        if ret < 0 {
            ret += n;
        }

        self.chunk_seed = Self::next_state(self.chunk_seed, self.world_seed);

        ret
    }
    pub fn choose2<T>(&mut self, a: T, b: T) -> T {
        match self.next_int_n(2) {
            0 => a,
            1 => b,
            _ => unreachable!()
        }
    }
    pub fn choose4<T>(&mut self, a: T, b: T, c: T, d: T) -> T {
        match self.next_int_n(4) {
            0 => a,
            1 => b,
            2 => c,
            3 => d,
            _ => unreachable!()
        }
    }
    // Used by MapZoom
    pub fn select_mode_or_random(&mut self, a: i32, a1: i32, b: i32, b1: i32) -> i32 {
        if        a1 == b  && b  == b1 {
            a1
        } else if a  == a1 && a  == b  {
            a
        } else if a  == a1 && a  == b1 {
            a
        } else if a  == b  && a  == b1 {
            a
        } else if a  == a1 && b  != b1 {
            a
        } else if a  == b  && a1 != b1 {
            a
        } else if a  == b1 && a1 != b  {
            a
        } else if a1 == b  && a  != b1 {
            a1
        } else if a1 == b1 && a  != b  {
            a1
        } else if b  == b1 && a  != a1 {
            b
        } else {
            self.choose4(a, a1, b, b1)
        }
    }
    // Reversed functions
    pub fn previous_state(s: i64, k: i64) -> Vec<i64> {
        let s = Wrapping(s);
        let k = Wrapping(k);
        let ass_cs = s - k;

        Self::bruteforce_state(ass_cs.0, 0, 0, 64)
    }
    pub fn previous_state_lower_bits(s: i64, k: i64, bits: u8) -> Vec<i64> {
        let s = Wrapping(s);
        let k = Wrapping(k);
        let ass_cs = s - k;

        Self::bruteforce_state(ass_cs.0, 0, 0, bits)
    }
    pub fn bruteforce_state(ass_cs: i64, start_x: i64, start_bit: u8, total_bits: u8) -> Vec<i64> {
        let ass_cs = Wrapping(ass_cs);
        let a = Wrapping(mc_lcg_const::A);
        let c = Wrapping(mc_lcg_const::C);
        // Bitwise bruteforce
        let mut sols = vec![];
        let mask = mask_up_to_bit(start_bit);
        let mut mask = Wrapping(mask);
        let mut x = Wrapping(start_x) & mask;
        for i in start_bit..total_bits {
            mask |= Wrapping(1 << i);
            let y = x * x * a + x * c;
            if y & mask != ass_cs & mask {
                x |= Wrapping(1 << i);
                // y2 will always be even, so there are 2 solutions per state
                let y2 = x * x * a + x * c;
                if y2 & mask != ass_cs & mask {
                    //panic!("Invalid state! s: {}, x: {}", ass_cs, x);
                    return vec![];
                }
            } else {
                // Check for multiple solutions
                let x2 = x | Wrapping(1 << i);
                let y3 = x2 * x2 * a + x2 * c;
                if y3 & mask == ass_cs & mask {
                    sols.extend(Self::bruteforce_state(ass_cs.0, x2.0, i + 1, total_bits));
                }
            }
        }

        sols.push(x.0);

        sols
    }
    /// Return a `k` such that `next_state(seed, k) == seed`
    pub fn state_unchanged(seed: i64) -> i64 {
        let z = McRng::next_state(seed, 0);
        (Wrapping(seed) - Wrapping(z)).0
    }
    /// Return the world seed as entered to set_world_seed.
    pub fn original_world_seed(base_seed: i64, world_seed: i64) -> Vec<i64> {
        Self::original_world_seed_lower_bits(base_seed, world_seed, 64)
    }
    pub fn original_world_seed_lower_bits(base_seed: i64, world_seed: i64, bits: u8) -> Vec<i64> {
        let mut r = Self::default();
        r.set_base_seed(base_seed);
        let bs = r.base_seed();
        let mut x = vec![world_seed];
        x = x.into_iter().flat_map(|x| Self::previous_state_lower_bits(x, bs, bits)).collect();
        x = x.into_iter().flat_map(|x| Self::previous_state_lower_bits(x, bs, bits)).collect();
        x = x.into_iter().flat_map(|x| Self::previous_state_lower_bits(x, bs, bits)).collect();

        x
    }
    /// Returns the world seed which makes `set_chunk_seed(chunk_x, chunk_z) == chunk_seed`
    pub fn world_seed_from_chunk_seed(chunk_seed: i64, chunk_x: i64, chunk_z: i64) -> Vec<i64> {
        Self::world_seed_from_chunk_seed_lower_bits(chunk_seed, chunk_x, chunk_z, 64)
    }
    pub fn world_seed_from_chunk_seed_lower_bits(chunk_seed: i64, chunk_x: i64, chunk_z: i64, bits: u8) -> Vec<i64> {
        let mut x = vec![chunk_seed];
        x = x.into_iter().flat_map(|x| Self::previous_state_lower_bits(x, chunk_z as i64, bits)).collect();
        x = x.into_iter().flat_map(|x| Self::previous_state_lower_bits(x, chunk_x as i64, bits)).collect();
        x = x.into_iter().flat_map(|x| Self::previous_state_lower_bits(x, chunk_z as i64, bits)).collect();
        x = x.into_iter().flat_map(|x| Self::previous_state_lower_bits(x, chunk_x as i64, bits)).collect();

        x
    }
    // Functions to get the seed from the result of two consecutive calls to next_int_n(1024)
    /// Given a list of seed candidates, filter them based on the expected result for `chunk_x,
    /// chunk_z`
    pub fn filter_candidates_world_seed_from_2_next_int_1024_inside<F: Fn(i32, i32) -> bool>(candidates: &mut Vec<i64>, inside: F, chunk_x: i64, chunk_z: i64) {
        candidates.retain(|&x| {
            let mut r = McRng { world_seed: x, base_seed: 0, chunk_seed: 0 };
            r.set_chunk_seed(chunk_x, chunk_z);
            let x = r.next_int_n(1024);
            let z = r.next_int_n(1024);
            inside(x, z)
        });
    }
    pub fn filter_candidates_world_seed_from_2_next_int_1024_uncertain(candidates: &mut Vec<i64>, x_lo: i32, x_hi: i32, z_lo: i32, z_hi: i32, chunk_x: i64, chunk_z: i64) {
        Self::filter_candidates_world_seed_from_2_next_int_1024_inside(candidates, |x, z| x_lo <= x && x <= x_hi && z_lo <= z && z <= z_hi, chunk_x, chunk_z)
    }
    pub fn world_seed_from_2_next_int_1024_inside<F: Fn(i32, i32) -> bool>(x_lo: i32, x_hi: i32, inside: F, chunk_x: i64, chunk_z: i64) -> Vec<i64> {
        let (x_lo, x_hi) = (x_lo as i64, x_hi as i64);
        let mut all_ws = vec![];

        for x in x_lo..=x_hi {
            for i in 0..(1 << 24) {
                let fx = (x << 24) | i;
                let wsv = McRng::world_seed_from_chunk_seed_lower_bits(fx, chunk_x, chunk_z, 34);
                for ws in wsv {
                    let ws = ws & ((1 << 34) - 1);
                    let fz = McRng::next_state(fx, ws);
                    let fz_1024 = (fz >> 24) & 0x3FF;
                    if inside(x as i32, fz_1024 as i32) {
                        all_ws.push(ws);
                    }
                }
            }
        }
        all_ws.sort();
        // This dedup is probably redundant
        all_ws.dedup();

        all_ws
    }
    /// It is always prefered to use `filter_candidates_world_seed_from_2_next_int_1024_uncertain`
    /// and call this function only to get the initial list of candidates.
    /// The runtime is `2^24 * (x_hi - x_lo + 1)`, `z` only affects the number of candidates.
    pub fn world_seed_from_2_next_int_1024_uncertain(x_lo: i32, x_hi: i32, z_lo: i32, z_hi: i32, chunk_x: i64, chunk_z: i64) -> Vec<i64> {
        Self::world_seed_from_2_next_int_1024_inside(x_lo, x_hi, |_x, z| z_lo <= z && z <= z_hi, chunk_x, chunk_z)
    }
    /// Find the world_seed from the result of two consecutive calls to `next_int_n(1024)`,
    /// when the chunk seed was set to `chunk_x, chunk_z`.
    /// This function returns around 2^14 candidates.
    pub fn world_seed_from_2_next_int_1024(x: i32, z: i32, chunk_x: i64, chunk_z: i64) -> Vec<i64> {
        Self::world_seed_from_2_next_int_1024_uncertain(x, x, z, z, chunk_x, chunk_z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mc_rng_init() {
        let mut r = McRng::new(10, 1234);
        assert_eq!(r.base_seed(), -8738471090773341224);
        assert_eq!(r.world_seed(), 7389398735251303610);
        assert_eq!(r.chunk_seed(), 0);

        r.set_chunk_seed(4, 8);
        assert_eq!(r.chunk_seed(), 5766878170509519170);
    }

    #[test]
    fn original_world_seed_bits() {
        let base_seed = 10;
        let seed = 1234;
        let r = McRng::new(base_seed, seed);
        let world_seed = r.world_seed();
        let ows = McRng::original_world_seed(base_seed, world_seed);
        assert!(ows.contains(&seed));

        let world_seed = r.world_seed() & 0xFF;
        let ows = McRng::original_world_seed_lower_bits(base_seed, world_seed, 8);
        assert!(ows.contains(&(seed & 0xFF)));
    }

    #[test]
    fn world_seed_from_chunk_seed() {
        let base_seed = 10;
        let seed = 1234;
        let x = 13535;
        let z = 4997;
        let mut r = McRng::new(base_seed, seed);
        r.set_chunk_seed(x, z);
        let world_seed = r.world_seed();
        let chunk_seed = r.chunk_seed();
        let ws = McRng::world_seed_from_chunk_seed(chunk_seed, x, z);
        assert!(ws.contains(&world_seed));
    }

    #[test]
    fn world_seed_from_chunk_seed_lower_bits() {
        let mask = (1 << 34) - 1;
        let seed = 1234;
        let x = 13535;
        let z = 4997;
        let base_seed = 10;
        let mut r = McRng::new(base_seed, seed);
        r.set_chunk_seed(x, z);
        let world_seed = r.world_seed();
        let chunk_seed = r.chunk_seed() & mask;
        let ws = McRng::world_seed_from_chunk_seed_lower_bits(chunk_seed, x, z, 34);
        assert!(ws.contains(&(world_seed & mask)));
        let ws1 = ws;

        let x = x + 1;
        r.set_chunk_seed(x, z);
        let world_seed = r.world_seed();
        let chunk_seed = r.chunk_seed() & mask;
        let ws = McRng::world_seed_from_chunk_seed_lower_bits(chunk_seed, x, z, 34);
        assert!(ws.contains(&(world_seed & mask)));

        let ws2 = ws;
        assert_eq!(ws1, ws2);
    }

    #[test]
    fn previous_next() {
        let k = 10;
        let s0 = 1234;
        let s1 = McRng::next_state(s0, k);
        let s2 = McRng::previous_state(s1, k);

        assert_eq!(s2.len(), 2);
        assert!(s2.contains(&s0));

        let s3 = McRng::previous_state(s0, k);
        let s4: Vec<i64> = s3.iter().map(|&x| McRng::next_state(x, k)).collect();
        assert_eq!(s4.len(), 2);
        //assert!(s4.contains(&s0));
        assert_eq!(s4[0], s0);
        assert_eq!(s4[1], s0);
    }

    #[test]
    fn previous_tree() {
        let k = 10;
        let s0 = 1234;
        let mut s = vec![s0];
        let n = 1000;
        
        for _i in 0..n {
            s = s.iter().flat_map(|&x| McRng::previous_state(x, k)).collect();
        }

        // It may look like each call to previous will double the possible
        // states
        //assert_eq!(s.len(), (1 << n));
        // But actually there are still only two possible paths, which meet at
        // s0
        assert_eq!(s.len(), 2);
    }

    #[test]
    fn negative_seed_next_int() {
        let mut r = McRng::new(10, 1234);
        let chunk_x = 4;
        let chunk_z = 8;
        r.set_chunk_seed(chunk_x, chunk_z);
        let r_chunk_seed = r.chunk_seed();

        assert!(r_chunk_seed > 0);

        let a = r.next_int_n(1024) as i64;
        println!("Positive:");
        println!("{:08X} vs {:08X}", r_chunk_seed, a << 24);
        assert_eq!((r_chunk_seed >> 24) & 0x3FF, a);

        let mut r = McRng::new(10, 1234);
        let chunk_x = 5;
        let chunk_z = 8;
        r.set_chunk_seed(chunk_x, chunk_z);
        let r_chunk_seed = r.chunk_seed();

        assert!(r_chunk_seed < 0);

        let a = r.next_int_n(1024) as i64;
        println!("Negative:");
        println!("{:08X} vs {:08X}", r_chunk_seed, a << 24);
        assert_eq!((r_chunk_seed >> 24) & 0x3FF, a);
    }

    #[test]
    #[ignore] // Takes a few minutes to run if compiled without optimization
    fn seed_from_next_int() {
        let seed = 0xABCD;
        let mut r = McRng::new(10, seed);
        let chunk_x = 6;
        let chunk_z = 8;
        r.set_chunk_seed(chunk_x, chunk_z);

        let x = r.next_int_n(1024);
        let z = r.next_int_n(1024);

        let all_ws = McRng::world_seed_from_2_next_int_1024(x, z, chunk_x, chunk_z);
        println!("Expected to find 2^14: {}", 1 << 14);
        println!("Found:  {}", all_ws.len());
        //println!("{:08X?}", all_ws);

        let chunk_x = 5;
        let chunk_z = 8;
        r.set_chunk_seed(chunk_x, chunk_z);

        let x = r.next_int_n(1024);
        let z = r.next_int_n(1024);

        let all_ws2 = McRng::world_seed_from_2_next_int_1024(x, z, chunk_x, chunk_z);
        println!("Found:  {}", all_ws2.len());
        //println!("{:08X?}", all_ws2);

        use std::collections::HashSet;
        let candidates1 = all_ws.clone();
        let allhs1 = all_ws.into_iter().collect::<HashSet<_>>();
        let allhs2 = all_ws2.into_iter().collect::<HashSet<_>>();
        let allhs = allhs1.intersection(&allhs2).cloned().collect::<HashSet<_>>();
        let expected_candidates2 = allhs.clone();

        println!("Intersection of 2: {}", allhs.len());

        println!("{:08X?}", allhs);
        let allhs = allhs.into_iter().flat_map(|x| McRng::original_world_seed_lower_bits(10, x, 34)).collect::<Vec<_>>();
        println!("{:08X?}", allhs);
        assert!(allhs.contains(&(seed & ((1 << 34) - 1))));

        let mut candidates2 = candidates1.clone();
        McRng::filter_candidates_world_seed_from_2_next_int_1024_uncertain(&mut candidates2, x, x, z, z, chunk_x, chunk_z);
        assert_eq!(expected_candidates2, candidates2.into_iter().collect::<HashSet<_>>());
    }

    #[test]
    #[ignore] // Takes many minutes to run if compiled without optimization
    fn seed_from_next_int_ffas() {
        let seed = 0xABCD;
        let mut r = McRng::new(10, seed);
        let chunk_x = 6;
        let chunk_z = 8;
        r.set_chunk_seed(chunk_x, chunk_z);

        let x = r.next_int_n(1024);
        let z = r.next_int_n(1024);

        let mask = !mask_up_to_bit(6) as i32; // we only know 3 bits of each

        let all_ws = McRng::world_seed_from_2_next_int_1024_uncertain(x & mask, x | !mask, z & mask, z | !mask, chunk_x, chunk_z);
        println!("Expected to find 2^14 * 2^14: {}", 1 << 28);
        println!("Found:  {}", all_ws.len());

        let chunk_x = 5;
        let chunk_z = 8;
        r.set_chunk_seed(chunk_x, chunk_z);

        let x = r.next_int_n(1024);
        let z = r.next_int_n(1024);
        let mut candidates2 = all_ws.clone();
        McRng::filter_candidates_world_seed_from_2_next_int_1024_uncertain(&mut candidates2, x & mask, x | !mask, z & mask, z | !mask, chunk_x, chunk_z);
        println!("Expected to find 2^14 * 2^14 / 2^6: {}", 1 << 22);
        println!("Found:  {}", candidates2.len());
        //println!("{:08X?}", candidates2);

        let chunk_x = 5;
        let chunk_z = 9;
        r.set_chunk_seed(chunk_x, chunk_z);

        let x = r.next_int_n(1024);
        let z = r.next_int_n(1024);
        McRng::filter_candidates_world_seed_from_2_next_int_1024_uncertain(&mut candidates2, x & mask, x | !mask, z & mask, z | !mask, chunk_x, chunk_z);
        println!("Expected to find 2^14 * 2^14 / 2^12: {}", 1 << 16);
        println!("Found:  {}", candidates2.len());
        //println!("{:08X?}", candidates2);

        let chunk_x = 6;
        let chunk_z = 9;
        r.set_chunk_seed(chunk_x, chunk_z);

        let x = r.next_int_n(1024);
        let z = r.next_int_n(1024);
        McRng::filter_candidates_world_seed_from_2_next_int_1024_uncertain(&mut candidates2, x & mask, x | !mask, z & mask, z | !mask, chunk_x, chunk_z);
        println!("Expected to find 2^14 * 2^14 / 2^18: {}", 1 << 10);
        println!("Found:  {}", candidates2.len());
        //println!("{:08X?}", candidates2);

        let chunk_x = 7;
        let chunk_z = 9;
        r.set_chunk_seed(chunk_x, chunk_z);

        let x = r.next_int_n(1024);
        let z = r.next_int_n(1024);
        McRng::filter_candidates_world_seed_from_2_next_int_1024_uncertain(&mut candidates2, x & mask, x | !mask, z & mask, z | !mask, chunk_x, chunk_z);
        println!("Expected to find 2^14 * 2^14 / 2^24: {}", 1 << 4);
        println!("Found:  {}", candidates2.len());
        //println!("{:08X?}", candidates2);

        let chunk_x = 8;
        let chunk_z = 9;
        r.set_chunk_seed(chunk_x, chunk_z);

        let x = r.next_int_n(1024);
        let z = r.next_int_n(1024);
        McRng::filter_candidates_world_seed_from_2_next_int_1024_uncertain(&mut candidates2, x & mask, x | !mask, z & mask, z | !mask, chunk_x, chunk_z);
        println!("Expected to find 2^14 * 2^14 / 2^30: {}", 1 << 0);
        println!("Found:  {}", candidates2.len());
        println!("{:08X?}", candidates2);
        panic!("success");
        /*
---- mc_rng::tests::seed_from_next_int_ffas stdout ----
Expected to find 2^14 * 2^14: 268435456
Found:  268418219
Expected to find 2^14 * 2^14 / 2^6: 4194304
Found:  4192626
Expected to find 2^14 * 2^14 / 2^12: 65536
Found:  65813
Expected to find 2^14 * 2^14 / 2^18: 1024
Found:  1038
Expected to find 2^14 * 2^14 / 2^24: 14
Found:  18
Expected to find 2^14 * 2^14 / 2^30: 1
Found:  1
[1B737D10]
thread 'mc_rng::tests::seed_from_next_int_ffas' panicked at 'success', src/mc_rng.rs:519:9
        */
    }

    #[test]
    fn mask_64() {
        assert_eq!(mask_up_to_bit(63), !0i64);
        assert_eq!(mask_up_to_bit(0), 1i64);
    }

    #[test]
    fn next_state_just_sums_k() {
        let seed = 1234;
        let k = 5678;
        let a = McRng::next_state(seed, k);
        let b = McRng::next_state(seed, k + 1);
        assert_eq!(b - a, 1);

        let z = McRng::next_state(seed, 0);
        let c = McRng::next_state(seed, seed - z);
        assert_eq!(c, seed);

        let zz = McRng::state_unchanged(seed);
        assert_eq!(zz, seed - z);
        assert_eq!(McRng::next_state(seed, zz), seed);
    }
}
