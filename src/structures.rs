use crate::chunk::Chunk;
use crate::java_rng::mask;
use crate::java_rng::JavaRng;

// ceil(2^24 / 100)
const ONE_PERCENT_OF_2_24: i32 = 0x28f5d;
const SA: i64 = 0x4f9939f508;
const SB: i64 = 0x1ef1565bd5;
const K_TREASURE: i32 = 10387320;

fn calculate_seeded_rng_data(c: &Chunk, k: i32) -> i64 {
    let chunk_x = c.x as i64;
    let chunk_z = c.z as i64;
    let x = chunk_x.wrapping_mul(SA);
    let z = chunk_z.wrapping_mul(SB);

    // s = x * SA + z * SB + SK
    x.wrapping_add(z).wrapping_add(k as i64)
}

fn calculate_treasure_data(c: &Chunk) -> i64 {
    calculate_seeded_rng_data(c, K_TREASURE)
}

pub fn seeded_rng(seed: i64, c: &Chunk, k: i32) -> JavaRng {
    seeded_rng_with_data(seed, calculate_seeded_rng_data(c, k))
}

fn seeded_rng_with_data(seed: i64, x: i64) -> JavaRng {
    let s = x.wrapping_add(seed);

    JavaRng::with_seed(s as u64)
}

// Perform the reverse of seeded_rng_with_data. Note that only the bottom 48 bits will be
// recovered.
//
// ```ignore
// seed = seed & mask(48);
// assert!(reverse_seeded_rng_with_data(&seeded_rng_with_data(seed, x), x) == seed);
// ```
fn reverse_seeded_rng_with_data(r: &JavaRng, x: i64) -> i64 {
    (r.get_seed() as i64).wrapping_sub(x) & mask(48) as i64
}

fn is_treasure_data(seed: i64, x: i64) -> bool {
    let i = treasure_chunk_check_data(seed, x);

    // See treasure_float_to_int_optimization test
    i < ONE_PERCENT_OF_2_24
}

pub fn is_treasure_chunk(seed: i64, c: &Chunk) -> bool {
    is_treasure_data(seed, calculate_treasure_data(c))
}

fn int_as_24_bit_float(i: i32) -> f32 {
    i as f32 / (1 << 24) as f32
}

fn treasure_chunk_check(seed: i64, c: &Chunk) -> i32 {
    treasure_chunk_check_data(seed, calculate_treasure_data(c))
}

fn treasure_chunk_check_data(seed: i64, x: i64) -> i32 {
    let mut r = seeded_rng_with_data(seed, x);

    r.next(24)
}

/// Given a 42-bit seed, return the equivalent 48-bit seed that could generate
/// a buried treasure at the given chunk coordinates.
/// This function may return false positives.
pub fn treasure_expand42(seed: i64, c: &Chunk) -> i64 {
    treasure_expand42_data(seed, calculate_treasure_data(c))
}

pub fn treasure_expand42_data(seed: i64, d: i64) -> i64 {
    let lower_42 = seed & mask(42) as i64;

    let mut r = seeded_rng_with_data(lower_42, d);
    // Simulate call to next_float()
    r.next_float();
    // Set bits [47, 42] to 0
    r.set_raw_seed(r.get_raw_seed() & mask(42));
    // Undo next_float()
    r.previous();

    // Recover seed from rng and data
    reverse_seeded_rng_with_data(&r, d)
}

pub fn treasure_seed_finder(treasure_chunks: &[Chunk], max_errors: usize) -> Vec<i64> {
    TreasureChunks::new(treasure_chunks, max_errors).find_seed()
}

pub struct TreasureChunks {
    treasure_chunks: Vec<Chunk>,
    treasure_data: Vec<i64>,
    max_errors: usize,
}

impl TreasureChunks {
    pub fn new(treasure_chunks: &[Chunk], max_errors: usize) -> Self {
        let treasure_data = treasure_chunks
            .iter()
            .map(|c| calculate_treasure_data(c))
            .collect();
        let treasure_chunks = treasure_chunks.iter().cloned().collect();

        Self {
            treasure_chunks,
            treasure_data,
            max_errors,
        }
    }

    pub fn find_seed(&self) -> Vec<i64> {
        self.find_seed_range(0, 1 << 42)
    }

    pub fn find_seed_range(&self, lo: u64, hi: u64) -> Vec<i64> {
        let mut r = vec![];
        let first = self.treasure_data[0];

        'nextseed: for lower_42 in lo..hi {
            let seed = treasure_expand42_data(lower_42 as i64, first);
            let mut errors = 0;
            for t in &self.treasure_data {
                if !is_treasure_data(seed, *t) {
                    errors += 1;
                    if errors >= self.max_errors {
                        continue 'nextseed;
                    }
                }
            }
            r.push(seed);
        }

        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constant_one_percent_of_2_24() {
        assert_eq!(ONE_PERCENT_OF_2_24, (0.01 * (1 << 24) as f32).ceil() as i32);
    }

    #[test]
    #[ignore] // no need to run every time
    fn treasure_float_to_int_optimization() {
        fn original(i: i32) -> bool {
            int_as_24_bit_float(i) < 0.01
        }
        fn no_float(i: i32) -> bool {
            i < ONE_PERCENT_OF_2_24
        }

        // There are only 2^24 floats between 0 and 1
        for i in 0..(1 << 24) {
            assert_eq!(original(i), no_float(i), "{}", i);
        }
    }

    #[test]
    fn treasure_checks() {
        let world = 1234;
        let treasures = vec![
            Chunk::new(-47, -28),
            Chunk::new(-57, -21),
            Chunk::new(-58, -20),
        ];

        for c in &treasures {
            assert!(is_treasure_chunk(world, c));
        }

        let world = 0;
        let treasures = vec![Chunk::new(35, 44)];

        for c in &treasures {
            assert!(is_treasure_chunk(world, c));
        }
    }

    #[test]
    fn treasure_get_bits_from_world_seed() {
        // Best case: next(24) returns 0
        // Bits [47, 24] of the PRNG are all 0
        // Worst case: next(24) returns ONE_PERCENT_OF_2_24 - 1
        // Which is 0b101000111101011100
        //            765432109876543210
        // So bit 17 of the output of next(24) is 1
        // This is bit 24+17 = 41 of the PRNG
        // Bits [47, 42] of the PRNG are all 0
        // We know the value of the 6 most significant bits!
        // Therefore, if a chunk contains a treasure, we can automatically
        // find the value of bits [47, 42], but we still need to bruteforce
        // the lower 42 bits.

        // Example: seed 0
        let lower_42 = 0;
        let chunk = Chunk::new(35, 44);
        let s = treasure_expand42(lower_42, &chunk);
        assert_eq!(s, 0);

        // Example2: seed 2^42
        let lower_42 = 0;
        let chunk = Chunk::new(19, 12);
        let s = treasure_expand42(lower_42, &chunk);
        assert_eq!(s, 1 << 42);
    }

    #[test]
    fn treasure_expand42_returns_48_bits() {
        // This test will fail if you remove the "& mask(48)" from reverse_seeded_rng_with_data
        let seed = 1234;
        let chunk = Chunk::new(-47, -28);
        let expanded_seed = treasure_expand42(seed, &chunk) as u64;
        assert!(expanded_seed < (1u64 << 48));
    }

    // Success! Consecutive seeds have almost the same treasures
    #[test]
    fn treasure_consecutive_seeds() {
        let world = 1234;
        let treasures = vec![
            Chunk::new(-47, -28),
            Chunk::new(-57, -21),
            Chunk::new(-58, -20),
        ];

        for world in world..world + 20 {
            for c in &treasures {
                let good = is_treasure_chunk(world, c);
                if !good {
                    println!("{}: {:?}", world, c);
                }
                //assert!(good);
            }
        }
    }
}
