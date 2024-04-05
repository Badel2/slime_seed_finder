//! Given a list of spawners, find all the multi-spawner intersection points.
//!
//! Since all the spawners have the same activation radius, this problem is equivalent to finding
//! the intersections between N spheres of the same radius.

use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::collections::HashSet;

/// Given a list of spawner coordinates, returns the list of multi-spawners: points that are in the
/// activation radius of more than 1 spawner, sorted by number of spawners that can be activated
/// there.
pub fn find_multi_spawners(
    all_dungeons: Vec<((i64, i64, i64), String)>,
) -> Vec<FindMultiSpawnersOutput> {
    // Segregate dungeons into buckets such that two dungeons that can be active at once are always
    // in adjacent buckets
    let spawner_activation_radius = 16;
    let bucket_side_length = spawner_activation_radius * 2 + 4;

    let buckets = segregate_into_buckets(all_dungeons, bucket_side_length);
    let mut multispawners = vec![];

    // For each bucket: load all the dungeons from this bucket and the 26 adjacent ones
    // And try to find groups of dungeons that are close to each other
    for bucket in buckets.keys() {
        let mut spawners = load_bucket_and_26_neighbors(&buckets, bucket);
        let orig_len = spawners.len();
        remove_all_spawners_that_are_not_connected(&mut spawners, spawner_activation_radius * 2);
        log::debug!(
            "Found {} spawners in bucket {:?} ({} before removing unconnected ones)",
            spawners.len(),
            bucket,
            orig_len
        );

        match spawners.len() {
            // No multi dungeons here
            0 | 1 => {}
            2 => {
                // Simple case, just calculate the midpoint
                let a = &spawners[0];
                let b = &spawners[1];
                let midpoint = calc_midpoint([a.0, b.0].iter().cloned());
                multispawners.push(FindMultiSpawnersOutput {
                    optimal_position: midpoint,
                    spawners: spawners.into_iter().cloned().collect(),
                });
            }
            _n => {
                // General case: more than 2 dungeons
                // Because of the precondition `remove_all_spawners_that_are_not_connected`, we are
                // guaranteed that there exists an intersection. However that intersection may not
                // always include all the dungeons, consider the case n=3. It is possible that all
                // 3 dungeons intersect, or it is possible that one of them intersects with the
                // other 2, but they never intersect all 3.
                // Calculate bounding box of all the spawners, and iterate all the possible
                // positions inside that bounding box and inside the current bucket.
                // This works because if we want to find the approximate location of an
                // intersection of n spheres, we can first find the intersection of n cubes (such
                // that each cube has side length equal to the diameter of the sphere).
                // However this algorithm does not iterate over the intersection points yet, it
                // iterates over all the points.
                let mut bb = bounding_box(spawners.as_slice());
                // TODO: this makes it faster but leads to missed intersections
                // see test intersection_in_different_bucket
                //clamp_bb_to_bucket(&mut bb, bucket, bucket_side_length);
                let more_multispawners =
                    find_multispawners_in_bb(&bb, &spawners, spawner_activation_radius);
                multispawners.extend(more_multispawners);
            }
        }
    }

    // Even if we ensure that each bucket does not return any duplicate spawners (subsets of one
    // another), it is possible that two adjacent buckets have the same spawners, if the
    // intersection is at the bucket boundary. Or if the bucket only has 2 spawners.
    remove_duplicate_keys_again(&mut multispawners);

    multispawners.sort_by_key(|k| {
        // Number of spawners (higher first), then x coordinate, then z coordinate, then y coordinate
        (
            !k.spawners.len(),
            OrderedFloat(k.optimal_position.x),
            OrderedFloat(k.optimal_position.z),
            OrderedFloat(k.optimal_position.y),
        )
    });

    multispawners
}

/// Given a list of coordinates and a bucket size, segregate the coordinates into buckets of that
/// size. Since the coordinates are 3D, a bucket is a 3D cube.
/// This improves the performance of some algorithms, as instead of checking all the coordinates
/// they only need to check the coordinates in nearby buckets.
fn segregate_into_buckets<V>(
    list: Vec<((i64, i64, i64), V)>,
    size: u64,
) -> HashMap<(i64, i64, i64), Vec<((i64, i64, i64), V)>> {
    let size = size as i64;
    let mut buckets: HashMap<(i64, i64, i64), Vec<_>> = HashMap::new();

    for el in list {
        let ((x, y, z), v) = el;
        // We must use div_euclid because `-1 / size` must return `-1` instead of `0`.
        let bucket_id = (x.div_euclid(size), y.div_euclid(size), z.div_euclid(size));
        buckets.entry(bucket_id).or_default().push(((x, y, z), v));
    }

    buckets
}

/// Load all the coordinates from the target bucket and its 26-connected neighbors.
/// 26 extra because there are 27 buckets in 3x3x3.
fn load_bucket_and_26_neighbors<'b, V>(
    buckets: &'b HashMap<(i64, i64, i64), Vec<((i64, i64, i64), V)>>,
    (x, y, z): &(i64, i64, i64),
) -> Vec<&'b ((i64, i64, i64), V)> {
    let mut v = vec![];

    for i in -1..=1 {
        for j in -1..=1 {
            for k in -1..=1 {
                if let Some(bucket) = buckets.get(&(x + i, y + j, z + k)) {
                    v.extend(bucket);
                }
            }
        }
    }

    v
}

/// Return the distance between 2 3D points, squared.
/// This is useful because comparing distances can be done faster if comparing the distances
/// squared, as we avoid sqrt operations.
fn distance3dsquared(a: &(i64, i64, i64), b: &(i64, i64, i64)) -> f64 {
    let x = (a.0 - b.0) as f64;
    let y = (a.1 - b.1) as f64;
    let z = (a.2 - b.2) as f64;

    x * x + y * y + z * z
}

/// Given a list of spawner coordinates, we want to remove the ones that are not connected to any
/// other spawner. Connected means that the distance is less than max_distance. Mutates the vec in
/// place.
///
/// Time complexity: O(n^2), could be improved using a 3D quad tree maybe
fn remove_all_spawners_that_are_not_connected<V>(
    v: &mut Vec<&((i64, i64, i64), V)>,
    max_distance: u64,
) {
    let max_distance_squared = (max_distance as f64) * (max_distance as f64);
    let mut i = 0;

    while i < v.len() {
        let mut connected = false;
        for j in 0..v.len() {
            if i == j {
                continue;
            }
            let a = &v[i].0;
            let b = &v[j].0;
            if distance3dsquared(a, b) < max_distance_squared {
                connected = true;
                break;
            }
        }

        // If not connected remove it immediately
        if !connected {
            v.swap_remove(i);
        } else {
            i += 1;
        }
    }
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FloatPosition {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct FindMultiSpawnersOutput {
    pub optimal_position: FloatPosition,
    pub spawners: Vec<((i64, i64, i64), String)>,
}

#[derive(Debug)]
struct BoundingBox {
    x_min: i64,
    x_max: i64,
    y_min: i64,
    y_max: i64,
    z_min: i64,
    z_max: i64,
}

/// Return the smallest 3D rectangle (bounding box) that contains all the points in `p`.
fn bounding_box<V>(p: &[&((i64, i64, i64), V)]) -> BoundingBox {
    use std::cmp::max;
    use std::cmp::min;

    let mut x_min = p[0].0 .0;
    let mut x_max = p[0].0 .0;
    let mut y_min = p[0].0 .1;
    let mut y_max = p[0].0 .1;
    let mut z_min = p[0].0 .2;
    let mut z_max = p[0].0 .2;

    for ((x, y, z), _) in p.iter().skip(1) {
        x_min = min(x_min, *x);
        x_max = max(x_max, *x);
        y_min = min(y_min, *y);
        y_max = max(y_max, *y);
        z_min = min(z_min, *z);
        z_max = max(z_max, *z);
    }

    BoundingBox {
        x_min,
        x_max,
        y_min,
        y_max,
        z_min,
        z_max,
    }
}

/// A bounding box is a 3D rectangle. A bucket is also a 3D rectangle but with all sides the same
/// length (a cube). This function mutates the bounding box so that it only preserves the part of
/// the bounding box that intersects with the cube.
fn clamp_bb_to_bucket(bb: &mut BoundingBox, bucket: &(i64, i64, i64), bucket_side_length: u64) {
    use std::cmp::max;
    use std::cmp::min;

    let l = bucket_side_length as i64;
    let b_x_min = bucket.0 * l;
    let b_x_max = (bucket.0 + 1) * l;
    let b_y_min = bucket.1 * l;
    let b_y_max = (bucket.1 + 1) * l;
    let b_z_min = bucket.2 * l;
    let b_z_max = (bucket.2 + 1) * l;

    bb.x_min = max(bb.x_min, b_x_min);
    bb.x_max = min(bb.x_max, b_x_max);
    bb.y_min = max(bb.y_min, b_y_min);
    bb.y_max = min(bb.y_max, b_y_max);
    bb.z_min = max(bb.z_min, b_z_min);
    bb.z_max = min(bb.z_max, b_z_max);
}

/// Returns true if all of the elements that are true in `a` are also true in `b`
fn a_is_subset_of_b(a: &[bool], b: &[bool]) -> bool {
    assert_eq!(a.len(), b.len());

    for (i, x) in a.iter().enumerate() {
        if *x {
            if b[i] {
                // True in a and true in b, ok
                continue;
            } else {
                return false;
            }
        }
    }

    true
}

/// Removes all keys that are a subset of other keys.
///
/// `v` cannot contain any duplicates
fn remove_duplicate_keys(v: &mut Vec<(Vec<bool>, (i64, i64, i64))>) {
    // Check if any v[j].0 is a subset of v[i].0
    let mut to_remove = HashSet::new();

    for i in 0..v.len() {
        for j in 0..v.len() {
            if i == j {
                continue;
            }

            if a_is_subset_of_b(&v[j].0, &v[i].0) {
                to_remove.insert(j);
            }
        }
    }

    let mut to_remove: Vec<_> = to_remove.into_iter().collect();
    to_remove.sort();

    for i in to_remove.into_iter().rev() {
        v.swap_remove(i);
    }
}

fn a_is_subset_of_b_again<V>(a: &[((i64, i64, i64), V)], b: &[((i64, i64, i64), V)]) -> bool {
    for (a_pos, _) in a {
        // If all the spawners of a are also present in b
        let mut found = false;
        for (b_pos, _) in b {
            if b_pos == a_pos {
                found = true;
                break;
            }
        }

        if !found {
            // At least one spawner of a is not present in b
            return false;
        }
    }

    true
}

fn remove_duplicate_keys_again(v: &mut Vec<FindMultiSpawnersOutput>) {
    use std::cmp::max;
    // Check if any v[j].0 is a subset of v[i].0
    let mut to_remove = HashSet::new();

    for i in 0..v.len() {
        for j in 0..v.len() {
            if i == j {
                continue;
            }

            if a_is_subset_of_b_again(&v[j].spawners, &v[i].spawners) {
                if v[j].spawners.len() == v[i].spawners.len() {
                    // Sometimes it happens that a and b are equal, so they are both subsets of each other
                    // and they disappear after removing duplicates. So handle that case by removing the
                    // one with highest index.
                    to_remove.insert(max(i, j));
                } else {
                    // If all the spawners of a are also present in b, remove a
                    to_remove.insert(j);
                }
            }
        }
    }

    let mut to_remove: Vec<_> = to_remove.into_iter().collect();
    to_remove.sort();

    for i in to_remove.into_iter().rev() {
        v.swap_remove(i);
    }
}

/// Find all the intersections of spawners inside the bounding box. Returns a list of intersections
/// sorted by number of spawners. Removes any duplicates and subsets: if there is an intersection
/// A-B and another intersection A-B-C, it only returns A-B-C. However if the intersections are A-B
/// and A-C-D, it returns both of them.
///
/// Performance: O(bb_side^3 * num_spawners)
/// (that's really bad but bb_side and num_spawners should be small)
fn find_multispawners_in_bb(
    bb: &BoundingBox,
    spawners: &[&((i64, i64, i64), String)],
    max_distance: u64,
) -> Vec<FindMultiSpawnersOutput> {
    let mut multispawners = vec![];

    let mut hm = HashMap::new();
    let max_distance_squared = (max_distance as f64) * (max_distance as f64);

    // Calculate the distance to all the spawners from a given (x, y, z) coordinate.
    // Returns:
    // * A bitset of which spawners intersect there
    // * The number of spawners that intersect there
    // * A score consisting of the sum of (distances squared)
    let distance_to_all = |(x, y, z)| {
        let mut key = Vec::with_capacity(spawners.len());
        let mut num_true = 0;
        let mut score = 0.0;
        for s in spawners.iter() {
            let dist = distance3dsquared(&(x, y, z), &s.0);
            if dist <= max_distance_squared {
                key.push(true);
                num_true += 1;
                // TODO: not sure if this is a good score function or we should sqrt the distance here
                // But it doesn't matter much, we will find a point that may not be optimal but it
                // will be valid.
                score += dist;
            } else {
                key.push(false);
            }
        }
        (key, num_true, score)
    };

    // Iterate over all the integer coordinates of the bounding box
    // TODO: this will not find "micro intersections" that only happen at float coordinates but
    // disappear at integer coordinates (so at x=1 there is no intersection, at x=1.5 there is, but
    // at x=2 there is not), but I don't know if those exist yet
    for x in bb.x_min..=bb.x_max {
        for y in bb.y_min..=bb.y_max {
            for z in bb.z_min..=bb.z_max {
                let (hm_key, num_true, score) = distance_to_all((x, y, z));
                if num_true <= 1 {
                    // This point is close to 0 or 1 dungeons, so not a multi dungeon point
                } else if let Some((prev_score, _prev_pos)) = hm.get(&hm_key) {
                    // We already have another point that intersects the same dungeons
                    if score < *prev_score {
                        // Smaller distance = better match
                        hm.insert(hm_key, (score, (x, y, z)));
                    }
                } else {
                    // New intersection
                    hm.insert(hm_key, (score, (x, y, z)));
                }
            }
        }
    }

    // Deduplicate matches: given [true, false, true] and [true, true, true] we only want to keep
    // [true, true, true]
    let mut key_list: Vec<_> = hm
        .into_iter()
        .map(|(hm_key, (_score, pos))| (hm_key, pos))
        .collect();
    remove_duplicate_keys(&mut key_list);
    for (hm_key, pos) in key_list {
        let mut sp = vec![];

        for (i, b) in hm_key.iter().enumerate() {
            if *b {
                sp.push(spawners[i].clone());
            }
        }

        multispawners.push(FindMultiSpawnersOutput {
            optimal_position: FloatPosition {
                x: pos.0 as f64,
                y: pos.1 as f64,
                z: pos.2 as f64,
            },
            spawners: sp,
        });
    }

    multispawners
}

/// Return the center of mass of n points
// Note that this can only be used to find the intersection of n spawners if n=2
fn calc_midpoint<I: Iterator<Item = (i64, i64, i64)>>(p: I) -> FloatPosition {
    let mut n = 0;
    let mut x = 0.0;
    let mut y = 0.0;
    let mut z = 0.0;

    for (px, py, pz) in p {
        x += px as f64;
        y += py as f64;
        z += pz as f64;
        n += 1;
    }

    let n = n as f64;
    x /= n;
    y /= n;
    z /= n;

    FloatPosition { x, y, z }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_in_different_bucket() {
        // Bug: if the intersection does not fall in the same bucket as any spawner, and
        // num_spawners > 2, this used to fail to find it
        let pos = (-16776961, 16779271, -520093696);
        let offsets = [(-13, -1, -1), (0, 8, -12), (0, -12, 4)];

        let all_dungeons: Vec<_> = offsets
            .iter()
            .map(|(dx, dy, dz)| {
                (
                    (
                        (pos.0 as i64).saturating_add(*dx as i64),
                        (pos.1 as i64).saturating_add(*dy as i64),
                        (pos.2 as i64).saturating_add(*dz as i64),
                    ),
                    "".to_string(),
                )
            })
            .collect();

        let res = find_multi_spawners(all_dungeons.clone());

        assert_eq!(res.len(), 1);
        assert_eq!(res[0].spawners.len(), all_dungeons.len());
        assert_eq!(
            HashSet::<_>::from_iter(res[0].spawners.iter()),
            HashSet::from_iter(all_dungeons.iter())
        );
    }

    /// Returns true if the player can active that spawner from the exact position.
    /// When standing at the center of block (0, 0), the position is (0.5, 0.5).
    fn is_spawner_active(player_position: FloatPosition, spawner: (i64, i64, i64)) -> bool {
        todo!("implement this function, it may be useful for more precise tests")
    }

    #[test]
    fn precise_activation_radius_1() {
        // Values taken from a minecraft world
        let all_dungeons = vec![
            ((-28, -60, 37), "".to_string()),
            ((-28, -60, 68), "".to_string()),
        ];

        let res = find_multi_spawners(all_dungeons.clone());
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].spawners.len(), all_dungeons.len());
        assert_eq!(
            HashSet::<_>::from_iter(res[0].spawners.iter()),
            HashSet::from_iter(all_dungeons.iter())
        );
    }

    #[test]
    fn precise_activation_radius_2() {
        // Values taken from a minecraft world
        // Move 1 dungeon 1 block away, there is no intersection now
        let all_dungeons = vec![
            ((-28, -60, 37), "".to_string()),
            ((-28, -60, 69), "".to_string()),
        ];

        let res = find_multi_spawners(all_dungeons.clone());
        assert_eq!(res, vec![]);
    }

    #[test]
    fn precise_activation_radius_3() {
        // Values taken from a minecraft world
        let all_dungeons = vec![
            ((-7, -44, -64), "".to_string()),
            ((8, -44, -79), "".to_string()),
            ((8, -60, -64), "".to_string()),
            ((8, -29, -64), "".to_string()),
            ((8, -44, -48), "".to_string()),
            ((23, -44, -64), "".to_string()),
        ];

        let res = find_multi_spawners(all_dungeons.clone());
        assert_eq!(res.len(), 1);
        assert_eq!(res[0].spawners.len(), all_dungeons.len());
        assert_eq!(
            HashSet::<_>::from_iter(res[0].spawners.iter()),
            HashSet::from_iter(all_dungeons.iter())
        );
    }
}
