use libc;
extern "C" {
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    //==============================================================================
    // Essentials
    //==============================================================================
    #[no_mangle]
    static mut biomes: [Biome_0; 256];
}
pub const Cold: BiomeTempCategory = 3;
pub const coldBeach: BiomeID = 26;
pub const mesa: BiomeID = 37;
// 0-9
pub const sky: BiomeID = 9;
pub const coldTaiga: BiomeID = 30;
pub const FlowerForest_G: BiomeGroup = 6;
pub const coldTaigaHills: BiomeID = 31;
pub const warmOcean: BiomeID = 44;
pub const IceSpikes_G: BiomeGroup = 7;
pub const deepOcean: BiomeID = 24;
pub const BIOME_NUM: BiomeID = 51;
pub const forestHills: BiomeID = 18;
pub const none: BiomeID = -1;
pub const extremeHills: BiomeID = 3;
pub const birchForest: BiomeID = 27;
// 20-29
pub const roofedForest: BiomeID = 29;
pub const skyIslandMedium: BiomeID = 41;
pub const MesaBryce_G: BiomeGroup = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OceanRnd {
    pub d: [libc::c_int; 512],
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub c: libc::c_double,
}
// 30-39
pub const mesaPlateau: BiomeID = 39;
pub const frozenOcean: BiomeID = 10;
pub const Oceanic: BiomeTempCategory = 0;
pub type BiomeGroup = libc::c_uint;
pub const MushroomIsland_G: BiomeGroup = 5;
pub const jungleEdge: BiomeID = 23;
pub const Other_G: BiomeGroup = 0;
pub const warmDeepOcean: BiomeID = 47;
pub const mushroomIslandShore: BiomeID = 15;
pub const WarmOcean_G: BiomeGroup = 13;
pub const Mesa_G: BiomeGroup = 4;
//#warning "Using no SIMD extensions."
pub type BiomeID = libc::c_int;
pub const FrozenOcean_G: BiomeGroup = 10;
pub const skyIslandBarren: BiomeID = 43;
pub const LukewarmOcean_G: BiomeGroup = 12;
// 10-19
pub const taigaHills: BiomeID = 19;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Layer {
    pub baseSeed: int64_t,
    pub worldSeed: int64_t,
    pub chunkSeed: int64_t,
    pub oceanRnd: *mut OceanRnd_0,
    pub scale: libc::c_int,
    pub getMap: Option<
        unsafe extern "C" fn(
            _: *mut Layer_0,
            _: *mut libc::c_int,
            _: libc::c_int,
            _: libc::c_int,
            _: libc::c_int,
            _: libc::c_int,
        ) -> (),
    >,
    pub p: *mut Layer_0,
    pub p2: *mut Layer_0,
}
pub const lukewarmDeepOcean: BiomeID = 48;
pub const mushroomIsland: BiomeID = 14;
pub const frozenRiver: BiomeID = 11;
pub const Ocean_G: BiomeGroup = 1;
pub const taiga: BiomeID = 5;
pub const Warm: BiomeTempCategory = 1;
pub const swampland: BiomeID = 6;
pub const desertHills: BiomeID = 17;
// 1.13
pub const skyIslandLow: BiomeID = 40;
pub const frozenDeepOcean: BiomeID = 50;
pub type OceanRnd_0 = OceanRnd;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type int64_t = __int64_t;
pub const savannaPlateau: BiomeID = 36;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Biome {
    pub id: libc::c_int,
    pub type_0: libc::c_int,
    pub height: libc::c_double,
    pub temp: libc::c_double,
    pub tempCat: libc::c_int,
}
pub const ocean: BiomeID = 0;
pub const beach: BiomeID = 16;
pub const plains: BiomeID = 1;
pub const Freezing: BiomeTempCategory = 4;
pub const hell: BiomeID = 8;
pub const river: BiomeID = 7;
pub const Jungle_G: BiomeGroup = 2;
pub const Lush: BiomeTempCategory = 2;
pub const icePlains: BiomeID = 12;
// 40-49
pub const coldDeepOcean: BiomeID = 49;
pub const SunflowerPlains_G: BiomeGroup = 9;
pub const megaTaiga: BiomeID = 32;
pub const jungle: BiomeID = 21;
pub const iceMountains: BiomeID = 13;
pub const skyIslandHigh: BiomeID = 42;
pub const lukewarmOcean: BiomeID = 45;
pub type Biome_0 = Biome;
pub const coldOcean: BiomeID = 46;
pub const Unknown: BiomeTempCategory = 5;
pub const MegaTaiga_G: BiomeGroup = 3;
pub const megaTaigaHills: BiomeID = 33;
pub const stoneBeach: BiomeID = 25;
pub const desert: BiomeID = 2;
pub type Layer_0 = Layer;
pub const extremeHillsPlus: BiomeID = 34;
pub const extremeHillsEdge: BiomeID = 20;
pub const mesaPlateau_F: BiomeID = 38;
pub const birchForestHills: BiomeID = 28;
pub const ColdOcean_G: BiomeGroup = 11;
pub const forest: BiomeID = 4;
pub type BiomeTempCategory = libc::c_uint;
pub const savanna: BiomeID = 35;
pub const jungleHills: BiomeID = 22;
/* ********************* C copy of the Java Random methods **********************
 */
unsafe extern "C" fn setSeed(mut seed: *mut int64_t) -> () {
    *seed = ((*seed ^ 0x5deece66di64) as libc::c_longlong
        & (1i64 << 48i32) - 1i32 as libc::c_longlong) as int64_t;
}
unsafe extern "C" fn next(mut seed: *mut int64_t, bits: libc::c_int) -> libc::c_int {
    *seed = ((*seed * 0x5deece66di64 + 0xbi32 as libc::c_long) as libc::c_longlong
        & (1i64 << 48i32) - 1i32 as libc::c_longlong) as int64_t;
    return (*seed >> 48i32 - bits) as libc::c_int;
}
unsafe extern "C" fn nextInt(mut seed: *mut int64_t, n: libc::c_int) -> libc::c_int {
    let mut bits: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    let m: libc::c_int = n - 1i32;
    if m & n == 0i32 {
        return (n as libc::c_long * next(seed, 31i32) as libc::c_long >> 31i32) as libc::c_int;
    } else {
        loop {
            bits = next(seed, 31i32);
            val = bits % n;
            if !(bits - val + m < 0i32) {
                break;
            }
        }
        return val;
    };
}
unsafe extern "C" fn nextLong(mut seed: *mut int64_t) -> int64_t {
    return ((next(seed, 32i32) as int64_t) << 32i32) + next(seed, 32i32) as libc::c_long;
}
unsafe extern "C" fn nextFloat(mut seed: *mut int64_t) -> libc::c_float {
    return next(seed, 24i32) as libc::c_float / (1i32 << 24i32) as libc::c_float;
}
unsafe extern "C" fn nextDouble(mut seed: *mut int64_t) -> libc::c_double {
    return (((next(seed, 26i32) as int64_t) << 27i32) + next(seed, 27i32) as libc::c_long)
        as libc::c_double / (1i64 << 53i32) as libc::c_double;
}
// Custom, faster alternative for the first and second call to nextInt(24)
unsafe extern "C" fn firstInt24(mut seed: int64_t) -> libc::c_int {
    seed ^= 0x5deece66di64;
    seed = seed * 0x5deece66di64 & 0xffffffffffffi64;
    seed >>= 17i32;
    return (seed % 24i32 as libc::c_long) as libc::c_int;
}
unsafe extern "C" fn secondInt24(mut seed: int64_t) -> libc::c_int {
    seed ^= 0x5deece66di64;
    seed = seed * 0x5deece66di64 + 0xbi32 as libc::c_long & 0xffffffffffffi64;
    seed = seed * 0x5deece66di64 & 0xffffffffffffi64;
    seed >>= 17i32;
    return (seed % 24i32 as libc::c_long) as libc::c_int;
}
/* skipNextN
 * ---------
 * Jumps forwards in the random number sequence by simulating 'n' calls to next.
 */
unsafe extern "C" fn skipNextN(mut seed: *mut int64_t, n: libc::c_int) -> () {
    let mut i: libc::c_int = 0i32;
    while i < n {
        *seed = *seed * 0x5deece66di64 + 0xbi32 as libc::c_long;
        i += 1
    }
    *seed &= 0xffffffffffffi64;
}
/* invSeed48
 * ---------
 * Returns the previous 48-bit seed which will generate 'nseed'.
 * The upper 16 bits are ignored, both here and in the generator.
 */
unsafe extern "C" fn invSeed48(mut nseed: int64_t) -> int64_t {
    let x: int64_t = 0x5deece66di64;
    let xinv: int64_t = 0xdfe05bcb1365i64 as int64_t;
    let y: int64_t = 0xbi64 as int64_t;
    let m48: int64_t = 0xffffffffffffi64 as int64_t;
    let mut a: int64_t = nseed >> 32i32;
    let mut b: int64_t = (nseed as libc::c_longlong & 0xffffffffi64) as int64_t;
    if 0 != b as libc::c_longlong & 0x80000000i64 {
        a += 1
    }
    let mut q: int64_t = (b << 16i32) - y - (a << 16i32) * x & m48;
    let mut k: int64_t = 0i32 as int64_t;
    while k <= 5i32 as libc::c_long {
        let mut d: int64_t = (x - (q + (k << 48i32))) % x;
        // force the modulo and keep it positive
        d = (d + x) % x;
        if d < 65536i32 as libc::c_long {
            let mut c: int64_t = (q + d) * xinv & m48;
            if c < 65536i32 as libc::c_long {
                return ((a << 16i32) + c - y) * xinv & m48;
            }
        }
        k += 1
    }
    return -1i32 as int64_t;
}
unsafe extern "C" fn __uint16_identity(mut __x: __uint16_t) -> __uint16_t {
    return __x;
}
unsafe extern "C" fn __uint32_identity(mut __x: __uint32_t) -> __uint32_t {
    return __x;
}
unsafe extern "C" fn __uint64_identity(mut __x: __uint64_t) -> __uint64_t {
    return __x;
}
//==============================================================================
// Static Helpers
//==============================================================================
unsafe extern "C" fn getBiomeType(mut id: libc::c_int) -> libc::c_int {
    return biomes[(id & 0xffi32) as usize].type_0;
}
unsafe extern "C" fn biomeExists(mut id: libc::c_int) -> libc::c_int {
    return (0 == biomes[(id & 0xffi32) as usize].id & !0xffi32) as libc::c_int;
}
unsafe extern "C" fn getTempCategory(mut id: libc::c_int) -> libc::c_int {
    return biomes[(id & 0xffi32) as usize].tempCat;
}
unsafe extern "C" fn equalOrPlateau(mut id1: libc::c_int, mut id2: libc::c_int) -> libc::c_int {
    if id1 == id2 {
        return 1i32;
    } else if id1 == mesaPlateau_F as libc::c_int || id1 == mesaPlateau as libc::c_int {
        return (id2 == mesaPlateau_F as libc::c_int || id2 == mesaPlateau as libc::c_int)
            as libc::c_int;
    } else if 0 == biomeExists(id1) || 0 == biomeExists(id2) {
        return 0i32;
    } else {
        // adjust for asymmetric equality (workaround to simulate a bug in the MC java code)
        if id1 >= 128i32 || id2 >= 128i32 {
            // skip biomes that did not overload the isEqualTo() method
            if id2 == 130i32
                || id2 == 133i32
                || id2 == 134i32
                || id2 == 149i32
                || id2 == 151i32
                || id2 == 155i32
                || id2 == 156i32
                || id2 == 157i32
                || id2 == 158i32
                || id2 == 163i32
                || id2 == 164i32
            {
                return 0i32;
            }
        }
        return (getBiomeType(id1) == getBiomeType(id2)) as libc::c_int;
    };
}
unsafe extern "C" fn canBeNeighbors(mut id1: libc::c_int, mut id2: libc::c_int) -> libc::c_int {
    if 0 != equalOrPlateau(id1, id2) {
        return 1i32;
    } else if 0 == biomeExists(id1) || 0 == biomeExists(id2) {
        return 0i32;
    } else {
        let mut tempCat1: libc::c_int = getTempCategory(id1);
        if tempCat1 == Lush as libc::c_int {
            return 1i32;
        } else {
            let mut tempCat2: libc::c_int = getTempCategory(id2);
            if tempCat2 == Lush as libc::c_int {
                return 1i32;
            } else {
                return (tempCat1 == tempCat2) as libc::c_int;
            }
        }
    };
}
unsafe extern "C" fn isShallowOcean(mut id: libc::c_int) -> libc::c_int {
    return (id == ocean as libc::c_int
        || id == frozenOcean as libc::c_int
        || id == warmOcean as libc::c_int
        || id == lukewarmOcean as libc::c_int
        || id == coldOcean as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn isOceanic(mut id: libc::c_int) -> libc::c_int {
    match id {
        0 | 24 | 44 | 47 | 45 | 48 | 46 | 49 | 10 | 50 => return 1i32,
        _ => return 0i32,
    };
}
unsafe extern "C" fn isBiomeSnowy(mut id: libc::c_int) -> libc::c_int {
    return (0 != biomeExists(id) && biomes[(id & 0xffi32) as usize].temp < 0.1f64) as libc::c_int;
}
unsafe extern "C" fn mcNextInt(mut layer: *mut Layer_0, mut mod_0: libc::c_int) -> libc::c_int {
    let mut ret: libc::c_int = (((*layer).chunkSeed >> 24i32) % mod_0 as int64_t) as libc::c_int;
    if ret < 0i32 {
        ret += mod_0
    }
    (*layer).chunkSeed = ((*layer).chunkSeed as libc::c_longlong
        * ((*layer).chunkSeed as libc::c_longlong * 6364136223846793005i64
            + 1442695040888963407i64)) as int64_t;
    (*layer).chunkSeed += (*layer).worldSeed;
    return ret;
}
unsafe extern "C" fn setChunkSeed(
    mut layer: *mut Layer_0,
    mut chunkX: int64_t,
    mut chunkZ: int64_t,
) -> () {
    (*layer).chunkSeed = (*layer).worldSeed;
    (*layer).chunkSeed = ((*layer).chunkSeed as libc::c_longlong
        * ((*layer).chunkSeed as libc::c_longlong * 6364136223846793005i64
            + 1442695040888963407i64)) as int64_t;
    (*layer).chunkSeed += chunkX;
    (*layer).chunkSeed = ((*layer).chunkSeed as libc::c_longlong
        * ((*layer).chunkSeed as libc::c_longlong * 6364136223846793005i64
            + 1442695040888963407i64)) as int64_t;
    (*layer).chunkSeed += chunkZ;
    (*layer).chunkSeed = ((*layer).chunkSeed as libc::c_longlong
        * ((*layer).chunkSeed as libc::c_longlong * 6364136223846793005i64
            + 1442695040888963407i64)) as int64_t;
    (*layer).chunkSeed += chunkX;
    (*layer).chunkSeed = ((*layer).chunkSeed as libc::c_longlong
        * ((*layer).chunkSeed as libc::c_longlong * 6364136223846793005i64
            + 1442695040888963407i64)) as int64_t;
    (*layer).chunkSeed += chunkZ;
}
unsafe extern "C" fn setBaseSeed(mut layer: *mut Layer_0, mut seed: int64_t) -> () {
    (*layer).baseSeed = seed;
    (*layer).baseSeed = ((*layer).baseSeed as libc::c_longlong
        * ((*layer).baseSeed as libc::c_longlong * 6364136223846793005i64 + 1442695040888963407i64))
        as int64_t;
    (*layer).baseSeed += seed;
    (*layer).baseSeed = ((*layer).baseSeed as libc::c_longlong
        * ((*layer).baseSeed as libc::c_longlong * 6364136223846793005i64 + 1442695040888963407i64))
        as int64_t;
    (*layer).baseSeed += seed;
    (*layer).baseSeed = ((*layer).baseSeed as libc::c_longlong
        * ((*layer).baseSeed as libc::c_longlong * 6364136223846793005i64 + 1442695040888963407i64))
        as int64_t;
    (*layer).baseSeed += seed;
    (*layer).p = 0 as *mut Layer_0;
    (*layer).worldSeed = 0i32 as int64_t;
    (*layer).chunkSeed = 0i32 as int64_t;
}
unsafe extern "C" fn selectRandom2(
    mut l: *mut Layer_0,
    mut a1: libc::c_int,
    mut a2: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = mcNextInt(l, 2i32);
    return if i == 0i32 { a1 } else { a2 };
}
unsafe extern "C" fn selectRandom4(
    mut l: *mut Layer_0,
    mut a1: libc::c_int,
    mut a2: libc::c_int,
    mut a3: libc::c_int,
    mut a4: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = mcNextInt(l, 4i32);
    return if i == 0i32 {
        a1
    } else if i == 1i32 {
        a2
    } else if i == 2i32 {
        a3
    } else {
        a4
    };
}
unsafe extern "C" fn selectModeOrRandom(
    mut l: *mut Layer_0,
    mut a1: libc::c_int,
    mut a2: libc::c_int,
    mut a3: libc::c_int,
    mut a4: libc::c_int,
) -> libc::c_int {
    let mut rndarg: libc::c_int = selectRandom4(l, a1, a2, a3, a4);
    if a2 == a3 && a3 == a4 {
        return a2;
    } else if a1 == a2 && a1 == a3 {
        return a1;
    } else if a1 == a2 && a1 == a4 {
        return a1;
    } else if a1 == a3 && a1 == a4 {
        return a1;
    } else if a1 == a2 && a3 != a4 {
        return a1;
    } else if a1 == a3 && a2 != a4 {
        return a1;
    } else if a1 == a4 && a2 != a3 {
        return a1;
    } else if a2 == a3 && a1 != a4 {
        return a2;
    } else if a2 == a4 && a1 != a3 {
        return a2;
    } else if a3 == a4 && a1 != a2 {
        return a3;
    } else {
        return rndarg;
    };
}
#[no_mangle]
pub unsafe extern "C" fn getBiomeGroup(mut biome: libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    // Most biomes are basically everywhere, so we only make an effort to
    // count up the ones that have a good chance of being far away. The list
    // also focuses on biomes with items that don't occur elsewhere (packed ice,
    // teracotta, podzol, jungle saplings, cocoa beans, certain flowers, etc.)
    // A list of bomes that completes the Adventuring Time advancement would
    // also be a cool option.
    match biome {
        0 | 24 => return Ocean_G as libc::c_int,
        21 | 22 | 23 => {
            // Jungle M
            current_block = 13351138545237764899;
        }
        149 => {
            current_block = 13351138545237764899;
        }
        151 => {
            current_block = 3901725675897593422;
        }
        32 | 33 => {
            // Mega Spruce Taiga
            current_block = 7330028102315261357;
        }
        160 => {
            current_block = 7330028102315261357;
        }
        161 => {
            current_block = 10516056920901022514;
        }
        37 | 38 | 39 => {
            // Mesa Plateau F M
            current_block = 16073803459897220414;
        }
        166 => {
            current_block = 16073803459897220414;
        }
        167 => {
            current_block = 13513432006312082761;
        }
        14 | 15 => return MushroomIsland_G as libc::c_int,
        132 => return FlowerForest_G as libc::c_int,
        140 => return IceSpikes_G as libc::c_int,
        165 => return MesaBryce_G as libc::c_int,
        129 => return SunflowerPlains_G as libc::c_int,
        10 | 50 => return FrozenOcean_G as libc::c_int,
        46 | 49 => return ColdOcean_G as libc::c_int,
        45 | 48 => return LukewarmOcean_G as libc::c_int,
        44 => {
            // Does not occur in the game, nor reality.
            current_block = 4754063082398300628;
        }
        47 => {
            current_block = 4754063082398300628;
        }
        _ => return Other_G as libc::c_int,
    }
    match current_block {
        4754063082398300628 => return WarmOcean_G as libc::c_int,
        16073803459897220414 => {
            // Mesa Plateau M
            current_block = 13513432006312082761;
        }
        13351138545237764899 => {
            // Jungle Edge M
            current_block = 3901725675897593422;
        }
        7330028102315261357 => {
            // Mega Spruce Taiga Hills
            current_block = 10516056920901022514;
        }
        _ => {}
    }
    match current_block {
        3901725675897593422 => return Jungle_G as libc::c_int,
        13513432006312082761 => return Mesa_G as libc::c_int,
        _ => return MegaTaiga_G as libc::c_int,
    };
}
#[no_mangle]
pub static mut biomeGroupNames: [*const libc::c_char; 14] = unsafe {
    [
        b"Other\x00" as *const u8 as *const libc::c_char,
        b"Ocean\x00" as *const u8 as *const libc::c_char,
        b"Jungle\x00" as *const u8 as *const libc::c_char,
        b"Mega taiga\x00" as *const u8 as *const libc::c_char,
        b"Mesa\x00" as *const u8 as *const libc::c_char,
        b"Mushroom island\x00" as *const u8 as *const libc::c_char,
        b"Flower forest\x00" as *const u8 as *const libc::c_char,
        b"Ice spikes\x00" as *const u8 as *const libc::c_char,
        b"Mesa Bryce\x00" as *const u8 as *const libc::c_char,
        b"Sunflower plains\x00" as *const u8 as *const libc::c_char,
        b"Frozen ocean\x00" as *const u8 as *const libc::c_char,
        b"Cold ocean\x00" as *const u8 as *const libc::c_char,
        b"Lukewarm ocean\x00" as *const u8 as *const libc::c_char,
        b"Warm ocean\x00" as *const u8 as *const libc::c_char,
    ]
};
#[no_mangle]
pub static mut biomeNames: [*const libc::c_char; 256] = unsafe {
    [
        b"Ocean\x00" as *const u8 as *const libc::c_char,
        b"Plains\x00" as *const u8 as *const libc::c_char,
        b"Desert\x00" as *const u8 as *const libc::c_char,
        b"Extreme Hills\x00" as *const u8 as *const libc::c_char,
        b"Forest\x00" as *const u8 as *const libc::c_char,
        b"Taiga\x00" as *const u8 as *const libc::c_char,
        b"Swampland\x00" as *const u8 as *const libc::c_char,
        b"River\x00" as *const u8 as *const libc::c_char,
        b"Hell\x00" as *const u8 as *const libc::c_char,
        b"Sky\x00" as *const u8 as *const libc::c_char,
        b"Frozen Ocean\x00" as *const u8 as *const libc::c_char,
        b"Frozen River\x00" as *const u8 as *const libc::c_char,
        b"Ice Plains\x00" as *const u8 as *const libc::c_char,
        b"Ice Mountains\x00" as *const u8 as *const libc::c_char,
        b"Mushroom Island\x00" as *const u8 as *const libc::c_char,
        b"Mushroom Island Shore\x00" as *const u8 as *const libc::c_char,
        b"Beach\x00" as *const u8 as *const libc::c_char,
        b"Desert Hills\x00" as *const u8 as *const libc::c_char,
        b"Forest Hills\x00" as *const u8 as *const libc::c_char,
        b"Taiga Hills\x00" as *const u8 as *const libc::c_char,
        b"Extreme Hills Edge\x00" as *const u8 as *const libc::c_char,
        b"Jungle\x00" as *const u8 as *const libc::c_char,
        b"Jungle Hills\x00" as *const u8 as *const libc::c_char,
        b"Jungle Edge\x00" as *const u8 as *const libc::c_char,
        b"Deep Ocean\x00" as *const u8 as *const libc::c_char,
        b"Stone Beach\x00" as *const u8 as *const libc::c_char,
        b"Cold Beach\x00" as *const u8 as *const libc::c_char,
        b"Birch Forest\x00" as *const u8 as *const libc::c_char,
        b"Birch Forest Hills\x00" as *const u8 as *const libc::c_char,
        b"Roofed Forest\x00" as *const u8 as *const libc::c_char,
        b"Cold Taiga\x00" as *const u8 as *const libc::c_char,
        b"Cold Taiga Hills\x00" as *const u8 as *const libc::c_char,
        b"Mega Taiga\x00" as *const u8 as *const libc::c_char,
        b"Mega Taiga Hills\x00" as *const u8 as *const libc::c_char,
        b"Extreme Hills+\x00" as *const u8 as *const libc::c_char,
        b"Savanna\x00" as *const u8 as *const libc::c_char,
        b"Savanna Plateau\x00" as *const u8 as *const libc::c_char,
        b"Mesa\x00" as *const u8 as *const libc::c_char,
        b"Mesa Plateau F\x00" as *const u8 as *const libc::c_char,
        b"Mesa Plateau\x00" as *const u8 as *const libc::c_char,
        b"Sky Island Low\x00" as *const u8 as *const libc::c_char,
        b"Sky Island Medium\x00" as *const u8 as *const libc::c_char,
        b"Sky Island High\x00" as *const u8 as *const libc::c_char,
        b"Sky Island Barren\x00" as *const u8 as *const libc::c_char,
        b"Warm Ocean\x00" as *const u8 as *const libc::c_char,
        b"Lukewarm Ocean\x00" as *const u8 as *const libc::c_char,
        b"Cold Ocean\x00" as *const u8 as *const libc::c_char,
        b"Warm Deep Ocean\x00" as *const u8 as *const libc::c_char,
        b"Lukewarm Deep Ocean\x00" as *const u8 as *const libc::c_char,
        b"Cold Deep Ocean\x00" as *const u8 as *const libc::c_char,
        b"Frozen Deep Ocean\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #51\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #52\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #53\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #54\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #55\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #56\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #57\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #58\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #59\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #60\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #61\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #62\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #63\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #64\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #65\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #66\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #67\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #68\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #69\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #70\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #71\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #72\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #73\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #74\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #75\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #76\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #77\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #78\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #79\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #80\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #81\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #82\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #83\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #84\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #85\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #86\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #87\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #88\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #89\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #90\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #91\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #92\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #93\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #94\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #95\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #96\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #97\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #98\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #99\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #100\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #101\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #102\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #103\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #104\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #105\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #106\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #107\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #108\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #109\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #110\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #111\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #112\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #113\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #114\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #115\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #116\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #117\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #118\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #119\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #120\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #121\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #122\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #123\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #124\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #125\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #126\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #127\x00" as *const u8 as *const libc::c_char,
        b"Ocean M\x00" as *const u8 as *const libc::c_char,
        b"Sunflower Plains\x00" as *const u8 as *const libc::c_char,
        b"Desert M\x00" as *const u8 as *const libc::c_char,
        b"Extreme Hills M\x00" as *const u8 as *const libc::c_char,
        b"Flower Forest\x00" as *const u8 as *const libc::c_char,
        b"Taiga M\x00" as *const u8 as *const libc::c_char,
        b"Swampland M\x00" as *const u8 as *const libc::c_char,
        b"River M\x00" as *const u8 as *const libc::c_char,
        b"Hell M\x00" as *const u8 as *const libc::c_char,
        b"Sky M\x00" as *const u8 as *const libc::c_char,
        b"Frozen Ocean M\x00" as *const u8 as *const libc::c_char,
        b"Frozen River M\x00" as *const u8 as *const libc::c_char,
        b"Ice Plains Spikes\x00" as *const u8 as *const libc::c_char,
        b"Ice Mountains M\x00" as *const u8 as *const libc::c_char,
        b"Mushroom Island M\x00" as *const u8 as *const libc::c_char,
        b"Mushroom Island Shore M\x00" as *const u8 as *const libc::c_char,
        b"Beach M\x00" as *const u8 as *const libc::c_char,
        b"Desert Hills M\x00" as *const u8 as *const libc::c_char,
        b"Forest Hills M\x00" as *const u8 as *const libc::c_char,
        b"Taiga Hills M\x00" as *const u8 as *const libc::c_char,
        b"Extreme Hills Edge M\x00" as *const u8 as *const libc::c_char,
        b"Jungle M\x00" as *const u8 as *const libc::c_char,
        b"Jungle Hills M\x00" as *const u8 as *const libc::c_char,
        b"Jungle Edge M\x00" as *const u8 as *const libc::c_char,
        b"Deep Ocean M\x00" as *const u8 as *const libc::c_char,
        b"Stone Beach M\x00" as *const u8 as *const libc::c_char,
        b"Cold Beach M\x00" as *const u8 as *const libc::c_char,
        b"Birch Forest M\x00" as *const u8 as *const libc::c_char,
        b"Birch Forest Hills M\x00" as *const u8 as *const libc::c_char,
        b"Roofed Forest M\x00" as *const u8 as *const libc::c_char,
        b"Cold Taiga M\x00" as *const u8 as *const libc::c_char,
        b"Cold Taiga Hills M\x00" as *const u8 as *const libc::c_char,
        b"Mega Spruce Taiga\x00" as *const u8 as *const libc::c_char,
        b"Mega Spruce Taiga Hills\x00" as *const u8 as *const libc::c_char,
        b"Extreme Hills+ M\x00" as *const u8 as *const libc::c_char,
        b"Savanna M\x00" as *const u8 as *const libc::c_char,
        b"Savanna Plateau M\x00" as *const u8 as *const libc::c_char,
        b"Mesa Bryce\x00" as *const u8 as *const libc::c_char,
        b"Mesa Plateau F M\x00" as *const u8 as *const libc::c_char,
        b"Mesa Plateau M\x00" as *const u8 as *const libc::c_char,
        b"Sky Island Low M\x00" as *const u8 as *const libc::c_char,
        b"Sky Island Medium M\x00" as *const u8 as *const libc::c_char,
        b"Sky Island High M\x00" as *const u8 as *const libc::c_char,
        b"Sky Island Barren M\x00" as *const u8 as *const libc::c_char,
        b"Warm Ocean M\x00" as *const u8 as *const libc::c_char,
        b"Lukewarm Ocean M\x00" as *const u8 as *const libc::c_char,
        b"Cold Ocean M\x00" as *const u8 as *const libc::c_char,
        b"Warm Deep Ocean M\x00" as *const u8 as *const libc::c_char,
        b"Lukewarm Deep Ocean M\x00" as *const u8 as *const libc::c_char,
        b"Cold Deep Ocean M\x00" as *const u8 as *const libc::c_char,
        b"Frozen Deep Ocean M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #51 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #52 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #53 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #54 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #55 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #56 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #57 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #58 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #59 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #60 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #61 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #62 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #63 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #64 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #65 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #66 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #67 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #68 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #69 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #70 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #71 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #72 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #73 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #74 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #75 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #76 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #77 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #78 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #79 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #80 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #81 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #82 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #83 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #84 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #85 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #86 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #87 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #88 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #89 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #90 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #91 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #92 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #93 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #94 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #95 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #96 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #97 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #98 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #99 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #100 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #101 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #102 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #103 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #104 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #105 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #106 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #107 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #108 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #109 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #110 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #111 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #112 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #113 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #114 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #115 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #116 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #117 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #118 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #119 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #120 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #121 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #122 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #123 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #124 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #125 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #126 M\x00" as *const u8 as *const libc::c_char,
        b"UNKNOWN #127 M\x00" as *const u8 as *const libc::c_char,
    ]
};
#[no_mangle]
pub unsafe extern "C" fn initBiomeColours(mut biomeColours: *mut [libc::c_uchar; 3]) -> () {
    // This colouring scheme is taken from the AMIDST program:
    // https://github.com/toolbox4minecraft/amidst
    // https://sourceforge.net/projects/amidst.mirror/
    memset(
        biomeColours as *mut libc::c_void,
        0i32,
        (256i32 * 3i32) as libc::c_ulong,
    );
    setBiomeColour(
        biomeColours,
        ocean as libc::c_int,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        112i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        plains as libc::c_int,
        141i32 as libc::c_uchar,
        179i32 as libc::c_uchar,
        96i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        desert as libc::c_int,
        250i32 as libc::c_uchar,
        148i32 as libc::c_uchar,
        24i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        extremeHills as libc::c_int,
        96i32 as libc::c_uchar,
        96i32 as libc::c_uchar,
        96i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        forest as libc::c_int,
        5i32 as libc::c_uchar,
        102i32 as libc::c_uchar,
        33i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        taiga as libc::c_int,
        11i32 as libc::c_uchar,
        102i32 as libc::c_uchar,
        89i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        swampland as libc::c_int,
        7i32 as libc::c_uchar,
        249i32 as libc::c_uchar,
        178i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        river as libc::c_int,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        255i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        hell as libc::c_int,
        255i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        sky as libc::c_int,
        128i32 as libc::c_uchar,
        128i32 as libc::c_uchar,
        255i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        frozenOcean as libc::c_int,
        112i32 as libc::c_uchar,
        112i32 as libc::c_uchar,
        214i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        frozenRiver as libc::c_int,
        160i32 as libc::c_uchar,
        160i32 as libc::c_uchar,
        255i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        icePlains as libc::c_int,
        255i32 as libc::c_uchar,
        255i32 as libc::c_uchar,
        255i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        iceMountains as libc::c_int,
        160i32 as libc::c_uchar,
        160i32 as libc::c_uchar,
        160i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        mushroomIsland as libc::c_int,
        255i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        255i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        mushroomIslandShore as libc::c_int,
        160i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        255i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        beach as libc::c_int,
        250i32 as libc::c_uchar,
        222i32 as libc::c_uchar,
        85i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        desertHills as libc::c_int,
        210i32 as libc::c_uchar,
        95i32 as libc::c_uchar,
        18i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        forestHills as libc::c_int,
        34i32 as libc::c_uchar,
        85i32 as libc::c_uchar,
        28i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        taigaHills as libc::c_int,
        22i32 as libc::c_uchar,
        57i32 as libc::c_uchar,
        51i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        extremeHillsEdge as libc::c_int,
        114i32 as libc::c_uchar,
        120i32 as libc::c_uchar,
        154i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        jungle as libc::c_int,
        83i32 as libc::c_uchar,
        123i32 as libc::c_uchar,
        9i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        jungleHills as libc::c_int,
        44i32 as libc::c_uchar,
        66i32 as libc::c_uchar,
        5i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        jungleEdge as libc::c_int,
        98i32 as libc::c_uchar,
        139i32 as libc::c_uchar,
        23i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        deepOcean as libc::c_int,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        48i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        stoneBeach as libc::c_int,
        162i32 as libc::c_uchar,
        162i32 as libc::c_uchar,
        132i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        coldBeach as libc::c_int,
        250i32 as libc::c_uchar,
        240i32 as libc::c_uchar,
        192i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        birchForest as libc::c_int,
        48i32 as libc::c_uchar,
        116i32 as libc::c_uchar,
        68i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        birchForestHills as libc::c_int,
        31i32 as libc::c_uchar,
        95i32 as libc::c_uchar,
        50i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        roofedForest as libc::c_int,
        64i32 as libc::c_uchar,
        81i32 as libc::c_uchar,
        26i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        coldTaiga as libc::c_int,
        49i32 as libc::c_uchar,
        85i32 as libc::c_uchar,
        74i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        coldTaigaHills as libc::c_int,
        36i32 as libc::c_uchar,
        63i32 as libc::c_uchar,
        54i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        megaTaiga as libc::c_int,
        89i32 as libc::c_uchar,
        102i32 as libc::c_uchar,
        81i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        megaTaigaHills as libc::c_int,
        69i32 as libc::c_uchar,
        79i32 as libc::c_uchar,
        62i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        extremeHillsPlus as libc::c_int,
        80i32 as libc::c_uchar,
        112i32 as libc::c_uchar,
        80i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        savanna as libc::c_int,
        189i32 as libc::c_uchar,
        178i32 as libc::c_uchar,
        95i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        savannaPlateau as libc::c_int,
        167i32 as libc::c_uchar,
        157i32 as libc::c_uchar,
        100i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        mesa as libc::c_int,
        217i32 as libc::c_uchar,
        69i32 as libc::c_uchar,
        21i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        mesaPlateau_F as libc::c_int,
        176i32 as libc::c_uchar,
        151i32 as libc::c_uchar,
        101i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        mesaPlateau as libc::c_int,
        202i32 as libc::c_uchar,
        140i32 as libc::c_uchar,
        101i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        warmOcean as libc::c_int,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        172i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        lukewarmOcean as libc::c_int,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        144i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        coldOcean as libc::c_int,
        32i32 as libc::c_uchar,
        32i32 as libc::c_uchar,
        112i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        warmDeepOcean as libc::c_int,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        80i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        lukewarmDeepOcean as libc::c_int,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        64i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        coldDeepOcean as libc::c_int,
        32i32 as libc::c_uchar,
        32i32 as libc::c_uchar,
        56i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        frozenDeepOcean as libc::c_int,
        64i32 as libc::c_uchar,
        64i32 as libc::c_uchar,
        144i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        ocean as libc::c_int + 128i32,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        112i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        plains as libc::c_int + 128i32,
        141i32 as libc::c_uchar,
        179i32 as libc::c_uchar,
        96i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        desert as libc::c_int + 128i32,
        250i32 as libc::c_uchar,
        148i32 as libc::c_uchar,
        24i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        extremeHills as libc::c_int + 128i32,
        96i32 as libc::c_uchar,
        96i32 as libc::c_uchar,
        96i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        forest as libc::c_int + 128i32,
        5i32 as libc::c_uchar,
        102i32 as libc::c_uchar,
        33i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        taiga as libc::c_int + 128i32,
        11i32 as libc::c_uchar,
        102i32 as libc::c_uchar,
        89i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        swampland as libc::c_int + 128i32,
        7i32 as libc::c_uchar,
        249i32 as libc::c_uchar,
        178i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        river as libc::c_int + 128i32,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        255i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        hell as libc::c_int + 128i32,
        255i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        sky as libc::c_int + 128i32,
        128i32 as libc::c_uchar,
        128i32 as libc::c_uchar,
        255i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        frozenOcean as libc::c_int + 128i32,
        144i32 as libc::c_uchar,
        144i32 as libc::c_uchar,
        160i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        frozenRiver as libc::c_int + 128i32,
        160i32 as libc::c_uchar,
        160i32 as libc::c_uchar,
        255i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        icePlains as libc::c_int + 128i32,
        140i32 as libc::c_uchar,
        180i32 as libc::c_uchar,
        180i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        iceMountains as libc::c_int + 128i32,
        160i32 as libc::c_uchar,
        160i32 as libc::c_uchar,
        160i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        mushroomIsland as libc::c_int + 128i32,
        255i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        255i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        mushroomIslandShore as libc::c_int + 128i32,
        160i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        255i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        beach as libc::c_int + 128i32,
        250i32 as libc::c_uchar,
        222i32 as libc::c_uchar,
        85i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        desertHills as libc::c_int + 128i32,
        210i32 as libc::c_uchar,
        95i32 as libc::c_uchar,
        18i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        forestHills as libc::c_int + 128i32,
        34i32 as libc::c_uchar,
        85i32 as libc::c_uchar,
        28i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        taigaHills as libc::c_int + 128i32,
        22i32 as libc::c_uchar,
        57i32 as libc::c_uchar,
        51i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        extremeHillsEdge as libc::c_int + 128i32,
        114i32 as libc::c_uchar,
        120i32 as libc::c_uchar,
        154i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        jungle as libc::c_int + 128i32,
        83i32 as libc::c_uchar,
        123i32 as libc::c_uchar,
        9i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        jungleHills as libc::c_int + 128i32,
        44i32 as libc::c_uchar,
        66i32 as libc::c_uchar,
        5i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        jungleEdge as libc::c_int + 128i32,
        98i32 as libc::c_uchar,
        139i32 as libc::c_uchar,
        23i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        deepOcean as libc::c_int + 128i32,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        48i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        stoneBeach as libc::c_int + 128i32,
        162i32 as libc::c_uchar,
        162i32 as libc::c_uchar,
        132i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        coldBeach as libc::c_int + 128i32,
        250i32 as libc::c_uchar,
        240i32 as libc::c_uchar,
        192i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        birchForest as libc::c_int + 128i32,
        48i32 as libc::c_uchar,
        116i32 as libc::c_uchar,
        68i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        birchForestHills as libc::c_int + 128i32,
        31i32 as libc::c_uchar,
        95i32 as libc::c_uchar,
        50i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        roofedForest as libc::c_int + 128i32,
        64i32 as libc::c_uchar,
        81i32 as libc::c_uchar,
        26i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        coldTaiga as libc::c_int + 128i32,
        49i32 as libc::c_uchar,
        85i32 as libc::c_uchar,
        74i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        coldTaigaHills as libc::c_int + 128i32,
        36i32 as libc::c_uchar,
        63i32 as libc::c_uchar,
        54i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        megaTaiga as libc::c_int + 128i32,
        89i32 as libc::c_uchar,
        102i32 as libc::c_uchar,
        81i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        megaTaigaHills as libc::c_int + 128i32,
        69i32 as libc::c_uchar,
        79i32 as libc::c_uchar,
        62i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        extremeHillsPlus as libc::c_int + 128i32,
        80i32 as libc::c_uchar,
        112i32 as libc::c_uchar,
        80i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        savanna as libc::c_int + 128i32,
        189i32 as libc::c_uchar,
        178i32 as libc::c_uchar,
        95i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        savannaPlateau as libc::c_int + 128i32,
        167i32 as libc::c_uchar,
        157i32 as libc::c_uchar,
        100i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        mesa as libc::c_int + 128i32,
        217i32 as libc::c_uchar,
        69i32 as libc::c_uchar,
        21i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        mesaPlateau_F as libc::c_int + 128i32,
        176i32 as libc::c_uchar,
        151i32 as libc::c_uchar,
        101i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        mesaPlateau as libc::c_int + 128i32,
        202i32 as libc::c_uchar,
        140i32 as libc::c_uchar,
        101i32 as libc::c_uchar,
    );
}
/* Global biome colour table. */
#[no_mangle]
pub unsafe extern "C" fn setBiomeColour(
    mut biomeColour: *mut [libc::c_uchar; 3],
    mut biome: libc::c_int,
    mut r: libc::c_uchar,
    mut g: libc::c_uchar,
    mut b: libc::c_uchar,
) -> () {
    (*biomeColour.offset(biome as isize))[0usize] = r;
    (*biomeColour.offset(biome as isize))[1usize] = g;
    (*biomeColour.offset(biome as isize))[2usize] = b;
}
#[no_mangle]
pub unsafe extern "C" fn initBiomeTypeColours(mut biomeColours: *mut [libc::c_uchar; 3]) -> () {
    memset(
        biomeColours as *mut libc::c_void,
        0i32,
        (256i32 * 3i32) as libc::c_ulong,
    );
    setBiomeColour(
        biomeColours,
        Oceanic as libc::c_int,
        0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
        0xa0i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        Warm as libc::c_int,
        0xffi32 as libc::c_uchar,
        0xc0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        Lush as libc::c_int,
        0i32 as libc::c_uchar,
        0xa0i32 as libc::c_uchar,
        0i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        Cold as libc::c_int,
        0x60i32 as libc::c_uchar,
        0x60i32 as libc::c_uchar,
        0x60i32 as libc::c_uchar,
    );
    setBiomeColour(
        biomeColours,
        Freezing as libc::c_int,
        0xffi32 as libc::c_uchar,
        0xffi32 as libc::c_uchar,
        0xffi32 as libc::c_uchar,
    );
}
