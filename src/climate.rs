//! Biome properties used since 1.18 to convert biome noise into a concrete biome id.

use std::ops::RangeInclusive;

#[derive(Debug, Default)]
pub struct Climate {
    pub temperature: i64,
    pub humidity: i64,
    pub continentalness: i64,
    pub erosion: i64,
    pub depth: i64,
    pub weirdness: i64,
}

impl Climate {
    pub fn distance(&self, other: &Climate) -> i64 {
        diff_squared(self.temperature, other.temperature)
            + diff_squared(self.humidity, other.humidity)
            + diff_squared(self.continentalness, other.continentalness)
            + diff_squared(self.erosion, other.erosion)
            + diff_squared(self.depth, other.depth)
            + diff_squared(self.weirdness, other.weirdness)
    }
}

#[derive(Clone, Debug)]
pub struct ClimateRange {
    pub temperature: RangeInclusive<i64>,
    pub humidity: RangeInclusive<i64>,
    pub continentalness: RangeInclusive<i64>,
    pub erosion: RangeInclusive<i64>,
    pub depth: RangeInclusive<i64>,
    pub weirdness: RangeInclusive<i64>,
}

impl ClimateRange {
    pub fn distance(&self, other: &Climate) -> i64 {
        diff_squared_range(&self.temperature, other.temperature)
            + diff_squared_range(&self.humidity, other.humidity)
            + diff_squared_range(&self.continentalness, other.continentalness)
            + diff_squared_range(&self.erosion, other.erosion)
            + diff_squared_range(&self.depth, other.depth)
            + diff_squared_range(&self.weirdness, other.weirdness)
    }

    /// Returns distance if the distance is lower than `max_distance`, or `None` if the distance is
    /// greater than that.
    pub fn distance_if_lower_than(&self, other: &Climate, max_distance: i64) -> Option<i64> {
        let mut d = diff_squared_range(&self.temperature, other.temperature);
        if d > max_distance {
            return None;
        }
        d += diff_squared_range(&self.humidity, other.humidity);
        if d > max_distance {
            return None;
        }
        d += diff_squared_range(&self.continentalness, other.continentalness);
        if d > max_distance {
            return None;
        }
        d += diff_squared_range(&self.erosion, other.erosion);
        if d > max_distance {
            return None;
        }
        d += diff_squared_range(&self.depth, other.depth);
        if d > max_distance {
            return None;
        }
        d += diff_squared_range(&self.weirdness, other.weirdness);
        if d > max_distance {
            return None;
        }

        Some(d)
    }
}

fn diff_squared(a: i64, b: i64) -> i64 {
    let diff = a - b;
    diff * diff
}

fn diff_squared_range(a: &RangeInclusive<i64>, b: i64) -> i64 {
    let above = b - a.end();
    let below = a.start() - b;
    let diff = std::cmp::max(std::cmp::max(above, below), 0);
    diff * diff
}
