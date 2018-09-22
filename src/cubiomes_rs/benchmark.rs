use libc;
extern "C" {
    pub type _IO_FILE_plus;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    //==============================================================================
    // Essentials
    //==============================================================================
    #[no_mangle]
    static mut biomes: [Biome_0; 256];
    /* initBiomes() has to be called before any of the generators can be used */
    #[no_mangle]
    fn initBiomes() -> ();
    #[no_mangle]
    fn setupGeneratorMC17() -> LayerStack;
    #[no_mangle]
    fn setupGeneratorMC113() -> LayerStack;
    /* Allocates an amount of memory required to generate an area of dimensions
     * 'sizeX' by 'sizeZ' for the magnification of the current top layer.
     */
    #[no_mangle]
    fn allocCache(layer: *mut Layer_0, sizeX: libc::c_int, sizeZ: libc::c_int) -> *mut libc::c_int;
    /* Sets the world seed for the generator */
    #[no_mangle]
    fn applySeed(g: *mut LayerStack, seed: int64_t) -> ();
    /* Generates the specified area using the current generator settings and stores
     * the biomeIDs in 'out'.
     * The biomeIDs will be indexed in the form: out[x + z*areaWidth]
     * It is recommended that 'out' is allocated using allocCache() for the correct
     * buffer size.
     */
    #[no_mangle]
    fn genArea(
        layer: *mut Layer_0,
        out: *mut libc::c_int,
        areaX: libc::c_int,
        areaZ: libc::c_int,
        areaWidth: libc::c_int,
        areaHeight: libc::c_int,
    ) -> ();
    #[no_mangle]
    static mut _IO_2_1_stdin_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stdout_: _IO_FILE_plus;
    #[no_mangle]
    static mut _IO_2_1_stderr_: _IO_FILE_plus;
    #[no_mangle]
    static mut stdin: *mut _IO_FILE;
    #[no_mangle]
    static mut stdout: *mut _IO_FILE;
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn printf(_: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    static mut __tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut __daylight: libc::c_int;
    #[no_mangle]
    static mut __timezone: libc::c_long;
    #[no_mangle]
    static mut tzname: [*mut libc::c_char; 2];
    #[no_mangle]
    static mut daylight: libc::c_int;
    #[no_mangle]
    static mut timezone: libc::c_long;
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type int64_t = __int64_t;
pub type size_t = libc::c_ulong;
pub type LayerStack = LayerStack_0;
pub type time_t = __time_t;
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StructureConfig {
    pub seed: int64_t,
    pub regionSize: libc::c_int,
    pub chunkRange: libc::c_int,
    pub properties: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
//#warning "Using no SIMD extensions."
pub type BiomeID = libc::c_int;
pub const BIOME_NUM: BiomeID = 51;
pub const frozenDeepOcean: BiomeID = 50;
// 40-49
pub const coldDeepOcean: BiomeID = 49;
pub const lukewarmDeepOcean: BiomeID = 48;
pub const warmDeepOcean: BiomeID = 47;
pub const coldOcean: BiomeID = 46;
pub const lukewarmOcean: BiomeID = 45;
pub const warmOcean: BiomeID = 44;
pub const skyIslandBarren: BiomeID = 43;
pub const skyIslandHigh: BiomeID = 42;
pub const skyIslandMedium: BiomeID = 41;
// 1.13
pub const skyIslandLow: BiomeID = 40;
// 30-39
pub const mesaPlateau: BiomeID = 39;
pub const mesaPlateau_F: BiomeID = 38;
pub const mesa: BiomeID = 37;
pub const savannaPlateau: BiomeID = 36;
pub const savanna: BiomeID = 35;
pub const extremeHillsPlus: BiomeID = 34;
pub const megaTaigaHills: BiomeID = 33;
pub const megaTaiga: BiomeID = 32;
pub const coldTaigaHills: BiomeID = 31;
pub const coldTaiga: BiomeID = 30;
// 20-29
pub const roofedForest: BiomeID = 29;
pub const birchForestHills: BiomeID = 28;
pub const birchForest: BiomeID = 27;
pub const coldBeach: BiomeID = 26;
pub const stoneBeach: BiomeID = 25;
pub const deepOcean: BiomeID = 24;
pub const jungleEdge: BiomeID = 23;
pub const jungleHills: BiomeID = 22;
pub const jungle: BiomeID = 21;
pub const extremeHillsEdge: BiomeID = 20;
// 10-19
pub const taigaHills: BiomeID = 19;
pub const forestHills: BiomeID = 18;
pub const desertHills: BiomeID = 17;
pub const beach: BiomeID = 16;
pub const mushroomIslandShore: BiomeID = 15;
pub const mushroomIsland: BiomeID = 14;
pub const iceMountains: BiomeID = 13;
pub const icePlains: BiomeID = 12;
pub const frozenRiver: BiomeID = 11;
pub const frozenOcean: BiomeID = 10;
// 0-9
pub const sky: BiomeID = 9;
pub const hell: BiomeID = 8;
pub const river: BiomeID = 7;
pub const swampland: BiomeID = 6;
pub const taiga: BiomeID = 5;
pub const forest: BiomeID = 4;
pub const extremeHills: BiomeID = 3;
pub const desert: BiomeID = 2;
pub const plains: BiomeID = 1;
pub const ocean: BiomeID = 0;
pub const none: BiomeID = -1;
pub type BiomeTempCategory = libc::c_uint;
pub const Unknown: BiomeTempCategory = 5;
pub const Freezing: BiomeTempCategory = 4;
pub const Cold: BiomeTempCategory = 3;
pub const Lush: BiomeTempCategory = 2;
pub const Warm: BiomeTempCategory = 1;
pub const Oceanic: BiomeTempCategory = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Biome {
    pub id: libc::c_int,
    pub type_0: libc::c_int,
    pub height: libc::c_double,
    pub temp: libc::c_double,
    pub tempCat: libc::c_int,
}
pub type Biome_0 = Biome;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct OceanRnd {
    pub d: [libc::c_int; 512],
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub c: libc::c_double,
}
pub type OceanRnd_0 = OceanRnd;
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
pub type Layer_0 = Layer;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LayerStack_0 {
    pub layers: *mut Layer_0,
    pub layerNum: libc::c_int,
}
pub type StructureConfig_0 = StructureConfig;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
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
/* For desert temples, igloos, jungle temples and witch huts prior to 1.13. */
static mut FEATURE_CONFIG: StructureConfig_0 = unsafe {
    StructureConfig {
        seed: 14357617i32 as int64_t,
        regionSize: 32i32,
        chunkRange: 24i32,
        properties: 0i32,
    }
};
/* 1.13 separated feature seeds by type */
static mut DESERT_PYRAMID_CONFIG: StructureConfig_0 = unsafe {
    StructureConfig {
        seed: 14357617i32 as int64_t,
        regionSize: 32i32,
        chunkRange: 24i32,
        properties: 0i32,
    }
};
static mut IGLOO_CONFIG: StructureConfig_0 = unsafe {
    StructureConfig {
        seed: 14357618i32 as int64_t,
        regionSize: 32i32,
        chunkRange: 24i32,
        properties: 0i32,
    }
};
static mut JUNGLE_PYRAMID_CONFIG: StructureConfig_0 = unsafe {
    StructureConfig {
        seed: 14357619i32 as int64_t,
        regionSize: 32i32,
        chunkRange: 24i32,
        properties: 0i32,
    }
};
static mut SWAMP_HUT_CONFIG: StructureConfig_0 = unsafe {
    StructureConfig {
        seed: 14357620i32 as int64_t,
        regionSize: 32i32,
        chunkRange: 24i32,
        properties: 0i32,
    }
};
static mut VILLAGE_CONFIG: StructureConfig_0 = unsafe {
    StructureConfig {
        seed: 10387312i32 as int64_t,
        regionSize: 32i32,
        chunkRange: 24i32,
        properties: 0i32,
    }
};
static mut OCEAN_RUIN_CONFIG: StructureConfig_0 = unsafe {
    StructureConfig {
        seed: 14357621i32 as int64_t,
        regionSize: 16i32,
        chunkRange: 8i32,
        properties: 2i32,
    }
};
static mut SHIPWRECK_CONFIG: StructureConfig_0 = unsafe {
    StructureConfig {
        seed: 165745295i32 as int64_t,
        regionSize: 15i32,
        chunkRange: 7i32,
        properties: 0i32,
    }
};
static mut MONUMENT_CONFIG: StructureConfig_0 = unsafe {
    StructureConfig {
        seed: 10387313i32 as int64_t,
        regionSize: 32i32,
        chunkRange: 27i32,
        properties: 1i32,
    }
};
static mut MANSION_CONFIG: StructureConfig_0 = unsafe {
    StructureConfig {
        seed: 10387319i32 as int64_t,
        regionSize: 80i32,
        chunkRange: 60i32,
        properties: 1i32,
    }
};
static mut templeBiomeList: [libc::c_int; 7] = unsafe {
    [
        desert as libc::c_int,
        desertHills as libc::c_int,
        jungle as libc::c_int,
        jungleHills as libc::c_int,
        swampland as libc::c_int,
        icePlains as libc::c_int,
        coldTaiga as libc::c_int,
    ]
};
static mut biomesToSpawnIn: [libc::c_int; 7] = unsafe {
    [
        forest as libc::c_int,
        plains as libc::c_int,
        taiga as libc::c_int,
        taigaHills as libc::c_int,
        forestHills as libc::c_int,
        jungle as libc::c_int,
        jungleHills as libc::c_int,
    ]
};
static mut oceanMonumentBiomeList1: [libc::c_int; 12] = unsafe {
    [
        ocean as libc::c_int,
        deepOcean as libc::c_int,
        river as libc::c_int,
        frozenRiver as libc::c_int,
        frozenOcean as libc::c_int,
        frozenDeepOcean as libc::c_int,
        coldOcean as libc::c_int,
        coldDeepOcean as libc::c_int,
        lukewarmOcean as libc::c_int,
        lukewarmDeepOcean as libc::c_int,
        warmOcean as libc::c_int,
        warmDeepOcean as libc::c_int,
    ]
};
static mut oceanMonumentBiomeList2: [libc::c_int; 5] = unsafe {
    [
        frozenDeepOcean as libc::c_int,
        coldDeepOcean as libc::c_int,
        deepOcean as libc::c_int,
        lukewarmDeepOcean as libc::c_int,
        warmDeepOcean as libc::c_int,
    ]
};
static mut villageBiomeList: [libc::c_int; 4] = unsafe {
    [
        plains as libc::c_int,
        desert as libc::c_int,
        savanna as libc::c_int,
        taiga as libc::c_int,
    ]
};
static mut mansionBiomeList: [libc::c_int; 2] = unsafe {
    [
        roofedForest as libc::c_int,
        roofedForest as libc::c_int + 128i32,
    ]
};
static mut achievementBiomes: [libc::c_int; 36] = unsafe {
    [
        ocean as libc::c_int,
        plains as libc::c_int,
        desert as libc::c_int,
        extremeHills as libc::c_int,
        forest as libc::c_int,
        taiga as libc::c_int,
        swampland as libc::c_int,
        river as libc::c_int,
        frozenRiver as libc::c_int,
        icePlains as libc::c_int,
        iceMountains as libc::c_int,
        mushroomIsland as libc::c_int,
        mushroomIslandShore as libc::c_int,
        beach as libc::c_int,
        desertHills as libc::c_int,
        forestHills as libc::c_int,
        taigaHills as libc::c_int,
        jungle as libc::c_int,
        jungleHills as libc::c_int,
        jungleEdge as libc::c_int,
        deepOcean as libc::c_int,
        stoneBeach as libc::c_int,
        coldBeach as libc::c_int,
        birchForest as libc::c_int,
        birchForestHills as libc::c_int,
        roofedForest as libc::c_int,
        coldTaiga as libc::c_int,
        coldTaigaHills as libc::c_int,
        megaTaiga as libc::c_int,
        megaTaigaHills as libc::c_int,
        extremeHillsPlus as libc::c_int,
        savanna as libc::c_int,
        savannaPlateau as libc::c_int,
        mesa as libc::c_int,
        mesaPlateau_F as libc::c_int,
        mesaPlateau as libc::c_int,
    ]
};
//==============================================================================
// Globals
//==============================================================================
/* ******************************* SEED FINDING *********************************
 *
 *  If we want to find rare seeds that meet multiple custom criteria then we
 *  should test each condition, starting with the one that is the cheapest
 *  to test for, while ruling out the most seeds.
 *
 *  Biome checks are quite expensive and should be applied late in the
 *  condition chain (to avoid as many unnecessary checks as possible).
 *  Fortunately we can often rule out vast amounts of seeds before hand.
 */
/* ************************** Quad-Structure Checks *****************************
 *
 *  Several tricks can be applied to determine candidate seeds for quad
 *  temples (inc. witch huts).
 *
 *  Minecraft uses a 48-bit pseudo random number generator (PRNG) to determine
 *  the position of it's structures. The remaining top 16 bits do not influence
 *  the structure positioning. Additionally the position of most structures in a
 *  world can be translated by applying the following transformation to the
 *  seed:
 *
 *  seed2 = seed1 - dregX * 341873128712 - dregZ * 132897987541;
 *
 *  Here seed1 and seed2 have the same structure positioning, but moved by a
 *  region offset of (dregX,dregZ). [a region is 32x32 chunks].
 *
 *  For a quad-structure, we mainly care about relative positioning, so we can
 *  get away with just checking the regions near the origin: (0,0),(0,1),(1,0)
 *  and (1,1) and then move the structures to the desired position.
 *
 *  Lastly we can recognise a that the transformation of relative region-
 *  coordinates imposes some restrictions in the PRNG, such that
 *  perfect-position quad-structure-seeds can only have certain values for the
 *  lower 16-bits in their seeds.
 *
 *
 ** The Set of all Quad-Witch-Huts
 *
 *  These conditions only leave 32 free bits which can comfortably be brute-
 *  forced to get the entire set of quad-structure candidates. Each of the seeds
 *  found this way describes an entire set of possible quad-witch-huts (with
 *  degrees of freedom for region-transposition, and the top 16-bit bits).
 *
 */
//==============================================================================
// Moving Structures
//==============================================================================
/* Transposes a base seed such that structures are moved by the specified region
 * vector, (regX, regZ).
 */
unsafe extern "C" fn moveStructure(
    baseSeed: int64_t,
    regX: libc::c_int,
    regZ: libc::c_int,
) -> int64_t {
    return baseSeed
        - regX as libc::c_long * 341873128712i64
        - regZ as libc::c_long * 132897987541i64 & 0xffffffffffffi64;
}
//==============================================================================
// Finding Properties of Structures
//==============================================================================
/* Initialises and returns a random seed used in the (16x16) chunk generation.
 * This random object is used for recursiveGenerate() which is responsible for
 * generating caves, ravines, mineshafts, and virtually all other structures.
 */
unsafe extern "C" fn chunkGenerateRnd(
    worldSeed: int64_t,
    chunkX: libc::c_int,
    chunkZ: libc::c_int,
) -> int64_t {
    let mut rnd: int64_t = worldSeed;
    setSeed(&mut rnd);
    rnd = nextLong(&mut rnd) * chunkX as libc::c_long
        ^ nextLong(&mut rnd) * chunkZ as libc::c_long
        ^ worldSeed;
    setSeed(&mut rnd);
    return rnd;
}
#[no_mangle]
pub unsafe extern "C" fn bench(mut g: LayerStack, mut ver: *mut libc::c_char) -> () {
    let mut quarterRes: *mut Layer_0 =
        &mut *g.layers.offset((g.layerNum - 2i32) as isize) as *mut Layer_0;
    let mut cache: *mut libc::c_int = allocCache(quarterRes, 1024i32 * 2i32, 1024i32 * 2i32);
    let mut start: time_t = time(0 as *mut time_t);
    let mut x: int64_t = 0i32 as int64_t;
    while x < 100i32 as libc::c_long {
        applySeed(&mut g, x);
        genArea(
            quarterRes,
            cache,
            -1024i32,
            -1024i32,
            1024i32 * 2i32,
            1024i32 * 2i32,
        );
        x += 1
    }
    printf(
        b"%s - %d in %ld seconds.\n\x00" as *const u8 as *const libc::c_char,
        ver,
        100i32,
        time(0 as *mut time_t) - start,
    );
    free(cache as *mut libc::c_void);
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    initBiomes();
    let mut g_1_7: LayerStack = setupGeneratorMC17();
    let mut g_1_13: LayerStack = setupGeneratorMC113();
    bench(
        g_1_7,
        b"1.7\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    bench(
        g_1_13,
        b"1.13\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return 0i32;
}
pub fn main() -> () {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
