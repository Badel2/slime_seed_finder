use std::num::Wrapping;

// The constants used by the Linear Congruential Generator
pub mod mc_lcg_const {
    pub const A: i64 = 6364136223846793005;
    pub const C: i64 = 1442695040888963407;
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
    // Is it possible that this operation will leave the state unchanged?
    // A*s*s + C*s + k = s
    pub fn next_state(s: i64, k: i64) -> i64
    {
        let s: Wrapping<i64> = Wrapping(s);
        let k: Wrapping<i64> = Wrapping(k);

        (s * (s * Wrapping(mc_lcg_const::A) + Wrapping(mc_lcg_const::C)) + k).0
    }
    pub fn previous_state(s: i64, k: i64) -> Vec<i64> {
        let s = Wrapping(s);
        let k = Wrapping(k);
        let ass_cs = s - k;

        Self::bruteforce_state(ass_cs.0, 0, 0)
    }
    pub fn bruteforce_state(ass_cs: i64, start_x: i64, start_bit: u8) -> Vec<i64> {
        let ass_cs = Wrapping(ass_cs);
        let a = Wrapping(mc_lcg_const::A);
        let c = Wrapping(mc_lcg_const::C);
        // Bitwise bruteforce
        let mut sols = vec![];
        let mask = (1 << (start_bit + 1)) - 1;
        let mut mask = Wrapping(mask);
        let mut x = Wrapping(start_x) & mask;
        for i in start_bit..64 {
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
                    sols.extend(Self::bruteforce_state(ass_cs.0, x2.0, i + 1));
                }
            }
        }

        sols.push(x.0);

        sols
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
}
