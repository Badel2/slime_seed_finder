use libc;
extern "C" {
    pub type _IO_FILE_plus;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    //==============================================================================
    // Essentials
    //==============================================================================
    #[no_mangle]
    pub static mut biomes: [Biome_0; 256];
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn printf(_: *const libc::c_char, ...) -> libc::c_int;
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
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
}
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type int64_t = __int64_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
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
pub type _IO_lock_t = ();
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
pub type BiomeType = libc::c_uint;
pub const BTYPE_NUM: BiomeType = 17;
pub const Mesa: BiomeType = 16;
pub const Savanna: BiomeType = 15;
pub const StoneBeach: BiomeType = 14;
pub const Jungle: BiomeType = 13;
pub const Beach: BiomeType = 12;
pub const MushroomIsland: BiomeType = 11;
pub const Snow: BiomeType = 10;
pub const Sky: BiomeType = 9;
pub const Hell: BiomeType = 8;
pub const River: BiomeType = 7;
pub const Swamp: BiomeType = 6;
pub const Taiga: BiomeType = 5;
pub const Forest: BiomeType = 4;
pub const Hills: BiomeType = 3;
pub const Desert: BiomeType = 2;
pub const Plains: BiomeType = 1;
pub const Ocean: BiomeType = 0;
pub type BiomeTempCategory = libc::c_uint;
pub const Unknown: BiomeTempCategory = 5;
pub const Freezing: BiomeTempCategory = 4;
pub const Cold: BiomeTempCategory = 3;
pub const Lush: BiomeTempCategory = 2;
pub const Warm: BiomeTempCategory = 1;
pub const Oceanic: BiomeTempCategory = 0;
#[derive(Copy, Clone, Debug)]
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
/* initBiomes() has to be called before any of the generators can be used */
#[no_mangle]
pub unsafe extern "C" fn initBiomes() -> () {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < 256i32 {
        biomes[i as usize].id = none as libc::c_int;
        i += 1
    }
    let hDefault: libc::c_double = 0.1f64;
    let hShallowWaters: libc::c_double = -0.5f64;
    let hOceans: libc::c_double = -1.0f64;
    let hDeepOceans: libc::c_double = -1.8f64;
    let hLowPlains: libc::c_double = 0.125f64;
    let hMidPlains: libc::c_double = 0.2f64;
    let hLowHills: libc::c_double = 0.45f64;
    let hHighPlateaus: libc::c_double = 1.5f64;
    let hMidHills: libc::c_double = 1.0f64;
    let hShores: libc::c_double = 0.0f64;
    let hRockyWaters: libc::c_double = 0.1f64;
    let hLowIslands: libc::c_double = 0.2f64;
    let hPartiallySubmerged: libc::c_double = -0.2f64;
    initAddBiome(
        ocean as libc::c_int,
        Oceanic as libc::c_int,
        Ocean as libc::c_int,
        0.5f64 as libc::c_float,
        hOceans as libc::c_float,
    );
    initAddBiome(
        plains as libc::c_int,
        Lush as libc::c_int,
        Plains as libc::c_int,
        0.8f64 as libc::c_float,
        hDefault as libc::c_float,
    );
    initAddBiome(
        desert as libc::c_int,
        Warm as libc::c_int,
        Desert as libc::c_int,
        2.0f64 as libc::c_float,
        hLowPlains as libc::c_float,
    );
    initAddBiome(
        extremeHills as libc::c_int,
        Lush as libc::c_int,
        Hills as libc::c_int,
        0.2f64 as libc::c_float,
        hMidHills as libc::c_float,
    );
    initAddBiome(
        forest as libc::c_int,
        Lush as libc::c_int,
        Forest as libc::c_int,
        0.7f64 as libc::c_float,
        hDefault as libc::c_float,
    );
    initAddBiome(
        taiga as libc::c_int,
        Lush as libc::c_int,
        Taiga as libc::c_int,
        0.25f64 as libc::c_float,
        hMidPlains as libc::c_float,
    );
    initAddBiome(
        swampland as libc::c_int,
        Lush as libc::c_int,
        Swamp as libc::c_int,
        0.8f64 as libc::c_float,
        hPartiallySubmerged as libc::c_float,
    );
    initAddBiome(
        river as libc::c_int,
        Lush as libc::c_int,
        River as libc::c_int,
        0.5f64 as libc::c_float,
        hShallowWaters as libc::c_float,
    );
    initAddBiome(
        hell as libc::c_int,
        Warm as libc::c_int,
        Hell as libc::c_int,
        2.0f64 as libc::c_float,
        hDefault as libc::c_float,
    );
    initAddBiome(
        sky as libc::c_int,
        Lush as libc::c_int,
        Sky as libc::c_int,
        0.5f64 as libc::c_float,
        hDefault as libc::c_float,
    );
    initAddBiome(
        frozenOcean as libc::c_int,
        Oceanic as libc::c_int,
        Ocean as libc::c_int,
        0.0f64 as libc::c_float,
        hOceans as libc::c_float,
    );
    initAddBiome(
        frozenRiver as libc::c_int,
        Cold as libc::c_int,
        River as libc::c_int,
        0.0f64 as libc::c_float,
        hShallowWaters as libc::c_float,
    );
    initAddBiome(
        icePlains as libc::c_int,
        Cold as libc::c_int,
        Snow as libc::c_int,
        0.0f64 as libc::c_float,
        hLowPlains as libc::c_float,
    );
    initAddBiome(
        iceMountains as libc::c_int,
        Cold as libc::c_int,
        Snow as libc::c_int,
        0.0f64 as libc::c_float,
        hLowHills as libc::c_float,
    );
    initAddBiome(
        mushroomIsland as libc::c_int,
        Lush as libc::c_int,
        MushroomIsland as libc::c_int,
        0.9f64 as libc::c_float,
        hLowIslands as libc::c_float,
    );
    initAddBiome(
        mushroomIslandShore as libc::c_int,
        Lush as libc::c_int,
        MushroomIsland as libc::c_int,
        0.9f64 as libc::c_float,
        hShores as libc::c_float,
    );
    initAddBiome(
        beach as libc::c_int,
        Lush as libc::c_int,
        Beach as libc::c_int,
        0.8f64 as libc::c_float,
        hShores as libc::c_float,
    );
    initAddBiome(
        desertHills as libc::c_int,
        Warm as libc::c_int,
        Desert as libc::c_int,
        2.0f64 as libc::c_float,
        hLowHills as libc::c_float,
    );
    initAddBiome(
        forestHills as libc::c_int,
        Lush as libc::c_int,
        Forest as libc::c_int,
        0.7f64 as libc::c_float,
        hLowHills as libc::c_float,
    );
    initAddBiome(
        taigaHills as libc::c_int,
        Lush as libc::c_int,
        Taiga as libc::c_int,
        0.25f64 as libc::c_float,
        hLowHills as libc::c_float,
    );
    initAddBiome(
        extremeHillsEdge as libc::c_int,
        Lush as libc::c_int,
        Hills as libc::c_int,
        0.2f64 as libc::c_float,
        hMidHills as libc::c_float,
    );
    initAddBiome(
        jungle as libc::c_int,
        Lush as libc::c_int,
        Jungle as libc::c_int,
        0.95f64 as libc::c_float,
        hDefault as libc::c_float,
    );
    initAddBiome(
        jungleHills as libc::c_int,
        Lush as libc::c_int,
        Jungle as libc::c_int,
        0.95f64 as libc::c_float,
        hLowHills as libc::c_float,
    );
    initAddBiome(
        jungleEdge as libc::c_int,
        Lush as libc::c_int,
        Jungle as libc::c_int,
        0.95f64 as libc::c_float,
        hDefault as libc::c_float,
    );
    initAddBiome(
        deepOcean as libc::c_int,
        Oceanic as libc::c_int,
        Ocean as libc::c_int,
        0.5f64 as libc::c_float,
        hDeepOceans as libc::c_float,
    );
    initAddBiome(
        stoneBeach as libc::c_int,
        Lush as libc::c_int,
        StoneBeach as libc::c_int,
        0.2f64 as libc::c_float,
        hRockyWaters as libc::c_float,
    );
    initAddBiome(
        coldBeach as libc::c_int,
        Cold as libc::c_int,
        Beach as libc::c_int,
        0.05f64 as libc::c_float,
        hShores as libc::c_float,
    );
    initAddBiome(
        birchForest as libc::c_int,
        Lush as libc::c_int,
        Forest as libc::c_int,
        0.6f64 as libc::c_float,
        hDefault as libc::c_float,
    );
    initAddBiome(
        birchForestHills as libc::c_int,
        Lush as libc::c_int,
        Forest as libc::c_int,
        0.6f64 as libc::c_float,
        hLowHills as libc::c_float,
    );
    initAddBiome(
        roofedForest as libc::c_int,
        Lush as libc::c_int,
        Forest as libc::c_int,
        0.7f64 as libc::c_float,
        hDefault as libc::c_float,
    );
    initAddBiome(
        coldTaiga as libc::c_int,
        Cold as libc::c_int,
        Taiga as libc::c_int,
        -0.5f64 as libc::c_float,
        hMidPlains as libc::c_float,
    );
    initAddBiome(
        coldTaigaHills as libc::c_int,
        Cold as libc::c_int,
        Taiga as libc::c_int,
        -0.5f64 as libc::c_float,
        hLowHills as libc::c_float,
    );
    initAddBiome(
        megaTaiga as libc::c_int,
        Lush as libc::c_int,
        Taiga as libc::c_int,
        0.3f64 as libc::c_float,
        hMidPlains as libc::c_float,
    );
    initAddBiome(
        megaTaigaHills as libc::c_int,
        Lush as libc::c_int,
        Taiga as libc::c_int,
        0.3f64 as libc::c_float,
        hLowHills as libc::c_float,
    );
    initAddBiome(
        extremeHillsPlus as libc::c_int,
        Lush as libc::c_int,
        Hills as libc::c_int,
        0.2f64 as libc::c_float,
        hMidHills as libc::c_float,
    );
    initAddBiome(
        savanna as libc::c_int,
        Warm as libc::c_int,
        Savanna as libc::c_int,
        1.2f64 as libc::c_float,
        hLowPlains as libc::c_float,
    );
    initAddBiome(
        savannaPlateau as libc::c_int,
        Warm as libc::c_int,
        Savanna as libc::c_int,
        1.0f64 as libc::c_float,
        hHighPlateaus as libc::c_float,
    );
    initAddBiome(
        mesa as libc::c_int,
        Warm as libc::c_int,
        Mesa as libc::c_int,
        2.0f64 as libc::c_float,
        hDefault as libc::c_float,
    );
    initAddBiome(
        mesaPlateau_F as libc::c_int,
        Warm as libc::c_int,
        Mesa as libc::c_int,
        2.0f64 as libc::c_float,
        hHighPlateaus as libc::c_float,
    );
    initAddBiome(
        mesaPlateau as libc::c_int,
        Warm as libc::c_int,
        Mesa as libc::c_int,
        2.0f64 as libc::c_float,
        hHighPlateaus as libc::c_float,
    );
    // TODO: determine the 1.13 biome properties
    initAddBiome(
        skyIslandLow as libc::c_int,
        Lush as libc::c_int,
        Sky as libc::c_int,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
    );
    initAddBiome(
        skyIslandMedium as libc::c_int,
        Lush as libc::c_int,
        Sky as libc::c_int,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
    );
    initAddBiome(
        skyIslandHigh as libc::c_int,
        Lush as libc::c_int,
        Sky as libc::c_int,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
    );
    initAddBiome(
        skyIslandBarren as libc::c_int,
        Lush as libc::c_int,
        Sky as libc::c_int,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
    );
    initAddBiome(
        warmOcean as libc::c_int,
        Oceanic as libc::c_int,
        Ocean as libc::c_int,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
    );
    initAddBiome(
        lukewarmOcean as libc::c_int,
        Oceanic as libc::c_int,
        Ocean as libc::c_int,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
    );
    initAddBiome(
        coldOcean as libc::c_int,
        Oceanic as libc::c_int,
        Ocean as libc::c_int,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
    );
    initAddBiome(
        warmDeepOcean as libc::c_int,
        Oceanic as libc::c_int,
        Ocean as libc::c_int,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
    );
    initAddBiome(
        lukewarmDeepOcean as libc::c_int,
        Oceanic as libc::c_int,
        Ocean as libc::c_int,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
    );
    initAddBiome(
        coldDeepOcean as libc::c_int,
        Oceanic as libc::c_int,
        Ocean as libc::c_int,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
    );
    initAddBiome(
        frozenDeepOcean as libc::c_int,
        Oceanic as libc::c_int,
        Ocean as libc::c_int,
        0i32 as libc::c_float,
        0i32 as libc::c_float,
    );
    createMutation(plains as libc::c_int);
    createMutation(desert as libc::c_int);
    createMutation(extremeHills as libc::c_int);
    createMutation(forest as libc::c_int);
    createMutation(taiga as libc::c_int);
    createMutation(swampland as libc::c_int);
    createMutation(icePlains as libc::c_int);
    createMutation(jungle as libc::c_int);
    createMutation(jungleEdge as libc::c_int);
    createMutation(birchForest as libc::c_int);
    createMutation(birchForestHills as libc::c_int);
    createMutation(roofedForest as libc::c_int);
    createMutation(coldTaiga as libc::c_int);
    createMutation(megaTaiga as libc::c_int);
    createMutation(megaTaigaHills as libc::c_int);
    createMutation(extremeHillsPlus as libc::c_int);
    createMutation(savanna as libc::c_int);
    createMutation(savannaPlateau as libc::c_int);
    createMutation(mesa as libc::c_int);
    createMutation(mesaPlateau_F as libc::c_int);
    createMutation(mesaPlateau as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn createMutation(mut id: libc::c_int) -> () {
    biomes[(id + 128i32) as usize] = biomes[id as usize];
    biomes[(id + 128i32) as usize].id = id + 128i32;
}
#[no_mangle]
pub unsafe extern "C" fn initAddBiome(
    mut id: libc::c_int,
    mut tempCat: libc::c_int,
    mut biometype: libc::c_int,
    mut temp: libc::c_float,
    mut height: libc::c_float,
) -> () {
    if 0 != id & !0xffi32 {
        return;
    } else {
        biomes[id as usize].id = id;
        biomes[id as usize].type_0 = biometype;
        biomes[id as usize].temp = temp as libc::c_double;
        biomes[id as usize].height = height as libc::c_double;
        biomes[id as usize].tempCat = tempCat;
        return;
    };
}
/* Applies the given world seed to the layer and all dependent layers. */
#[no_mangle]
pub unsafe extern "C" fn setWorldSeed(mut layer: *mut Layer_0, mut seed: int64_t) -> () {
    if !(*layer).p2.is_null() && (*layer).getMap != Some(mapHills) {
        setWorldSeed((*layer).p2, seed);
    }
    if !(*layer).p.is_null() {
        setWorldSeed((*layer).p, seed);
    }
    if !(*layer).oceanRnd.is_null() {
        oceanRndInit((*layer).oceanRnd, seed);
    }
    (*layer).worldSeed = seed;
    (*layer).worldSeed = ((*layer).worldSeed as libc::c_longlong
        * ((*layer).worldSeed as libc::c_longlong * 6364136223846793005i64
            + 1442695040888963407i64)) as int64_t;
    (*layer).worldSeed += (*layer).baseSeed;
    (*layer).worldSeed = ((*layer).worldSeed as libc::c_longlong
        * ((*layer).worldSeed as libc::c_longlong * 6364136223846793005i64
            + 1442695040888963407i64)) as int64_t;
    (*layer).worldSeed += (*layer).baseSeed;
    (*layer).worldSeed = ((*layer).worldSeed as libc::c_longlong
        * ((*layer).worldSeed as libc::c_longlong * 6364136223846793005i64
            + 1442695040888963407i64)) as int64_t;
    (*layer).worldSeed += (*layer).baseSeed;
}
unsafe extern "C" fn oceanRndInit(mut rnd: *mut OceanRnd_0, mut seed: int64_t) -> () {
    let mut i: libc::c_int = 0i32;
    memset(
        rnd as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<OceanRnd_0>() as libc::c_ulong,
    );
    setSeed(&mut seed);
    (*rnd).a = nextDouble(&mut seed) * 256.0f64;
    (*rnd).b = nextDouble(&mut seed) * 256.0f64;
    (*rnd).c = nextDouble(&mut seed) * 256.0f64;
    i = 0i32;
    while i < 256i32 {
        (*rnd).d[i as usize] = i;
        i += 1
    }
    i = 0i32;
    while i < 256i32 {
        let mut n3: libc::c_int = nextInt(&mut seed, 256i32 - i) + i;
        let mut n4: libc::c_int = (*rnd).d[i as usize];
        (*rnd).d[i as usize] = (*rnd).d[n3 as usize];
        (*rnd).d[n3 as usize] = n4;
        (*rnd).d[(i + 256i32) as usize] = (*rnd).d[i as usize];
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn mapHills(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut pX: libc::c_int = areaX - 1i32;
    let mut pZ: libc::c_int = areaZ - 1i32;
    let mut pWidth: libc::c_int = areaWidth + 2i32;
    let mut pHeight: libc::c_int = areaHeight + 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut buf: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*l).p2.is_null() {
        printf(
            b"mapHills() requires two parents! Use setupMultiLayer()\n\x00" as *const u8
                as *const libc::c_char,
        );
        exit(1i32);
    } else {
        buf = malloc(
            ((pWidth * pHeight) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
        memcpy(
            buf as *mut libc::c_void,
            out as *const libc::c_void,
            ((pWidth * pHeight) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        (*(*l).p2).getMap.expect("non-null function pointer")(
            (*l).p2,
            out,
            pX,
            pZ,
            pWidth,
            pHeight,
        );
        z = 0i32;
        while z < areaHeight {
            x = 0i32;
            while x < areaWidth {
                setChunkSeed(l, (x + areaX) as int64_t, (z + areaZ) as int64_t);
                // biome branch
                let mut a11: libc::c_int = *buf.offset((x + 1i32 + (z + 1i32) * pWidth) as isize);
                // river branch
                let mut b11: libc::c_int = *out.offset((x + 1i32 + (z + 1i32) * pWidth) as isize);
                let mut idx: libc::c_int = x + z * areaWidth;
                let mut var12: libc::c_int = ((b11 - 2i32) % 29i32 == 0i32) as libc::c_int;
                if a11 != 0i32 && b11 >= 2i32 && (b11 - 2i32) % 29i32 == 1i32 && a11 < 128i32 {
                    *out.offset(idx as isize) = if 0 != biomeExists(a11 + 128i32) {
                        a11 + 128i32
                    } else {
                        a11
                    }
                } else if mcNextInt(l, 3i32) != 0i32 && 0 == var12 {
                    *out.offset(idx as isize) = a11
                } else {
                    let mut hillID: libc::c_int = a11;
                    match a11 {
                        2 => hillID = desertHills as libc::c_int,
                        4 => hillID = forestHills as libc::c_int,
                        27 => hillID = birchForestHills as libc::c_int,
                        29 => hillID = plains as libc::c_int,
                        5 => hillID = taigaHills as libc::c_int,
                        32 => hillID = megaTaigaHills as libc::c_int,
                        30 => hillID = coldTaigaHills as libc::c_int,
                        1 => {
                            hillID = if mcNextInt(l, 3i32) == 0i32 {
                                forestHills as libc::c_int
                            } else {
                                forest as libc::c_int
                            }
                        }
                        12 => hillID = iceMountains as libc::c_int,
                        21 => hillID = jungleHills as libc::c_int,
                        0 => hillID = deepOcean as libc::c_int,
                        3 => hillID = extremeHillsPlus as libc::c_int,
                        35 => hillID = savannaPlateau as libc::c_int,
                        _ => {
                            if 0 != equalOrPlateau(a11, mesaPlateau_F as libc::c_int) {
                                hillID = mesa as libc::c_int
                            } else if a11 == deepOcean as libc::c_int && mcNextInt(l, 3i32) == 0i32
                            {
                                hillID = if mcNextInt(l, 2i32) == 0i32 {
                                    plains as libc::c_int
                                } else {
                                    forest as libc::c_int
                                }
                            }
                        }
                    }
                    if 0 != var12 && hillID != a11 {
                        if 0 != biomeExists(hillID + 128i32) {
                            hillID += 128i32
                        } else {
                            hillID = a11
                        }
                    }
                    if hillID == a11 {
                        *out.offset(idx as isize) = a11
                    } else {
                        let mut a10: libc::c_int =
                            *buf.offset((x + 1i32 + (z + 0i32) * pWidth) as isize);
                        let mut a21: libc::c_int =
                            *buf.offset((x + 2i32 + (z + 1i32) * pWidth) as isize);
                        let mut a01: libc::c_int =
                            *buf.offset((x + 0i32 + (z + 1i32) * pWidth) as isize);
                        let mut a12: libc::c_int =
                            *buf.offset((x + 1i32 + (z + 2i32) * pWidth) as isize);
                        let mut equals: libc::c_int = 0i32;
                        if 0 != equalOrPlateau(a10, a11) {
                            equals += 1
                        }
                        if 0 != equalOrPlateau(a21, a11) {
                            equals += 1
                        }
                        if 0 != equalOrPlateau(a01, a11) {
                            equals += 1
                        }
                        if 0 != equalOrPlateau(a12, a11) {
                            equals += 1
                        }
                        if equals >= 3i32 {
                            *out.offset(idx as isize) = hillID
                        } else {
                            *out.offset(idx as isize) = a11
                        }
                    }
                }
                x += 1
            }
            z += 1
        }
        free(buf as *mut libc::c_void);
        return;
    };
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
//==============================================================================
// Static Helpers
//==============================================================================
unsafe extern "C" fn getBiomeType(mut id: libc::c_int) -> libc::c_int {
    return biomes[(id & 0xffi32) as usize].type_0;
}
unsafe extern "C" fn biomeExists(mut id: libc::c_int) -> libc::c_int {
    return (0 == biomes[(id & 0xffi32) as usize].id & !0xffi32) as libc::c_int;
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
unsafe extern "C" fn getTempCategory(mut id: libc::c_int) -> libc::c_int {
    return biomes[(id & 0xffi32) as usize].tempCat;
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
//==============================================================================
// Layers
//==============================================================================
// A null layer does nothing, and can be used to apply a layer to existing data.
#[no_mangle]
pub unsafe extern "C" fn mapNull(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> () {
}
// A skip layer simply calls its first parent without modification.
// This can be used as an easy way to skip a layer in a generator.
#[no_mangle]
pub unsafe extern "C" fn mapSkip(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut x: libc::c_int,
    mut z: libc::c_int,
    mut w: libc::c_int,
    mut h: libc::c_int,
) -> () {
    if (*l).p.is_null() {
        printf(
            b"mapSkip() requires a non-null parent layer.\n\x00" as *const u8
                as *const libc::c_char,
        );
        exit(1i32);
    } else {
        let mut ps: libc::c_int = (*(*l).p).scale;
        let mut s: libc::c_int = (*l).scale;
        if ps == s {
            (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, x, z, w, h);
        } else if ps == s << 1i32 {
            mapSkipZoom1(l, out, x, z, w, h);
        } else if ps == s << 2i32 {
            mapSkipZoom2(l, out, x, z, w, h);
        } else {
            printf(
                b"Invalid scale for skip layer: cannot convert from scale %d to %d.\n\x00"
                    as *const u8 as *const libc::c_char,
                ps,
                s,
            );
            exit(1i32);
        }
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mapSkipZoom2(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    areaX -= 2i32;
    areaZ -= 2i32;
    let mut pX: libc::c_int = areaX >> 2i32;
    let mut pZ: libc::c_int = areaZ >> 2i32;
    let mut pWidth: libc::c_int = (areaWidth >> 2i32) + 2i32;
    let mut pHeight: libc::c_int = (areaHeight >> 2i32) + 2i32;
    let mut newWidth: libc::c_int = pWidth - 1i32 << 2i32;
    let mut newHeight: libc::c_int = pHeight - 1i32 << 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut buf: *mut libc::c_int = malloc(
        (((newWidth + 1i32) * (newHeight + 1i32)) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
    z = 0i32;
    while z < pHeight - 1i32 {
        x = 0i32;
        while x < pWidth - 1i32 {
            let mut v00: libc::c_int = *out.offset((x + (z + 0i32) * pWidth) as isize) & 255i32;
            j = 0i32;
            while j < 4i32 {
                let mut idx: libc::c_int = ((z << 2i32) + j) * newWidth + (x << 2i32);
                i = 0i32;
                while i < 4i32 {
                    let fresh0 = idx;
                    idx = idx + 1;
                    *buf.offset(fresh0 as isize) = v00;
                    i += 1
                }
                j += 1
            }
            x += 1
        }
        z += 1
    }
    z = 0i32;
    while z < areaHeight {
        memcpy(
            &mut *out.offset((z * areaWidth) as isize) as *mut libc::c_int as *mut libc::c_void,
            &mut *buf.offset(((z + (areaZ & 3i32)) * newWidth + (areaX & 3i32)) as isize)
                as *mut libc::c_int as *const libc::c_void,
            (areaWidth as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        z += 1
    }
    free(buf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mapSkipZoom1(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut pX: libc::c_int = areaX >> 1i32;
    let mut pZ: libc::c_int = areaZ >> 1i32;
    let mut pWidth: libc::c_int = (areaWidth >> 1i32) + 2i32;
    let mut pHeight: libc::c_int = (areaHeight >> 1i32) + 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
    let mut newWidth: libc::c_int = pWidth - 1i32 << 1i32;
    let mut newHeight: libc::c_int = pHeight - 1i32 << 1i32;
    let mut idx: libc::c_int = 0;
    let mut buf: *mut libc::c_int = malloc(
        (((newWidth + 1i32) * (newHeight + 1i32)) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    z = 0i32;
    while z < pHeight - 1i32 {
        idx = (z << 1i32) * newWidth;
        x = 0i32;
        while x < pWidth - 1i32 {
            let mut a: libc::c_int = *out.offset((x + (z + 0i32) * pWidth) as isize);
            *buf.offset(idx as isize) = a;
            *buf.offset((idx + newWidth) as isize) = a;
            idx += 1;
            *buf.offset(idx as isize) = a;
            *buf.offset((idx + newWidth) as isize) = a;
            idx += 1;
            x += 1
        }
        z += 1
    }
    z = 0i32;
    while z < areaHeight {
        memcpy(
            &mut *out.offset((z * areaWidth) as isize) as *mut libc::c_int as *mut libc::c_void,
            &mut *buf.offset(((z + (areaZ & 1i32)) * newWidth + (areaX & 1i32)) as isize)
                as *mut libc::c_int as *const libc::c_void,
            (areaWidth as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        z += 1
    }
    free(buf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mapIsland(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let ws: int64_t = (*l).worldSeed;
    let ss: int64_t = (ws as libc::c_longlong
        * (ws as libc::c_longlong * 6364136223846793005i64 + 1442695040888963407i64))
        as int64_t;
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            let chunkX: int64_t = (x + areaX) as int64_t;
            let chunkZ: int64_t = (z + areaZ) as int64_t;
            let mut cs: int64_t = ss;
            cs += chunkX;
            cs = (cs as libc::c_longlong
                * (cs as libc::c_longlong * 6364136223846793005i64 + 1442695040888963407i64))
                as int64_t;
            cs += chunkZ;
            cs = (cs as libc::c_longlong
                * (cs as libc::c_longlong * 6364136223846793005i64 + 1442695040888963407i64))
                as int64_t;
            cs += chunkX;
            cs = (cs as libc::c_longlong
                * (cs as libc::c_longlong * 6364136223846793005i64 + 1442695040888963407i64))
                as int64_t;
            cs += chunkZ;
            *out.offset((x + z * areaWidth) as isize) =
                ((cs >> 24i32) % 10i32 as libc::c_long == 0i32 as libc::c_long) as libc::c_int;
            x += 1
        }
        z += 1
    }
    if areaX > -areaWidth && areaX <= 0i32 && areaZ > -areaHeight && areaZ <= 0i32 {
        *out.offset((-areaX + -areaZ * areaWidth) as isize) = 1i32
    };
}
#[no_mangle]
pub unsafe extern "C" fn mapZoom(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut i: libc::c_int = 0;
    let mut pX: libc::c_int = areaX >> 1i32;
    let mut pZ: libc::c_int = areaZ >> 1i32;
    let mut pWidth: libc::c_int = (areaWidth >> 1i32) + 2i32;
    let mut pHeight: libc::c_int = (areaHeight >> 1i32) + 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
    let mut newWidth: libc::c_int = pWidth - 1i32 << 1i32;
    let mut newHeight: libc::c_int = pHeight - 1i32 << 1i32;
    let mut idx: libc::c_int = 0;
    let mut a: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut buf: *mut libc::c_int = malloc(
        (((newWidth + 1i32) * (newHeight + 1i32)) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    let ws: libc::c_int = (*l).worldSeed as libc::c_int;
    let ss: libc::c_int = (ws as libc::c_long
        * ((ws * 1284865837i32) as libc::c_long + 4150755663i64))
        as libc::c_int;
    z = 0i32;
    while z < pHeight - 1i32 {
        idx = (z << 1i32) * newWidth;
        a = *out.offset(((z + 0i32) * pWidth) as isize);
        b = *out.offset(((z + 1i32) * pWidth) as isize);
        x = 0i32;
        while x < pWidth - 1i32 {
            let mut a1: libc::c_int = *out.offset((x + 1i32 + (z + 0i32) * pWidth) as isize);
            let mut b1: libc::c_int = *out.offset((x + 1i32 + (z + 1i32) * pWidth) as isize);
            let chunkX: libc::c_int = x + pX << 1i32;
            let chunkZ: libc::c_int = z + pZ << 1i32;
            let mut cs: libc::c_int = ss;
            cs += chunkX;
            cs = (cs as libc::c_long * ((cs * 1284865837i32) as libc::c_long + 4150755663i64))
                as libc::c_int;
            cs += chunkZ;
            cs = (cs as libc::c_long * ((cs * 1284865837i32) as libc::c_long + 4150755663i64))
                as libc::c_int;
            cs += chunkX;
            cs = (cs as libc::c_long * ((cs * 1284865837i32) as libc::c_long + 4150755663i64))
                as libc::c_int;
            cs += chunkZ;
            *buf.offset(idx as isize) = a;
            *buf.offset((idx + newWidth) as isize) = if 0 != cs >> 24i32 & 1i32 { b } else { a };
            idx += 1;
            cs = (cs as libc::c_long * ((cs * 1284865837i32) as libc::c_long + 4150755663i64))
                as libc::c_int;
            cs += ws;
            *buf.offset(idx as isize) = if 0 != cs >> 24i32 & 1i32 { a1 } else { a };
            if (*(*l).p).getMap == Some(mapIsland) {
                //selectRandom4
                cs = (cs as libc::c_long * ((cs * 1284865837i32) as libc::c_long + 4150755663i64))
                    as libc::c_int;
                cs += ws;
                i = cs >> 24i32 & 3i32;
                *buf.offset((idx + newWidth) as isize) = if i == 0i32 {
                    a
                } else if i == 1i32 {
                    a1
                } else if i == 2i32 {
                    b
                } else {
                    b1
                }
            } else if a1 == b && b == b1 {
                *buf.offset((idx + newWidth) as isize) = a1
            } else if a == a1 && a == b {
                *buf.offset((idx + newWidth) as isize) = a
            } else if a == a1 && a == b1 {
                *buf.offset((idx + newWidth) as isize) = a
            } else if a == b && a == b1 {
                *buf.offset((idx + newWidth) as isize) = a
            } else if a == a1 && b != b1 {
                *buf.offset((idx + newWidth) as isize) = a
            } else if a == b && a1 != b1 {
                *buf.offset((idx + newWidth) as isize) = a
            } else if a == b1 && a1 != b {
                *buf.offset((idx + newWidth) as isize) = a
            } else if a1 == b && a != b1 {
                *buf.offset((idx + newWidth) as isize) = a1
            } else if a1 == b1 && a != b {
                *buf.offset((idx + newWidth) as isize) = a1
            } else if b == b1 && a != a1 {
                *buf.offset((idx + newWidth) as isize) = b
            } else {
                cs = (cs as libc::c_long * ((cs * 1284865837i32) as libc::c_long + 4150755663i64))
                    as libc::c_int;
                cs += ws;
                let i_0: libc::c_int = cs >> 24i32 & 3i32;
                *buf.offset((idx + newWidth) as isize) = if i_0 == 0i32 {
                    a
                } else if i_0 == 1i32 {
                    a1
                } else if i_0 == 2i32 {
                    b
                } else {
                    b1
                }
            }
            idx += 1;
            a = a1;
            b = b1;
            x += 1
        }
        z += 1
    }
    z = 0i32;
    while z < areaHeight {
        memcpy(
            &mut *out.offset((z * areaWidth) as isize) as *mut libc::c_int as *mut libc::c_void,
            &mut *buf.offset(((z + (areaZ & 1i32)) * newWidth + (areaX & 1i32)) as isize)
                as *mut libc::c_int as *const libc::c_void,
            (areaWidth as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        z += 1
    }
    free(buf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mapAddIsland(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut pX: libc::c_int = areaX - 1i32;
    let mut pZ: libc::c_int = areaZ - 1i32;
    let mut pWidth: libc::c_int = areaWidth + 2i32;
    let mut pHeight: libc::c_int = areaHeight + 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
    let ws: int64_t = (*l).worldSeed;
    let ss: int64_t = (ws as libc::c_longlong
        * (ws as libc::c_longlong * 6364136223846793005i64 + 1442695040888963407i64))
        as int64_t;
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            let mut v00: libc::c_int = *out.offset((x + 0i32 + (z + 0i32) * pWidth) as isize);
            let mut v20: libc::c_int = *out.offset((x + 2i32 + (z + 0i32) * pWidth) as isize);
            let mut v02: libc::c_int = *out.offset((x + 0i32 + (z + 2i32) * pWidth) as isize);
            let mut v22: libc::c_int = *out.offset((x + 2i32 + (z + 2i32) * pWidth) as isize);
            let mut v11: libc::c_int = *out.offset((x + 1i32 + (z + 1i32) * pWidth) as isize);
            if v11 == 0i32 && (v00 != 0i32 || v20 != 0i32 || v02 != 0i32 || v22 != 0i32) {
                let chunkX: int64_t = (x + areaX) as int64_t;
                let chunkZ: int64_t = (z + areaZ) as int64_t;
                let mut cs: int64_t = ss;
                cs += chunkX;
                cs = (cs as libc::c_longlong
                    * (cs as libc::c_longlong * 6364136223846793005i64 + 1442695040888963407i64))
                    as int64_t;
                cs += chunkZ;
                cs = (cs as libc::c_longlong
                    * (cs as libc::c_longlong * 6364136223846793005i64 + 1442695040888963407i64))
                    as int64_t;
                cs += chunkX;
                cs = (cs as libc::c_longlong
                    * (cs as libc::c_longlong * 6364136223846793005i64 + 1442695040888963407i64))
                    as int64_t;
                cs += chunkZ;
                let mut v: libc::c_int = 1i32;
                let mut inc: libc::c_int = 0i32;
                if v00 != 0i32 {
                    inc += 1;
                    v = v00;
                    cs = (cs as libc::c_longlong
                        * (cs as libc::c_longlong * 6364136223846793005i64
                            + 1442695040888963407i64)) as int64_t;
                    cs += ws
                }
                if v20 != 0i32 {
                    inc += 1;
                    if inc == 1i32
                        || cs as libc::c_longlong & 1i64 << 24i32 == 0i32 as libc::c_longlong
                    {
                        v = v20
                    }
                    cs = (cs as libc::c_longlong
                        * (cs as libc::c_longlong * 6364136223846793005i64
                            + 1442695040888963407i64)) as int64_t;
                    cs += ws
                }
                if v02 != 0i32 {
                    inc += 1;
                    match inc {
                        1 => v = v02,
                        2 => {
                            if cs as libc::c_longlong & 1i64 << 24i32 == 0i32 as libc::c_longlong {
                                v = v02
                            }
                        }
                        _ => {
                            if (cs >> 24i32) % 3i32 as libc::c_long == 0i32 as libc::c_long {
                                v = v02
                            }
                        }
                    }
                    cs = (cs as libc::c_longlong
                        * (cs as libc::c_longlong * 6364136223846793005i64
                            + 1442695040888963407i64)) as int64_t;
                    cs += ws
                }
                if v22 != 0i32 {
                    inc += 1;
                    match inc {
                        1 => v = v22,
                        2 => {
                            if cs as libc::c_longlong & 1i64 << 24i32 == 0i32 as libc::c_longlong {
                                v = v22
                            }
                        }
                        3 => {
                            if (cs >> 24i32) % 3i32 as libc::c_long == 0i32 as libc::c_long {
                                v = v22
                            }
                        }
                        _ => {
                            if cs as libc::c_longlong & 3i64 << 24i32 == 0i32 as libc::c_longlong {
                                v = v22
                            }
                        }
                    }
                    cs = (cs as libc::c_longlong
                        * (cs as libc::c_longlong * 6364136223846793005i64
                            + 1442695040888963407i64)) as int64_t;
                    cs += ws
                }
                if (cs >> 24i32) % 3i32 as libc::c_long == 0i32 as libc::c_long {
                    *out.offset((x + z * areaWidth) as isize) = v
                } else if v == 4i32 {
                    *out.offset((x + z * areaWidth) as isize) = 4i32
                } else {
                    *out.offset((x + z * areaWidth) as isize) = 0i32
                }
            } else if v11 > 0i32 && (v00 == 0i32 || v20 == 0i32 || v02 == 0i32 || v22 == 0i32) {
                //setChunkSeed(l, (int64_t)(x + areaX), (int64_t)(z + areaZ));
                //if (mcNextInt(l, 5) == 0)...
                let chunkX_0: int64_t = (x + areaX) as int64_t;
                let chunkZ_0: int64_t = (z + areaZ) as int64_t;
                let mut cs_0: int64_t = ss;
                cs_0 += chunkX_0;
                cs_0 = (cs_0 as libc::c_longlong
                    * (cs_0 as libc::c_longlong * 6364136223846793005i64 + 1442695040888963407i64))
                    as int64_t;
                cs_0 += chunkZ_0;
                cs_0 = (cs_0 as libc::c_longlong
                    * (cs_0 as libc::c_longlong * 6364136223846793005i64 + 1442695040888963407i64))
                    as int64_t;
                cs_0 += chunkX_0;
                cs_0 = (cs_0 as libc::c_longlong
                    * (cs_0 as libc::c_longlong * 6364136223846793005i64 + 1442695040888963407i64))
                    as int64_t;
                cs_0 += chunkZ_0;
                if (cs_0 >> 24i32) % 5i32 as libc::c_long == 0i32 as libc::c_long {
                    *out.offset((x + z * areaWidth) as isize) =
                        if v11 == 4i32 { 4i32 } else { 0i32 }
                } else {
                    *out.offset((x + z * areaWidth) as isize) = v11
                }
            } else {
                *out.offset((x + z * areaWidth) as isize) = v11
            }
            x += 1
        }
        z += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn mapRemoveTooMuchOcean(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut pX: libc::c_int = areaX - 1i32;
    let mut pZ: libc::c_int = areaZ - 1i32;
    let mut pWidth: libc::c_int = areaWidth + 2i32;
    let mut pHeight: libc::c_int = areaHeight + 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            let mut v11: libc::c_int = *out.offset((x + 1i32 + (z + 1i32) * pWidth) as isize);
            *out.offset((x + z * areaWidth) as isize) = v11;
            if !(*out.offset((x + 1i32 + (z + 0i32) * pWidth) as isize) != 0i32) {
                if !(*out.offset((x + 2i32 + (z + 1i32) * pWidth) as isize) != 0i32) {
                    if !(*out.offset((x + 0i32 + (z + 1i32) * pWidth) as isize) != 0i32) {
                        if !(*out.offset((x + 1i32 + (z + 2i32) * pWidth) as isize) != 0i32) {
                            if v11 == 0i32 {
                                setChunkSeed(l, (x + areaX) as int64_t, (z + areaZ) as int64_t);
                                if mcNextInt(l, 2i32) == 0i32 {
                                    *out.offset((x + z * areaWidth) as isize) = 1i32
                                }
                            }
                        }
                    }
                }
            }
            x += 1
        }
        z += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn mapAddSnow(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut pX: libc::c_int = areaX - 1i32;
    let mut pZ: libc::c_int = areaZ - 1i32;
    let mut pWidth: libc::c_int = areaWidth + 2i32;
    let mut pHeight: libc::c_int = areaHeight + 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            let mut v11: libc::c_int = *out.offset((x + 1i32 + (z + 1i32) * pWidth) as isize);
            if 0 != isShallowOcean(v11) {
                *out.offset((x + z * areaWidth) as isize) = v11
            } else {
                setChunkSeed(l, (x + areaX) as int64_t, (z + areaZ) as int64_t);
                let mut r: libc::c_int = mcNextInt(l, 6i32);
                let mut v: libc::c_int = 0;
                if r == 0i32 {
                    v = 4i32
                } else if r <= 1i32 {
                    v = 3i32
                } else {
                    v = 1i32
                }
                *out.offset((x + z * areaWidth) as isize) = v
            }
            x += 1
        }
        z += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn mapCoolWarm(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut pX: libc::c_int = areaX - 1i32;
    let mut pZ: libc::c_int = areaZ - 1i32;
    let mut pWidth: libc::c_int = areaWidth + 2i32;
    let mut pHeight: libc::c_int = areaHeight + 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            let mut v11: libc::c_int = *out.offset((x + 1i32 + (z + 1i32) * pWidth) as isize);
            if v11 == 1i32 {
                let mut v10: libc::c_int = *out.offset((x + 1i32 + (z + 0i32) * pWidth) as isize);
                let mut v21: libc::c_int = *out.offset((x + 2i32 + (z + 1i32) * pWidth) as isize);
                let mut v01: libc::c_int = *out.offset((x + 0i32 + (z + 1i32) * pWidth) as isize);
                let mut v12: libc::c_int = *out.offset((x + 1i32 + (z + 2i32) * pWidth) as isize);
                if v10 == 3i32
                    || v10 == 4i32
                    || v21 == 3i32
                    || v21 == 4i32
                    || v01 == 3i32
                    || v01 == 4i32
                    || v12 == 3i32
                    || v12 == 4i32
                {
                    v11 = 2i32
                }
            }
            *out.offset((x + z * areaWidth) as isize) = v11;
            x += 1
        }
        z += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn mapHeatIce(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut pX: libc::c_int = areaX - 1i32;
    let mut pZ: libc::c_int = areaZ - 1i32;
    let mut pWidth: libc::c_int = areaWidth + 2i32;
    let mut pHeight: libc::c_int = areaHeight + 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            let mut v11: libc::c_int = *out.offset((x + 1i32 + (z + 1i32) * pWidth) as isize);
            if v11 == 4i32 {
                let mut v10: libc::c_int = *out.offset((x + 1i32 + (z + 0i32) * pWidth) as isize);
                let mut v21: libc::c_int = *out.offset((x + 2i32 + (z + 1i32) * pWidth) as isize);
                let mut v01: libc::c_int = *out.offset((x + 0i32 + (z + 1i32) * pWidth) as isize);
                let mut v12: libc::c_int = *out.offset((x + 1i32 + (z + 2i32) * pWidth) as isize);
                if v10 == 1i32
                    || v10 == 2i32
                    || v21 == 1i32
                    || v21 == 2i32
                    || v01 == 1i32
                    || v01 == 2i32
                    || v12 == 1i32
                    || v12 == 2i32
                {
                    v11 = 3i32
                }
            }
            *out.offset((x + z * areaWidth) as isize) = v11;
            x += 1
        }
        z += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn mapSpecial(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    (*(*l).p).getMap.expect("non-null function pointer")(
        (*l).p,
        out,
        areaX,
        areaZ,
        areaWidth,
        areaHeight,
    );
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            let mut v: libc::c_int = *out.offset((x + z * areaWidth) as isize);
            if !(v == 0i32) {
                setChunkSeed(l, (x + areaX) as int64_t, (z + areaZ) as int64_t);
                if mcNextInt(l, 13i32) == 0i32 {
                    v |= 1i32 + mcNextInt(l, 15i32) << 8i32 & 0xf00i32;
                    // 1 to 1 mapping so 'out' can be overwritten immediately
                    *out.offset((x + z * areaWidth) as isize) = v
                }
            }
            x += 1
        }
        z += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn mapAddMushroomIsland(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut current_block: u64;
    let mut pX: libc::c_int = areaX - 1i32;
    let mut pZ: libc::c_int = areaZ - 1i32;
    let mut pWidth: libc::c_int = areaWidth + 2i32;
    let mut pHeight: libc::c_int = areaHeight + 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            let mut v11: libc::c_int = *out.offset((x + 1i32 + (z + 1i32) * pWidth) as isize);
            // surrounded by ocean?
            if v11 == 0i32
                && 0 == *out.offset((x + 0i32 + (z + 0i32) * pWidth) as isize)
                && 0 == *out.offset((x + 2i32 + (z + 0i32) * pWidth) as isize)
                && 0 == *out.offset((x + 0i32 + (z + 2i32) * pWidth) as isize)
                && 0 == *out.offset((x + 2i32 + (z + 2i32) * pWidth) as isize)
            {
                setChunkSeed(l, (x + areaX) as int64_t, (z + areaZ) as int64_t);
                if mcNextInt(l, 100i32) == 0i32 {
                    *out.offset((x + z * areaWidth) as isize) = mushroomIsland as libc::c_int;
                    current_block = 10680521327981672866;
                } else {
                    current_block = 735147466149431745;
                }
            } else {
                current_block = 735147466149431745;
            }
            match current_block {
                735147466149431745 => *out.offset((x + z * areaWidth) as isize) = v11,
                _ => {}
            }
            x += 1
        }
        z += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn mapDeepOcean(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut pX: libc::c_int = areaX - 1i32;
    let mut pZ: libc::c_int = areaZ - 1i32;
    let mut pWidth: libc::c_int = areaWidth + 2i32;
    let mut pHeight: libc::c_int = areaHeight + 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            let mut v11: libc::c_int = *out.offset((x + 1i32 + (z + 1i32) * pWidth) as isize);
            if 0 != isShallowOcean(v11) {
                // count adjacent oceans
                let mut oceans: libc::c_int = 0i32;
                if 0 != isShallowOcean(*out.offset((x + 1i32 + (z + 0i32) * pWidth) as isize)) {
                    oceans += 1
                }
                if 0 != isShallowOcean(*out.offset((x + 2i32 + (z + 1i32) * pWidth) as isize)) {
                    oceans += 1
                }
                if 0 != isShallowOcean(*out.offset((x + 0i32 + (z + 1i32) * pWidth) as isize)) {
                    oceans += 1
                }
                if 0 != isShallowOcean(*out.offset((x + 1i32 + (z + 2i32) * pWidth) as isize)) {
                    oceans += 1
                }
                if oceans > 3i32 {
                    if v11 == warmOcean as libc::c_int {
                        v11 = warmDeepOcean as libc::c_int
                    } else if v11 == lukewarmOcean as libc::c_int {
                        v11 = lukewarmDeepOcean as libc::c_int
                    } else if v11 == ocean as libc::c_int {
                        v11 = deepOcean as libc::c_int
                    } else if v11 == coldOcean as libc::c_int {
                        v11 = coldDeepOcean as libc::c_int
                    } else if v11 == frozenOcean as libc::c_int {
                        v11 = frozenDeepOcean as libc::c_int
                    } else {
                        v11 = deepOcean as libc::c_int
                    }
                }
            }
            *out.offset((x + z * areaWidth) as isize) = v11;
            x += 1
        }
        z += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn mapBiome(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    (*(*l).p).getMap.expect("non-null function pointer")(
        (*l).p,
        out,
        areaX,
        areaZ,
        areaWidth,
        areaHeight,
    );
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            let mut idx: libc::c_int = x + z * areaWidth;
            let mut id: libc::c_int = *out.offset(idx as isize);
            let mut hasHighBit: libc::c_int = (id & 0xf00i32) >> 8i32;
            id &= -0xf01i32;
            if getBiomeType(id) == Ocean as libc::c_int || id == mushroomIsland as libc::c_int {
                *out.offset(idx as isize) = id
            } else {
                setChunkSeed(l, (x + areaX) as int64_t, (z + areaZ) as int64_t);
                match id {
                    1 => {
                        if 0 != hasHighBit {
                            *out.offset(idx as isize) = if mcNextInt(l, 3i32) == 0i32 {
                                mesaPlateau as libc::c_int
                            } else {
                                mesaPlateau_F as libc::c_int
                            }
                        } else {
                            *out.offset(idx as isize) = warmBiomes[mcNextInt(l, 6i32) as usize]
                        }
                    }
                    2 => {
                        if 0 != hasHighBit {
                            *out.offset(idx as isize) = jungle as libc::c_int
                        } else {
                            *out.offset(idx as isize) = lushBiomes[mcNextInt(l, 6i32) as usize]
                        }
                    }
                    3 => {
                        if 0 != hasHighBit {
                            *out.offset(idx as isize) = megaTaiga as libc::c_int
                        } else {
                            *out.offset(idx as isize) = coldBiomes[mcNextInt(l, 4i32) as usize]
                        }
                    }
                    4 => *out.offset(idx as isize) = snowBiomes[mcNextInt(l, 4i32) as usize],
                    _ => *out.offset(idx as isize) = mushroomIsland as libc::c_int,
                }
            }
            x += 1
        }
        z += 1
    }
}
#[no_mangle]
pub static mut snowBiomes: [libc::c_int; 4] = unsafe {
    [
        icePlains as libc::c_int,
        icePlains as libc::c_int,
        icePlains as libc::c_int,
        coldTaiga as libc::c_int,
    ]
};
#[no_mangle]
pub static mut coldBiomes: [libc::c_int; 4] = unsafe {
    [
        forest as libc::c_int,
        extremeHills as libc::c_int,
        taiga as libc::c_int,
        plains as libc::c_int,
    ]
};
#[no_mangle]
pub static mut lushBiomes: [libc::c_int; 6] = unsafe {
    [
        forest as libc::c_int,
        roofedForest as libc::c_int,
        extremeHills as libc::c_int,
        plains as libc::c_int,
        birchForest as libc::c_int,
        swampland as libc::c_int,
    ]
};
#[no_mangle]
pub static mut warmBiomes: [libc::c_int; 6] = unsafe {
    [
        desert as libc::c_int,
        desert as libc::c_int,
        desert as libc::c_int,
        savanna as libc::c_int,
        savanna as libc::c_int,
        plains as libc::c_int,
    ]
};
#[no_mangle]
pub unsafe extern "C" fn mapRiverInit(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    (*(*l).p).getMap.expect("non-null function pointer")(
        (*l).p,
        out,
        areaX,
        areaZ,
        areaWidth,
        areaHeight,
    );
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            if *out.offset((x + z * areaWidth) as isize) > 0i32 {
                setChunkSeed(l, (x + areaX) as int64_t, (z + areaZ) as int64_t);
                *out.offset((x + z * areaWidth) as isize) = mcNextInt(l, 299999i32) + 2i32
            } else {
                *out.offset((x + z * areaWidth) as isize) = 0i32
            }
            x += 1
        }
        z += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn mapBiomeEdge(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut pX: libc::c_int = areaX - 1i32;
    let mut pZ: libc::c_int = areaZ - 1i32;
    let mut pWidth: libc::c_int = areaWidth + 2i32;
    let mut pHeight: libc::c_int = areaHeight + 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            let mut v11: libc::c_int = *out.offset((x + 1i32 + (z + 1i32) * pWidth) as isize);
            let mut v10: libc::c_int = *out.offset((x + 1i32 + (z + 0i32) * pWidth) as isize);
            let mut v21: libc::c_int = *out.offset((x + 2i32 + (z + 1i32) * pWidth) as isize);
            let mut v01: libc::c_int = *out.offset((x + 0i32 + (z + 1i32) * pWidth) as isize);
            let mut v12: libc::c_int = *out.offset((x + 1i32 + (z + 2i32) * pWidth) as isize);
            /* !replaceEdgeIfNecessary(out, x + z*areaWidth, v10, v21, v01, v12, v11, extremeHills, extremeHillsEdge) &&*/
            if 0 == replaceEdge(
                out,
                x + z * areaWidth,
                v10,
                v21,
                v01,
                v12,
                v11,
                mesaPlateau_F as libc::c_int,
                mesa as libc::c_int,
            )
                && 0 == replaceEdge(
                    out,
                    x + z * areaWidth,
                    v10,
                    v21,
                    v01,
                    v12,
                    v11,
                    mesaPlateau as libc::c_int,
                    mesa as libc::c_int,
                )
                && 0 == replaceEdge(
                    out,
                    x + z * areaWidth,
                    v10,
                    v21,
                    v01,
                    v12,
                    v11,
                    megaTaiga as libc::c_int,
                    taiga as libc::c_int,
                ) {
                if v11 == desert as libc::c_int {
                    if v10 != icePlains as libc::c_int
                        && v21 != icePlains as libc::c_int
                        && v01 != icePlains as libc::c_int
                        && v12 != icePlains as libc::c_int
                    {
                        *out.offset((x + z * areaWidth) as isize) = v11
                    } else {
                        *out.offset((x + z * areaWidth) as isize) = extremeHillsPlus as libc::c_int
                    }
                } else if v11 == swampland as libc::c_int {
                    if v10 != desert as libc::c_int
                        && v21 != desert as libc::c_int
                        && v01 != desert as libc::c_int
                        && v12 != desert as libc::c_int
                        && v10 != coldTaiga as libc::c_int
                        && v21 != coldTaiga as libc::c_int
                        && v01 != coldTaiga as libc::c_int
                        && v12 != coldTaiga as libc::c_int
                        && v10 != icePlains as libc::c_int
                        && v21 != icePlains as libc::c_int
                        && v01 != icePlains as libc::c_int
                        && v12 != icePlains as libc::c_int
                    {
                        if v10 != jungle as libc::c_int
                            && v12 != jungle as libc::c_int
                            && v21 != jungle as libc::c_int
                            && v01 != jungle as libc::c_int
                        {
                            *out.offset((x + z * areaWidth) as isize) = v11
                        } else {
                            *out.offset((x + z * areaWidth) as isize) = jungleEdge as libc::c_int
                        }
                    } else {
                        *out.offset((x + z * areaWidth) as isize) = plains as libc::c_int
                    }
                } else {
                    *out.offset((x + z * areaWidth) as isize) = v11
                }
            }
            x += 1
        }
        z += 1
    }
}
// replaceEdgeIfNecessary() always returns 0 in the only place it is used in
// Minecraft, making it redundant.
/*
static inline int replaceEdgeIfNecessary(int *out, int idx, int v10, int v21, int v01, int v12, int id, int baseID, int edgeID)
{
    if (!equalOrPlateau(id, baseID)) return 0;

    if (canBeNeighbors(v10, baseID) && canBeNeighbors(v21, baseID) && canBeNeighbors(v01, baseID) && canBeNeighbors(v12, baseID))
        out[idx] = id;
    else
        out[idx] = edgeID;

    return 1;
}
*/
unsafe extern "C" fn replaceEdge(
    mut out: *mut libc::c_int,
    mut idx: libc::c_int,
    mut v10: libc::c_int,
    mut v21: libc::c_int,
    mut v01: libc::c_int,
    mut v12: libc::c_int,
    mut id: libc::c_int,
    mut baseID: libc::c_int,
    mut edgeID: libc::c_int,
) -> libc::c_int {
    if id != baseID {
        return 0i32;
    } else {
        if 0 != equalOrPlateau(v10, baseID)
            && 0 != equalOrPlateau(v21, baseID)
            && 0 != equalOrPlateau(v01, baseID)
            && 0 != equalOrPlateau(v12, baseID)
        {
            *out.offset(idx as isize) = id
        } else {
            *out.offset(idx as isize) = edgeID
        }
        return 1i32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mapRiver(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut pX: libc::c_int = areaX - 1i32;
    let mut pZ: libc::c_int = areaZ - 1i32;
    let mut pWidth: libc::c_int = areaWidth + 2i32;
    let mut pHeight: libc::c_int = areaHeight + 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            let mut v01: libc::c_int =
                reduceID(*out.offset((x + 0i32 + (z + 1i32) * pWidth) as isize));
            let mut v21: libc::c_int =
                reduceID(*out.offset((x + 2i32 + (z + 1i32) * pWidth) as isize));
            let mut v10: libc::c_int =
                reduceID(*out.offset((x + 1i32 + (z + 0i32) * pWidth) as isize));
            let mut v12: libc::c_int =
                reduceID(*out.offset((x + 1i32 + (z + 2i32) * pWidth) as isize));
            let mut v11: libc::c_int =
                reduceID(*out.offset((x + 1i32 + (z + 1i32) * pWidth) as isize));
            if v11 == v01 && v11 == v10 && v11 == v21 && v11 == v12 {
                *out.offset((x + z * areaWidth) as isize) = -1i32
            } else {
                *out.offset((x + z * areaWidth) as isize) = river as libc::c_int
            }
            x += 1
        }
        z += 1
    }
}
unsafe extern "C" fn reduceID(mut id: libc::c_int) -> libc::c_int {
    return if id >= 2i32 { 2i32 + (id & 1i32) } else { id };
}
#[no_mangle]
pub unsafe extern "C" fn mapSmooth(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut pX: libc::c_int = areaX - 1i32;
    let mut pZ: libc::c_int = areaZ - 1i32;
    let mut pWidth: libc::c_int = areaWidth + 2i32;
    let mut pHeight: libc::c_int = areaHeight + 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            let mut v11: libc::c_int = *out.offset((x + 1i32 + (z + 1i32) * pWidth) as isize);
            let mut v10: libc::c_int = *out.offset((x + 1i32 + (z + 0i32) * pWidth) as isize);
            let mut v21: libc::c_int = *out.offset((x + 2i32 + (z + 1i32) * pWidth) as isize);
            let mut v01: libc::c_int = *out.offset((x + 0i32 + (z + 1i32) * pWidth) as isize);
            let mut v12: libc::c_int = *out.offset((x + 1i32 + (z + 2i32) * pWidth) as isize);
            if v01 == v21 && v10 == v12 {
                setChunkSeed(l, (x + areaX) as int64_t, (z + areaZ) as int64_t);
                if mcNextInt(l, 2i32) == 0i32 {
                    v11 = v01
                } else {
                    v11 = v10
                }
            } else {
                if v01 == v21 {
                    v11 = v01
                }
                if v10 == v12 {
                    v11 = v10
                }
            }
            *out.offset((x + z * areaWidth) as isize) = v11;
            x += 1
        }
        z += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn mapRareBiome(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut pX: libc::c_int = areaX - 1i32;
    let mut pZ: libc::c_int = areaZ - 1i32;
    let mut pWidth: libc::c_int = areaWidth + 2i32;
    let mut pHeight: libc::c_int = areaHeight + 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            setChunkSeed(l, (x + areaX) as int64_t, (z + areaZ) as int64_t);
            let mut v11: libc::c_int = *out.offset((x + 1i32 + (z + 1i32) * pWidth) as isize);
            if mcNextInt(l, 57i32) == 0i32 && v11 == plains as libc::c_int {
                // Sunflower Plains
                *out.offset((x + z * areaWidth) as isize) = plains as libc::c_int + 128i32
            } else {
                *out.offset((x + z * areaWidth) as isize) = v11
            }
            x += 1
        }
        z += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn mapShore(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut pX: libc::c_int = areaX - 1i32;
    let mut pZ: libc::c_int = areaZ - 1i32;
    let mut pWidth: libc::c_int = areaWidth + 2i32;
    let mut pHeight: libc::c_int = areaHeight + 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            let mut v11: libc::c_int = *out.offset((x + 1i32 + (z + 1i32) * pWidth) as isize);
            let mut v10: libc::c_int = *out.offset((x + 1i32 + (z + 0i32) * pWidth) as isize);
            let mut v21: libc::c_int = *out.offset((x + 2i32 + (z + 1i32) * pWidth) as isize);
            let mut v01: libc::c_int = *out.offset((x + 0i32 + (z + 1i32) * pWidth) as isize);
            let mut v12: libc::c_int = *out.offset((x + 1i32 + (z + 2i32) * pWidth) as isize);
            let mut biome: libc::c_int = if 0 != biomeExists(v11) { v11 } else { 0i32 };
            if v11 == mushroomIsland as libc::c_int {
                if v10 != ocean as libc::c_int
                    && v21 != ocean as libc::c_int
                    && v01 != ocean as libc::c_int
                    && v12 != ocean as libc::c_int
                {
                    *out.offset((x + z * areaWidth) as isize) = v11
                } else {
                    *out.offset((x + z * areaWidth) as isize) = mushroomIslandShore as libc::c_int
                }
            } else if getBiomeType(biome) == Jungle as libc::c_int {
                if 0 != isBiomeJFTO(v10)
                    && 0 != isBiomeJFTO(v21)
                    && 0 != isBiomeJFTO(v01)
                    && 0 != isBiomeJFTO(v12)
                {
                    if 0 == isOceanic(v10)
                        && 0 == isOceanic(v21)
                        && 0 == isOceanic(v01)
                        && 0 == isOceanic(v12)
                    {
                        *out.offset((x + z * areaWidth) as isize) = v11
                    } else {
                        *out.offset((x + z * areaWidth) as isize) = beach as libc::c_int
                    }
                } else {
                    *out.offset((x + z * areaWidth) as isize) = jungleEdge as libc::c_int
                }
            } else if v11 != extremeHills as libc::c_int
                && v11 != extremeHillsPlus as libc::c_int
                && v11 != extremeHillsEdge as libc::c_int
            {
                if 0 != isBiomeSnowy(biome) {
                    replaceOcean(
                        out,
                        x + z * areaWidth,
                        v10,
                        v21,
                        v01,
                        v12,
                        v11,
                        coldBeach as libc::c_int,
                    );
                } else if v11 != mesa as libc::c_int && v11 != mesaPlateau_F as libc::c_int {
                    if v11 != ocean as libc::c_int
                        && v11 != deepOcean as libc::c_int
                        && v11 != river as libc::c_int
                        && v11 != swampland as libc::c_int
                    {
                        if 0 == isOceanic(v10)
                            && 0 == isOceanic(v21)
                            && 0 == isOceanic(v01)
                            && 0 == isOceanic(v12)
                        {
                            *out.offset((x + z * areaWidth) as isize) = v11
                        } else {
                            *out.offset((x + z * areaWidth) as isize) = beach as libc::c_int
                        }
                    } else {
                        *out.offset((x + z * areaWidth) as isize) = v11
                    }
                } else if 0 == isOceanic(v10)
                    && 0 == isOceanic(v21)
                    && 0 == isOceanic(v01)
                    && 0 == isOceanic(v12)
                {
                    if getBiomeType(v10) == Mesa as libc::c_int
                        && getBiomeType(v21) == Mesa as libc::c_int
                        && getBiomeType(v01) == Mesa as libc::c_int
                        && getBiomeType(v12) == Mesa as libc::c_int
                    {
                        *out.offset((x + z * areaWidth) as isize) = v11
                    } else {
                        *out.offset((x + z * areaWidth) as isize) = desert as libc::c_int
                    }
                } else {
                    *out.offset((x + z * areaWidth) as isize) = v11
                }
            } else {
                replaceOcean(
                    out,
                    x + z * areaWidth,
                    v10,
                    v21,
                    v01,
                    v12,
                    v11,
                    stoneBeach as libc::c_int,
                );
            }
            x += 1
        }
        z += 1
    }
}
unsafe extern "C" fn replaceOcean(
    mut out: *mut libc::c_int,
    mut idx: libc::c_int,
    mut v10: libc::c_int,
    mut v21: libc::c_int,
    mut v01: libc::c_int,
    mut v12: libc::c_int,
    mut id: libc::c_int,
    mut replaceID: libc::c_int,
) -> libc::c_int {
    if 0 != isOceanic(id) {
        return 0i32;
    } else {
        if 0 == isOceanic(v10) && 0 == isOceanic(v21) && 0 == isOceanic(v01) && 0 == isOceanic(v12)
        {
            *out.offset(idx as isize) = id
        } else {
            *out.offset(idx as isize) = replaceID
        }
        return 1i32;
    };
}
unsafe extern "C" fn isBiomeJFTO(mut id: libc::c_int) -> libc::c_int {
    return (0 != biomeExists(id)
        && (getBiomeType(id) == Jungle as libc::c_int
            || id == forest as libc::c_int
            || id == taiga as libc::c_int
            || 0 != isOceanic(id))) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mapRiverMix(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut idx: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut buf: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*l).p2.is_null() {
        printf(
            b"mapRiverMix() requires two parents! Use setupMultiLayer()\n\x00" as *const u8
                as *const libc::c_char,
        );
        exit(1i32);
    } else {
        len = areaWidth * areaHeight;
        buf = malloc(
            (len as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        // biome chain
        (*(*l).p).getMap.expect("non-null function pointer")(
            (*l).p,
            out,
            areaX,
            areaZ,
            areaWidth,
            areaHeight,
        );
        memcpy(
            buf as *mut libc::c_void,
            out as *const libc::c_void,
            (len as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        // rivers
        (*(*l).p2).getMap.expect("non-null function pointer")(
            (*l).p2,
            out,
            areaX,
            areaZ,
            areaWidth,
            areaHeight,
        );
        idx = 0i32;
        while idx < len {
            if 0 != isOceanic(*buf.offset(idx as isize)) {
                *out.offset(idx as isize) = *buf.offset(idx as isize)
            } else if *out.offset(idx as isize) == river as libc::c_int {
                if *buf.offset(idx as isize) == icePlains as libc::c_int {
                    *out.offset(idx as isize) = frozenRiver as libc::c_int
                } else if *buf.offset(idx as isize) == mushroomIsland as libc::c_int
                    || *buf.offset(idx as isize) == mushroomIslandShore as libc::c_int
                {
                    *out.offset(idx as isize) = mushroomIslandShore as libc::c_int
                } else {
                    *out.offset(idx as isize) = *out.offset(idx as isize) & 255i32
                }
            } else {
                *out.offset(idx as isize) = *buf.offset(idx as isize)
            }
            idx += 1
        }
        free(buf as *mut libc::c_void);
        return;
    };
}
// 1.13 layers
#[no_mangle]
pub unsafe extern "C" fn mapHills113(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut pX: libc::c_int = areaX - 1i32;
    let mut pZ: libc::c_int = areaZ - 1i32;
    let mut pWidth: libc::c_int = areaWidth + 2i32;
    let mut pHeight: libc::c_int = areaHeight + 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut buf: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*l).p2.is_null() {
        printf(
            b"mapHills() requires two parents! Use setupMultiLayer()\n\x00" as *const u8
                as *const libc::c_char,
        );
        exit(1i32);
    } else {
        buf = malloc(
            ((pWidth * pHeight) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
        memcpy(
            buf as *mut libc::c_void,
            out as *const libc::c_void,
            ((pWidth * pHeight) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        (*(*l).p2).getMap.expect("non-null function pointer")(
            (*l).p2,
            out,
            pX,
            pZ,
            pWidth,
            pHeight,
        );
        z = 0i32;
        while z < areaHeight {
            x = 0i32;
            while x < areaWidth {
                setChunkSeed(l, (x + areaX) as int64_t, (z + areaZ) as int64_t);
                // biome branch
                let mut a11: libc::c_int = *buf.offset((x + 1i32 + (z + 1i32) * pWidth) as isize);
                // river branch
                let mut b11: libc::c_int = *out.offset((x + 1i32 + (z + 1i32) * pWidth) as isize);
                let mut idx: libc::c_int = x + z * areaWidth;
                let mut bn: libc::c_int = (b11 - 2i32) % 29i32;
                if !(0 != isOceanic(a11) || b11 < 2i32 || bn != 1i32 || a11 >= 128i32) {
                    *out.offset(idx as isize) = if 0 != biomeExists(a11 + 128i32) {
                        a11 + 128i32
                    } else {
                        a11
                    }
                } else if mcNextInt(l, 3i32) == 0i32 || bn == 0i32 {
                    let mut hillID: libc::c_int = a11;
                    match a11 {
                        2 => hillID = desertHills as libc::c_int,
                        4 => hillID = forestHills as libc::c_int,
                        27 => hillID = birchForestHills as libc::c_int,
                        29 => hillID = plains as libc::c_int,
                        5 => hillID = taigaHills as libc::c_int,
                        32 => hillID = megaTaigaHills as libc::c_int,
                        30 => hillID = coldTaigaHills as libc::c_int,
                        1 => {
                            hillID = if mcNextInt(l, 3i32) == 0i32 {
                                forestHills as libc::c_int
                            } else {
                                forest as libc::c_int
                            }
                        }
                        12 => hillID = iceMountains as libc::c_int,
                        21 => hillID = jungleHills as libc::c_int,
                        0 => hillID = deepOcean as libc::c_int,
                        3 => hillID = extremeHillsPlus as libc::c_int,
                        35 => hillID = savannaPlateau as libc::c_int,
                        _ => {
                            if 0 != equalOrPlateau(a11, mesaPlateau_F as libc::c_int) {
                                hillID = mesa as libc::c_int
                            } else if (a11 == deepOcean as libc::c_int
                                || a11 == lukewarmDeepOcean as libc::c_int
                                || a11 == coldDeepOcean as libc::c_int
                                || a11 == frozenDeepOcean as libc::c_int)
                                && mcNextInt(l, 3i32) == 0i32
                            {
                                hillID = if mcNextInt(l, 2i32) == 0i32 {
                                    plains as libc::c_int
                                } else {
                                    forest as libc::c_int
                                }
                            }
                        }
                    }
                    if bn == 0i32 && hillID != a11 {
                        if 0 != biomeExists(hillID + 128i32) {
                            hillID += 128i32
                        } else {
                            hillID = a11
                        }
                    }
                    if hillID != a11 {
                        let mut a10: libc::c_int =
                            *buf.offset((x + 1i32 + (z + 0i32) * pWidth) as isize);
                        let mut a21: libc::c_int =
                            *buf.offset((x + 2i32 + (z + 1i32) * pWidth) as isize);
                        let mut a01: libc::c_int =
                            *buf.offset((x + 0i32 + (z + 1i32) * pWidth) as isize);
                        let mut a12: libc::c_int =
                            *buf.offset((x + 1i32 + (z + 2i32) * pWidth) as isize);
                        let mut equals: libc::c_int = 0i32;
                        if 0 != equalOrPlateau(a10, a11) {
                            equals += 1
                        }
                        if 0 != equalOrPlateau(a21, a11) {
                            equals += 1
                        }
                        if 0 != equalOrPlateau(a01, a11) {
                            equals += 1
                        }
                        if 0 != equalOrPlateau(a12, a11) {
                            equals += 1
                        }
                        if equals >= 3i32 {
                            *out.offset(idx as isize) = hillID
                        } else {
                            *out.offset(idx as isize) = a11
                        }
                    } else {
                        *out.offset(idx as isize) = a11
                    }
                } else {
                    *out.offset(idx as isize) = a11
                }
                x += 1
            }
            z += 1
        }
        free(buf as *mut libc::c_void);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mapOceanTemp(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut rnd: *mut OceanRnd_0 = (*l).oceanRnd;
    z = 0i32;
    while z < areaHeight {
        x = 0i32;
        while x < areaWidth {
            let mut tmp: libc::c_double = getOceanTemp(
                rnd,
                (x + areaX) as libc::c_double / 8.0f64,
                (z + areaZ) as libc::c_double / 8.0f64,
                0i32 as libc::c_double,
            );
            if tmp > 0.4f64 {
                *out.offset((x + z * areaWidth) as isize) = warmOcean as libc::c_int
            } else if tmp > 0.2f64 {
                *out.offset((x + z * areaWidth) as isize) = lukewarmOcean as libc::c_int
            } else if tmp < -0.4f64 {
                *out.offset((x + z * areaWidth) as isize) = frozenOcean as libc::c_int
            } else if tmp < -0.2f64 {
                *out.offset((x + z * areaWidth) as isize) = coldOcean as libc::c_int
            } else {
                *out.offset((x + z * areaWidth) as isize) = ocean as libc::c_int
            }
            x += 1
        }
        z += 1
    }
}
unsafe extern "C" fn getOceanTemp(
    mut rnd: *const OceanRnd_0,
    mut d1: libc::c_double,
    mut d2: libc::c_double,
    mut d3: libc::c_double,
) -> libc::c_double {
    d1 += (*rnd).a;
    d2 += (*rnd).b;
    d3 += (*rnd).c;
    let mut i1: libc::c_int = d1 as libc::c_int - (d1 < 0i32 as libc::c_double) as libc::c_int;
    let mut i2: libc::c_int = d2 as libc::c_int - (d2 < 0i32 as libc::c_double) as libc::c_int;
    let mut i3: libc::c_int = d3 as libc::c_int - (d3 < 0i32 as libc::c_double) as libc::c_int;
    d1 -= i1 as libc::c_double;
    d2 -= i2 as libc::c_double;
    d3 -= i3 as libc::c_double;
    let mut t1: libc::c_double = d1 * d1 * d1 * (d1 * (d1 * 6.0f64 - 15.0f64) + 10.0f64);
    let mut t2: libc::c_double = d2 * d2 * d2 * (d2 * (d2 * 6.0f64 - 15.0f64) + 10.0f64);
    let mut t3: libc::c_double = d3 * d3 * d3 * (d3 * (d3 * 6.0f64 - 15.0f64) + 10.0f64);
    i1 &= 0xffi32;
    i2 &= 0xffi32;
    i3 &= 0xffi32;
    let mut a1: libc::c_int = (*rnd).d[i1 as usize] + i2;
    let mut a2: libc::c_int = (*rnd).d[a1 as usize] + i3;
    let mut a3: libc::c_int = (*rnd).d[(a1 + 1i32) as usize] + i3;
    let mut b1: libc::c_int = (*rnd).d[(i1 + 1i32) as usize] + i2;
    let mut b2: libc::c_int = (*rnd).d[b1 as usize] + i3;
    let mut b3: libc::c_int = (*rnd).d[(b1 + 1i32) as usize] + i3;
    let mut l1: libc::c_double = indexedLerp((*rnd).d[a2 as usize], d1, d2, d3);
    let mut l2: libc::c_double =
        indexedLerp((*rnd).d[b2 as usize], d1 - 1i32 as libc::c_double, d2, d3);
    let mut l3: libc::c_double =
        indexedLerp((*rnd).d[a3 as usize], d1, d2 - 1i32 as libc::c_double, d3);
    let mut l4: libc::c_double = indexedLerp(
        (*rnd).d[b3 as usize],
        d1 - 1i32 as libc::c_double,
        d2 - 1i32 as libc::c_double,
        d3,
    );
    let mut l5: libc::c_double = indexedLerp(
        (*rnd).d[(a2 + 1i32) as usize],
        d1,
        d2,
        d3 - 1i32 as libc::c_double,
    );
    let mut l6: libc::c_double = indexedLerp(
        (*rnd).d[(b2 + 1i32) as usize],
        d1 - 1i32 as libc::c_double,
        d2,
        d3 - 1i32 as libc::c_double,
    );
    let mut l7: libc::c_double = indexedLerp(
        (*rnd).d[(a3 + 1i32) as usize],
        d1,
        d2 - 1i32 as libc::c_double,
        d3 - 1i32 as libc::c_double,
    );
    let mut l8: libc::c_double = indexedLerp(
        (*rnd).d[(b3 + 1i32) as usize],
        d1 - 1i32 as libc::c_double,
        d2 - 1i32 as libc::c_double,
        d3 - 1i32 as libc::c_double,
    );
    l1 = lerp(t1, l1, l2);
    l3 = lerp(t1, l3, l4);
    l5 = lerp(t1, l5, l6);
    l7 = lerp(t1, l7, l8);
    l1 = lerp(t2, l1, l3);
    l5 = lerp(t2, l5, l7);
    return lerp(t3, l1, l5);
}
unsafe extern "C" fn indexedLerp(
    mut idx: libc::c_int,
    d1: libc::c_double,
    d2: libc::c_double,
    d3: libc::c_double,
) -> libc::c_double {
    idx &= 0xfi32;
    return cEdgeX[idx as usize] * d1 + cEdgeY[idx as usize] * d2 + cEdgeZ[idx as usize] * d3;
}
#[no_mangle]
pub static mut cEdgeZ: [libc::c_double; 16] = unsafe {
    [
        0.0f64, 0.0f64, 0.0f64, 0.0f64, 1.0f64, 1.0f64, -1.0f64, -1.0f64, 1.0f64, 1.0f64, -1.0f64,
        -1.0f64, 0.0f64, 1.0f64, 0.0f64, -1.0f64,
    ]
};
#[no_mangle]
pub static mut cEdgeY: [libc::c_double; 16] = unsafe {
    [
        1.0f64, 1.0f64, -1.0f64, -1.0f64, 0.0f64, 0.0f64, 0.0f64, 0.0f64, 1.0f64, -1.0f64, 1.0f64,
        -1.0f64, 1.0f64, -1.0f64, 1.0f64, -1.0f64,
    ]
};
/* Table of vectors to cube edge centres (12 + 4 extra), used for ocean PRNG */
#[no_mangle]
pub static mut cEdgeX: [libc::c_double; 16] = unsafe {
    [
        1.0f64, -1.0f64, 1.0f64, -1.0f64, 1.0f64, -1.0f64, 1.0f64, -1.0f64, 0.0f64, 0.0f64, 0.0f64,
        0.0f64, 1.0f64, 0.0f64, -1.0f64, 0.0f64,
    ]
};
unsafe extern "C" fn lerp(
    part: libc::c_double,
    from: libc::c_double,
    to: libc::c_double,
) -> libc::c_double {
    return from + part * (to - from);
}
#[no_mangle]
pub unsafe extern "C" fn mapOceanMix(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut current_block: u64;
    let mut landX: libc::c_int = areaX - 8i32;
    let mut landZ: libc::c_int = areaZ - 8i32;
    let mut landWidth: libc::c_int = areaWidth + 17i32;
    let mut landHeight: libc::c_int = areaHeight + 17i32;
    let mut map1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut map2: *mut libc::c_int = 0 as *mut libc::c_int;
    if (*l).p2.is_null() {
        printf(
            b"mapOceanMix() requires two parents! Use setupMultiLayer()\n\x00" as *const u8
                as *const libc::c_char,
        );
        exit(1i32);
    } else {
        (*(*l).p).getMap.expect("non-null function pointer")(
            (*l).p,
            out,
            landX,
            landZ,
            landWidth,
            landHeight,
        );
        map1 = malloc(
            ((landWidth * landHeight) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        memcpy(
            map1 as *mut libc::c_void,
            out as *const libc::c_void,
            ((landWidth * landHeight) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        (*(*l).p2).getMap.expect("non-null function pointer")(
            (*l).p2,
            out,
            areaX,
            areaZ,
            areaWidth,
            areaHeight,
        );
        map2 = malloc(
            ((areaWidth * areaHeight) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) as *mut libc::c_int;
        memcpy(
            map2 as *mut libc::c_void,
            out as *const libc::c_void,
            ((areaWidth * areaHeight) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        let mut x: libc::c_int = 0;
        let mut z: libc::c_int = 0;
        z = 0i32;
        while z < areaHeight {
            x = 0i32;
            while x < areaWidth {
                let mut landID: libc::c_int =
                    *map1.offset((x + 8i32 + (z + 8i32) * landWidth) as isize);
                let mut oceanID: libc::c_int = *map2.offset((x + z * areaWidth) as isize);
                if 0 == isOceanic(landID) {
                    *out.offset((x + z * areaWidth) as isize) = landID
                } else {
                    let mut i: libc::c_int = -8i32;
                    's_42: loop {
                        if !(i <= 8i32) {
                            current_block = 13513818773234778473;
                            break;
                        }
                        let mut j: libc::c_int = -8i32;
                        while j <= 8i32 {
                            let mut nearbyID: libc::c_int =
                                *map1.offset((x + i + 8i32 + (z + j + 8i32) * landWidth) as isize);
                            if !(0 != isOceanic(nearbyID)) {
                                if oceanID == warmOcean as libc::c_int {
                                    *out.offset((x + z * areaWidth) as isize) =
                                        lukewarmOcean as libc::c_int;
                                    current_block = 15427931788582360902;
                                    break 's_42;
                                } else if oceanID == frozenOcean as libc::c_int {
                                    *out.offset((x + z * areaWidth) as isize) =
                                        coldOcean as libc::c_int;
                                    current_block = 15427931788582360902;
                                    break 's_42;
                                }
                            }
                            j += 4i32
                        }
                        i += 4i32
                    }
                    match current_block {
                        15427931788582360902 => {}
                        _ => {
                            if landID == deepOcean as libc::c_int {
                                if oceanID == lukewarmOcean as libc::c_int {
                                    *out.offset((x + z * areaWidth) as isize) =
                                        lukewarmDeepOcean as libc::c_int;
                                    current_block = 15427931788582360902;
                                } else if oceanID == ocean as libc::c_int {
                                    *out.offset((x + z * areaWidth) as isize) =
                                        deepOcean as libc::c_int;
                                    current_block = 15427931788582360902;
                                } else if oceanID == coldOcean as libc::c_int {
                                    *out.offset((x + z * areaWidth) as isize) =
                                        coldDeepOcean as libc::c_int;
                                    current_block = 15427931788582360902;
                                } else if oceanID == frozenOcean as libc::c_int {
                                    *out.offset((x + z * areaWidth) as isize) =
                                        frozenDeepOcean as libc::c_int;
                                    current_block = 15427931788582360902;
                                } else {
                                    current_block = 12349973810996921269;
                                }
                            } else {
                                current_block = 12349973810996921269;
                            }
                            match current_block {
                                15427931788582360902 => {}
                                _ => *out.offset((x + z * areaWidth) as isize) = oceanID,
                            }
                        }
                    }
                }
                x += 1
            }
            z += 1
        }
        free(map1 as *mut libc::c_void);
        free(map2 as *mut libc::c_void);
        return;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mapVoronoiZoom(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    areaX -= 2i32;
    areaZ -= 2i32;
    let mut pX: libc::c_int = areaX >> 2i32;
    let mut pZ: libc::c_int = areaZ >> 2i32;
    let mut pWidth: libc::c_int = (areaWidth >> 2i32) + 2i32;
    let mut pHeight: libc::c_int = (areaHeight >> 2i32) + 2i32;
    let mut newWidth: libc::c_int = pWidth - 1i32 << 2i32;
    let mut newHeight: libc::c_int = pHeight - 1i32 << 2i32;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut buf: *mut libc::c_int = malloc(
        (((newWidth + 1i32) * (newHeight + 1i32)) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    (*(*l).p).getMap.expect("non-null function pointer")((*l).p, out, pX, pZ, pWidth, pHeight);
    z = 0i32;
    while z < pHeight - 1i32 {
        let mut v00: libc::c_int = *out.offset(((z + 0i32) * pWidth) as isize);
        let mut v01: libc::c_int = *out.offset(((z + 1i32) * pWidth) as isize);
        x = 0i32;
        while x < pWidth - 1i32 {
            setChunkSeed(l, (x + pX << 2i32) as int64_t, (z + pZ << 2i32) as int64_t);
            let mut da1: libc::c_double =
                (mcNextInt(l, 1024i32) as libc::c_double / 1024.0f64 - 0.5f64) * 3.6f64;
            let mut da2: libc::c_double =
                (mcNextInt(l, 1024i32) as libc::c_double / 1024.0f64 - 0.5f64) * 3.6f64;
            setChunkSeed(
                l,
                (x + pX + 1i32 << 2i32) as int64_t,
                (z + pZ << 2i32) as int64_t,
            );
            let mut db1: libc::c_double =
                (mcNextInt(l, 1024i32) as libc::c_double / 1024.0f64 - 0.5f64) * 3.6f64 + 4.0f64;
            let mut db2: libc::c_double =
                (mcNextInt(l, 1024i32) as libc::c_double / 1024.0f64 - 0.5f64) * 3.6f64;
            setChunkSeed(
                l,
                (x + pX << 2i32) as int64_t,
                (z + pZ + 1i32 << 2i32) as int64_t,
            );
            let mut dc1: libc::c_double =
                (mcNextInt(l, 1024i32) as libc::c_double / 1024.0f64 - 0.5f64) * 3.6f64;
            let mut dc2: libc::c_double =
                (mcNextInt(l, 1024i32) as libc::c_double / 1024.0f64 - 0.5f64) * 3.6f64 + 4.0f64;
            setChunkSeed(
                l,
                (x + pX + 1i32 << 2i32) as int64_t,
                (z + pZ + 1i32 << 2i32) as int64_t,
            );
            let mut dd1: libc::c_double =
                (mcNextInt(l, 1024i32) as libc::c_double / 1024.0f64 - 0.5f64) * 3.6f64 + 4.0f64;
            let mut dd2: libc::c_double =
                (mcNextInt(l, 1024i32) as libc::c_double / 1024.0f64 - 0.5f64) * 3.6f64 + 4.0f64;
            let mut v10: libc::c_int =
                *out.offset((x + 1i32 + (z + 0i32) * pWidth) as isize) & 255i32;
            let mut v11: libc::c_int =
                *out.offset((x + 1i32 + (z + 1i32) * pWidth) as isize) & 255i32;
            j = 0i32;
            while j < 4i32 {
                let mut idx: libc::c_int = ((z << 2i32) + j) * newWidth + (x << 2i32);
                i = 0i32;
                while i < 4i32 {
                    let mut da: libc::c_double = (j as libc::c_double - da2)
                        * (j as libc::c_double - da2)
                        + (i as libc::c_double - da1) * (i as libc::c_double - da1);
                    let mut db: libc::c_double = (j as libc::c_double - db2)
                        * (j as libc::c_double - db2)
                        + (i as libc::c_double - db1) * (i as libc::c_double - db1);
                    let mut dc: libc::c_double = (j as libc::c_double - dc2)
                        * (j as libc::c_double - dc2)
                        + (i as libc::c_double - dc1) * (i as libc::c_double - dc1);
                    let mut dd: libc::c_double = (j as libc::c_double - dd2)
                        * (j as libc::c_double - dd2)
                        + (i as libc::c_double - dd1) * (i as libc::c_double - dd1);
                    if da < db && da < dc && da < dd {
                        let fresh1 = idx;
                        idx = idx + 1;
                        *buf.offset(fresh1 as isize) = v00
                    } else if db < da && db < dc && db < dd {
                        let fresh2 = idx;
                        idx = idx + 1;
                        *buf.offset(fresh2 as isize) = v10
                    } else if dc < da && dc < db && dc < dd {
                        let fresh3 = idx;
                        idx = idx + 1;
                        *buf.offset(fresh3 as isize) = v01
                    } else {
                        let fresh4 = idx;
                        idx = idx + 1;
                        *buf.offset(fresh4 as isize) = v11
                    }
                    i += 1
                }
                j += 1
            }
            v00 = v10;
            v01 = v11;
            x += 1
        }
        z += 1
    }
    z = 0i32;
    while z < areaHeight {
        memcpy(
            &mut *out.offset((z * areaWidth) as isize) as *mut libc::c_int as *mut libc::c_void,
            &mut *buf.offset(((z + (areaZ & 3i32)) * newWidth + (areaX & 3i32)) as isize)
                as *mut libc::c_int as *const libc::c_void,
            (areaWidth as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
        );
        z += 1
    }
    free(buf as *mut libc::c_void);
}
