use crate::chunk::Chunk;
use crate::chunk::Point;
use crate::java_rng::JavaRng;
use crate::seed_info::MinecraftVersion;

/// Return all the 48-bit world seeds that can generate dungeons at the specified coordinates.
pub fn dungeon_seed_finder(dungeons: &[(i32, i32, i32)], version: MinecraftVersion) -> Vec<u64> {
    dungeon_seed_finder_range(dungeons, version, 0, 1 << 48)
}

/// Return all the 48-bit world seeds that can generate dungeons at the specified coordinates.
/// Range: [0, 1 << 48)
pub fn dungeon_seed_finder_range(
    dungeons: &[(i32, i32, i32)],
    version: MinecraftVersion,
    range_lo: u64,
    range_hi: u64,
) -> Vec<u64> {
    assert!(range_hi <= (1 << 48));
    let chunks: Vec<_> = dungeons
        .iter()
        .cloned()
        .map(|(x, y, z)| {
            let Chunk {
                x: chunk_x,
                z: chunk_z,
            } = spawner_coordinates_to_chunk(x as i64, z as i64);
            ((chunk_x, chunk_z), (x, y, z))
        })
        .collect();
    let check_fn = |world_seed: u64, chunk_x, chunk_z, (x, y, z)| match version {
        MinecraftVersion::JavaAlpha1_2_5 => Some(
            populate_alpha_1_0_4_check_dungeon(world_seed as i64, chunk_x, chunk_z, (x, y, z))
                .is_some(),
        ),
        MinecraftVersion::JavaBeta => {
            populate_alpha_1_2_6_check_dungeon(world_seed as i64, chunk_x, chunk_z, (x, y, z))
        }
        _ => unimplemented!(),
    };
    let mut r = vec![];
    'nextseed: for world_seed in range_lo..range_hi {
        //let world_seed = 536274160436487309 & ((1 << 48) - 1);
        let mut matches = 0;
        let mut misses = 0;
        let expected_matches = chunks.len();
        for ((chunk_x, chunk_z), (x, y, z)) in chunks.iter().cloned() {
            match check_fn(world_seed, chunk_x, chunk_z, (x, y, z)) {
                None => {
                    // This chunk has a lava lake, so we don't know how to check for dungeons
                    // (we could check using bruteforce but it's probably not worth it)
                    misses += 1;
                }
                Some(false) => {
                    // Probably no, unless this chunk has more than one dungeon
                    misses += 1;
                }
                Some(true) => {
                    // Probably yes
                    matches += 1;
                }
            }

            if misses >= 3 && matches == 0 {
                continue 'nextseed;
            }
        }

        log::info!("{} {}/{}", world_seed, matches, expected_matches);
        r.push(world_seed);
    }

    r
}

/// Mutate the RNG in the same way as the code that tries to generate water lakes.
/// This is always correct, unlike advance_lava_lake
pub fn advance_water_lake(r: &mut JavaRng) {
    if r.next_int_n(4) == 0 {
        // 3 calls to next
        // TODO: this could be 4 calls if we implement last_next_int_n(4)
        r.next_n_calls(3);
        let s = r.next_int_n(4) + 4;
        let s = s as u64;

        // 6 * s calls to next_double
        // 1 call to next_double == 2 calls to next
        // TODO: investigate using a lookup table with the constants, there are only 4 possible
        // values for l and it may be worth to hardcode it instead of calculating it every time
        r.next_n_calls(2 * 6 * s);
    }
}

/// Mutate the RNG in the same way as the code that tries to generate water lakes.
/// This is always correct, unlike advance_lava_lake
/// This is the unoptimized version, advance_water_lake should do exactly the same but faster
pub fn advance_water_lake_safe(r: &mut JavaRng) {
    // 25% chance of attempting to generate lake
    if r.next_int_n(4) == 0 {
        // x = chunk_x * 16
        // z = chunk_z * 16
        let (x, z) = (0, 0);
        // These are the coordinates of the attempted lake
        let _x = x + r.next_int_n(16) + 8;
        let _y = r.next_int_n(128);
        let _z = z + r.next_int_n(16) + 8;

        // I guess this is the radius or something
        let s = r.next_int_n(4) + 4;

        for _ in 0..s {
            r.next_double();
            r.next_double();
            r.next_double();
            r.next_double();
            r.next_double();
            r.next_double();
        }

        // Now there are some checks to determine if the lake actually generates, but they do not
        // modify the rng. In the case of lava lakes, they do modify the rng.
        // In the case of lava lakes, between 0 and 16 * 16 * 4 calls to nextInt(2)
    }
}

/// The different blocks that can be found at the floor of a dungeon
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DungeonFloorTile {
    /// The dungeon generated with a hole in the floor. This is rare, but possible.
    Air,
    /// Normal cobblestone.
    Cobble,
    /// Mossy cobblestone.
    Mossy,
    /// Unknown: not sure. Use this if the dungeon is already explored and there are some missing
    /// blocks.
    Unknown,
}

/// There are 4 possible dungeon sizes. This is the size of the floor, including the walls.
/// For example, a 9x9 dungeon should have 81 blocks at the floor level, and 7x7 blocks of air at
/// the other levels (+1 block of cobblestone wall in each direction, 8 * 4 = 32, and 32 + 49 = 81)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DungeonSize {
    X7Z7, // 7x7
    X7Z9, // 7x9
    X9Z7, // 9x7
    X9Z9, // 9x9
}

impl DungeonSize {
    fn floor_area(&self) -> u8 {
        match self {
            DungeonSize::X7Z7 => 7 * 7,
            DungeonSize::X7Z9 => 7 * 9,
            DungeonSize::X9Z7 => 9 * 7,
            DungeonSize::X9Z9 => 9 * 9,
        }
    }

    fn next_ints(&self) -> (u8, u8) {
        match self {
            DungeonSize::X7Z7 => (0, 0),
            DungeonSize::X7Z9 => (0, 1),
            DungeonSize::X9Z7 => (1, 0),
            DungeonSize::X9Z9 => (1, 1),
        }
    }

    /// Find the dungeon size given the coordinates of two corners inside the dungeon.
    ///
    /// Input: two coordinates in the format ((x1, z1), (x2, z2)) that correspond to any two opposite
    /// corners of the dungeon. Note that this are the corners before breaking the walls, so if the
    /// dungeon has no walls, imagine there is a wall on the 1-block margin of the floor.
    ///
    /// Example:
    ///
    /// ```
    /// use slime_seed_finder::population::DungeonSize;
    /// // This dungeon should be 7x5 in size
    /// let ds = DungeonSize::from_corners_inside((34, 156), (28, 160));
    /// // But the floor is 9x7
    /// assert_eq!(ds, DungeonSize::X9Z7);
    /// ```
    pub fn from_corners_inside(c1: (i32, i32), c2: (i32, i32)) -> Self {
        let f = |a: i32, b: i32| match (a - b).abs() {
            4 => 0,
            6 => 1,
            d => panic!(
                "Invalid dungeon size: {}. Coordinates: {:?} {:?}",
                d, c1, c2
            ),
        };

        match (f(c1.0, c2.0), f(c1.1, c2.1)) {
            (0, 0) => DungeonSize::X7Z7,
            (0, 1) => DungeonSize::X7Z9,
            (1, 0) => DungeonSize::X9Z7,
            (1, 1) => DungeonSize::X9Z9,
            _ => unreachable!(),
        }
    }
}

pub struct MossyFloor {
    size: DungeonSize,
    tiles: Vec<DungeonFloorTile>,
}

impl MossyFloor {
    /// Parse mossy floor from human readable string
    ///
    /// ```ignore
    /// MMMCCAC??
    /// MMCMCCM??
    /// ```
    ///
    /// M = Mossy cobblestone
    /// C = Normal cobblestone
    /// A = Air with no block under it
    /// ? = Unknown
    ///
    /// Orientation: the most positive x, z corner must be at the bottom left
    /// This is because the algorithm starts from the most negative x, z corner which is the top
    /// right:
    ///
    /// ```ignore
    /// 7654321
    /// .....98
    /// ```
    pub fn parse(x: &str) -> Result<Self, String> {
        let mut line_size = None;
        let mut tiles = vec![];
        let mut num_lines = 0;
        for line in x.split_terminator(|c| c == '\n' || c == ';') {
            if let Some(line_size) = line_size {
                if line.len() != line_size {
                    return Err(format!("All lines should have the same length"));
                }
            } else {
                match line.len() {
                    7 | 9 => {}
                    x => return Err(format!("Invalid line length {}, should be 7 or 9", x)),
                }
                line_size = Some(line.len());
            }
            let mut ts = vec![];
            for c in line.chars() {
                let t = match c {
                    'A' => DungeonFloorTile::Air,
                    'C' => DungeonFloorTile::Cobble,
                    'M' => DungeonFloorTile::Mossy,
                    '?' => DungeonFloorTile::Unknown,
                    c => return Err(format!("Invalid tile: {:?}", c)),
                };
                ts.push(t);
            }
            ts.reverse();
            tiles.extend(ts);
            num_lines += 1;
        }

        match num_lines {
            7 | 9 => {}
            x => return Err(format!("Invalid number of lines {}, should be 7 or 9", x)),
        }

        let size = match (num_lines, line_size.unwrap()) {
            (7, 7) => DungeonSize::X7Z7,
            (7, 9) => DungeonSize::X7Z9,
            (9, 7) => DungeonSize::X9Z7,
            (9, 9) => DungeonSize::X9Z9,
            _ => unreachable!(),
        };

        Ok(Self { size, tiles })
    }

    fn all_mossy(size: DungeonSize) -> Self {
        Self {
            size,
            tiles: vec![DungeonFloorTile::Mossy; size.floor_area() as usize],
        }
    }
}

/// Find a rng state that will generate the dungeon with the correct chunk offset coordinates,
/// size, and floor.
///
/// range: [0, 1 << 40)
pub fn dungeon_rng_bruteforce_range(
    (x, y, z): (u8, u8, u8),
    floor: &MossyFloor,
    range_lo: u64,
    range_hi: u64,
) -> Vec<JavaRng> {
    let (wx, wz) = floor.size.next_ints();
    assert!(range_hi <= (1 << 40));
    let mut v = vec![];
    'nextseed: for seed in range_lo..range_hi {
        // Given a RNG that will return x on r.next_int_n(128),
        // return the RNG that will generate (x, y, z) or (x, z, y)
        // And mutates the input r so that r.next_int_n(2) == wx
        // TODO: benchmark this function returning Vec vs returning ArrayVec vs returning iterator
        let check_any_version = || -> Vec<(JavaRng, JavaRng)> {
            let mut v = vec![];
            // The first step of the algorithm is slightly different starting from Minecraft 1.7
            for version in 0.. {
                match version {
                    0 => {
                        if y >= 128 {
                            continue;
                        }
                        // Before Minecraft 1.7, the y coordinate uses nextInt(128), therefore we
                        // can only use 7 bits of y, not 8. Here we bruteforce the one extra bit
                        for t in 0..=1 {
                            // Use with_raw_seed using y as the top bits of the seed. This is because the value of the
                            // internal seed is the result of the previous call to next. So this way we assure that the
                            // y coordinate is correct, and we save 7 bits to bruteforce. We still need 41 extra bits
                            let mut r = JavaRng::with_raw_seed(((y as u64) << (48 - 7)) | (t << 40) | seed);
                            // r.next_int would return the z coordinate, so go back 2 times
                            r.previous_n_calls(2);
                            let r_clone = r;
                            if r.next_int_n(16) != x as i32 {
                                continue;
                            }
                            if r.next_int_n(128) != y as i32 {
                                unreachable!();
                            }
                            if r.next_int_n(16) != z as i32 {
                                continue;
                            }
                            v.push((r_clone, r));
                        }
                    }
                    1 => {
                        // Starting from Minecraft 1.7, the y coordinate uses next_int_n(256)
                        // Therefore we only need to bruteforce 40 bits.
                        let mut r = JavaRng::with_raw_seed(((y as u64) << (48 - 8)) | seed);
                        // r.next_int would return the z coordinate, so go back 2 times
                        r.previous_n_calls(2);
                        let r_clone = r;
                        if r.next_int_n(16) != x as i32 {
                            continue;
                        }
                        if r.next_int_n(256) != y as i32 {
                            unreachable!();
                        }
                        if r.next_int_n(16) != z as i32 {
                            continue;
                        }
                        v.push((r_clone, r));
                    }
                    // In theory this is all that's needed to support 1.13 and 1.15, but each
                    // additional version makes the bruteforce slower. And since the conversion
                    // from 1.13 dungeon seed to world seed is not implemented yet, it doesnt make
                    // much sense to support this.
                    // But we could add a mc_version parameter to this function and enable this.
                    /*
                    2 => {
                        // Starting from Minecraft 1.13, the dungeon coordinates x and z need to be
                        // added +8.
                        // TODO: check if chunk coordinates also need to be modified
                        let x = (x + 8) & 0xF;
                        let z = (z + 8) & 0xF;
                        let mut r = JavaRng::with_raw_seed(((y as u64) << (48 - 8)) | seed);

                        // r.next_int would return the z coordinate, so go back 2 times
                        r.previous_n_calls(2);
                        let r_clone = r;
                        if r.next_int_n(16) != x as i32 {
                            continue;
                        }
                        if r.next_int_n(256) != y as i32 {
                            unreachable!();
                            continue;
                        }
                        if r.next_int_n(16) != z as i32 {
                            continue;
                        }
                        v.push((r_clone, r));
                    }
                    3 => {
                        // Starting from Minecraft 1.15, the order of y and z is swapped
                        // So try the new order if the old fails
                        let x = (x + 8) & 0xF;
                        let z = (z + 8) & 0xF;
                        let mut r = JavaRng::with_raw_seed(((y as u64) << (48 - 8)) | seed);

                        // r.next_int would return wx, so go back 3 times
                        r.previous_n_calls(3);
                        let r_clone = r;
                        if r.next_int_n(16) != x as i32 {
                            continue;
                        }
                        if r.next_int_n(16) != z as i32 {
                            continue;
                        }
                        if r.next_int_n(256) != y as i32 {
                            unreachable!();
                            continue;
                        }
                        v.push((r_clone, r));
                    }
                    */
                    _ => break,
                }
            }

            return v;
        };

        // r_clone.get_seed() is what we refer to as "dungeon seed": it will generate the correct
        // dungeon starting from the x value
        for (r_clone, mut r) in check_any_version() {
            if r.next_int_n(2) != wx as i32 {
                continue;
            }
            if r.next_int_n(2) != wz as i32 {
                continue;
            }

            // Check floor
            'nexttile: for mt in &floor.tiles {
                match mt {
                    DungeonFloorTile::Mossy => {
                        if r.next_int_n(4) == 0 {
                            continue 'nextseed;
                        }
                    }
                    DungeonFloorTile::Cobble => {
                        if r.next_int_n(4) != 0 {
                            continue 'nextseed;
                        }
                    }
                    DungeonFloorTile::Unknown => {
                        // This is only correct when the tile was originally Mossy or Cobble
                        // If the tile was Air, it will give the wrong result
                        // Since the probability of the tile being Air is small, we assume it never
                        // happens
                        r.next_int_n(4);
                    }
                    DungeonFloorTile::Air => {}
                }
            }

            // Everything correct, this is a candidate seed
            println!("Found candidate dungeon seed {}", r_clone.get_seed());
            v.push(r_clone);
        }
    }

    v
}

// This algorithm works for versions
// Alpha v1.2.6 - Beta ???
// 0.2.8 - 1.6.6_01
// If you remove the lakes, it also works for
// Probably since Java Edition Infdev 20100625-2 when dungeons were first introduced
// Alpha v1.0.4 - Alpha v1.2.5
// 0.2.1 - 0.2.7
/// This more generic method should work for any version up to and including 1.12 (I think)
/// In 1.13 the round_to_odd function was modified
///
/// Returns the rng that will generate the dungeon coords on its next 3 calls to next_int, and an
/// index that can be used with rng.previous_n_calls(index) to recover the original rng seeded with
/// the chunk population seed
///
/// seq contains the sequence of skips to check.
/// For example, if every 5 calls to next_int do one dungeon check, we may want to check only calls
/// [0, 5, 10, 15, 20, 25, 30, 35]
/// These need to be encoded as the difference, so for example `repeat(5).take(8)`
/// But if there is something generated before the dungeons that can leave the rng in any state
/// between 1 and 100 skips, then it may be easier to just check all the calls:
/// [0, 1, 2, 3, ...]
/// This can be encoded as `repeat(0).take(135)`
/// The number of elements in the iterator will be the number of dungeon checks before giving up
pub fn populate_any_version_check_dungeon<I>(
    world_seed: i64,
    dungeon: (i32, i32, i32),
    seq: I,
) -> Option<(JavaRng, u64)>
where
    I: IntoIterator<Item = u64>,
{
    let (dungeon_x, dungeon_y, dungeon_z) = dungeon;
    let Chunk {
        x: chunk_x,
        z: chunk_z,
    } = spawner_coordinates_to_chunk(dungeon_x as i64, dungeon_z as i64);

    let x_offset = chunk_x * 16;
    let z_offset = chunk_z * 16;

    let chunk_seed = world_seed_to_chunk_population_seed(world_seed, chunk_x, chunk_z);
    //let chunk_seed_new = world_seed_to_chunk_population_seed_1_14(world_seed, chunk_x, chunk_z);
    let mut r = JavaRng::with_seed(chunk_seed as u64);

    // Undo the first +1 from next_n_calls(s + 1)
    r.previous();
    let mut n_calls: u64 = !0;

    for s in seq {
        // TODO: if s == 0 we can reuse old_z and old_y as new_y and new_x
        n_calls = n_calls.wrapping_add(s + 1);
        r.next_n_calls(s + 1);
        let r_clone = r.clone();
        let mut r2 = r.clone();
        let x = x_offset + r2.next_int_n(16) + 8;
        let y = r2.next_int_n(128);
        let z = z_offset + r2.next_int_n(16) + 8;
        if x == dungeon_x && y == dungeon_y && z == dungeon_z {
            return Some((r_clone, n_calls));
        }
    }

    None
}

// This is the dungeon generation algorithm in its first implementation
pub fn populate_alpha_1_0_4_check_dungeon(
    world_seed: i64,
    chunk_x: i32,
    chunk_z: i32,
    dungeon: (i32, i32, i32),
) -> Option<JavaRng> {
    let (dungeon_x, dungeon_y, dungeon_z) = dungeon;

    let x_offset = chunk_x * 16;
    let z_offset = chunk_z * 16;

    let chunk_seed = world_seed_to_chunk_population_seed(world_seed, chunk_x, chunk_z);
    let mut r = JavaRng::with_seed(chunk_seed as u64);

    // 8 dungeon tries
    for _ in 0..8 {
        let r_clone = r.clone();
        let x = x_offset + r.next_int_n(16) + 8;
        let y = r.next_int_n(128);
        let z = z_offset + r.next_int_n(16) + 8;
        if x == dungeon_x && y == dungeon_y && z == dungeon_z {
            return Some(r_clone);
        }
        //println!("Would generate dungeon at {} {} {}", l1, i2, j2);
        // new WorldGenDungeons
        // 2 calls to next_int_n(2) if there is no dungeon
        r.next_int_n(2);
        r.next_int_n(2);
    }

    None
}

// This is the same as populate_alpha_1_0_4 but with lakes before dungeons
// None: maybe
// true: probably yes
// false: probably no
// dungeon: spawner coordinates
pub fn populate_alpha_1_2_6_check_dungeon(
    world_seed: i64,
    chunk_x: i32,
    chunk_z: i32,
    dungeon: (i32, i32, i32),
) -> Option<bool> {
    let (dungeon_x, dungeon_y, dungeon_z) = dungeon;

    let x_offset = chunk_x * 16;
    let z_offset = chunk_z * 16;

    let chunk_seed = world_seed_to_chunk_population_seed(world_seed, chunk_x, chunk_z);
    let mut r = JavaRng::with_seed(chunk_seed as u64);

    advance_water_lake(&mut r);

    if r.next_int_n(8) == 0 {
        let _x = x_offset + r.next_int_n(16) + 8;
        // TODO: there is a very small probability that the call to next_int_n(120) will result in
        // more than 1 call to next, because of modulo bias
        let temp = r.next_int_n(120) + 8;
        let lava_y = r.next_int_n(temp);
        let _z = z_offset + r.next_int_n(16) + 8;

        // TODO: lava lakes with y >= 64 have a 10% chance of generating
        // This has the exact same problem as the slime chunk algorithm
        if lava_y < 64 || r.next_int_n(10) == 0 {
            // new WorldGenLayes lava
            return None;
            //advance_lakes(&mut r, true);
        }
    }

    // 8 dungeon tries
    for _ in 0..8 {
        //println!("Dungeon seed: {}", r.get_seed());
        let x = x_offset + r.next_int_n(16) + 8;
        let y = r.next_int_n(128);
        let z = z_offset + r.next_int_n(16) + 8;
        if x == dungeon_x && y == dungeon_y && z == dungeon_z {
            return Some(true);
        }
        //println!("Would generate dungeon at {} {} {}", l1, i2, j2);
        // TODO: this may mutate r
        // new WorldGenDungeons
        // 2 calls to next_int_n(2) if there is no dungeon
        r.next_int_n(2);
        r.next_int_n(2);
    }

    Some(false)
}

/// Convert the block coordinates of a dungeon spawner to the coordinates of the chunk that
/// generated that dungeon.
// chunk_x * 16 + random.nextInt(16) + 8
// chunk_x = 0: [8, 8 + 15]
pub fn spawner_coordinates_to_chunk(spawner_x: i64, spawner_z: i64) -> Chunk {
    Chunk::from_point(Point {
        x: spawner_x - 8,
        z: spawner_z - 8,
    })
}

/// Convert the block coordinates of a dungeon spawner to the number returned by the corresponding
/// call to nextInt.
// chunk_x * 16 + random.nextInt(16) + 8
// random.nextInt(128) or random.nextInt(256)
// chunk_z * 16 + random.nextInt(16) + 8
pub fn spawner_coordinates_to_next_int(
    spawner_x: i64,
    spawner_y: i64,
    spawner_z: i64,
) -> (u8, u8, u8) {
    //(((spawner_x - 8) & 0xF) as u8, spawner_y as u8, ((spawner_z - 8) & 0xF) as u8)
    let Chunk {
        x: chunk_x,
        z: chunk_z,
    } = spawner_coordinates_to_chunk(spawner_x, spawner_z);
    (
        (spawner_x - 8 - (chunk_x as i64) * 16) as u8,
        spawner_y as u8,
        (spawner_z - 8 - (chunk_z as i64) * 16) as u8,
    )
}

//  4 =>  5
//  3 =>  3
//  2 =>  3
//  1 =>  1
//  0 =>  1
// -1 =>  1
// -2 => -1
// -3 => -1
// -4 => -3
pub fn round_to_odd(x: i64) -> i64 {
    // x / 2 * 2 + 1
    x.wrapping_div(2).wrapping_mul(2).wrapping_add(1)
}

pub fn reverse_round_to_odd(m: i64) -> Vec<i64> {
    // TODO: we could return empty vector if m is not odd
    debug_assert!((m & 1) == 1);

    if m == 1 {
        vec![-1, 0, 1]
    } else if m > 1 {
        vec![m - 1, m]
    } else if m == i64::MIN + 1 {
        // TODO: is it worth it to handle this edge case?
        // We can just do m.wrapping_sub(2) and return an invalid candidate
        // But it's a candidate so it does not have to be always valid...
        vec![m - 1]
    } else {
        // m < -1
        vec![m - 2, m - 1]
    }
}

pub fn reverse_round_to_odd_bits(m: i64, bits: u8) -> Vec<i64> {
    match bits {
        0 => vec![],
        1 => vec![0, 1],
        64 => reverse_round_to_odd(m),
        bits => {
            let msk = (1i64 << bits).wrapping_sub(1);
            vec![(m - 2) & msk, (m - 1) & msk, m & msk]
        }
    }
}

/// Reverse the operation `m | 1`.
pub fn reverse_bitwise_or_1_bits(m: i64, bits: u8) -> Vec<i64> {
    match bits {
        0 => vec![],
        bits => {
            let msk = (1i64 << bits).wrapping_sub(1);
            vec![m & msk, (m ^ 1) & msk]
        }
    }
}

/// Reverse both `round_to_odd(m)` and `m | 1`.
// This is identical to  reverse_round_to_odd_bits expect when bits = 64
pub fn reverse_round_to_odd_bits_any_version(m: i64, bits: u8) -> Vec<i64> {
    match bits {
        0 => vec![],
        1 => vec![0, 1],
        bits => {
            let msk = (1i64 << bits).wrapping_sub(1);
            vec![(m - 2) & msk, (m - 1) & msk, m & msk]
        }
    }
}


/// Given a world seed, calculate the chunk population seed for the given chunk coordinates.
/// Works for versions <= Minecraft Java 1.12.
/// For 1.13 and above, use world_seed_to_chunk_population_seed_1_13.
pub fn world_seed_to_chunk_population_seed(world_seed: i64, chunk_x: i32, chunk_z: i32) -> u64 {
    let mut r = JavaRng::with_seed(world_seed as u64);

    let m = round_to_odd(r.next_long() as i64);
    let n = round_to_odd(r.next_long() as i64);

    // (x * m + z * n) ^ world_seed
    (((chunk_x as i64)
        .wrapping_mul(m)
        .wrapping_add((chunk_z as i64).wrapping_mul(n)))
        ^ world_seed) as u64
}

/// Given a world seed, calculate the chunk population seed for the given chunk coordinates.
/// Works for versions >= Minecraft Java 1.13.
/// For 1.12 and below, use world_seed_to_chunk_population_seed.
pub fn world_seed_to_chunk_population_seed_1_13(
    world_seed: i64,
    chunk_x: i32,
    chunk_z: i32,
) -> u64 {
    let mut r = JavaRng::with_seed(world_seed as u64);

    // This is much easier to reverse than round_to_odd
    // You only need to bruteforce the last bit, which is set to 1 here
    let m = r.next_long() | 1;
    let n = r.next_long() | 1;

    // (x * i1 + z * j1) ^ world_seed
    (((chunk_x as i64)
        .wrapping_mul(m)
        .wrapping_add((chunk_z as i64).wrapping_mul(n)))
        ^ world_seed) as u64
}

// Returns true if all the elements of a slice are different.
// Do not use with large slices.
fn all_unique<T: PartialEq>(a: &[T]) -> bool {
    for (i, x) in a.iter().enumerate() {
        for y in a.iter().skip(i + 1) {
            if x == y {
                return false;
            }
        }
    }

    true
}

/// Given 3 different (population_seed, chunk_x, chunk_z), find world_seed:
///
/// ```ignore
/// population_seed1 = (x1 * M + z1 * N) ^ world_seed
/// population_seed2 = (x2 * M + z2 * N) ^ world_seed
/// population_seed3 = (x3 * M + z3 * N) ^ world_seed
///
/// p12 = population_seed1 ^ population_seed2
/// p23 = population_seed2 ^ population_seed3
/// p12 = (x1 * M + z1 * N) ^ (x2 * M + z2 * N)
/// p13 = (x1 * M + z1 * N) ^ (x3 * M + z3 * N)
/// ```
///
/// Using p12 and p13, we can bruteforce M and N one bit at a time, meaning that instead of
/// bruteforcing the 2^48 possible world seeds, we only need to check the 4 combinations of (M, N)
/// 48 times. This is possible because both p12 and p13 are a function of two unknowns: (M, N) and
/// to calculate the lower k bits of p12 you only need the lower k bits of M and N.
// TODO: I think I missed the fact that N = f(M), that we can calculate some bits of N if we
// know some bits of M. Investigate this, it should be possible to implement a function like this
// that only needs 2 chunk population seeds, and maybe even 1.
pub fn chunk_population_seed_to_world_seed(
    i1: (u64, i32, i32),
    i2: (u64, i32, i32),
    i3: (u64, i32, i32),
) -> Vec<i64> {
    let (p1, x1, z1) = i1;
    let (p2, x2, z2) = i2;
    let (p3, x3, z3) = i3;

    // TODO: remove this check?
    if !all_unique(&[(x1, z1), (x2, z2), (x3, z3)]) {
        panic!("Input chunks must be different, otherwise this function explodes quadratically. Found inputs: {:?}", (i1, i2, i3));
    }

    let p1 = p1 as i64 & ((1 << 48) - 1);
    let p2 = p2 as i64 & ((1 << 48) - 1);
    let p3 = p3 as i64 & ((1 << 48) - 1);

    // p12 = (x1 * M + z1 * N) ^ (x2 * M + z2 * N)
    // p13 = (x1 * M + z1 * N) ^ (x3 * M + z3 * N)
    let p12 = p1 ^ p2;
    let p13 = p1 ^ p3;

    // The constants must be odd
    let m = 1;
    let n = 1;

    fn check_mn(
        mn: &[(i64, i64)],
        i: u8,
        x1z1: (i32, i32),
        x2z2: (i32, i32),
        x3z3: (i32, i32),
        p12: i64,
        p13: i64,
    ) -> Vec<(i64, i64)> {
        let (x1, z1) = x1z1;
        let (x2, z2) = x2z2;
        let (x3, z3) = x3z3;
        let (x1, z1) = (x1 as i64, z1 as i64);
        let (x2, z2) = (x2 as i64, z2 as i64);
        let (x3, z3) = (x3 as i64, z3 as i64);
        let mut valid_mn = vec![];
        let msk = (1i64 << i).wrapping_sub(1);

        for (mut m, mut n) in mn.iter().cloned() {
            let e12_00 = x1.wrapping_mul(m).wrapping_add(z1.wrapping_mul(n))
                ^ x2.wrapping_mul(m).wrapping_add(z2.wrapping_mul(n));
            let e13_00 = x1.wrapping_mul(m).wrapping_add(z1.wrapping_mul(n))
                ^ x3.wrapping_mul(m).wrapping_add(z3.wrapping_mul(n));

            if ((e12_00 & msk) == (p12 & msk)) && ((e13_00 & msk) == (p13 & msk)) {
                valid_mn.push((m, n));
            }

            n |= 1 << i;
            let e12_01 = x1.wrapping_mul(m).wrapping_add(z1.wrapping_mul(n))
                ^ x2.wrapping_mul(m).wrapping_add(z2.wrapping_mul(n));
            let e13_01 = x1.wrapping_mul(m).wrapping_add(z1.wrapping_mul(n))
                ^ x3.wrapping_mul(m).wrapping_add(z3.wrapping_mul(n));

            if ((e12_01 & msk) == (p12 & msk)) && ((e13_01 & msk) == (p13 & msk)) {
                valid_mn.push((m, n));
            }

            m |= 1 << i;
            let e12_11 = x1.wrapping_mul(m).wrapping_add(z1.wrapping_mul(n))
                ^ x2.wrapping_mul(m).wrapping_add(z2.wrapping_mul(n));
            let e13_11 = x1.wrapping_mul(m).wrapping_add(z1.wrapping_mul(n))
                ^ x3.wrapping_mul(m).wrapping_add(z3.wrapping_mul(n));

            if ((e12_11 & msk) == (p12 & msk)) && ((e13_11 & msk) == (p13 & msk)) {
                valid_mn.push((m, n));
            }

            n &= !(1 << i);
            let e12_10 = x1.wrapping_mul(m).wrapping_add(z1.wrapping_mul(n))
                ^ x2.wrapping_mul(m).wrapping_add(z2.wrapping_mul(n));
            let e13_10 = x1.wrapping_mul(m).wrapping_add(z1.wrapping_mul(n))
                ^ x3.wrapping_mul(m).wrapping_add(z3.wrapping_mul(n));

            if ((e12_10 & msk) == (p12 & msk)) && ((e13_10 & msk) == (p13 & msk)) {
                valid_mn.push((m, n));
            }
        }

        valid_mn
    }

    let mut cc = vec![(m, n)];
    // For each unknown bit
    for i in 1..48 {
        let new_cc = check_mn(&cc, i, (x1, z1), (x2, z2), (x3, z3), p12, p13);
        //println!("{}: {:?}", i, new_cc);
        //println!("{}: {:?}", i, new_cc.len());
        cc = new_cc;
    }

    let ws = cc
        .into_iter()
        .map(|(m, n)| {
            let vm: Vec<_> = reverse_round_to_odd_bits_any_version(m, 48)
                .into_iter()
                .flat_map(|x| JavaRng::extend_long_48(x as u64))
                .map(|x| x as i64)
                .collect();
            // TODO: maybe we can avoid this call to reverse and check if
            // round_to_odd(r.next_long()) == n
            let vn: Vec<_> = reverse_round_to_odd_bits_any_version(n, 48)
                .into_iter()
                .flat_map(|x| JavaRng::extend_long_48(x as u64))
                .map(|x| x as i64)
                .collect();

            (vm, vn)
        })
        .flat_map(|(vm, vn)| {
            let mut ws = vec![];
            for m in vm {
                // This cannot fail because we created m using extend_long_48
                let mut r = JavaRng::create_from_long(m as u64).unwrap();
                let world_seed = r.get_seed() as i64;
                let _m = r.next_long();
                let n = r.next_long();

                if vn.contains(&n) {
                    ws.push(world_seed);
                }
            }

            ws
        })
        .collect();

    ws
}

/// Slightly more efficient version of chunk_population_seed_to_world_seed that can only be used if
/// you know the full 64 bits of the chunk population seed. Since the JavaRng only uses 48 bits,
/// getting 64 bits should not be possible in practice. Still, it may be useful in tests.
pub fn chunk_population_seed_to_world_seed_64(
    i1: (i64, i32, i32),
    i2: (i64, i32, i32),
    i3: (i64, i32, i32),
) -> Vec<i64> {
    let (p1, x1, z1) = i1;
    let (p2, x2, z2) = i2;
    let (p3, x3, z3) = i3;

    if !all_unique(&[(x1, z1), (x2, z2), (x3, z3)]) {
        panic!("Input chunks must be different, otherwise this function explodes quadratically. Found inputs: {:?}", (i1, i2, i3));
    }

    // p12 = (x1 * M + z1 * N) ^ (x2 * M + z2 * N)
    // p13 = (x1 * M + z1 * N) ^ (x3 * M + z3 * N)
    let p12 = p1 ^ p2;
    let p13 = p1 ^ p3;

    // The constants must be odd
    let m = 1;
    let n = 1;

    fn check_mn(
        mn: &[(i64, i64)],
        i: u8,
        x1z1: (i32, i32),
        x2z2: (i32, i32),
        x3z3: (i32, i32),
        p12: i64,
        p13: i64,
    ) -> Vec<(i64, i64)> {
        let (x1, z1) = x1z1;
        let (x2, z2) = x2z2;
        let (x3, z3) = x3z3;
        let (x1, z1) = (x1 as i64, z1 as i64);
        let (x2, z2) = (x2 as i64, z2 as i64);
        let (x3, z3) = (x3 as i64, z3 as i64);
        let mut valid_mn = vec![];
        let msk = (1i64 << i).wrapping_sub(1);

        for (mut m, mut n) in mn.iter().cloned() {
            let e12_00 = x1.wrapping_mul(m).wrapping_add(z1.wrapping_mul(n))
                ^ x2.wrapping_mul(m).wrapping_add(z2.wrapping_mul(n));
            let e13_00 = x1.wrapping_mul(m).wrapping_add(z1.wrapping_mul(n))
                ^ x3.wrapping_mul(m).wrapping_add(z3.wrapping_mul(n));

            if ((e12_00 & msk) == (p12 & msk)) && ((e13_00 & msk) == (p13 & msk)) {
                valid_mn.push((m, n));
            }

            n |= 1 << i;
            let e12_01 = x1.wrapping_mul(m).wrapping_add(z1.wrapping_mul(n))
                ^ x2.wrapping_mul(m).wrapping_add(z2.wrapping_mul(n));
            let e13_01 = x1.wrapping_mul(m).wrapping_add(z1.wrapping_mul(n))
                ^ x3.wrapping_mul(m).wrapping_add(z3.wrapping_mul(n));

            if ((e12_01 & msk) == (p12 & msk)) && ((e13_01 & msk) == (p13 & msk)) {
                valid_mn.push((m, n));
            }

            m |= 1 << i;
            let e12_11 = x1.wrapping_mul(m).wrapping_add(z1.wrapping_mul(n))
                ^ x2.wrapping_mul(m).wrapping_add(z2.wrapping_mul(n));
            let e13_11 = x1.wrapping_mul(m).wrapping_add(z1.wrapping_mul(n))
                ^ x3.wrapping_mul(m).wrapping_add(z3.wrapping_mul(n));

            if ((e12_11 & msk) == (p12 & msk)) && ((e13_11 & msk) == (p13 & msk)) {
                valid_mn.push((m, n));
            }

            n &= !(1 << i);
            let e12_10 = x1.wrapping_mul(m).wrapping_add(z1.wrapping_mul(n))
                ^ x2.wrapping_mul(m).wrapping_add(z2.wrapping_mul(n));
            let e13_10 = x1.wrapping_mul(m).wrapping_add(z1.wrapping_mul(n))
                ^ x3.wrapping_mul(m).wrapping_add(z3.wrapping_mul(n));

            if ((e12_10 & msk) == (p12 & msk)) && ((e13_10 & msk) == (p13 & msk)) {
                valid_mn.push((m, n));
            }
        }

        valid_mn
    }

    let mut cc = vec![(m, n)];
    // For each unknown bit
    for i in 1..64 {
        let new_cc = check_mn(&cc, i, (x1, z1), (x2, z2), (x3, z3), p12, p13);
        println!("{}: {:?}", i, new_cc);
        //println!("{}: {:?}", i, new_cc.len());
        cc = new_cc;
    }

    let cc_no_odd: Vec<(i64, i64)> = cc
        .into_iter()
        .flat_map(|(m, n)| {
            let tuple_combinations = |a: Vec<i64>, b: Vec<i64>| {
                let mut v = Vec::with_capacity(a.len() * b.len());
                for x in a {
                    for y in &b {
                        v.push((x.clone(), y.clone()));
                    }
                }
                v
            };

            let vm = reverse_round_to_odd(m);
            let vn = reverse_round_to_odd(n);

            tuple_combinations(vm, vn)
        })
        .collect();
    println!("Reversed round_to_odd: {} candidates", cc_no_odd.len());

    let mut ws = vec![];
    for (m, n) in cc_no_odd {
        let mut r = match JavaRng::create_from_long(m as u64) {
            Some(r) => r,
            None => {
                println!("{:?} cannot create_from_long", (m, n));
                continue;
            }
        };

        let world_seed = r.get_seed() as i64;

        assert_eq!(r.next_long(), m);
        let should_be_n = r.next_long();

        if should_be_n == n {
            ws.push(world_seed);
        } else {
            println!("{:?} n does not match: {}", (m, n), should_be_n);
            continue;
        }
    }

    ws
}

/// Given only one (population_seed, chunk_x, chunk_z), find world_seed:
///
/// population_seed = (x * i1 + z * j1) ^ world_seed
fn chunk_population_seed_to_world_seed_one(
    population_seed: i64,
    chunk_x: i32,
    chunk_z: i32,
) -> Vec<i64> {
    // Trivial case
    if chunk_x == 0 && chunk_z == 0 {
        return vec![population_seed];
    }
    if chunk_x == chunk_z {
        // (x * (i1 + j1)) ^ world_seed
        // i1j1 = i1 + j1
        // i1j1 = round_to_odd(r.next_long()) + round_to_odd(r.next_long())
        // i1j1 is always even
    }
    if chunk_z == 0 {
        // let r = JavaRng::with_seed(world_seed)
        // i1 = round_to_odd(r.next_long())
        // (x * i1) ^ world_seed
        /*
        let mut s = 0;
        loop {
            let mut r = JavaRng::with_raw_seed(s);
            let i1 = round_to_odd(r.next_long());
            let cand = (x * i1) ^ world_seed;
            if cand == population_seed ^ lcg_const::A {

            }
        }
        */
    }
    // mod 2:
    // population_seed = (x * i1 + z * j1) ^ world_seed
    // population_seed = (x * 1 + z * 1) ^ world_seed
    // population_seed = (x + z) ^ world_seed
    // population_seed = (x ^ z) ^ world_seed
    // world_seed = x ^ z ^ population_seed
    unimplemented!();
}

/// Use 3 different (dungeon_seed, chunk_x, chunk_z) to find the world seed
/// The algorithm works by bruteforcing all the possible chunk population seeds given a dungeon
/// seed. This is accomplished by using `r.previous()` to reverse the population process, and then
/// using chunk_population_seed_to_world_seed to find the world seed.
pub fn dungeon_seed_to_world_seed_alpha_1_2_6(
    i1: (u64, i32, i32),
    i2: (u64, i32, i32),
    i3: (u64, i32, i32),
) -> Vec<i64> {
    let (s1, x1, z1) = i1;
    let (s2, x2, z2) = i2;
    let (s3, x3, z3) = i3;
    let p1d = JavaRng::with_seed(s1);
    let p2d = JavaRng::with_seed(s2);
    let p3d = JavaRng::with_seed(s3);

    // Max number of calls to previous:
    // 5 per dungeon try, 7 dungeon tries max
    // 3 + 12 * 7 + 1 per water lake gen
    // not counted: 4 + 12 * 7 + 1 + unknown per lava lake gen
    // 5 error margin
    // (the two next_int used to check if generate water lake and lava lake are always counted)
    let l = 5 * 7 + 3 + 12 * 7 + 1 + 5;

    let mut p1 = p1d;
    // Always call previous 2 times first because of the two lake checks
    p1.previous();
    for _ in 0..=l {
        p1.previous();
        let mut p2 = p2d;
        p2.previous();
        for _ in 0..=l {
            p2.previous();
            let mut p3 = p3d;
            p3.previous();
            for _ in 0..=l {
                p3.previous();

                let seeds = chunk_population_seed_to_world_seed(
                    (p1.get_seed(), x1, z1),
                    (p2.get_seed(), x2, z2),
                    (p3.get_seed(), x3, z3),
                );

                if seeds.len() > 0 {
                    return seeds;
                }
            }
        }
    }

    vec![]
}

/// Use 3 different (dungeon_seed, chunk_x, chunk_z, max_calls_to_previous) to find the world seed
/// The algorithm works by bruteforcing all the possible chunk population seeds given a dungeon
/// seed. This is accomplished by using `r.previous()` to reverse the population process, and then
/// using chunk_population_seed_to_world_seed to find the world seed.
///
/// The value of max_calls_to_previous depends on the minecraft version and the other features of
/// the chunk. For example, a water lake in the same chunk as the dungeon increases the limit by
/// 88. 128 is considered to be a safe limit, but if the chunk contains a lava lake, the limit can
/// increase by an additional 1024 in the worst case scenario.
// TODO: the number of checks is max_calls_to_previous**3. If we can implement this using 2 dungeon
// seeds instead of 3, the number of check will be max_calls_to_previous**2.
pub fn dungeon_seed_to_world_seed_any_version(
    i1: (u64, i32, i32, u32),
    i2: (u64, i32, i32, u32),
    i3: (u64, i32, i32, u32),
) -> Vec<i64> {
    let (s1, x1, z1, l1) = i1;
    let (s2, x2, z2, l2) = i2;
    let (s3, x3, z3, l3) = i3;
    let p1d = JavaRng::with_seed(s1);
    let p2d = JavaRng::with_seed(s2);
    let p3d = JavaRng::with_seed(s3);

    let mut p1 = p1d;
    // Call next() to cancel out the first call to previous
    p1.next(32);
    for _ in 0..=l1 {
        p1.previous();
        let mut p2 = p2d;
        p2.next(32);
        for _ in 0..=l2 {
            p2.previous();
            let mut p3 = p3d;
            p3.next(32);
            for _ in 0..=l3 {
                p3.previous();

                let seeds = chunk_population_seed_to_world_seed(
                    (p1.get_seed(), x1, z1),
                    (p2.get_seed(), x2, z2),
                    (p3.get_seed(), x3, z3),
                );

                if seeds.len() > 0 {
                    return seeds;
                }
            }
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper: given a rng with dungeon seed, check if it actually generates the expected dungeon
    // This is supposed to be the same algorithm as the one used in Minecraft
    fn check_rng_dungeon(mut r: JavaRng, (x, y, z): (i32, i32, i32), floor: &MossyFloor) {
        let (wx, wz) = floor.size.next_ints();
        let Chunk { x: cx, z: cz } = spawner_coordinates_to_chunk(x as i64, z as i64);
        assert_eq!(cx * 16 + 8 + r.next_int_n(16), x as i32);
        assert_eq!(r.next_int_n(128), y as i32);
        assert_eq!(cz * 16 + 8 + r.next_int_n(16), z as i32);
        let rand_wx = r.next_int_n(2);
        assert_eq!(rand_wx as u8, wx);
        let l = rand_wx + 2;
        let rand_wz = r.next_int_n(2);
        assert_eq!(rand_wz as u8, wz);
        let i1 = rand_wz + 2;

        // Mossy cobblestone madness
        let (i, j, k) = (x, y, z);
        let b0 = 3;
        let block_buildable = |_x, _y, _z| {
            // All blocks are buildable except
            // MaterialLogic: PLANT, ORIENTABLE, SNOW_LAYER
            // MaterialTransparent: AIR, FIRE
            // MaterialLiquid: WATER, LAVA
            //
            // But that's irrelevant, if there is cobblestone (mossy or not) on the ground it means the
            // material was buildable. If there is no cobblestone, then it was not buildable.

            true
        };
        let mut floor_iter = floor.tiles.iter().cloned();
        for k1 in (i - l - 1)..=(i + l + 1) {
            for l1 in ((j - 1)..=(j + b0)).rev() {
                for i2 in (k - i1 - 1)..=(k + i1 + 1) {
                    if k1 != i - l - 1
                        && l1 != j - 1
                        && i2 != k - i1 - 1
                        && k1 != i + l + 1
                        && i1 != j + b0 + 1
                        && i2 != k + i1 + 1
                    {
                        // Set air
                    } else if l1 >= 0 && !block_buildable(k1, l1 - 1, i2) {
                        // Set air
                    } else if block_buildable(k1, l1, i2) {
                        if l1 == j - 1 {
                            match floor_iter.next().unwrap() {
                                DungeonFloorTile::Air => {}
                                DungeonFloorTile::Cobble => {
                                    assert_eq!(r.next_int_n(4), 0);
                                }
                                DungeonFloorTile::Mossy => {
                                    assert_ne!(r.next_int_n(4), 0);
                                }
                                DungeonFloorTile::Unknown => {
                                    r.next_int_n(4);
                                }
                            }
                        } else {
                            // Normal cobblestone walls
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn population_seed_reverse() {
        // Benchmark: approx 2 ms per run with debug logs
        let world_seed = 2;
        let (x1, z1) = (1, 2);
        let (x2, z2) = (4, 3);
        let (x3, z3) = (16, 14);
        let p1 = world_seed_to_chunk_population_seed_1_13(world_seed, x1, z1);
        let p2 = world_seed_to_chunk_population_seed_1_13(world_seed, x2, z2);
        let p3 = world_seed_to_chunk_population_seed_1_13(world_seed, x3, z3);
        let old_p1 = world_seed_to_chunk_population_seed(world_seed, x1, z1);
        let old_p2 = world_seed_to_chunk_population_seed(world_seed, x2, z2);
        let old_p3 = world_seed_to_chunk_population_seed(world_seed, x3, z3);
        // Check that the chunk population seed has changed in 1.13
        assert_ne!(p1, old_p1);
        assert_ne!(p2, old_p2);
        assert_ne!(p3, old_p3);

        // But both the old seed and the new seed results in one candidate, the world seed
        let candidates =
            chunk_population_seed_to_world_seed((p1, x1, z1), (p2, x2, z2), (p3, x3, z3));
        assert_eq!(candidates, vec![world_seed]);
        let old_candidates =
            chunk_population_seed_to_world_seed((old_p1, x1, z1), (old_p2, x2, z2), (old_p3, x3, z3));
        assert_eq!(old_candidates, vec![world_seed]);
    }


    #[test]
    #[should_panic = "Input chunks must be different, otherwise this function explodes quadratically."]
    fn population_seed_reverse_not_all_unique() {
        let world_seed = 1234;
        let (x1, z1) = (1, 2);
        let (x2, z2) = (1, 2);
        let (x3, z3) = (16, 14);
        let p1 = world_seed_to_chunk_population_seed(world_seed, x1, z1);
        let p2 = world_seed_to_chunk_population_seed(world_seed, x2, z2);
        let p3 = world_seed_to_chunk_population_seed(world_seed, x3, z3);

        let _candidates =
            chunk_population_seed_to_world_seed((p1, x1, z1), (p2, x2, z2), (p3, x3, z3));
    }

    #[test]
    fn population_seed_reverse_number_of_steps() {
        let world_seed: i64 = 536274160436487309;
        let dungeon_seed = 107285387795734;
        let (wx, _wy, wz) = (31, 15, 158);
        let Chunk {
            x: chunk_x,
            z: chunk_z,
        } = spawner_coordinates_to_chunk(wx, wz);
        let p1 = world_seed_to_chunk_population_seed(world_seed, chunk_x, chunk_z);

        let steps = || {
            let mut r = JavaRng::with_seed(p1 as u64);
            for i in 0..1000 {
                let should_be_dungeon_seed = r.get_seed();
                if should_be_dungeon_seed == dungeon_seed {
                    return Some(i);
                }
                r.next(1);
            }

            None
        };

        assert_eq!(steps().expect("NOT FOUND"), 37);
    }

    #[test]
    fn rev_round_to_odd() {
        for i in (-100..=100)
            .chain(i64::MIN..=i64::MIN + 10)
            .chain(i64::MAX - 10..=i64::MAX)
        {
            let j = round_to_odd(i);
            let js = reverse_round_to_odd(j);
            assert!(
                js.contains(&i),
                "round_to_odd({}) = {}, not in {:?}",
                i,
                j,
                js
            );
            for jjs in js {
                assert_eq!(
                    round_to_odd(jjs),
                    j,
                    "round_to_odd(reverse_round_to_odd({})) != {} when using candidate {}",
                    j,
                    j,
                    jjs
                );
            }

            let bits = 4;
            let msk = (1 << bits) - 1;
            let mut js = reverse_round_to_odd_bits(j & msk, bits);
            for x in &mut js {
                *x &= msk;
            }
            assert!(
                js.contains(&(i & msk)),
                "round_to_odd_bits({}) = {}, not in {:?}",
                i,
                j,
                js
            );
        }
    }

    #[test]
    fn first_dungeon_test() {
        let dungeon = (31, 15, 158);
        // Dungeon seed obtained using:
        //let world_seed: i64 = 536274160436487309;
        //let Chunk { x: chunk_x, z: chunk_z } = spawner_coordinates_to_chunk(dungeon.0 as i64, dungeon.2 as i64);
        //let r = populate_alpha_1_2_6_check_dungeon(world_seed, chunk_x, chunk_z, dungeon);
        let r_seed: u64 = 107285387795734;

        let floor = MossyFloor::parse(
            "MMCCCMM\n\
             MMMCMMM\n\
             MMMCMCC\n\
             MCMMMMM\n\
             CMMMCMM\n\
             MMMMCMC\n\
             CMMMMCM\n\
             MMCMMMM\n\
             MMCMMMC\n",
        )
        .unwrap();
        check_rng_dungeon(
            JavaRng::with_seed(r_seed),
            (dungeon.0 as i32, dungeon.1 as i32, dungeon.2 as i32),
            &floor,
        );
    }

    #[test]
    fn dungeons_in_alpha_1_2_6() {
        let world_seed = 536274160436487309;
        let dungeon = (31, 15, 158);
        let Chunk {
            x: chunk_x,
            z: chunk_z,
        } = spawner_coordinates_to_chunk(dungeon.0 as i64, dungeon.2 as i64);
        let r = populate_alpha_1_2_6_check_dungeon(world_seed, chunk_x, chunk_z, dungeon);
        assert_eq!(r, Some(true));
        let dungeon = (-47, 43, 115);
        let Chunk {
            x: chunk_x,
            z: chunk_z,
        } = spawner_coordinates_to_chunk(dungeon.0 as i64, dungeon.2 as i64);
        let r = populate_alpha_1_2_6_check_dungeon(world_seed, chunk_x, chunk_z, dungeon);
        assert_eq!(r, Some(true));
        let dungeon = (-55, 40, 47);
        let Chunk {
            x: chunk_x,
            z: chunk_z,
        } = spawner_coordinates_to_chunk(dungeon.0 as i64, dungeon.2 as i64);
        let r = populate_alpha_1_2_6_check_dungeon(world_seed, chunk_x, chunk_z, dungeon);
        assert_eq!(r, Some(true));
        let dungeon = (-104, 54, 70);
        let Chunk {
            x: chunk_x,
            z: chunk_z,
        } = spawner_coordinates_to_chunk(dungeon.0 as i64, dungeon.2 as i64);
        let r = populate_alpha_1_2_6_check_dungeon(world_seed, chunk_x, chunk_z, dungeon);
        assert_eq!(r, Some(true));
        let dungeon = (97, 40, 61);
        let Chunk {
            x: chunk_x,
            z: chunk_z,
        } = spawner_coordinates_to_chunk(dungeon.0 as i64, dungeon.2 as i64);
        let r = populate_alpha_1_2_6_check_dungeon(world_seed, chunk_x, chunk_z, dungeon);
        assert_eq!(r, Some(true));
    }

    #[test]
    fn dng() {
        let world_seed = 90444174810516;
        let candidates = dungeon_seed_finder_range(
            &[
                (-257, 38, -187),
                (-221, 45, -193),
                (-258, 37, -207),
                (-180, 38, 47),
                (-167, 15, -1),
                (-181, 35, -35),
            ],
            MinecraftVersion::JavaBeta,
            world_seed,
            world_seed + 1,
        );
        assert_eq!(candidates, vec![world_seed]);
    }

    #[test]
    fn dmg() {
        let world_seed = 64329802687629;
        let candidates = dungeon_seed_finder_range(
            &[
                (31, 15, 158),
                (-47, 43, 115),
                (-55, 40, 47),
                (-104, 54, 70),
                (97, 40, 61),
            ],
            MinecraftVersion::JavaBeta,
            world_seed,
            world_seed + 1,
        );
        assert_eq!(candidates, vec![world_seed]);
    }

    // Wrapper function because the API changed and I don't want to rewrite the tests
    fn rng_for_dungeon_coord_next_ints(
        seed: &mut u64,
        coords: (u8, u8, u8),
        floor: &MossyFloor,
    ) -> Option<JavaRng> {
        let v = dungeon_rng_bruteforce_range(coords, floor, *seed & ((1 << 40) - 1), (*seed & ((1 << 40) - 1)) + 1);
        *seed = *seed + 1;
        if v.is_empty() {
            None
        } else {
            Some(v[0])
        }
    }

    #[test]
    fn smart_rng_iter_0() {
        // Benchmark: about 2 hours for the full 2^41 bruteforce using one CPU core
        // (31, 15, 158) 1307626838574/281474976710656 dungeon seed: 107285387795734
        //
        let (wx, wy, wz) = (31, 15, 158);
        let (x, y, z) = spawner_coordinates_to_next_int(wx, wy, wz);
        let floor = MossyFloor::parse(
            "MMCCCMM\n\
             MMMCMMM\n\
             MMMCMCC\n\
             MCMMMMM\n\
             CMMMCMM\n\
             MMMMCMC\n\
             CMMMMCM\n\
             MMCMMMM\n\
             MMCMMMC\n",
        )
        .unwrap();
        let mut i = 1307626838573;
        if let Some(r) = rng_for_dungeon_coord_next_ints(&mut i, (x, y, z), &floor) {
            let r_clone = r.clone();
            let r_seed = r.get_seed();
            println!(
                "{:?} {}/{} dungeon seed: {}",
                (wx, wy, wz),
                i,
                1u64 << 48,
                r_seed
            );
            check_rng_dungeon(r_clone, (wx as i32, wy as i32, wz as i32), &floor);
        } else {
            panic!("NOT FOUND");
        }
    }

    #[test]
    fn smart_rng_iter_1() {
        // Benchmark: about 2 hours for the full 2^41 bruteforce using one CPU core
        // (-47, 43, 115) 1671096212817/281474976710656 dungeon seed: 166727685018955
        //
        let (wx, wy, wz) = (-47, 43, 115);
        let (x, y, z) = spawner_coordinates_to_next_int(wx, wy, wz);
        let floor = MossyFloor::parse(
            "MMMMCCM\n\
             CMCMCMC\n\
             MMMCCMC\n\
             MCMMMMC\n\
             MMMMMCM\n\
             MMMMMMC\n\
             MMMMMMM\n",
        )
        .unwrap();
        let mut i = 1671096212816;
        if let Some(r) = rng_for_dungeon_coord_next_ints(&mut i, (x, y, z), &floor) {
            let r_clone = r.clone();
            let r_seed = r.get_seed();
            println!(
                "{:?} {}/{} dungeon seed: {}",
                (wx, wy, wz),
                i,
                1u64 << 48,
                r_seed
            );
            check_rng_dungeon(r_clone, (wx as i32, wy as i32, wz as i32), &floor);
        } else {
            panic!("NOT FOUND");
        }
    }

    #[test]
    fn smart_rng_iter_2() {
        // Benchmark: about 2 hours for the full 2^41 bruteforce using one CPU core
        // (-55, 40, 47) 1970611715798/281474976710656 dungeon seed: 168097064432526
        //
        let (wx, wy, wz) = (-55, 40, 47);
        let (x, y, z) = spawner_coordinates_to_next_int(wx, wy, wz);
        let floor = MossyFloor::parse(
            "MMMMMMMMM\n\
             CMMCCMCCM\n\
             MCMMCMCMM\n\
             MMMCCMMMM\n\
             MMCMMCMMM\n\
             MMMCMMCMM\n\
             MMMMMMCCM\n\
             MMMMMCMMM\n\
             MMMMMCMMC\n",
        )
        .unwrap();
        let mut i = 1970611715797;
        if let Some(r) = rng_for_dungeon_coord_next_ints(&mut i, (x, y, z), &floor) {
            let r_clone = r.clone();
            let r_seed = r.get_seed();
            println!(
                "{:?} {}/{} dungeon seed: {}",
                (wx, wy, wz),
                i,
                1u64 << 48,
                r_seed
            );
            check_rng_dungeon(r_clone, (wx as i32, wy as i32, wz as i32), &floor);
        } else {
            panic!("NOT FOUND");
        }
    }

    #[test]
    fn smart_rng_iter_3() {
        // Benchmark: about 2 hours for the full 2^41 bruteforce using one CPU core
        // (-104, 54, 70) 1200709802363/281474976710656 dungeon seed: 279435442168493
        //
        let (wx, wy, wz) = (-104, 54, 70);
        let (x, y, z) = spawner_coordinates_to_next_int(wx, wy, wz);
        let floor = MossyFloor::parse(
            "CMMMCMMMM\n\
             MCMMMMCMM\n\
             CMMMMMMCM\n\
             MMCMCMMMM\n\
             MMMMMMMMC\n\
             MMMCMMCMM\n\
             MMMMMCCMC\n\
             MCMCMMMCC\n\
             CMCMMMMMC\n",
        )
        .unwrap();
        let mut i = 1200709802362;
        if let Some(r) = rng_for_dungeon_coord_next_ints(&mut i, (x, y, z), &floor) {
            let r_clone = r.clone();
            let r_seed = r.get_seed();
            println!(
                "{:?} {}/{} dungeon seed: {}",
                (wx, wy, wz),
                i,
                1u64 << 48,
                r_seed
            );
            check_rng_dungeon(r_clone, (wx as i32, wy as i32, wz as i32), &floor);
        } else {
            panic!("NOT FOUND");
        }
    }

    #[test]
    fn smart_rng_iter_4() {
        // Benchmark: about 2 hours for the full 2^41 bruteforce using one CPU core
        // (97, 40, 61) 995249637892/281474976710656 dungeon seed: 197502383510412
        //
        let (wx, wy, wz) = (97, 40, 61);
        let (x, y, z) = spawner_coordinates_to_next_int(wx, wy, wz);
        let floor = MossyFloor::parse(
            "MMMMMMM\n\
             MMCCMMM\n\
             MMMMMMM\n\
             CMMMMMC\n\
             MMMMMMC\n\
             MMCMMMM\n\
             MMCCCMC\n",
        )
        .unwrap();
        let mut i = 995249637891;
        if let Some(r) = rng_for_dungeon_coord_next_ints(&mut i, (x, y, z), &floor) {
            let r_clone = r.clone();
            let r_seed = r.get_seed();
            println!(
                "{:?} {}/{} dungeon seed: {}",
                (wx, wy, wz),
                i,
                1u64 << 48,
                r_seed
            );
            check_rng_dungeon(r_clone, (wx as i32, wy as i32, wz as i32), &floor);
        } else {
            panic!("NOT FOUND");
        }
    }

    #[test]
    #[ignore] // Takes a few minutes to run
    fn population_reversal_with_5_seeds() {
        // (31, 15, 158) 1307626838574/281474976710656 dungeon seed: 107285387795734
        // (-47, 43, 115) 1671096212817/281474976710656 dungeon seed: 166727685018955
        // (-55, 40, 47) 1970611715798/281474976710656 dungeon seed: 168097064432526
        // (-104, 54, 70) 1200709802363/281474976710656 dungeon seed: 279435442168493
        // (97, 40, 61) 995249637892/281474976710656 dungeon seed: 197502383510412
        let world_seed = 64329802687629;
        let s1 = 279435442168493;
        let x1 = -104;
        let z1 = 70;
        let s2 = 166727685018955;
        let x2 = -47;
        let z2 = 115;
        let s3 = 168097064432526;
        let x3 = -55;
        let z3 = 47;

        let Chunk { x: x1, z: z1 } = spawner_coordinates_to_chunk(x1, z1);
        let Chunk { x: x2, z: z2 } = spawner_coordinates_to_chunk(x2, z2);
        let Chunk { x: x3, z: z3 } = spawner_coordinates_to_chunk(x3, z3);

        let found_seed =
            dungeon_seed_to_world_seed_alpha_1_2_6((s1, x1, z1), (s2, x2, z2), (s3, x3, z3));

        assert_eq!(found_seed, vec![world_seed]);
    }

    #[test]
    fn spawner_coordinates_basic() {
        let wz = 8;
        for wx in 8 - 32..8 - 16 {
            let Chunk { x, z } = spawner_coordinates_to_chunk(wx, wz);
            assert_eq!(x as i64, -2);
            assert_eq!(z, 0);
        }
        for wx in 8 - 16..8 {
            let Chunk { x, z } = spawner_coordinates_to_chunk(wx, wz);
            assert_eq!(x as i64, -1);
            assert_eq!(z, 0);
        }
        for wx in 8..8 + 16 {
            let Chunk { x, z } = spawner_coordinates_to_chunk(wx, wz);
            assert_eq!(x as i64, 0);
            assert_eq!(z, 0);
        }
        for wx in 8 + 16..8 + 32 {
            let Chunk { x, z } = spawner_coordinates_to_chunk(wx, wz);
            assert_eq!(x as i64, 1);
            assert_eq!(z, 0);
        }
        for wx in 8 + 32..8 + 48 {
            let Chunk { x, z } = spawner_coordinates_to_chunk(wx, wz);
            assert_eq!(x as i64, 2);
            assert_eq!(z, 0);
        }
    }

    #[test]
    fn spawner_coordinates() {
        for chunk_x in -2..=2 {
            for i in 0..16 {
                println!("{} {}", chunk_x, i);
                let wx = chunk_x * 16 + 8 + i;
                let wy = 1;
                let wz = wx;
                let Chunk { x, z: _ } = spawner_coordinates_to_chunk(wx, wz);
                assert_eq!(x as i64, chunk_x);
                let (x, _y, _z) = spawner_coordinates_to_next_int(wx, wy, wz);
                assert_eq!(x as i64, i);
            }
        }
    }

    #[test]
    fn smarter_rng_iter_0() {
        // Benchmark: about 2 hours for the full 2^41 bruteforce using one CPU core
        // (-257, 38, -187) 1797254117522/281474976710656 dungeon seed: 160019524375634
        //
        let (wx, wy, wz) = (-257, 38, -187);
        let (x, y, z) = spawner_coordinates_to_next_int(wx, wy, wz);
        let floor = MossyFloor::parse(
            "MCMMMMMCC\n\
             MMCMCMCMM\n\
             MCCMCCMMM\n\
             MMMMMCMMM\n\
             MMMMMMCMM\n\
             MCMMCCMMM\n\
             CMMMMMMCC\n\
             MMMCMCMCM\n\
             MCMCMMMMM\n",
        )
        .unwrap();
        let mut i = 1797254117521;
        if let Some(r) = rng_for_dungeon_coord_next_ints(&mut i, (x, y, z), &floor) {
            let r_clone = r.clone();
            let r_seed = r.get_seed();
            println!(
                "{:?} {}/{} dungeon seed: {}",
                (wx, wy, wz),
                i,
                1u64 << 48,
                r_seed
            );
            check_rng_dungeon(r_clone, (wx as i32, wy as i32, wz as i32), &floor);
        } else {
            panic!("NOT FOUND");
        }
    }

    #[test]
    fn smarter_rng_iter_1() {
        // Benchmark: about 2 hours for the full 2^41 bruteforce using one CPU core
        // (-221, 45, -193) 1749066351968/281474976710656 dungeon seed: 38563726144688
        //
        let (wx, wy, wz) = (-221, 45, -193);
        let (x, y, z) = spawner_coordinates_to_next_int(wx, wy, wz);
        let floor = MossyFloor::parse(
            "MCMCCMM\n\
             MMCMMMM\n\
             MMMMMCM\n\
             MMCMMMM\n\
             MMMMCMM\n\
             MMMMCMM\n\
             MMMMCMM\n",
        )
        .unwrap();
        let mut i = 1749066351967;
        if let Some(r) = rng_for_dungeon_coord_next_ints(&mut i, (x, y, z), &floor) {
            let r_clone = r.clone();
            let r_seed = r.get_seed();
            println!(
                "{:?} {}/{} dungeon seed: {}",
                (wx, wy, wz),
                i,
                1u64 << 48,
                r_seed
            );
            check_rng_dungeon(r_clone, (wx as i32, wy as i32, wz as i32), &floor);
        } else {
            panic!("NOT FOUND");
        }
    }

    #[test]
    fn smarter_rng_iter_2() {
        // Benchmark: about 2 hours for the full 2^41 bruteforce using one CPU core
        // (-258, 37, -207) 1742563779852/281474976710656 dungeon seed: 11597388407236
        //
        let (wx, wy, wz) = (-258, 37, -207);
        let (x, y, z) = spawner_coordinates_to_next_int(wx, wy, wz);
        let floor = MossyFloor::parse(
            "CMCMMCMMC\n\
             MMMMCMMMM\n\
             MMMMCMMMM\n\
             MMCCMMMMC\n\
             CMMMMMMMM\n\
             MMMMMMMMM\n\
             CCMCMMCMM\n\
             MMCCMMMMC\n\
             MMCMMMMCC\n",
        )
        .unwrap();
        let mut i = 1742563779851;
        if let Some(r) = rng_for_dungeon_coord_next_ints(&mut i, (x, y, z), &floor) {
            let r_clone = r.clone();
            let r_seed = r.get_seed();
            println!(
                "{:?} {}/{} dungeon seed: {}",
                (wx, wy, wz),
                i,
                1u64 << 48,
                r_seed
            );
            check_rng_dungeon(r_clone, (wx as i32, wy as i32, wz as i32), &floor);
        } else {
            panic!("NOT FOUND");
        }
    }

    #[test]
    #[ignore]
    fn smarter_rng_iter_3() {
        // TODO: rerun: this was incorrect
        // Benchmark: about 2 hours for the full 2^41 bruteforce using one CPU core
        //
        let (wx, wy, wz) = (-180, 38, 47);
        let (x, y, z) = spawner_coordinates_to_next_int(wx, wy, wz);
        let floor = MossyFloor::parse(
            "MCMMCMMMM\n\
             MMCCMMCMM\n\
             MCCMMCMCM\n\
             MMMMMMMMC\n\
             MMMCMMMMC\n\
             MMMCMMMMM\n\
             CCMMMMMCC\n",
        )
        .unwrap();
        let mut i = 0;
        if let Some(r) = rng_for_dungeon_coord_next_ints(&mut i, (x, y, z), &floor) {
            let r_clone = r.clone();
            let r_seed = r.get_seed();
            println!(
                "{:?} {}/{} dungeon seed: {}",
                (wx, wy, wz),
                i,
                1u64 << 48,
                r_seed
            );
            check_rng_dungeon(r_clone, (wx as i32, wy as i32, wz as i32), &floor);
        } else {
            panic!("NOT FOUND");
        }
    }

    #[test]
    #[ignore] // TODO: just in case
    fn smarter_rng_iter_4() {
        // WARNING: explored dungeon
        // Benchmark: about 2 hours for the full 2^41 bruteforce using one CPU core
        //
        let (wx, wy, wz) = (-181, 35, -35);
        let (x, y, z) = spawner_coordinates_to_next_int(wx, wy, wz);
        let floor = MossyFloor::parse(
            "MMMMMCM\n\
             MMMCMMM\n\
             MMMMMMM\n\
             MMCMMCM\n\
             MMCCMMC\n\
             MMMCCMM\n\
             MMCMMMC\n\
             CMMMMMM\n\
             MMCMMMM\n",
        )
        .unwrap();
        let mut i = 0;
        if let Some(r) = rng_for_dungeon_coord_next_ints(&mut i, (x, y, z), &floor) {
            let r_seed = r.get_seed();
            println!(
                "{:?} {}/{} dungeon seed: {}",
                (wx, wy, wz),
                i,
                1u64 << 48,
                r_seed
            );
            check_rng_dungeon(r, (wx as i32, wy as i32, wz as i32), &floor);
        } else {
            panic!("NOT FOUND");
        }
    }

    #[test]
    #[ignore] // TODO: just in case
    fn smarter_rng_iter_5() {
        // WARNING: explored dungeon
        // Benchmark: about 2 hours for the full 2^41 bruteforce using one CPU core
        //
        let (wx, wy, wz) = (-167, 15, -1);
        let (x, y, z) = spawner_coordinates_to_next_int(wx, wy, wz);
        let floor = MossyFloor::parse(
            "MMMMCMM\n\
             M?????M\n\
             C?????M\n\
             M?????M\n\
             C?????M\n\
             M?????C\n\
             CMMMMMM\n",
        )
        .unwrap();
        let mut i = 0;
        if let Some(r) = rng_for_dungeon_coord_next_ints(&mut i, (x, y, z), &floor) {
            let r_seed = r.get_seed();
            println!(
                "{:?} {}/{} dungeon seed: {}",
                (wx, wy, wz),
                i,
                1u64 << 48,
                r_seed
            );
            check_rng_dungeon(r, (wx as i32, wy as i32, wz as i32), &floor);
        } else {
            panic!("NOT FOUND");
        }
    }

    #[test]
    #[ignore] // Takes a few minutes to run
    fn population_reversal_with_3_seeds() {
        // (-221, 45, -193) 1749066351968/281474976710656 dungeon seed: 38563726144688
        // (-257, 38, -187) 1797254117522/281474976710656 dungeon seed: 160019524375634
        // (-258, 37, -207) 1742563779852/281474976710656 dungeon seed: 11597388407236
        let world_seed = 90444174810516;
        // SEED: 90444174810516
        // GG
        let s1 = 38563726144688;
        let x1 = -221;
        let z1 = -193;

        let s2 = 160019524375634;
        let x2 = -257;
        let z2 = -187;

        let s3 = 11597388407236;
        let x3 = -258;
        let z3 = -207;

        let Chunk { x: x1, z: z1 } = spawner_coordinates_to_chunk(x1, z1);
        let Chunk { x: x2, z: z2 } = spawner_coordinates_to_chunk(x2, z2);
        let Chunk { x: x3, z: z3 } = spawner_coordinates_to_chunk(x3, z3);

        let found_seed =
            dungeon_seed_to_world_seed_alpha_1_2_6((s1, x1, z1), (s2, x2, z2), (s3, x3, z3));

        assert_eq!(found_seed, vec![world_seed]);
    }

    #[test]
    fn dungeon_size_from_corners() {
        let c1 = (-182, 50);
        let c2 = (-178, 44);
        assert_eq!(DungeonSize::from_corners_inside(c1, c2), DungeonSize::X7Z9);
    }

    #[test]
    fn test_case_alpha_1_0_5() {
        let world_seed = -6390333441551068844;
        let (wx, wy, wz) = (-77, 43, 52);
        let floor = MossyFloor::parse(
            "MMMMMCCCM\n\
             MMMMMMMMM\n\
             AAMMMMMMM\n\
             AMCCMMMMM\n\
             AMCMMMMCM\n\
             CMMMMMMMM\n\
             MMMMMMMMM\n\
             MMMCMCMCM\n\
             MCMMMMMMM\n",
        )
        .unwrap();
        let Chunk {
            x: chunk_x,
            z: chunk_z,
        } = spawner_coordinates_to_chunk(wx, wz);
        let dungeon_rng = populate_alpha_1_0_4_check_dungeon(
            world_seed,
            chunk_x,
            chunk_z,
            (wx as i32, wy as i32, wz as i32),
        )
        .unwrap();
        check_rng_dungeon(dungeon_rng, (wx as i32, wy as i32, wz as i32), &floor);
    }

    #[test]
    fn test_case_alpha_1_0_4() {
        let world_seed = 4338067525402914687;
        let (wx, wy, wz) = (-170, 38, -343);
        let floor = MossyFloor::parse(
            "MMMCCMMMM\n\
             CMMCMMCMC\n\
             MMMMMMMMM\n\
             MMMMMMMCC\n\
             MCMMMMMCM\n\
             MMCCMCMMC\n\
             MMMMMCMCM\n\
             MMMMMMMMC\n\
             MMMMMMMMM\n",
        )
        .unwrap();
        let Chunk {
            x: chunk_x,
            z: chunk_z,
        } = spawner_coordinates_to_chunk(wx, wz);
        let dungeon_rng = populate_alpha_1_0_4_check_dungeon(
            world_seed,
            chunk_x,
            chunk_z,
            (wx as i32, wy as i32, wz as i32),
        )
        .unwrap();
        check_rng_dungeon(dungeon_rng, (wx as i32, wy as i32, wz as i32), &floor);
    }

    #[test]
    #[ignore] // TODO: implement 1.14 populate?
    fn test_case_1_14() {
        let world_seed: i64 = 5012404560470663646;
        let (wx, wy, wz) = (-72, 28, 218);
        let floor = MossyFloor::parse(
            "???????\n\
             CMM????\n\
             ?MM???C\n\
             MMCMMMM\n\
             ?MCMMMM\n\
             MCMCMMM\n\
             MMMCMMM\n\
             MCCMMMM\n\
             MMMMMMC\n",
        )
        .unwrap();
        panic!("Ignore unused variable warnings: {:?}", (world_seed, (wx, wy, wz), floor));
    }

    #[test]
    #[ignore] // TODO: implement 1.14 populate?
    fn test_case2_1_14() {
        // slime_seed_finder dungeon-seed --spawner-x=-272 --spawner-y=24 --spawner-z=-17 --floor="CMMMMMC;MMCMMMM;CMCMMMC;MMMMMCM;MMCMMMM;CMMMMMM;CCMMMMM;"
        // "-272,24,-17,93895796360180"
        let world_seed: i64 = 4969430222601064613;
        let (wx, wy, wz) = (-272, 24, -17);
        let floor = MossyFloor::parse(
            "CMMMMMC\n\
             MMCMMMM\n\
             CMCMMMC\n\
             MMMMMCM\n\
             MMCMMMM\n\
             CMMMMMM\n\
             CCMMMMM\n",
        )
        .unwrap();
        panic!("Ignore unused variable warnings: {:?}", (world_seed, (wx, wy, wz), floor));
    }

    #[test]
    #[ignore] // TODO: implement 1.7 populate?
    fn test_case_1_7() {
        // slime_seed_finder dungeon-seed --spawner-x=-348 --spawner-y=16 --spawner-z=132 --floor="MMMMCMMMM;CMMMMMMMM;MCMMMMCCM;CMMCMMMCM;CCMMCMMMM;MMCCMMCMM;CMCCCCCMC;MMMMMMMCM;CMMCMMCCC;"
        // ["191650212348642,-23,7"]
        //
        let world_seed: i64 = 4969430222601064613;
        let (wx, wy, wz) = (-348, 16, 132);
        let floor = MossyFloor::parse(
            "MMMMCMMMM\n\
             CMMMMMMMM\n\
             MCMMMMCCM\n\
             CMMCMMMCM\n\
             CCMMCMMMM\n\
             MMCCMMCMM\n\
             CMCCCCCMC\n\
             MMMMMMMCM\n\
             CMMCMMCCC\n",
        )
        .unwrap();
        panic!("Ignore unused variable warnings: {:?}", (world_seed, (wx, wy, wz), floor));
    }

    #[test]
    fn advance_water_lake_fast_vs_safe() {
        let mut ra = JavaRng::with_seed(1234);
        let mut rb = ra.clone();

        for _ in 0..100 {
            let seed = ra.get_seed();
            advance_water_lake(&mut ra);
            advance_water_lake_safe(&mut rb);
            assert_eq!(ra.get_seed(), rb.get_seed(), "fail with seed: {}", seed);
        }
    }
}
