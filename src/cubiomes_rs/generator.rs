use libc;
extern "C" {
    pub type _IO_FILE_plus;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    //==============================================================================
    // Essentials
    //==============================================================================
    #[no_mangle]
    static mut biomes: [Biome_0; 256];
    /* Applies the given world seed to the layer and all dependent layers. */
    #[no_mangle]
    fn setWorldSeed(layer: *mut Layer_0, seed: int64_t) -> ();
    //==============================================================================
    // Layers
    //==============================================================================
    // A null layer does nothing, and can be used to apply a layer to existing data.
    #[no_mangle]
    fn mapNull(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    // A skip layer simply calls its first parent without modification.
    // This can be used as an easy way to skip a layer in a generator.
    #[no_mangle]
    fn mapSkip(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapIsland(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapZoom(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapAddIsland(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapRemoveTooMuchOcean(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapAddSnow(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapCoolWarm(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapHeatIce(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapSpecial(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapAddMushroomIsland(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapDeepOcean(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapBiome(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapRiverInit(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapBiomeEdge(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapHills(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapRiver(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapSmooth(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapRareBiome(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapShore(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapRiverMix(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    // 1.13 layers
    #[no_mangle]
    fn mapHills113(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapOceanTemp(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        areaX: libc::c_int,
        areaZ: libc::c_int,
        areaWidth: libc::c_int,
        areaHeight: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapOceanMix(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        areaX: libc::c_int,
        areaZ: libc::c_int,
        areaWidth: libc::c_int,
        areaHeight: libc::c_int,
    ) -> ();
    #[no_mangle]
    fn mapVoronoiZoom(
        l: *mut Layer_0,
        out: *mut libc::c_int,
        x: libc::c_int,
        z: libc::c_int,
        w: libc::c_int,
        h: libc::c_int,
    ) -> ();
    #[no_mangle]
    static mut stderr: *mut _IO_FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
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
pub struct LayerStack {
    pub layers: *mut Layer_0,
    pub layerNum: libc::c_int,
}
pub type LayerStack_0 = LayerStack;
pub const MC_1_11: unnamed = 4;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub const MC_1_8: unnamed = 1;
pub type unnamed = libc::c_uint;
pub type _IO_lock_t = ();
pub const MC_1_9: unnamed = 2;
pub const MC_1_12: unnamed = 5;
pub const MC_1_10: unnamed = 3;
pub type FILE = _IO_FILE;
pub const MC_1_13: unnamed = 6;
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
pub const MC_1_7: unnamed = 0;
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
#[derive(Copy, Clone, Debug)]
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
pub unsafe extern "C" fn mcNextInt(mut layer: *mut Layer_0, mut mod_0: libc::c_int) -> libc::c_int {
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
pub unsafe extern "C" fn setChunkSeed(
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
pub unsafe extern "C" fn setBaseSeed(mut layer: *mut Layer_0, mut seed: int64_t) -> () {
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
/* Initialise an instance of a generator. */
#[no_mangle]
pub unsafe extern "C" fn setupGenerator(mcversion: libc::c_int) -> LayerStack_0 {
    if mcversion <= MC_1_12 as libc::c_int {
        return setupGeneratorMC17();
    } else {
        return setupGeneratorMC113();
    };
}
#[no_mangle]
pub unsafe extern "C" fn setupGeneratorMC113() -> LayerStack_0 {
    if biomes[plains as libc::c_int as usize].id == 0i32 {
        fprintf(stderr,
                b"Warning: The biomes have to be initialised first using initBiomes() before any generator can be used.\n\x00"
                    as *const u8 as *const libc::c_char);
    }
    let mut g: LayerStack_0 = LayerStack {
        layers: 0 as *mut Layer_0,
        layerNum: 0,
    };
    g.layerNum = 52i32;
    g.layers = malloc(
        (::std::mem::size_of::<Layer_0>() as libc::c_ulong)
            .wrapping_mul(g.layerNum as libc::c_ulong),
    ) as *mut Layer_0;
    //         SCALE    LAYER          PARENT      SEED  LAYER_FUNCTION
    setupLayer(
        4096i32,
        &mut *g.layers.offset(0isize),
        0 as *mut Layer_0,
        1i32,
        Some(mapIsland),
    );
    setupLayer(
        2048i32,
        &mut *g.layers.offset(1isize),
        &mut *g.layers.offset(0isize),
        2000i32,
        Some(mapZoom),
    );
    setupLayer(
        2048i32,
        &mut *g.layers.offset(2isize),
        &mut *g.layers.offset(1isize),
        1i32,
        Some(mapAddIsland),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(3isize),
        &mut *g.layers.offset(2isize),
        2001i32,
        Some(mapZoom),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(4isize),
        &mut *g.layers.offset(3isize),
        2i32,
        Some(mapAddIsland),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(5isize),
        &mut *g.layers.offset(4isize),
        50i32,
        Some(mapAddIsland),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(6isize),
        &mut *g.layers.offset(5isize),
        70i32,
        Some(mapAddIsland),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(7isize),
        &mut *g.layers.offset(6isize),
        2i32,
        Some(mapRemoveTooMuchOcean),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(8isize),
        &mut *g.layers.offset(7isize),
        2i32,
        Some(mapAddSnow),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(9isize),
        &mut *g.layers.offset(8isize),
        3i32,
        Some(mapAddIsland),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(10isize),
        &mut *g.layers.offset(9isize),
        2i32,
        Some(mapCoolWarm),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(11isize),
        &mut *g.layers.offset(10isize),
        2i32,
        Some(mapHeatIce),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(12isize),
        &mut *g.layers.offset(11isize),
        3i32,
        Some(mapSpecial),
    );
    setupLayer(
        512i32,
        &mut *g.layers.offset(13isize),
        &mut *g.layers.offset(12isize),
        2002i32,
        Some(mapZoom),
    );
    setupLayer(
        256i32,
        &mut *g.layers.offset(14isize),
        &mut *g.layers.offset(13isize),
        2003i32,
        Some(mapZoom),
    );
    setupLayer(
        256i32,
        &mut *g.layers.offset(15isize),
        &mut *g.layers.offset(14isize),
        4i32,
        Some(mapAddIsland),
    );
    setupLayer(
        256i32,
        &mut *g.layers.offset(16isize),
        &mut *g.layers.offset(15isize),
        5i32,
        Some(mapAddMushroomIsland),
    );
    setupLayer(
        256i32,
        &mut *g.layers.offset(17isize),
        &mut *g.layers.offset(16isize),
        4i32,
        Some(mapDeepOcean),
    );
    // biome layer chain
    setupLayer(
        256i32,
        &mut *g.layers.offset(18isize),
        &mut *g.layers.offset(17isize),
        200i32,
        Some(mapBiome),
    );
    setupLayer(
        128i32,
        &mut *g.layers.offset(19isize),
        &mut *g.layers.offset(18isize),
        1000i32,
        Some(mapZoom),
    );
    setupLayer(
        64i32,
        &mut *g.layers.offset(20isize),
        &mut *g.layers.offset(19isize),
        1001i32,
        Some(mapZoom),
    );
    setupLayer(
        64i32,
        &mut *g.layers.offset(21isize),
        &mut *g.layers.offset(20isize),
        1000i32,
        Some(mapBiomeEdge),
    );
    // basic river layer chain, used to determine where hills generate
    setupLayer(
        256i32,
        &mut *g.layers.offset(22isize),
        &mut *g.layers.offset(17isize),
        100i32,
        Some(mapRiverInit),
    );
    setupLayer(
        128i32,
        &mut *g.layers.offset(23isize),
        &mut *g.layers.offset(22isize),
        1000i32,
        Some(mapZoom),
    );
    setupLayer(
        64i32,
        &mut *g.layers.offset(24isize),
        &mut *g.layers.offset(23isize),
        1001i32,
        Some(mapZoom),
    );
    setupMultiLayer(
        64i32,
        &mut *g.layers.offset(25isize),
        &mut *g.layers.offset(21isize),
        &mut *g.layers.offset(24isize),
        1000i32,
        Some(mapHills113),
    );
    setupLayer(
        64i32,
        &mut *g.layers.offset(26isize),
        &mut *g.layers.offset(25isize),
        1001i32,
        Some(mapRareBiome),
    );
    setupLayer(
        32i32,
        &mut *g.layers.offset(27isize),
        &mut *g.layers.offset(26isize),
        1000i32,
        Some(mapZoom),
    );
    setupLayer(
        32i32,
        &mut *g.layers.offset(28isize),
        &mut *g.layers.offset(27isize),
        3i32,
        Some(mapAddIsland),
    );
    setupLayer(
        16i32,
        &mut *g.layers.offset(29isize),
        &mut *g.layers.offset(28isize),
        1001i32,
        Some(mapZoom),
    );
    setupLayer(
        16i32,
        &mut *g.layers.offset(30isize),
        &mut *g.layers.offset(29isize),
        1000i32,
        Some(mapShore),
    );
    setupLayer(
        8i32,
        &mut *g.layers.offset(31isize),
        &mut *g.layers.offset(30isize),
        1002i32,
        Some(mapZoom),
    );
    setupLayer(
        4i32,
        &mut *g.layers.offset(32isize),
        &mut *g.layers.offset(31isize),
        1003i32,
        Some(mapZoom),
    );
    setupLayer(
        4i32,
        &mut *g.layers.offset(33isize),
        &mut *g.layers.offset(32isize),
        1000i32,
        Some(mapSmooth),
    );
    // river layer chain
    setupLayer(
        128i32,
        &mut *g.layers.offset(34isize),
        &mut *g.layers.offset(22isize),
        1000i32,
        Some(mapZoom),
    );
    setupLayer(
        64i32,
        &mut *g.layers.offset(35isize),
        &mut *g.layers.offset(34isize),
        1001i32,
        Some(mapZoom),
    );
    setupLayer(
        32i32,
        &mut *g.layers.offset(36isize),
        &mut *g.layers.offset(35isize),
        1000i32,
        Some(mapZoom),
    );
    setupLayer(
        16i32,
        &mut *g.layers.offset(37isize),
        &mut *g.layers.offset(36isize),
        1001i32,
        Some(mapZoom),
    );
    setupLayer(
        8i32,
        &mut *g.layers.offset(38isize),
        &mut *g.layers.offset(37isize),
        1002i32,
        Some(mapZoom),
    );
    setupLayer(
        4i32,
        &mut *g.layers.offset(39isize),
        &mut *g.layers.offset(38isize),
        1003i32,
        Some(mapZoom),
    );
    setupLayer(
        4i32,
        &mut *g.layers.offset(40isize),
        &mut *g.layers.offset(39isize),
        1i32,
        Some(mapRiver),
    );
    setupLayer(
        4i32,
        &mut *g.layers.offset(41isize),
        &mut *g.layers.offset(40isize),
        1000i32,
        Some(mapSmooth),
    );
    setupMultiLayer(
        4i32,
        &mut *g.layers.offset(42isize),
        &mut *g.layers.offset(33isize),
        &mut *g.layers.offset(41isize),
        100i32,
        Some(mapRiverMix),
    );
    // ocean variants
    setupLayer(
        256i32,
        &mut *g.layers.offset(43isize),
        0 as *mut Layer_0,
        2i32,
        Some(mapOceanTemp),
    );
    let ref mut fresh0 = (*g.layers.offset(43isize)).oceanRnd;
    *fresh0 = malloc(::std::mem::size_of::<OceanRnd_0>() as libc::c_ulong) as *mut OceanRnd_0;
    setupLayer(
        128i32,
        &mut *g.layers.offset(44isize),
        &mut *g.layers.offset(43isize),
        2001i32,
        Some(mapZoom),
    );
    setupLayer(
        64i32,
        &mut *g.layers.offset(45isize),
        &mut *g.layers.offset(44isize),
        2002i32,
        Some(mapZoom),
    );
    setupLayer(
        32i32,
        &mut *g.layers.offset(46isize),
        &mut *g.layers.offset(45isize),
        2003i32,
        Some(mapZoom),
    );
    setupLayer(
        16i32,
        &mut *g.layers.offset(47isize),
        &mut *g.layers.offset(46isize),
        2004i32,
        Some(mapZoom),
    );
    setupLayer(
        8i32,
        &mut *g.layers.offset(48isize),
        &mut *g.layers.offset(47isize),
        2005i32,
        Some(mapZoom),
    );
    setupLayer(
        4i32,
        &mut *g.layers.offset(49isize),
        &mut *g.layers.offset(48isize),
        2006i32,
        Some(mapZoom),
    );
    setupMultiLayer(
        4i32,
        &mut *g.layers.offset(50isize),
        &mut *g.layers.offset(42isize),
        &mut *g.layers.offset(49isize),
        100i32,
        Some(mapOceanMix),
    );
    setupLayer(
        1i32,
        &mut *g.layers.offset(51isize),
        &mut *g.layers.offset(50isize),
        10i32,
        Some(mapVoronoiZoom),
    );
    return g;
}
/* Set up custom layers. */
#[no_mangle]
pub unsafe extern "C" fn setupLayer(
    mut scale: libc::c_int,
    mut l: *mut Layer_0,
    mut p: *mut Layer_0,
    mut s: libc::c_int,
    mut getMap: Option<
        unsafe extern "C" fn(
            _: *mut Layer_0,
            _: *mut libc::c_int,
            _: libc::c_int,
            _: libc::c_int,
            _: libc::c_int,
            _: libc::c_int,
        ) -> (),
    >,
) -> () {
    setBaseSeed(l, s as int64_t);
    (*l).scale = scale;
    (*l).p = p;
    (*l).p2 = 0 as *mut Layer_0;
    (*l).getMap = getMap;
    (*l).oceanRnd = 0 as *mut OceanRnd_0;
}
#[no_mangle]
pub unsafe extern "C" fn setupMultiLayer(
    mut scale: libc::c_int,
    mut l: *mut Layer_0,
    mut p1: *mut Layer_0,
    mut p2: *mut Layer_0,
    mut s: libc::c_int,
    mut getMap: Option<
        unsafe extern "C" fn(
            _: *mut Layer_0,
            _: *mut libc::c_int,
            _: libc::c_int,
            _: libc::c_int,
            _: libc::c_int,
            _: libc::c_int,
        ) -> (),
    >,
) -> () {
    setBaseSeed(l, s as int64_t);
    (*l).scale = scale;
    (*l).p = p1;
    (*l).p2 = p2;
    (*l).getMap = getMap;
    (*l).oceanRnd = 0 as *mut OceanRnd_0;
}
#[no_mangle]
pub unsafe extern "C" fn setupGeneratorMC17() -> LayerStack_0 {
    if biomes[plains as libc::c_int as usize].id == 0i32 {
        fprintf(stderr,
                b"Warning: The biomes have to be initialised first using initBiomes() before any generator can be used.\n\x00"
                    as *const u8 as *const libc::c_char);
    }
    let mut g: LayerStack_0 = LayerStack {
        layers: 0 as *mut Layer_0,
        layerNum: 0,
    };
    g.layerNum = 44i32;
    g.layers = malloc(
        (::std::mem::size_of::<Layer_0>() as libc::c_ulong)
            .wrapping_mul(g.layerNum as libc::c_ulong),
    ) as *mut Layer_0;
    //         SCALE    LAYER          PARENT      SEED  LAYER_FUNCTION
    setupLayer(
        4096i32,
        &mut *g.layers.offset(0isize),
        0 as *mut Layer_0,
        1i32,
        Some(mapIsland),
    );
    setupLayer(
        2048i32,
        &mut *g.layers.offset(1isize),
        &mut *g.layers.offset(0isize),
        2000i32,
        Some(mapZoom),
    );
    setupLayer(
        2048i32,
        &mut *g.layers.offset(2isize),
        &mut *g.layers.offset(1isize),
        1i32,
        Some(mapAddIsland),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(3isize),
        &mut *g.layers.offset(2isize),
        2001i32,
        Some(mapZoom),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(4isize),
        &mut *g.layers.offset(3isize),
        2i32,
        Some(mapAddIsland),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(5isize),
        &mut *g.layers.offset(4isize),
        50i32,
        Some(mapAddIsland),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(6isize),
        &mut *g.layers.offset(5isize),
        70i32,
        Some(mapAddIsland),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(7isize),
        &mut *g.layers.offset(6isize),
        2i32,
        Some(mapRemoveTooMuchOcean),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(8isize),
        &mut *g.layers.offset(7isize),
        2i32,
        Some(mapAddSnow),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(9isize),
        &mut *g.layers.offset(8isize),
        3i32,
        Some(mapAddIsland),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(10isize),
        &mut *g.layers.offset(9isize),
        2i32,
        Some(mapCoolWarm),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(11isize),
        &mut *g.layers.offset(10isize),
        2i32,
        Some(mapHeatIce),
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(12isize),
        &mut *g.layers.offset(11isize),
        3i32,
        Some(mapSpecial),
    );
    setupLayer(
        512i32,
        &mut *g.layers.offset(13isize),
        &mut *g.layers.offset(12isize),
        2002i32,
        Some(mapZoom),
    );
    setupLayer(
        256i32,
        &mut *g.layers.offset(14isize),
        &mut *g.layers.offset(13isize),
        2003i32,
        Some(mapZoom),
    );
    setupLayer(
        256i32,
        &mut *g.layers.offset(15isize),
        &mut *g.layers.offset(14isize),
        4i32,
        Some(mapAddIsland),
    );
    setupLayer(
        256i32,
        &mut *g.layers.offset(16isize),
        &mut *g.layers.offset(15isize),
        5i32,
        Some(mapAddMushroomIsland),
    );
    setupLayer(
        256i32,
        &mut *g.layers.offset(17isize),
        &mut *g.layers.offset(16isize),
        4i32,
        Some(mapDeepOcean),
    );
    // biome layer chain
    setupLayer(
        256i32,
        &mut *g.layers.offset(18isize),
        &mut *g.layers.offset(17isize),
        200i32,
        Some(mapBiome),
    );
    setupLayer(
        128i32,
        &mut *g.layers.offset(19isize),
        &mut *g.layers.offset(18isize),
        1000i32,
        Some(mapZoom),
    );
    setupLayer(
        64i32,
        &mut *g.layers.offset(20isize),
        &mut *g.layers.offset(19isize),
        1001i32,
        Some(mapZoom),
    );
    setupLayer(
        64i32,
        &mut *g.layers.offset(21isize),
        &mut *g.layers.offset(20isize),
        1000i32,
        Some(mapBiomeEdge),
    );
    // basic river layer chain, used to determine where hills generate
    setupLayer(
        256i32,
        &mut *g.layers.offset(22isize),
        &mut *g.layers.offset(17isize),
        100i32,
        Some(mapRiverInit),
    );
    setupLayer(
        128i32,
        &mut *g.layers.offset(23isize),
        &mut *g.layers.offset(22isize),
        1000i32,
        Some(mapZoom),
    );
    setupLayer(
        64i32,
        &mut *g.layers.offset(24isize),
        &mut *g.layers.offset(23isize),
        1001i32,
        Some(mapZoom),
    );
    setupMultiLayer(
        64i32,
        &mut *g.layers.offset(25isize),
        &mut *g.layers.offset(21isize),
        &mut *g.layers.offset(24isize),
        1000i32,
        Some(mapHills),
    );
    setupLayer(
        64i32,
        &mut *g.layers.offset(26isize),
        &mut *g.layers.offset(25isize),
        1001i32,
        Some(mapRareBiome),
    );
    setupLayer(
        32i32,
        &mut *g.layers.offset(27isize),
        &mut *g.layers.offset(26isize),
        1000i32,
        Some(mapZoom),
    );
    setupLayer(
        32i32,
        &mut *g.layers.offset(28isize),
        &mut *g.layers.offset(27isize),
        3i32,
        Some(mapAddIsland),
    );
    setupLayer(
        16i32,
        &mut *g.layers.offset(29isize),
        &mut *g.layers.offset(28isize),
        1001i32,
        Some(mapZoom),
    );
    setupLayer(
        16i32,
        &mut *g.layers.offset(30isize),
        &mut *g.layers.offset(29isize),
        1000i32,
        Some(mapShore),
    );
    setupLayer(
        8i32,
        &mut *g.layers.offset(31isize),
        &mut *g.layers.offset(30isize),
        1002i32,
        Some(mapZoom),
    );
    setupLayer(
        4i32,
        &mut *g.layers.offset(32isize),
        &mut *g.layers.offset(31isize),
        1003i32,
        Some(mapZoom),
    );
    setupLayer(
        4i32,
        &mut *g.layers.offset(33isize),
        &mut *g.layers.offset(32isize),
        1000i32,
        Some(mapSmooth),
    );
    // river layer chain
    setupLayer(
        128i32,
        &mut *g.layers.offset(34isize),
        &mut *g.layers.offset(22isize),
        1000i32,
        Some(mapZoom),
    );
    setupLayer(
        64i32,
        &mut *g.layers.offset(35isize),
        &mut *g.layers.offset(34isize),
        1001i32,
        Some(mapZoom),
    );
    setupLayer(
        32i32,
        &mut *g.layers.offset(36isize),
        &mut *g.layers.offset(35isize),
        1000i32,
        Some(mapZoom),
    );
    setupLayer(
        16i32,
        &mut *g.layers.offset(37isize),
        &mut *g.layers.offset(36isize),
        1001i32,
        Some(mapZoom),
    );
    setupLayer(
        8i32,
        &mut *g.layers.offset(38isize),
        &mut *g.layers.offset(37isize),
        1002i32,
        Some(mapZoom),
    );
    setupLayer(
        4i32,
        &mut *g.layers.offset(39isize),
        &mut *g.layers.offset(38isize),
        1003i32,
        Some(mapZoom),
    );
    setupLayer(
        4i32,
        &mut *g.layers.offset(40isize),
        &mut *g.layers.offset(39isize),
        1i32,
        Some(mapRiver),
    );
    setupLayer(
        4i32,
        &mut *g.layers.offset(41isize),
        &mut *g.layers.offset(40isize),
        1000i32,
        Some(mapSmooth),
    );
    setupMultiLayer(
        4i32,
        &mut *g.layers.offset(42isize),
        &mut *g.layers.offset(33isize),
        &mut *g.layers.offset(41isize),
        100i32,
        Some(mapRiverMix),
    );
    setupLayer(
        1i32,
        &mut *g.layers.offset(43isize),
        &mut *g.layers.offset(42isize),
        10i32,
        Some(mapVoronoiZoom),
    );
    return g;
}
#[no_mangle]
pub unsafe extern "C" fn setupGeneratorMC17UpTo(mut l: libc::c_int) -> LayerStack_0 {
    if biomes[plains as libc::c_int as usize].id == 0i32 {
        fprintf(stderr,
                b"Warning: The biomes have to be initialised first using initBiomes() before any generator can be used.\n\x00"
                    as *const u8 as *const libc::c_char);
    }
    let mut g: LayerStack_0 = LayerStack {
        layers: 0 as *mut Layer_0,
        layerNum: 0,
    };
    g.layerNum = 44i32;
    g.layers = malloc(
        (::std::mem::size_of::<Layer_0>() as libc::c_ulong)
            .wrapping_mul(g.layerNum as libc::c_ulong),
    ) as *mut Layer_0;
    //         SCALE    LAYER          PARENT      SEED  LAYER_FUNCTION
    setupLayer(
        4096i32,
        &mut *g.layers.offset(0isize),
        0 as *mut Layer_0,
        1i32,
        if l > 0i32 {
            Some(mapIsland)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        2048i32,
        &mut *g.layers.offset(1isize),
        &mut *g.layers.offset(0isize),
        2000i32,
        if l > 1i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        2048i32,
        &mut *g.layers.offset(2isize),
        &mut *g.layers.offset(1isize),
        1i32,
        if l > 2i32 {
            Some(mapAddIsland)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(3isize),
        &mut *g.layers.offset(2isize),
        2001i32,
        if l > 3i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(4isize),
        &mut *g.layers.offset(3isize),
        2i32,
        if l > 4i32 {
            Some(mapAddIsland)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(5isize),
        &mut *g.layers.offset(4isize),
        50i32,
        if l > 5i32 {
            Some(mapAddIsland)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(6isize),
        &mut *g.layers.offset(5isize),
        70i32,
        if l > 6i32 {
            Some(mapAddIsland)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(7isize),
        &mut *g.layers.offset(6isize),
        2i32,
        if l > 7i32 {
            Some(mapRemoveTooMuchOcean)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(8isize),
        &mut *g.layers.offset(7isize),
        2i32,
        if l > 8i32 {
            Some(mapAddSnow)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(9isize),
        &mut *g.layers.offset(8isize),
        3i32,
        if l > 9i32 {
            Some(mapAddIsland)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(10isize),
        &mut *g.layers.offset(9isize),
        2i32,
        if l > 10i32 {
            Some(mapCoolWarm)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(11isize),
        &mut *g.layers.offset(10isize),
        2i32,
        if l > 11i32 {
            Some(mapHeatIce)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        1024i32,
        &mut *g.layers.offset(12isize),
        &mut *g.layers.offset(11isize),
        3i32,
        if l > 12i32 {
            Some(mapSpecial)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        512i32,
        &mut *g.layers.offset(13isize),
        &mut *g.layers.offset(12isize),
        2002i32,
        if l > 13i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        256i32,
        &mut *g.layers.offset(14isize),
        &mut *g.layers.offset(13isize),
        2003i32,
        if l > 14i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        256i32,
        &mut *g.layers.offset(15isize),
        &mut *g.layers.offset(14isize),
        4i32,
        if l > 15i32 {
            Some(mapAddIsland)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        256i32,
        &mut *g.layers.offset(16isize),
        &mut *g.layers.offset(15isize),
        5i32,
        if l > 16i32 {
            Some(mapAddMushroomIsland)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        256i32,
        &mut *g.layers.offset(17isize),
        &mut *g.layers.offset(16isize),
        4i32,
        if l > 17i32 {
            Some(mapDeepOcean)
        } else {
            Some(mapSkip)
        },
    );
    // biome layer chain
    setupLayer(
        256i32,
        &mut *g.layers.offset(18isize),
        &mut *g.layers.offset(17isize),
        200i32,
        if l > 18i32 {
            Some(mapBiome)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        128i32,
        &mut *g.layers.offset(19isize),
        &mut *g.layers.offset(18isize),
        1000i32,
        if l > 19i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        64i32,
        &mut *g.layers.offset(20isize),
        &mut *g.layers.offset(19isize),
        1001i32,
        if l > 20i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        64i32,
        &mut *g.layers.offset(21isize),
        &mut *g.layers.offset(20isize),
        1000i32,
        if l > 21i32 {
            Some(mapBiomeEdge)
        } else {
            Some(mapSkip)
        },
    );
    // basic river layer chain, used to determine where hills generate
    setupLayer(
        256i32,
        &mut *g.layers.offset(22isize),
        &mut *g.layers.offset(17isize),
        100i32,
        if l > 22i32 {
            Some(mapRiverInit)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        128i32,
        &mut *g.layers.offset(23isize),
        &mut *g.layers.offset(22isize),
        1000i32,
        if l > 23i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        64i32,
        &mut *g.layers.offset(24isize),
        &mut *g.layers.offset(23isize),
        1001i32,
        if l > 24i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupMultiLayer(
        64i32,
        &mut *g.layers.offset(25isize),
        &mut *g.layers.offset(21isize),
        &mut *g.layers.offset(24isize),
        1000i32,
        if l > 25i32 {
            Some(mapHills)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        64i32,
        &mut *g.layers.offset(26isize),
        &mut *g.layers.offset(25isize),
        1001i32,
        if l > 26i32 {
            Some(mapRareBiome)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        32i32,
        &mut *g.layers.offset(27isize),
        &mut *g.layers.offset(26isize),
        1000i32,
        if l > 27i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        32i32,
        &mut *g.layers.offset(28isize),
        &mut *g.layers.offset(27isize),
        3i32,
        if l > 28i32 {
            Some(mapAddIsland)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        16i32,
        &mut *g.layers.offset(29isize),
        &mut *g.layers.offset(28isize),
        1001i32,
        if l > 29i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        16i32,
        &mut *g.layers.offset(30isize),
        &mut *g.layers.offset(29isize),
        1000i32,
        if l > 30i32 {
            Some(mapShore)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        8i32,
        &mut *g.layers.offset(31isize),
        &mut *g.layers.offset(30isize),
        1002i32,
        if l > 31i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        4i32,
        &mut *g.layers.offset(32isize),
        &mut *g.layers.offset(31isize),
        1003i32,
        if l > 32i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        4i32,
        &mut *g.layers.offset(33isize),
        &mut *g.layers.offset(32isize),
        1000i32,
        if l > 33i32 {
            Some(mapSmooth)
        } else {
            Some(mapSkip)
        },
    );
    // river layer chain
    setupLayer(
        128i32,
        &mut *g.layers.offset(34isize),
        &mut *g.layers.offset(22isize),
        1000i32,
        if l > 34i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        64i32,
        &mut *g.layers.offset(35isize),
        &mut *g.layers.offset(34isize),
        1001i32,
        if l > 35i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        32i32,
        &mut *g.layers.offset(36isize),
        &mut *g.layers.offset(35isize),
        1000i32,
        if l > 36i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        16i32,
        &mut *g.layers.offset(37isize),
        &mut *g.layers.offset(36isize),
        1001i32,
        if l > 37i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        8i32,
        &mut *g.layers.offset(38isize),
        &mut *g.layers.offset(37isize),
        1002i32,
        if l > 38i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        4i32,
        &mut *g.layers.offset(39isize),
        &mut *g.layers.offset(38isize),
        1003i32,
        if l > 39i32 {
            Some(mapZoom)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        4i32,
        &mut *g.layers.offset(40isize),
        &mut *g.layers.offset(39isize),
        1i32,
        if l > 40i32 {
            Some(mapRiver)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        4i32,
        &mut *g.layers.offset(41isize),
        &mut *g.layers.offset(40isize),
        1000i32,
        if l > 41i32 {
            Some(mapSmooth)
        } else {
            Some(mapSkip)
        },
    );
    setupMultiLayer(
        4i32,
        &mut *g.layers.offset(42isize),
        &mut *g.layers.offset(33isize),
        &mut *g.layers.offset(41isize),
        100i32,
        if l > 42i32 {
            Some(mapRiverMix)
        } else {
            Some(mapSkip)
        },
    );
    setupLayer(
        1i32,
        &mut *g.layers.offset(43isize),
        &mut *g.layers.offset(42isize),
        10i32,
        if l > 43i32 {
            Some(mapVoronoiZoom)
        } else {
            Some(mapSkip)
        },
    );
    return g;
}
/* Cleans up and frees the generator layers */
#[no_mangle]
pub unsafe extern "C" fn freeGenerator(mut g: LayerStack_0) -> () {
    let mut i: libc::c_int = 0;
    i = 0i32;
    while i < g.layerNum {
        if !(*g.layers.offset(i as isize)).oceanRnd.is_null() {
            free((*g.layers.offset(i as isize)).oceanRnd as *mut libc::c_void);
        }
        i += 1
    }
    free(g.layers as *mut libc::c_void);
}
/* Calculates the minimum size of the buffers required to generate an area of
 * dimensions 'sizeX' by 'sizeZ' at the specified layer.
 */
#[no_mangle]
pub unsafe extern "C" fn calcRequiredBuf(
    mut layer: *mut Layer_0,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
) -> libc::c_int {
    let mut maxX: libc::c_int = areaX;
    let mut maxZ: libc::c_int = areaZ;
    getMaxArea(layer, areaX, areaZ, &mut maxX, &mut maxZ);
    return maxX * maxZ;
}
/* Recursively calculates the minimum buffer size required to generate an area
 * of the specified size from the current layer onwards.
 */
unsafe extern "C" fn getMaxArea(
    mut layer: *mut Layer_0,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut maxX: *mut libc::c_int,
    mut maxZ: *mut libc::c_int,
) -> () {
    if layer.is_null() {
        return;
    } else {
        if (*layer).getMap == Some(mapZoom) {
            areaX = (areaX >> 1i32) + 2i32;
            areaZ = (areaZ >> 1i32) + 2i32
        } else if (*layer).getMap == Some(mapVoronoiZoom) {
            areaX = (areaX >> 2i32) + 2i32;
            areaZ = (areaZ >> 2i32) + 2i32
        } else if (*layer).getMap == Some(mapOceanMix) {
            areaX += 17i32;
            areaZ += 17i32
        } else if (*layer).getMap != Some(mapNull)
            && (*layer).getMap != Some(mapSkip)
            && (*layer).getMap != Some(mapIsland)
            && (*layer).getMap != Some(mapSpecial)
            && (*layer).getMap != Some(mapBiome)
            && (*layer).getMap != Some(mapRiverInit)
            && (*layer).getMap != Some(mapRiverMix)
            && (*layer).getMap != Some(mapOceanTemp)
        {
            areaX += 2i32;
            areaZ += 2i32
        }
        if areaX > *maxX {
            *maxX = areaX
        }
        if areaZ > *maxZ {
            *maxZ = areaZ
        }
        getMaxArea((*layer).p, areaX, areaZ, maxX, maxZ);
        getMaxArea((*layer).p2, areaX, areaZ, maxX, maxZ);
        return;
    };
}
/* Allocates an amount of memory required to generate an area of dimensions
 * 'sizeX' by 'sizeZ' for the magnification of the current top layer.
 */
#[no_mangle]
pub unsafe extern "C" fn allocCache(
    mut layer: *mut Layer_0,
    mut sizeX: libc::c_int,
    mut sizeZ: libc::c_int,
) -> *mut libc::c_int {
    let mut size: libc::c_int = calcRequiredBuf(layer, sizeX, sizeZ);
    let mut ret: *mut libc::c_int = malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(size as libc::c_ulong),
    ) as *mut libc::c_int;
    memset(
        ret as *mut libc::c_void,
        0i32,
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong).wrapping_mul(size as libc::c_ulong),
    );
    return ret;
}
/* Sets the world seed for the generator */
#[no_mangle]
pub unsafe extern "C" fn applySeed(mut g: *mut LayerStack_0, mut seed: int64_t) -> () {
    // the seed has to be applied recursively
    setWorldSeed(
        &mut *(*g).layers.offset(((*g).layerNum - 1i32) as isize),
        seed,
    );
}
/* Generates the specified area using the current generator settings and stores
 * the biomeIDs in 'out'.
 * The biomeIDs will be indexed in the form: out[x + z*areaWidth]
 * It is recommended that 'out' is allocated using allocCache() for the correct
 * buffer size.
 */
#[no_mangle]
pub unsafe extern "C" fn genArea(
    mut layer: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    memset(
        out as *mut libc::c_void,
        0i32,
        ((areaWidth * areaHeight) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    );
    (*layer).getMap.expect("non-null function pointer")(
        layer, out, areaX, areaZ, areaWidth, areaHeight,
    );
}
