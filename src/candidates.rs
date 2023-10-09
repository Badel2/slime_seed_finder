use crate::java_rng::mask;
use crate::mc_rng::McRng;
// We need a way to abstract the bruteforcing:
//
// * Iterate over all the possible seeds up to N bits, this is used for the first step
// * Iterate over a range, this is used to implement multithreading
// * Iterate over a list of candidates, this is used to filter out some candidates
// * Iterate over a list of 18-bit candidates, but extend them to 48 bits. This is used when
// combining different layers.
//
// It seems that the last requirement is the most complex, so let's try to create an API based on
// that.
//
// The result of one step is a Vec<u18>, and the next step needs a Vec<u48> as input.
// Creating the Vec<u48> is not an option, we need to use iterators: use Iterator<Item = u48> as input.
//
// The output type will have to keep being Vec<u48>, because yield and generators are not stable yet.
// Well, or we can return impl Iterator<Item = u48>, should be pretty easy
//
// Problem: if we combine it with ranges there is no way to distinguish iterator end from empty
// range. Shouldn't be a problem, just check if the range is the last range.
//
// Problem: interoperability. Is it possible to write a C wrapper API if the API uses iterators?
// I don't know, but we can just write a wrapper that works with slices instead.
//
//
// Idea: bruteforce graph
//
// Each algorithm will have a "cost", that can be estimated at runtime or hardcoded, but should be
// calculated running the bruteforce for N steps and measuring the time. The unit used to represent
// cost should be "seconds per input candidate", stored as f64 for example.
//
// Then we just need to build a graph will all the connections that make sense and find the
// shortest path from 0 bits known to 64 bits known.
//
// Example graph: using layers slime18, slime48, extend48, biomes64
//
// From; To; Cost
// 0; slime18; slime18_cost * 2^18
// 0; slime48; slime48_cost * 2^48
// 0; extend48; extend48_cost * 2^48
// 0; biomes64; biomes64_cost * 2^64
// 0; 64;
// slime18; slime48; slime48_cost * 2^30 * slime18_candidates
// slime18; extend64; extend64_cost * 2^46 * slime18_candidates
// slime18; biomes64; biomes64_cost * 2^46 * slime18_candidates
// slime48; extend64; extend64_cost * 2^16 *
//
//
// Problem: output candidates. We also need to estimate the number of output candidates.
//
// Problem: preprocessing. Some layers may require some preprocessing before we can start
// bruteforcing. Do we need an additional heuristic to know which algorithms to check first?
// In that case we can use a hardcoded cost.
//
struct Candidates {
    list: Vec<CandidateKind>,
}

enum CandidateKind {
    Low(LowBitsCandidates),
}

struct LowBitsCandidates {
    num_known_bits: u8,
    patterns: Vec<u64>,
    add_similar_seed: bool,
}

impl CandidateKind {
    pub fn from_end_pillars_16(patterns: Vec<u64>) -> Self {
        Self::Low(LowBitsCandidates {
            num_known_bits: 16,
            patterns,
            add_similar_seed: false,
        })
    }
    pub fn from_slime_18(patterns: Vec<u64>) -> Self {
        Self::Low(LowBitsCandidates {
            num_known_bits: 18,
            patterns,
            add_similar_seed: false,
        })
    }
    pub fn from_map_zoom_26(patterns: Vec<u64>) -> Self {
        Self::Low(LowBitsCandidates {
            num_known_bits: 26,
            patterns,
            add_similar_seed: true,
        })
    }
    pub fn from_map_voronoi_34(patterns: Vec<u64>) -> Self {
        Self::Low(LowBitsCandidates {
            num_known_bits: 34,
            patterns,
            add_similar_seed: true,
        })
    }
    pub fn from_structures_48(patterns: Vec<u64>) -> Self {
        Self::Low(LowBitsCandidates {
            num_known_bits: 48,
            patterns,
            add_similar_seed: false,
        })
    }
}

trait CandidateLowBits {
    fn num_known_bits(&self) -> u8;
    fn add_similar_seed(&self) -> bool;
    fn patterns(&self) -> Vec<u64>;
}

#[derive(Debug, Default)]
pub struct CandidatesMapZoom26 {
    pub patterns: Vec<u64>,
}

impl CandidatesMapZoom26 {
    pub fn add_candidate(&mut self, candidate: u64) {
        self.patterns.push(candidate);
    }
    pub fn count(&self) -> usize {
        self.patterns.len() * 2
    }
}

impl IntoIterator for CandidatesMapZoom26 {
    type IntoIter = std::iter::FlatMap<
        std::vec::IntoIter<Self::Item>,
        std::iter::Chain<std::iter::Once<u64>, std::iter::Once<u64>>,
        fn(u64) -> std::iter::Chain<std::iter::Once<u64>, std::iter::Once<u64>>,
    >;
    type Item = u64;

    fn into_iter(self) -> Self::IntoIter {
        fn seed_and_similar(
            x: u64,
        ) -> std::iter::Chain<std::iter::Once<u64>, std::iter::Once<u64>> {
            let similar_seed = McRng::similar_biome_seed(x as i64) & ((1 << 26) - 1);
            std::iter::once(x).chain(std::iter::once(similar_seed as u64))
        }
        let ptr: fn(u64) -> std::iter::Chain<std::iter::Once<u64>, std::iter::Once<u64>> =
            seed_and_similar;

        self.patterns.into_iter().flat_map(ptr)
    }
}

impl CandidateLowBits for CandidatesMapZoom26 {
    fn num_known_bits(&self) -> u8 {
        26
    }
    fn add_similar_seed(&self) -> bool {
        true
    }
    fn patterns(&self) -> Vec<u64> {
        self.patterns.clone()
    }
}

struct MoreBits {
    input_bits: u8,
    output_bits: u8,
}

impl MoreBits {
    pub fn new(input_bits: u8, output_bits: u8) -> Self {
        assert_ne!(input_bits, 0);
        assert!(input_bits <= 64);
        assert!(output_bits <= 64);

        Self {
            input_bits,
            output_bits,
        }
    }

    /// Given an iterator with input_bits values, extend it to output_bits by enumerating all the
    /// possibilities.
    // The output will not be sorted
    // Output len = input len * (2 ** (output_bits - input_bits))
    pub fn more_bits<'a, I>(&'a self, iter: I) -> impl Iterator<Item = u64> + 'a
    where
        I: IntoIterator<Item = u64> + 'a,
    {
        let extra_bits = self.output_bits.saturating_sub(self.input_bits);
        let msk = mask(self.output_bits);
        iter.into_iter().flat_map(move |seed| {
            (0..(1 << extra_bits)).map(move |h| (h << self.input_bits) | (seed & msk))
        })
    }
}

/// Return an iterator over all the possible n-bit values
pub fn iter_bits_u32<'a>(n: u8) -> impl Iterator<Item = u32> + 'a {
    if n >= 32 {
        0..=u32::max_value()
    } else {
        0..=((1 << n) - 1)
    }
}

/// Return an iterator over all the possible n-bit values
pub fn iter_bits_u64<'a>(n: u8) -> impl Iterator<Item = u64> + 'a {
    if n >= 64 {
        0..=u64::max_value()
    } else {
        0..=((1 << n) - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_api_bits() {
        let m = MoreBits::new(1, 3);
        // An iterator of [0] is extended from 1 bit to 3 bits, the output is
        // [0, 2, 4, 6]
        let x0 = m.more_bits(std::iter::once(0));
        // Which is the same as a 3-bit iterator that skips odd elements
        let x1 = iter_bits_u64(3).step_by(2);

        let vx0: Vec<_> = x0.collect();
        assert_eq!(vx0, vec![0, 2, 4, 6]);
        let vx1: Vec<_> = x1.collect();
        assert_eq!(vx0, vx1);
    }
}
