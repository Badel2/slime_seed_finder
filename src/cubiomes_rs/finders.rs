use libc;
extern "C" {
    pub type _IO_FILE_plus;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    /* Applies the given world seed to the layer and all dependent layers. */
    #[no_mangle]
    fn setWorldSeed(layer: *mut Layer_0, seed: int64_t) -> ();
    /* Allocates an amount of memory required to generate an area of dimensions
     * 'sizeX' by 'sizeZ' for the magnification of the current top layer.
     */
    #[no_mangle]
    fn allocCache(layer: *mut Layer_0, sizeX: libc::c_int, sizeZ: libc::c_int) -> *mut libc::c_int;
    /* Set up custom layers. */
    #[no_mangle]
    fn setupLayer(
        scale: libc::c_int,
        l: *mut Layer_0,
        p: *mut Layer_0,
        s: libc::c_int,
        getMap: Option<
            unsafe extern "C" fn(
                _: *mut Layer_0,
                _: *mut libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
            ) -> (),
        >,
    ) -> ();
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
    fn remove(__filename: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn fscanf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn fgetc(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fread(__ptr: *mut libc::c_void, __size: size_t, __n: size_t, __stream: *mut FILE) -> size_t;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
    #[no_mangle]
    fn fseek(__stream: *mut FILE, __off: libc::c_long, __whence: libc::c_int) -> libc::c_int;
    #[no_mangle]
    fn rewind(__stream: *mut FILE) -> ();
    #[no_mangle]
    fn feof(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn perror(__s: *const libc::c_char) -> ();
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
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
    #[no_mangle]
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t_0,
        __start_routine: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    #[no_mangle]
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn round(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    static mut signgam: libc::c_int;
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
pub struct Pos {
    pub x: libc::c_int,
    pub z: libc::c_int,
}
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    __size: [libc::c_char; 56],
    __align: libc::c_long,
}
pub type pthread_attr_t_0 = pthread_attr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct quad_threadinfo_t {
    pub start: int64_t,
    pub end: int64_t,
    pub sconf: StructureConfig,
    pub threadID: libc::c_int,
    pub quality: libc::c_int,
    pub fnam: *const libc::c_char,
}
pub const Village: unnamed = 4;
pub const Swamp_Hut: unnamed = 3;
pub const Desert_Pyramid: unnamed = 0;
pub const Mansion: unnamed = 8;
pub type unnamed = libc::c_uint;
pub const Shipwreck: unnamed = 6;
pub const Monument: unnamed = 7;
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
pub type StructureConfig = StructureConfig_0;
pub const Ocean_Ruin: unnamed = 5;
pub const Jungle_Pyramid: unnamed = 2;
pub type quad_threadinfo_t_0 = quad_threadinfo_t;
pub type unnamed_0 = libc::c_uint;
pub const MC_1_13: unnamed_0 = 6;
pub const MC_1_12: unnamed_0 = 5;
pub const MC_1_11: unnamed_0 = 4;
pub const MC_1_10: unnamed_0 = 3;
pub const MC_1_9: unnamed_0 = 2;
pub const MC_1_8: unnamed_0 = 1;
pub const MC_1_7: unnamed_0 = 0;
/* Enumeration of the layer indices in the generator.
 */
pub type unnamed_1 = libc::c_uint;
pub const L13_NUM: unnamed_1 = 52;
pub const L13_VORONOI_ZOOM_1: unnamed_1 = 51;
pub const L13_OCEAN_MIX_4: unnamed_1 = 50;
pub const L13_ZOOM_4: unnamed_1 = 49;
pub const L13_ZOOM_8: unnamed_1 = 48;
pub const L13_ZOOM_16: unnamed_1 = 47;
pub const L13_ZOOM_32: unnamed_1 = 46;
pub const L13_ZOOM_64: unnamed_1 = 45;
pub const L13_ZOOM_128: unnamed_1 = 44;
// 1.13 layers
pub const L13_OCEAN_TEMP_256: unnamed_1 = 43;
pub const L_NUM: unnamed_1 = 44;
pub const L_VORONOI_ZOOM_1: unnamed_1 = 43;
pub const L_RIVER_MIX_4: unnamed_1 = 42;
pub const L_SMOOTH_4_RIVER: unnamed_1 = 41;
pub const L_RIVER_4: unnamed_1 = 40;
pub const L_ZOOM_4_RIVER: unnamed_1 = 39;
pub const L_ZOOM_8_RIVER: unnamed_1 = 38;
pub const L_ZOOM_16_RIVER: unnamed_1 = 37;
pub const L_ZOOM_32_RIVER: unnamed_1 = 36;
pub const L_ZOOM_64_RIVER: unnamed_1 = 35;
pub const L_ZOOM_128_RIVER: unnamed_1 = 34;
pub const L_SMOOTH_4: unnamed_1 = 33;
pub const L_ZOOM_4: unnamed_1 = 32;
pub const L_ZOOM_8: unnamed_1 = 31;
pub const L_SHORE_16: unnamed_1 = 30;
pub const L_ZOOM_16: unnamed_1 = 29;
pub const L_ADD_ISLAND_32: unnamed_1 = 28;
pub const L_ZOOM_32: unnamed_1 = 27;
pub const L_RARE_BIOME_64: unnamed_1 = 26;
/* Good entry for: minor biome types */
pub const L_HILLS_64: unnamed_1 = 25;
pub const L_ZOOM_64_HILLS: unnamed_1 = 24;
pub const L_ZOOM_128_HILLS: unnamed_1 = 23;
pub const L_RIVER_INIT_256: unnamed_1 = 22;
pub const L_BIOME_EDGE_64: unnamed_1 = 21;
pub const L_ZOOM_64: unnamed_1 = 20;
pub const L_ZOOM_128: unnamed_1 = 19;
/* Good entry for: major biome types */
pub const L_BIOME_256: unnamed_1 = 18;
pub const L_DEEP_OCEAN_256: unnamed_1 = 17;
/* Good entry for: mushroom biomes */
pub const L_ADD_MUSHROOM_ISLAND_256: unnamed_1 = 16;
pub const L_ADD_ISLAND_256: unnamed_1 = 15;
pub const L_ZOOM_256: unnamed_1 = 14;
pub const L_ZOOM_512: unnamed_1 = 13;
/* Good entry for: temperature categories */
pub const L_SPECIAL_1024: unnamed_1 = 12;
pub const L_HEAT_ICE_1024: unnamed_1 = 11;
pub const L_COOL_WARM_1024: unnamed_1 = 10;
pub const L_ADD_ISLAND_1024D: unnamed_1 = 9;
pub const L_ADD_SNOW_1024: unnamed_1 = 8;
pub const L_REMOVE_TOO_MUCH_OCEAN_1024: unnamed_1 = 7;
pub const L_ADD_ISLAND_1024C: unnamed_1 = 6;
pub const L_ADD_ISLAND_1024B: unnamed_1 = 5;
pub const L_ADD_ISLAND_1024A: unnamed_1 = 4;
pub const L_ZOOM_1024: unnamed_1 = 3;
pub const L_ADD_ISLAND_2048: unnamed_1 = 2;
pub const L_ZOOM_2048: unnamed_1 = 1;
pub const L_ISLAND_4096: unnamed_1 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct LayerStack {
    pub layers: *mut Layer_0,
    pub layerNum: libc::c_int,
}
pub type LayerStack_0 = LayerStack;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub const Igloo: unnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct StructureConfig_0 {
    pub seed: int64_t,
    pub regionSize: libc::c_int,
    pub chunkRange: libc::c_int,
    pub properties: libc::c_int,
}
pub type Pos_0 = Pos;
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
// Essentials
//==============================================================================
#[no_mangle]
pub static mut biomes: [Biome_0; 256] = unsafe {
    [Biome {
        id: 0,
        type_0: 0,
        height: 0.,
        temp: 0.,
        tempCat: 0,
    }; 256]
};
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
static mut FEATURE_CONFIG: StructureConfig = unsafe {
    StructureConfig_0 {
        seed: 14357617i32 as int64_t,
        regionSize: 32i32,
        chunkRange: 24i32,
        properties: 0i32,
    }
};
/* 1.13 separated feature seeds by type */
static mut DESERT_PYRAMID_CONFIG: StructureConfig = unsafe {
    StructureConfig_0 {
        seed: 14357617i32 as int64_t,
        regionSize: 32i32,
        chunkRange: 24i32,
        properties: 0i32,
    }
};
static mut IGLOO_CONFIG: StructureConfig = unsafe {
    StructureConfig_0 {
        seed: 14357618i32 as int64_t,
        regionSize: 32i32,
        chunkRange: 24i32,
        properties: 0i32,
    }
};
static mut JUNGLE_PYRAMID_CONFIG: StructureConfig = unsafe {
    StructureConfig_0 {
        seed: 14357619i32 as int64_t,
        regionSize: 32i32,
        chunkRange: 24i32,
        properties: 0i32,
    }
};
static mut SWAMP_HUT_CONFIG: StructureConfig = unsafe {
    StructureConfig_0 {
        seed: 14357620i32 as int64_t,
        regionSize: 32i32,
        chunkRange: 24i32,
        properties: 0i32,
    }
};
static mut VILLAGE_CONFIG: StructureConfig = unsafe {
    StructureConfig_0 {
        seed: 10387312i32 as int64_t,
        regionSize: 32i32,
        chunkRange: 24i32,
        properties: 0i32,
    }
};
static mut OCEAN_RUIN_CONFIG: StructureConfig = unsafe {
    StructureConfig_0 {
        seed: 14357621i32 as int64_t,
        regionSize: 16i32,
        chunkRange: 8i32,
        properties: 2i32,
    }
};
static mut SHIPWRECK_CONFIG: StructureConfig = unsafe {
    StructureConfig_0 {
        seed: 165745295i32 as int64_t,
        regionSize: 15i32,
        chunkRange: 7i32,
        properties: 0i32,
    }
};
static mut MONUMENT_CONFIG: StructureConfig = unsafe {
    StructureConfig_0 {
        seed: 10387313i32 as int64_t,
        regionSize: 32i32,
        chunkRange: 27i32,
        properties: 1i32,
    }
};
static mut MANSION_CONFIG: StructureConfig = unsafe {
    StructureConfig_0 {
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
// Saving & Loading Seeds
//==============================================================================
/* Loads a list of seeds from a file. The seeds should be written as decimal
 * UFT-8 numbers separated by newlines.
 * @fnam: file path
 * @scnt: number of valid seeds found in the file, which is also the number of
 *        elements in the returned buffer
 *
 * Return a pointer to dynamically allocated seed list.
 */
#[no_mangle]
pub unsafe extern "C" fn loadSavedSeeds(
    mut fnam: *const libc::c_char,
    mut scnt: *mut int64_t,
) -> *mut int64_t {
    let mut fp: *mut FILE = fopen(fnam, b"r\x00" as *const u8 as *const libc::c_char);
    let mut seed: int64_t = 0;
    let mut baseSeeds: *mut int64_t = 0 as *mut int64_t;
    if fp.is_null() {
        perror(b"ERR loadSavedSeeds: \x00" as *const u8 as *const libc::c_char);
        return 0 as *mut int64_t;
    } else {
        *scnt = 0i32 as int64_t;
        while 0 == feof(fp) {
            if fscanf(
                fp,
                b"%ld\x00" as *const u8 as *const libc::c_char,
                &mut seed as *mut int64_t,
            ) == 1i32
            {
                *scnt += 1
            } else {
                while 0 == feof(fp) && fgetc(fp) != '\n' as i32 {}
            }
        }
        baseSeeds = calloc(
            *scnt as libc::c_ulong,
            ::std::mem::size_of::<int64_t>() as libc::c_ulong,
        ) as *mut int64_t;
        rewind(fp);
        let mut i: int64_t = 0i32 as int64_t;
        while i < *scnt && 0 == feof(fp) {
            if fscanf(
                fp,
                b"%ld\x00" as *const u8 as *const libc::c_char,
                &mut *baseSeeds.offset(i as isize) as *mut int64_t,
            ) == 1i32
            {
                i += 1
            } else {
                while 0 == feof(fp) && fgetc(fp) != '\n' as i32 {}
            }
        }
        fclose(fp);
        return baseSeeds;
    };
}
//==============================================================================
// Multi-Structure-Base Checks
//==============================================================================
/* Calls the correct quad-base finder for the structure config, if available.
 * (Exits program otherwise.)
 */
#[no_mangle]
pub unsafe extern "C" fn isQuadBase(
    sconf: StructureConfig,
    seed: int64_t,
    qual: int64_t,
) -> libc::c_int {
    match sconf.properties {
        0 => return isQuadFeatureBase(sconf, seed, qual as libc::c_int),
        1 => return isLargeQuadBase(sconf, seed, qual as libc::c_int),
        2 => {
            fprintf(
                stderr,
                b"Quad-finder using power of 2 RNG is not implemented yet.\n\x00" as *const u8
                    as *const libc::c_char,
            );
            exit(-1i32);
        }
        3 => {
            fprintf(stderr,
                    b"Quad-finder for large structures using power of 2 RNG is not implemented yet.\n\x00"
                        as *const u8 as *const libc::c_char);
            exit(-1i32);
        }
        _ => {
            fprintf(
                stderr,
                b"Unknown properties field for structure: 0x%04X\n\x00" as *const u8
                    as *const libc::c_char,
                sconf.properties,
            );
            exit(-1i32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn isLargeQuadBase(
    sconf: StructureConfig,
    seed: int64_t,
    qual: libc::c_int,
) -> libc::c_int {
    // seed offsets for the regions (0,0) to (1,1)
    let reg00base: int64_t = sconf.seed;
    let reg01base: int64_t = 341873128712i64 + sconf.seed;
    let reg10base: int64_t = 132897987541i64 + sconf.seed;
    let reg11base: int64_t = 341873128712i64 + 132897987541i64 + sconf.seed;
    let range: libc::c_int = sconf.chunkRange;
    let rmin1: libc::c_int = range - 1i32;
    let mut s: int64_t = 0;
    let mut p: libc::c_int = 0;
    // & 0xffffffffffff;
    s = ((reg00base + seed) as libc::c_longlong ^ 0x5deece66di64) as int64_t;
    s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64 & 0xffffffffffffi64 as libc::c_longlong)
        as int64_t;
    p = (s >> 17i32) as libc::c_int % range;
    if p < rmin1 - qual {
        return 0i32;
    } else {
        s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
            & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
        p += (s >> 17i32) as libc::c_int % range;
        if p < 2i32 * rmin1 - qual {
            return 0i32;
        } else {
            s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
            p = (s >> 17i32) as libc::c_int % range;
            if p < rmin1 - qual {
                return 0i32;
            } else {
                s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                    & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
                p += (s >> 17i32) as libc::c_int % range;
                if p < 2i32 * rmin1 - qual {
                    return 0i32;
                } else {
                    // & 0xffffffffffff;
                    s = ((reg01base + seed) as libc::c_longlong ^ 0x5deece66di64) as int64_t;
                    s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                        & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
                    p = (s >> 17i32) as libc::c_int % range;
                    if p > qual {
                        return 0i32;
                    } else {
                        s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                            & 0xffffffffffffi64 as libc::c_longlong)
                            as int64_t;
                        p += (s >> 17i32) as libc::c_int % range;
                        if p > qual {
                            return 0i32;
                        } else {
                            s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                                & 0xffffffffffffi64 as libc::c_longlong)
                                as int64_t;
                            p = (s >> 17i32) as libc::c_int % range;
                            if p < rmin1 - qual {
                                return 0i32;
                            } else {
                                s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                                    & 0xffffffffffffi64 as libc::c_longlong)
                                    as int64_t;
                                p += (s >> 17i32) as libc::c_int % range;
                                if p < 2i32 * rmin1 - qual {
                                    return 0i32;
                                } else {
                                    // & 0xffffffffffff;
                                    s = ((reg10base + seed) as libc::c_longlong ^ 0x5deece66di64)
                                        as int64_t;
                                    s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                                        & 0xffffffffffffi64 as libc::c_longlong)
                                        as int64_t;
                                    p = (s >> 17i32) as libc::c_int % range;
                                    if p < rmin1 - qual {
                                        return 0i32;
                                    } else {
                                        s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                                            & 0xffffffffffffi64 as libc::c_longlong)
                                            as int64_t;
                                        p += (s >> 17i32) as libc::c_int % range;
                                        if p < 2i32 * rmin1 - qual {
                                            return 0i32;
                                        } else {
                                            s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                                                & 0xffffffffffffi64 as libc::c_longlong)
                                                as int64_t;
                                            p = (s >> 17i32) as libc::c_int % range;
                                            if p > qual {
                                                return 0i32;
                                            } else {
                                                s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                                                    & 0xffffffffffffi64 as libc::c_longlong)
                                                    as int64_t;
                                                p += (s >> 17i32) as libc::c_int % range;
                                                if p > qual {
                                                    return 0i32;
                                                } else {
                                                    // & 0xffffffffffff;
                                                    s = ((reg11base + seed) as libc::c_longlong
                                                        ^ 0x5deece66di64)
                                                        as int64_t;
                                                    s = (s as libc::c_longlong * 0x5deece66di64
                                                        + 0xbi64
                                                        & 0xffffffffffffi64 as libc::c_longlong)
                                                        as int64_t;
                                                    p = (s >> 17i32) as libc::c_int % range;
                                                    if p > qual {
                                                        return 0i32;
                                                    } else {
                                                        s = (s as libc::c_longlong * 0x5deece66di64
                                                            + 0xbi64
                                                            & 0xffffffffffffi64 as libc::c_longlong)
                                                            as int64_t;
                                                        p += (s >> 17i32) as libc::c_int % range;
                                                        if p > qual {
                                                            return 0i32;
                                                        } else {
                                                            s = (s as libc::c_longlong
                                                                * 0x5deece66di64
                                                                + 0xbi64
                                                                & 0xffffffffffffi64
                                                                    as libc::c_longlong)
                                                                as int64_t;
                                                            p = (s >> 17i32) as libc::c_int % range;
                                                            if p > qual {
                                                                return 0i32;
                                                            } else {
                                                                s = (s as libc::c_longlong
                                                                    * 0x5deece66di64
                                                                    + 0xbi64
                                                                    & 0xffffffffffffi64
                                                                        as libc::c_longlong)
                                                                    as int64_t;
                                                                p += (s >> 17i32) as libc::c_int
                                                                    % range;
                                                                if p > qual {
                                                                    return 0i32;
                                                                } else {
                                                                    return 1i32;
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
//==============================================================================
// Multi-Structure Checks
//==============================================================================
#[no_mangle]
pub unsafe extern "C" fn isQuadFeatureBase(
    sconf: StructureConfig,
    seed: int64_t,
    qual: libc::c_int,
) -> libc::c_int {
    // seed offsets for the regions (0,0) to (1,1)
    let reg00base: int64_t = sconf.seed;
    let reg01base: int64_t = 341873128712i64 + sconf.seed;
    let reg10base: int64_t = 132897987541i64 + sconf.seed;
    let reg11base: int64_t = 341873128712i64 + 132897987541i64 + sconf.seed;
    let range: libc::c_int = sconf.chunkRange;
    let upper: libc::c_int = range - qual - 1i32;
    let lower: libc::c_int = qual;
    let mut s: int64_t = 0;
    // & 0xffffffffffff;
    s = ((reg00base + seed) as libc::c_longlong ^ 0x5deece66di64) as int64_t;
    s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64 & 0xffffffffffffi64 as libc::c_longlong)
        as int64_t;
    if (s >> 17i32) as libc::c_int % range < upper {
        return 0i32;
    } else {
        s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
            & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
        if (s >> 17i32) as libc::c_int % range < upper {
            return 0i32;
        } else {
            // & 0xffffffffffff;
            s = ((reg01base + seed) as libc::c_longlong ^ 0x5deece66di64) as int64_t;
            s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
            if (s >> 17i32) as libc::c_int % range > lower {
                return 0i32;
            } else {
                s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                    & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
                if (s >> 17i32) as libc::c_int % range < upper {
                    return 0i32;
                } else {
                    // & 0xffffffffffff;
                    s = ((reg10base + seed) as libc::c_longlong ^ 0x5deece66di64) as int64_t;
                    s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                        & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
                    if (s >> 17i32) as libc::c_int % range < upper {
                        return 0i32;
                    } else {
                        s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                            & 0xffffffffffffi64 as libc::c_longlong)
                            as int64_t;
                        if (s >> 17i32) as libc::c_int % range > lower {
                            return 0i32;
                        } else {
                            // & 0xffffffffffff;
                            s = ((reg11base + seed) as libc::c_longlong ^ 0x5deece66di64)
                                as int64_t;
                            s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                                & 0xffffffffffffi64 as libc::c_longlong)
                                as int64_t;
                            if (s >> 17i32) as libc::c_int % range > lower {
                                return 0i32;
                            } else {
                                s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                                    & 0xffffffffffffi64 as libc::c_longlong)
                                    as int64_t;
                                if (s >> 17i32) as libc::c_int % range > lower {
                                    return 0i32;
                                } else {
                                    return 1i32;
                                }
                            }
                        }
                    }
                }
            }
        }
    };
}
/* Calls the correct tri-base finder for the structure config, if available.
 * (Exits program otherwise.)
 */
#[no_mangle]
pub unsafe extern "C" fn isTriBase(
    sconf: StructureConfig,
    seed: int64_t,
    qual: int64_t,
) -> libc::c_int {
    match sconf.properties {
        0 => return isTriFeatureBase(sconf, seed, qual as libc::c_int),
        1 => return isLargeTriBase(sconf, seed, qual as libc::c_int),
        2 => {
            fprintf(
                stderr,
                b"Quad-finder using power of 2 RNG is not implemented yet.\n\x00" as *const u8
                    as *const libc::c_char,
            );
            exit(-1i32);
        }
        3 => {
            fprintf(stderr,
                    b"Quad-finder for large structures using power of 2 RNG is not implemented yet.\n\x00"
                        as *const u8 as *const libc::c_char);
            exit(-1i32);
        }
        _ => {
            fprintf(
                stderr,
                b"Unknown properties field for structure: 0x%04X\n\x00" as *const u8
                    as *const libc::c_char,
                sconf.properties,
            );
            exit(-1i32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn isLargeTriBase(
    sconf: StructureConfig,
    seed: int64_t,
    qual: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    // seed offsets for the regions (0,0) to (1,1)
    let reg00base: int64_t = sconf.seed;
    let reg01base: int64_t = 341873128712i64 + sconf.seed;
    let reg10base: int64_t = 132897987541i64 + sconf.seed;
    let reg11base: int64_t = 341873128712i64 + 132897987541i64 + sconf.seed;
    let range: libc::c_int = sconf.chunkRange;
    let rmin1: libc::c_int = range - 1i32;
    let mut s: int64_t = 0;
    let mut p: libc::c_int = 0;
    let mut incomplete: libc::c_int = 0i32;
    // & 0xffffffffffff;
    s = ((reg00base + seed) as libc::c_longlong ^ 0x5deece66di64) as int64_t;
    s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64 & 0xffffffffffffi64 as libc::c_longlong)
        as int64_t;
    p = (s >> 17i32) as libc::c_int % range;
    if p < rmin1 - qual {
        current_block = 1658919162659523233;
    } else {
        s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
            & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
        p += (s >> 17i32) as libc::c_int % range;
        if p < 2i32 * rmin1 - qual {
            current_block = 1658919162659523233;
        } else {
            s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
            p = (s >> 17i32) as libc::c_int % range;
            if p < rmin1 - qual {
                current_block = 1658919162659523233;
            } else {
                s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                    & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
                p += (s >> 17i32) as libc::c_int % range;
                if p < 2i32 * rmin1 - qual {
                    current_block = 1658919162659523233;
                } else {
                    current_block = 6483416627284290920;
                }
            }
        }
    }
    match current_block {
        1658919162659523233 => incomplete = 1i32,
        _ => {}
    }
    // & 0xffffffffffff;
    s = ((reg01base + seed) as libc::c_longlong ^ 0x5deece66di64) as int64_t;
    s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64 & 0xffffffffffffi64 as libc::c_longlong)
        as int64_t;
    p = (s >> 17i32) as libc::c_int % range;
    if p > qual {
        current_block = 5282281128741493700;
    } else {
        s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
            & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
        p += (s >> 17i32) as libc::c_int % range;
        if p > qual {
            current_block = 5282281128741493700;
        } else {
            s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
            p = (s >> 17i32) as libc::c_int % range;
            if p < rmin1 - qual {
                current_block = 5282281128741493700;
            } else {
                s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                    & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
                p += (s >> 17i32) as libc::c_int % range;
                if p < 2i32 * rmin1 - qual {
                    current_block = 5282281128741493700;
                } else {
                    current_block = 12209867499936983673;
                }
            }
        }
    }
    match current_block {
        5282281128741493700 => {
            if 0 != incomplete {
                return 0i32;
            } else {
                incomplete = 2i32
            }
        }
        _ => {}
    }
    // & 0xffffffffffff;
    s = ((reg10base + seed) as libc::c_longlong ^ 0x5deece66di64) as int64_t;
    s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64 & 0xffffffffffffi64 as libc::c_longlong)
        as int64_t;
    p = (s >> 17i32) as libc::c_int % range;
    if p < rmin1 - qual {
        current_block = 15589590209569572093;
    } else {
        s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
            & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
        p += (s >> 17i32) as libc::c_int % range;
        if p < 2i32 * rmin1 - qual {
            current_block = 15589590209569572093;
        } else {
            s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
            p = (s >> 17i32) as libc::c_int % range;
            if p > qual {
                current_block = 15589590209569572093;
            } else {
                s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                    & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
                p += (s >> 17i32) as libc::c_int % range;
                if p > qual {
                    current_block = 15589590209569572093;
                } else {
                    current_block = 8457315219000651999;
                }
            }
        }
    }
    match current_block {
        15589590209569572093 => {
            if 0 != incomplete {
                return 0i32;
            } else {
                incomplete = 3i32
            }
        }
        _ => {}
    }
    // & 0xffffffffffff;
    s = ((reg11base + seed) as libc::c_longlong ^ 0x5deece66di64) as int64_t;
    s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64 & 0xffffffffffffi64 as libc::c_longlong)
        as int64_t;
    p = (s >> 17i32) as libc::c_int % range;
    if p > qual {
        current_block = 15532589080399555063;
    } else {
        s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
            & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
        p += (s >> 17i32) as libc::c_int % range;
        if p > qual {
            current_block = 15532589080399555063;
        } else {
            s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
            p = (s >> 17i32) as libc::c_int % range;
            if p > qual {
                current_block = 15532589080399555063;
            } else {
                s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64
                    & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
                p += (s >> 17i32) as libc::c_int % range;
                if p > qual {
                    current_block = 15532589080399555063;
                } else {
                    current_block = 10652014663920648156;
                }
            }
        }
    }
    match current_block {
        15532589080399555063 => {
            if 0 != incomplete {
                return 0i32;
            } else {
                incomplete = 4i32
            }
        }
        _ => {}
    }
    return if 0 != incomplete { incomplete } else { -1i32 };
}
#[no_mangle]
pub unsafe extern "C" fn isTriFeatureBase(
    sconf: StructureConfig,
    seed: int64_t,
    qual: libc::c_int,
) -> libc::c_int {
    // seed offsets for the regions (0,0) to (1,1)
    let reg00base: int64_t = sconf.seed;
    let reg01base: int64_t = 341873128712i64 + sconf.seed;
    let reg10base: int64_t = 132897987541i64 + sconf.seed;
    let reg11base: int64_t = 341873128712i64 + 132897987541i64 + sconf.seed;
    let range: libc::c_int = sconf.chunkRange;
    let upper: libc::c_int = range - qual - 1i32;
    let lower: libc::c_int = qual;
    let mut s: int64_t = 0;
    let mut missing: libc::c_int = 0i32;
    // & 0xffffffffffff;
    s = ((reg00base + seed) as libc::c_longlong ^ 0x5deece66di64) as int64_t;
    s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64 & 0xffffffffffffi64 as libc::c_longlong)
        as int64_t;
    if (s >> 17i32) as libc::c_int % range < upper
        || ((s as libc::c_longlong * 0x5deece66di64 + 0xbi64
            & 0xffffffffffffi64 as libc::c_longlong) >> 17i32) as libc::c_int % range
            < upper
    {
        missing += 1
    }
    // & 0xffffffffffff;
    s = ((reg01base + seed) as libc::c_longlong ^ 0x5deece66di64) as int64_t;
    s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64 & 0xffffffffffffi64 as libc::c_longlong)
        as int64_t;
    if (s >> 17i32) as libc::c_int % range > lower
        || ((s as libc::c_longlong * 0x5deece66di64 + 0xbi64
            & 0xffffffffffffi64 as libc::c_longlong) >> 17i32) as libc::c_int % range
            < upper
    {
        if 0 != missing {
            return 0i32;
        } else {
            missing += 1
        }
    }
    // & 0xffffffffffff;
    s = ((reg10base + seed) as libc::c_longlong ^ 0x5deece66di64) as int64_t;
    s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64 & 0xffffffffffffi64 as libc::c_longlong)
        as int64_t;
    if (s >> 17i32) as libc::c_int % range < upper
        || ((s as libc::c_longlong * 0x5deece66di64 + 0xbi64
            & 0xffffffffffffi64 as libc::c_longlong) >> 17i32) as libc::c_int % range
            > lower
    {
        if 0 != missing {
            return 0i32;
        } else {
            missing += 1
        }
    }
    // & 0xffffffffffff;
    s = ((reg11base + seed) as libc::c_longlong ^ 0x5deece66di64) as int64_t;
    s = (s as libc::c_longlong * 0x5deece66di64 + 0xbi64 & 0xffffffffffffi64 as libc::c_longlong)
        as int64_t;
    if (s >> 17i32) as libc::c_int % range > lower
        || ((s as libc::c_longlong * 0x5deece66di64 + 0xbi64
            & 0xffffffffffffi64 as libc::c_longlong) >> 17i32) as libc::c_int % range
            > lower
    {
        if 0 != missing {
            return 0i32;
        }
    }
    return 1i32;
}
/* Starts a multi-threaded search for structure base seeds  of the specified
 * quality (chunk tolerance). The result is saved in a file of path 'fnam'.
 */
#[no_mangle]
pub unsafe extern "C" fn search4QuadBases(
    mut fnam: *const libc::c_char,
    threads: libc::c_int,
    structureConfig: StructureConfig,
    quality: libc::c_int,
) -> () {
    let vla = threads as usize;
    let mut threadID: Vec<pthread_t> = ::std::vec::from_elem(0, vla);
    let vla_0 = threads as usize;
    let mut info: Vec<quad_threadinfo_t_0> = ::std::vec::from_elem(
        quad_threadinfo_t {
            start: 0,
            end: 0,
            sconf: StructureConfig_0 {
                seed: 0,
                regionSize: 0,
                chunkRange: 0,
                properties: 0,
            },
            threadID: 0,
            quality: 0,
            fnam: 0 as *const libc::c_char,
        },
        vla_0,
    );
    let mut t: int64_t = 0;
    t = 0i32 as int64_t;
    while t < threads as libc::c_long {
        (*info.as_mut_ptr().offset(t as isize)).threadID = t as libc::c_int;
        (*info.as_mut_ptr().offset(t as isize)).start =
            (t as libc::c_longlong * (1i64 << 48i32) / threads as libc::c_longlong
                & 0xffffffff0000i64 as libc::c_longlong) as int64_t;
        (*info.as_mut_ptr().offset(t as isize)).end =
            (((*info.as_mut_ptr().offset(t as isize)).start as libc::c_longlong
                + ((1i64 << 48i32) - 1i32 as libc::c_longlong) / threads as libc::c_longlong
                & 0xffffffff0000i64 as libc::c_longlong) + 1i32 as libc::c_longlong)
                as int64_t;
        let ref mut fresh0 = (*info.as_mut_ptr().offset(t as isize)).fnam;
        *fresh0 = fnam;
        (*info.as_mut_ptr().offset(t as isize)).quality = quality;
        (*info.as_mut_ptr().offset(t as isize)).sconf = structureConfig;
        t += 1
    }
    t = 0i32 as int64_t;
    while t < threads as libc::c_long {
        pthread_create(
            &mut *threadID.as_mut_ptr().offset(t as isize),
            0 as *const pthread_attr_t_0,
            Some(search4QuadBasesThread),
            &mut *info.as_mut_ptr().offset(t as isize) as *mut quad_threadinfo_t_0
                as *mut libc::c_void,
        );
        t += 1
    }
    t = 0i32 as int64_t;
    while t < threads as libc::c_long {
        pthread_join(
            *threadID.as_mut_ptr().offset(t as isize),
            0 as *mut *mut libc::c_void,
        );
        t += 1
    }
    // merge thread parts
    let mut fnamThread: [libc::c_char; 256] = [0; 256];
    let mut buffer: [libc::c_char; 4097] = [0; 4097];
    let mut fp: *mut FILE = fopen(fnam, b"w\x00" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        fprintf(
            stderr,
            b"Could not open \"%s\" for writing.\n\x00" as *const u8 as *const libc::c_char,
            fnam,
        );
        exit(-1i32);
    } else {
        let mut fpart: *mut FILE = 0 as *mut FILE;
        let mut n: libc::c_int = 0;
        t = 0i32 as int64_t;
        while t < threads as libc::c_long {
            sprintf(
                fnamThread.as_mut_ptr(),
                b"%s.part%d\x00" as *const u8 as *const libc::c_char,
                (*info.as_mut_ptr().offset(t as isize)).fnam,
                (*info.as_mut_ptr().offset(t as isize)).threadID,
            );
            fpart = fopen(
                fnamThread.as_mut_ptr(),
                b"r\x00" as *const u8 as *const libc::c_char,
            );
            if fpart.is_null() {
                perror(b"ERR search4QuadBases: \x00" as *const u8 as *const libc::c_char);
                break;
            } else {
                loop {
                    n = fread(
                        buffer.as_mut_ptr() as *mut libc::c_void,
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        4096i32 as size_t,
                        fpart,
                    ) as libc::c_int;
                    if !(0 != n) {
                        break;
                    }
                    if !(0 == fwrite(
                        buffer.as_mut_ptr() as *const libc::c_void,
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        n as size_t,
                        fp,
                    )) {
                        continue;
                    }
                    perror(b"ERR search4QuadBases: \x00" as *const u8 as *const libc::c_char);
                    fclose(fp);
                    fclose(fpart);
                    return;
                }
                fclose(fpart);
                remove(fnamThread.as_mut_ptr());
                t += 1
            }
        }
        fclose(fp);
        return;
    };
}
unsafe extern "C" fn search4QuadBasesThread(mut data: *mut libc::c_void) -> *mut libc::c_void {
    let mut info: quad_threadinfo_t_0 = *(data as *mut quad_threadinfo_t_0);
    let start: int64_t = info.start;
    let end: int64_t = info.end;
    let structureSeed: int64_t = info.sconf.seed;
    let mut seed: int64_t = 0;
    let mut lowerBits: *mut int64_t = 0 as *mut int64_t;
    let mut lowerBitsCnt: libc::c_int = 0;
    let mut lowerBitsIdx: libc::c_int = 0i32;
    let mut i: libc::c_int = 0;
    lowerBits = malloc(
        (0x10000i32 as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<int64_t>() as libc::c_ulong),
    ) as *mut int64_t;
    if info.quality == 1i32 {
        lowerBitsCnt = (::std::mem::size_of::<[int64_t; 4]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<int64_t>() as libc::c_ulong)
            as libc::c_int;
        i = 0i32;
        while i < lowerBitsCnt {
            *lowerBits.offset(i as isize) =
                lowerBaseBitsQ1[i as usize] - structureSeed & 0xffffi32 as libc::c_long;
            i += 1
        }
    } else if info.quality == 2i32 {
        lowerBitsCnt = (::std::mem::size_of::<[int64_t; 149]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<int64_t>() as libc::c_ulong)
            as libc::c_int;
        i = 0i32;
        while i < lowerBitsCnt {
            *lowerBits.offset(i as isize) =
                lowerBaseBitsQ2[i as usize] - structureSeed & 0xffffi32 as libc::c_long;
            i += 1
        }
    } else {
        printf(b"WARN search4QuadBasesThread: Lower bits for quality %d have not been defined => will try all combinations.\n\x00"
                   as *const u8 as *const libc::c_char, info.quality);
        lowerBitsCnt = 0x10000i32;
        i = 0i32;
        while i < lowerBitsCnt {
            *lowerBits.offset(i as isize) = i as int64_t;
            i += 1
        }
    }
    let mut fnam: [libc::c_char; 256] = [0; 256];
    sprintf(
        fnam.as_mut_ptr(),
        b"%s.part%d\x00" as *const u8 as *const libc::c_char,
        info.fnam,
        info.threadID,
    );
    let mut fp: *mut FILE = fopen(
        fnam.as_mut_ptr(),
        b"a+\x00" as *const u8 as *const libc::c_char,
    );
    if fp.is_null() {
        fprintf(
            stderr,
            b"Could not open \"%s\" for writing.\n\x00" as *const u8 as *const libc::c_char,
            fnam.as_mut_ptr(),
        );
        free(lowerBits as *mut libc::c_void);
        exit(-1i32);
    } else {
        seed = start;
        // Check the last entry in the file and use it as a starting point if it
        // exists. (I.e. loading the saved progress.)
        if 0 == fseek(fp, -31i32 as libc::c_long, 2i32) {
            let mut buf: [libc::c_char; 32] = [0; 32];
            if fread(
                buf.as_mut_ptr() as *mut libc::c_void,
                30i32 as size_t,
                1i32 as size_t,
                fp,
            ) > 0i32 as libc::c_ulong
            {
                let mut last_newline: *mut libc::c_char = strrchr(buf.as_mut_ptr(), '\n' as i32);
                if sscanf(
                    last_newline,
                    b"%ld\x00" as *const u8 as *const libc::c_char,
                    &mut seed as *mut int64_t,
                ) == 1i32
                {
                    while *lowerBits.offset(lowerBitsIdx as isize)
                        <= seed & 0xffffi32 as libc::c_long
                    {
                        lowerBitsIdx += 1
                    }
                    seed = (seed & 0xffffffff0000i64) + *lowerBits.offset(lowerBitsIdx as isize);
                    printf(
                        b"Thread %d starting from: %ld\n\x00" as *const u8 as *const libc::c_char,
                        info.threadID,
                        seed,
                    );
                } else {
                    seed = start
                }
            }
        }
        fseek(fp, 0i32 as libc::c_long, 2i32);
        while seed < end {
            if 0 != isQuadBase(info.sconf, seed, info.quality as int64_t) {
                fprintf(fp, b"%ld\n\x00" as *const u8 as *const libc::c_char, seed);
                fflush(fp);
            }
            //printf("Thread %d: %"PRId64"\n", info.threadID, seed);
            lowerBitsIdx += 1;
            if lowerBitsIdx >= lowerBitsCnt {
                lowerBitsIdx = 0i32;
                seed += 0x10000i32 as libc::c_long
            }
            seed = (seed & 0xffffffff0000i64) + *lowerBits.offset(lowerBitsIdx as isize)
        }
        fclose(fp);
        free(lowerBits as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    };
}
// for quad-structure with quality 2
#[no_mangle]
pub static mut lowerBaseBitsQ2: [int64_t; 149] = unsafe {
    [
        0x770i32 as int64_t,
        0x775i32 as int64_t,
        0x7adi32 as int64_t,
        0x7b2i32 as int64_t,
        0xc3ai32 as int64_t,
        0xc58i32 as int64_t,
        0xcbai32 as int64_t,
        0xcd8i32 as int64_t,
        0xe38i32 as int64_t,
        0xe5ai32 as int64_t,
        0xed8i32 as int64_t,
        0xedai32 as int64_t,
        0x111ci32 as int64_t,
        0x1c96i32 as int64_t,
        0x2048i32 as int64_t,
        0x20e8i32 as int64_t,
        0x2248i32 as int64_t,
        0x224ai32 as int64_t,
        0x22c8i32 as int64_t,
        0x258di32 as int64_t,
        0x272di32 as int64_t,
        0x2732i32 as int64_t,
        0x2739i32 as int64_t,
        0x2758i32 as int64_t,
        0x275di32 as int64_t,
        0x27c8i32 as int64_t,
        0x27c9i32 as int64_t,
        0x2aa9i32 as int64_t,
        0x2c3ai32 as int64_t,
        0x2cbai32 as int64_t,
        0x2eb8i32 as int64_t,
        0x308ci32 as int64_t,
        0x3206i32 as int64_t,
        0x371ai32 as int64_t,
        0x3890i32 as int64_t,
        0x3d0ai32 as int64_t,
        0x3f18i32 as int64_t,
        0x4068i32 as int64_t,
        0x40cai32 as int64_t,
        0x40e8i32 as int64_t,
        0x418ai32 as int64_t,
        0x4248i32 as int64_t,
        0x426ai32 as int64_t,
        0x42eai32 as int64_t,
        0x4732i32 as int64_t,
        0x4738i32 as int64_t,
        0x4739i32 as int64_t,
        0x4765i32 as int64_t,
        0x4768i32 as int64_t,
        0x476ai32 as int64_t,
        0x47b0i32 as int64_t,
        0x47b5i32 as int64_t,
        0x47d4i32 as int64_t,
        0x47d9i32 as int64_t,
        0x47e8i32 as int64_t,
        0x4c58i32 as int64_t,
        0x4e38i32 as int64_t,
        0x4eb8i32 as int64_t,
        0x4edai32 as int64_t,
        0x5118i32 as int64_t,
        0x520ai32 as int64_t,
        0x5618i32 as int64_t,
        0x5918i32 as int64_t,
        0x591di32 as int64_t,
        0x5a08i32 as int64_t,
        0x5e18i32 as int64_t,
        0x5f1ci32 as int64_t,
        0x60cai32 as int64_t,
        0x6739i32 as int64_t,
        0x6748i32 as int64_t,
        0x6749i32 as int64_t,
        0x6758i32 as int64_t,
        0x6776i32 as int64_t,
        0x67b4i32 as int64_t,
        0x67b9i32 as int64_t,
        0x67c9i32 as int64_t,
        0x67d8i32 as int64_t,
        0x67ddi32 as int64_t,
        0x67eci32 as int64_t,
        0x6c3ai32 as int64_t,
        0x6c58i32 as int64_t,
        0x6cbai32 as int64_t,
        0x6d9ai32 as int64_t,
        0x6e5ai32 as int64_t,
        0x6ed8i32 as int64_t,
        0x6edai32 as int64_t,
        0x7108i32 as int64_t,
        0x717ai32 as int64_t,
        0x751ai32 as int64_t,
        0x7618i32 as int64_t,
        0x791ci32 as int64_t,
        0x8068i32 as int64_t,
        0x8186i32 as int64_t,
        0x8248i32 as int64_t,
        0x824ai32 as int64_t,
        0x82c8i32 as int64_t,
        0x82eai32 as int64_t,
        0x8730i32 as int64_t,
        0x8739i32 as int64_t,
        0x8748i32 as int64_t,
        0x8768i32 as int64_t,
        0x87b9i32 as int64_t,
        0x87c9i32 as int64_t,
        0x87cei32 as int64_t,
        0x87d9i32 as int64_t,
        0x898di32 as int64_t,
        0x8c3ai32 as int64_t,
        0x8cdai32 as int64_t,
        0x8e38i32 as int64_t,
        0x8eb8i32 as int64_t,
        0x951ei32 as int64_t,
        0x9718i32 as int64_t,
        0x9a0ai32 as int64_t,
        0xa04ai32 as int64_t,
        0xa068i32 as int64_t,
        0xa0cai32 as int64_t,
        0xa0e8i32 as int64_t,
        0xa18ai32 as int64_t,
        0xa26ai32 as int64_t,
        0xa2e8i32 as int64_t,
        0xa2eai32 as int64_t,
        0xa43di32 as int64_t,
        0xa4e1i32 as int64_t,
        0xa589i32 as int64_t,
        0xa76di32 as int64_t,
        0xa7aci32 as int64_t,
        0xa7b1i32 as int64_t,
        0xa7edi32 as int64_t,
        0xa85di32 as int64_t,
        0xa86di32 as int64_t,
        0xaa2di32 as int64_t,
        0xb1f8i32 as int64_t,
        0xb217i32 as int64_t,
        0xb9f8i32 as int64_t,
        0xba09i32 as int64_t,
        0xba17i32 as int64_t,
        0xbb0fi32 as int64_t,
        0xc54ci32 as int64_t,
        0xc6f9i32 as int64_t,
        0xc954i32 as int64_t,
        0xc9cei32 as int64_t,
        0xd70bi32 as int64_t,
        0xd719i32 as int64_t,
        0xdc55i32 as int64_t,
        0xdf0bi32 as int64_t,
        0xe1c4i32 as int64_t,
        0xe556i32 as int64_t,
        0xe589i32 as int64_t,
        0xea5di32 as int64_t,
    ]
};
//==============================================================================
// Globals
//==============================================================================
// for quad-structure with quality 1
#[no_mangle]
pub static mut lowerBaseBitsQ1: [int64_t; 4] = unsafe {
    [
        0x3f18i32 as int64_t,
        0x520ai32 as int64_t,
        0x751ai32 as int64_t,
        0x9a0ai32 as int64_t,
    ]
};
//==============================================================================
// Finding Structure Positions
//==============================================================================
/* Fast implementation for finding the block position at which the structure
 * generation attempt will occur within the specified region.
 * This function applies for scattered-feature structureSeeds and villages.
 */
#[no_mangle]
pub unsafe extern "C" fn getStructurePos(
    config: StructureConfig,
    mut seed: int64_t,
    regionX: libc::c_int,
    regionZ: libc::c_int,
) -> Pos_0 {
    let mut pos: Pos_0 = Pos { x: 0, z: 0 };
    // set seed
    seed = regionX as libc::c_long * 341873128712i64
        + regionZ as libc::c_long * 132897987541i64
        + seed
        + config.seed;
    // & ((1LL << 48) - 1);
    seed = (seed as libc::c_longlong ^ 0x5deece66di64) as int64_t;
    seed = (seed as libc::c_longlong * 0x5deece66di64 + 0xbi64
        & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
    if 0 != config.properties & 2i32 {
        // Java RNG treats powers of 2 as a special case.
        pos.x = (config.chunkRange as libc::c_long * (seed >> 17i32) >> 31i32) as libc::c_int;
        seed = (seed as libc::c_longlong * 0x5deece66di64 + 0xbi64
            & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
        pos.z = (config.chunkRange as libc::c_long * (seed >> 17i32) >> 31i32) as libc::c_int
    } else {
        pos.x = (seed >> 17i32) as libc::c_int % config.chunkRange;
        seed = (seed as libc::c_longlong * 0x5deece66di64 + 0xbi64
            & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
        pos.z = (seed >> 17i32) as libc::c_int % config.chunkRange
    }
    pos.x = (regionX * config.regionSize + pos.x << 4i32) + 8i32;
    pos.z = (regionZ * config.regionSize + pos.z << 4i32) + 8i32;
    return pos;
}
/* Finds the chunk position within the specified region (a square region of
 * chunks depending on structure type) where the structure generation attempt
 * will occur.
 * This function applies for scattered-feature structureSeeds and villages.
 */
#[no_mangle]
pub unsafe extern "C" fn getStructureChunkInRegion(
    config: StructureConfig,
    mut seed: int64_t,
    regionX: libc::c_int,
    regionZ: libc::c_int,
) -> Pos_0 {
    /*
    // Vanilla like implementation.
    seed = regionX*341873128712 + regionZ*132897987541 + seed + structureSeed;
    setSeed(&(seed));

    Pos pos;
    pos.x = nextInt(&seed, 24);
    pos.z = nextInt(&seed, 24);
    */
    let mut pos: Pos_0 = Pos { x: 0, z: 0 };
    seed = regionX as libc::c_long * 341873128712i64
        + regionZ as libc::c_long * 132897987541i64
        + seed
        + config.seed;
    // & ((1LL << 48) - 1);
    seed = (seed as libc::c_longlong ^ 0x5deece66di64) as int64_t;
    seed = (seed as libc::c_longlong * 0x5deece66di64 + 0xbi64
        & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
    if 0 != config.properties & 2i32 {
        // Java RNG treats powers of 2 as a special case.
        pos.x = (config.chunkRange as libc::c_long * (seed >> 17i32) >> 31i32) as libc::c_int;
        seed = (seed as libc::c_longlong * 0x5deece66di64 + 0xbi64
            & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
        pos.z = (config.chunkRange as libc::c_long * (seed >> 17i32) >> 31i32) as libc::c_int
    } else {
        pos.x = (seed >> 17i32) as libc::c_int % config.chunkRange;
        seed = (seed as libc::c_longlong * 0x5deece66di64 + 0xbi64
            & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
        pos.z = (seed >> 17i32) as libc::c_int % config.chunkRange
    }
    return pos;
}
/* Fast implementation for finding the block position at which the ocean
 * monument or woodland mansion generation attempt will occur within the
 * specified region.
 */
#[no_mangle]
pub unsafe extern "C" fn getLargeStructurePos(
    mut config: StructureConfig,
    mut seed: int64_t,
    regionX: libc::c_int,
    regionZ: libc::c_int,
) -> Pos_0 {
    let mut pos: Pos_0 = Pos { x: 0, z: 0 };
    //TODO: if (config.properties & USE_POW2_RNG)...
    // set seed
    seed = regionX as libc::c_long * 341873128712i64
        + regionZ as libc::c_long * 132897987541i64
        + seed
        + config.seed;
    seed = ((seed as libc::c_longlong ^ 0x5deece66di64)
        & (1i64 << 48i32) - 1i32 as libc::c_longlong) as int64_t;
    seed = (seed as libc::c_longlong * 0x5deece66di64 + 0xbi64
        & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
    pos.x = ((seed >> 17i32) % config.chunkRange as libc::c_long) as libc::c_int;
    seed = (seed as libc::c_longlong * 0x5deece66di64 + 0xbi64
        & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
    pos.x = (pos.x as libc::c_long + (seed >> 17i32) % config.chunkRange as libc::c_long)
        as libc::c_int;
    seed = (seed as libc::c_longlong * 0x5deece66di64 + 0xbi64
        & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
    pos.z = ((seed >> 17i32) % config.chunkRange as libc::c_long) as libc::c_int;
    seed = (seed as libc::c_longlong * 0x5deece66di64 + 0xbi64
        & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
    pos.z = (pos.z as libc::c_long + (seed >> 17i32) % config.chunkRange as libc::c_long)
        as libc::c_int;
    pos.x = regionX * config.regionSize + (pos.x >> 1i32);
    pos.z = regionZ * config.regionSize + (pos.z >> 1i32);
    pos.x = pos.x * 16i32 + 8i32;
    pos.z = pos.z * 16i32 + 8i32;
    return pos;
}
/* Fast implementation for finding the chunk position at which the ocean
 * monument or woodland mansion generation attempt will occur within the
 * specified region.
 */
#[no_mangle]
pub unsafe extern "C" fn getLargeStructureChunkInRegion(
    mut config: StructureConfig,
    mut seed: int64_t,
    regionX: libc::c_int,
    regionZ: libc::c_int,
) -> Pos_0 {
    let mut pos: Pos_0 = Pos { x: 0, z: 0 };
    //TODO: if (config.properties & USE_POW2_RNG)...
    // set seed
    seed = regionX as libc::c_long * 341873128712i64
        + regionZ as libc::c_long * 132897987541i64
        + seed
        + config.seed;
    seed = ((seed as libc::c_longlong ^ 0x5deece66di64)
        & (1i64 << 48i32) - 1i32 as libc::c_longlong) as int64_t;
    seed = (seed as libc::c_longlong * 0x5deece66di64 + 0xbi64
        & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
    pos.x = ((seed >> 17i32) % config.chunkRange as libc::c_long) as libc::c_int;
    seed = (seed as libc::c_longlong * 0x5deece66di64 + 0xbi64
        & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
    pos.x = (pos.x as libc::c_long + (seed >> 17i32) % config.chunkRange as libc::c_long)
        as libc::c_int;
    seed = (seed as libc::c_longlong * 0x5deece66di64 + 0xbi64
        & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
    pos.z = ((seed >> 17i32) % config.chunkRange as libc::c_long) as libc::c_int;
    seed = (seed as libc::c_longlong * 0x5deece66di64 + 0xbi64
        & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
    pos.z = (pos.z as libc::c_long + (seed >> 17i32) % config.chunkRange as libc::c_long)
        as libc::c_int;
    pos.x >>= 1i32;
    pos.z >>= 1i32;
    return pos;
}
//==============================================================================
// Checking Biomes & Biome Helper Functions
//==============================================================================
/* Returns the biome for the specified block position.
 * (Alternatives should be considered first in performance critical code.)
 */
#[no_mangle]
pub unsafe extern "C" fn getBiomeAtPos(g: LayerStack_0, pos: Pos_0) -> libc::c_int {
    let mut map: *mut libc::c_int = allocCache(
        &mut *g.layers.offset((g.layerNum - 1i32) as isize),
        1i32,
        1i32,
    );
    genArea(
        &mut *g.layers.offset((g.layerNum - 1i32) as isize),
        map,
        pos.x,
        pos.z,
        1i32,
        1i32,
    );
    let mut biomeID: libc::c_int = *map.offset(0isize);
    free(map as *mut libc::c_void);
    return biomeID;
}
/* Finds a suitable pseudo-random location in the specified area.
 * This function is used to determine the positions of spawn and strongholds.
 * Warning: accurate, but slow!
 *
 * @mcversion        : Minecraft version (changed in: 1.7, 1.13)
 * @g                : generator layer stack
 * @cache            : biome buffer, set to NULL for temporary allocation
 * @centreX, centreZ : origin for the search
 * @range            : square 'radius' of the search
 * @isValid          : boolean array of valid biome ids (size = 256)
 * @seed             : seed used for the RNG
 *                     (initialise RNG using setSeed(&seed))
 * @passes           : number of valid biomes passed, set to NULL to ignore this
 */
#[no_mangle]
pub unsafe extern "C" fn findBiomePosition(
    mcversion: libc::c_int,
    g: LayerStack_0,
    mut cache: *mut libc::c_int,
    centerX: libc::c_int,
    centerZ: libc::c_int,
    range: libc::c_int,
    mut isValid: *const libc::c_int,
    mut seed: *mut int64_t,
    mut passes: *mut libc::c_int,
) -> Pos_0 {
    let mut x1: libc::c_int = centerX - range >> 2i32;
    let mut z1: libc::c_int = centerZ - range >> 2i32;
    let mut x2: libc::c_int = centerX + range >> 2i32;
    let mut z2: libc::c_int = centerZ + range >> 2i32;
    let mut width: libc::c_int = x2 - x1 + 1i32;
    let mut height: libc::c_int = z2 - z1 + 1i32;
    let mut map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut found: libc::c_int = 0;
    let mut layer: *mut Layer_0 =
        &mut *g.layers.offset(L_RIVER_MIX_4 as libc::c_int as isize) as *mut Layer_0;
    let mut out: Pos_0 = Pos { x: 0, z: 0 };
    if (*layer).scale != 4i32 {
        printf(
            b"WARN findBiomePosition: The generator has unexpected scale %d at layer %d.\n\x00"
                as *const u8 as *const libc::c_char,
            (*layer).scale,
            L_RIVER_MIX_4 as libc::c_int,
        );
    }
    map = if !cache.is_null() {
        cache
    } else {
        allocCache(layer, width, height)
    };
    genArea(layer, map, x1, z1, width, height);
    out.x = centerX;
    out.z = centerZ;
    found = 0i32;
    if mcversion >= MC_1_13 as libc::c_int {
        i = 0i32;
        j = 2i32;
        while i < width * height {
            if !(0 == *isValid.offset((*map.offset(i as isize) & 0xffi32) as isize)) {
                if found == 0i32 || {
                    let fresh1 = j;
                    j = j + 1;
                    nextInt(seed, fresh1) == 0i32
                } {
                    out.x = x1 + i % width << 2i32;
                    out.z = z1 + i / width << 2i32;
                    found = 1i32
                }
            }
            i += 1
        }
        found = j - 2i32
    } else {
        i = 0i32;
        while i < width * height {
            if 0 != *isValid.offset((*map.offset(i as isize) & 0xffi32) as isize)
                && (found == 0i32 || nextInt(seed, found + 1i32) == 0i32)
            {
                out.x = x1 + i % width << 2i32;
                out.z = z1 + i / width << 2i32;
                found += 1
            }
            i += 1
        }
    }
    if cache.is_null() {
        free(map as *mut libc::c_void);
    }
    if !passes.is_null() {
        *passes = found
    }
    return out;
}
/* Determines if the given area contains only biomes specified by 'biomeList'.
 * This function is used to determine the positions of villages, ocean monuments
 * and mansions.
 * Warning: accurate, but slow!
 *
 * @g          : generator layer stack
 * @cache      : biome buffer, set to NULL for temporary allocation
 * @posX, posZ : centre for the check
 * @radius     : 'radius' of the check area
 * @isValid    : boolean array of valid biome ids (size = 256)
 */
#[no_mangle]
pub unsafe extern "C" fn areBiomesViable(
    g: LayerStack_0,
    mut cache: *mut libc::c_int,
    posX: libc::c_int,
    posZ: libc::c_int,
    radius: libc::c_int,
    mut isValid: *const libc::c_int,
) -> libc::c_int {
    let mut x1: libc::c_int = posX - radius >> 2i32;
    let mut z1: libc::c_int = posZ - radius >> 2i32;
    let mut x2: libc::c_int = posX + radius >> 2i32;
    let mut z2: libc::c_int = posZ + radius >> 2i32;
    let mut width: libc::c_int = x2 - x1 + 1i32;
    let mut height: libc::c_int = z2 - z1 + 1i32;
    let mut i: libc::c_int = 0;
    let mut map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut layer: *mut Layer_0 =
        &mut *g.layers.offset(L_RIVER_MIX_4 as libc::c_int as isize) as *mut Layer_0;
    if (*layer).scale != 4i32 {
        printf(
            b"WARN areBiomesViable: The generator has unexpected scale %d at layer %d.\n\x00"
                as *const u8 as *const libc::c_char,
            (*layer).scale,
            L_RIVER_MIX_4 as libc::c_int,
        );
    }
    map = if !cache.is_null() {
        cache
    } else {
        allocCache(layer, width, height)
    };
    genArea(layer, map, x1, z1, width, height);
    i = 0i32;
    while i < width * height {
        if 0 == *isValid.offset((*map.offset(i as isize) & 0xffi32) as isize) {
            if cache.is_null() {
                free(map as *mut libc::c_void);
            }
            return 0i32;
        } else {
            i += 1
        }
    }
    if cache.is_null() {
        free(map as *mut libc::c_void);
    }
    return 1i32;
}
/* Finds the smallest radius (by square around the origin) at which all the
 * specified biomes are present. The input map is assumed to be a square of
 * side length 'sideLen'.
 *
 * @map             : square biome map to be tested
 * @sideLen         : side length of the square map (should be 2*radius+1)
 * @biomes          : list of biomes to check for
 * @bnum            : length of 'biomes'
 * @ignoreMutations : flag to count mutated biomes as their original form
 *
 * Return the radius on the square map that covers all biomes in the list.
 * If the map does not contain all the specified biomes, -1 is returned.
 */
#[no_mangle]
pub unsafe extern "C" fn getBiomeRadius(
    mut map: *const libc::c_int,
    mapSide: libc::c_int,
    mut biomes_0: *const libc::c_int,
    bnum: libc::c_int,
    ignoreMutations: libc::c_int,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut blist: [libc::c_int; 256] = [0; 256];
    let mut mask: libc::c_int = if 0 != ignoreMutations {
        0x7fi32
    } else {
        0xffi32
    };
    let mut radiusMax: libc::c_int = mapSide / 2i32;
    if mapSide & 1i32 == 0i32 {
        printf(
            b"WARN getBiomeRadius: Side length of the square map should be an odd integer.\n\x00"
                as *const u8 as *const libc::c_char,
        );
    }
    memset(
        blist.as_mut_ptr() as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<[libc::c_int; 256]>() as libc::c_ulong,
    );
    r = 1i32;
    while r < radiusMax {
        i = radiusMax - r;
        while i <= radiusMax + r {
            blist[(*map.offset(((radiusMax - r) * mapSide + i) as isize) & mask) as usize] = 1i32;
            blist[(*map.offset(((radiusMax + r - 1i32) * mapSide + i) as isize) & mask) as usize] =
                1i32;
            blist[(*map.offset((mapSide * i + (radiusMax - r)) as isize) & mask) as usize] = 1i32;
            blist[(*map.offset((mapSide * i + (radiusMax + r - 1i32)) as isize) & mask) as usize] =
                1i32;
            i += 1
        }
        b = 0i32;
        while b < bnum && 0 != blist[(*biomes_0.offset(b as isize) & mask) as usize] {
            b += 1
        }
        if b >= bnum {
            break;
        }
        r += 1
    }
    return if r != radiusMax { r } else { -1i32 };
}
//==============================================================================
// Finding Strongholds and Spawn
//==============================================================================
/* Finds the block positions of the strongholds in the world. Note that the
 * number of strongholds was increased from 3 to 128 in MC 1.9.
 * Warning: Slow!
 *
 * @mcversion : Minecraft version (changed in 1.7, 1.9, 1.13)
 * @g         : generator layer stack [worldSeed should be applied before call!]
 * @cache     : biome buffer, set to NULL for temporary allocation
 * @locations : output block positions
 * @worldSeed : world seed of the generator
 * @maxSH     : Stop when this many strongholds have been found. A value of 0
 *              defaults to 3 for mcversion <= MC_1_8, and to 128 for >= MC_1_9.
 * @maxRadius : Stop searching if the radius exceeds this value in meters.
 *              Set this to 0 to ignore this condition.
 *
 * Returned is the number of strongholds found.
 */
#[no_mangle]
pub unsafe extern "C" fn findStrongholds(
    mcversion: libc::c_int,
    mut g: *mut LayerStack_0,
    mut cache: *mut libc::c_int,
    mut locations: *mut Pos_0,
    mut worldSeed: int64_t,
    mut maxSH: libc::c_int,
    maxRadius: libc::c_int,
) -> libc::c_int {
    let mut validStrongholdBiomes: *const libc::c_int = getValidStrongholdBiomes();
    let mut i: libc::c_int = 0;
    let mut x: libc::c_int = 0;
    let mut z: libc::c_int = 0;
    let mut distance: libc::c_double = 0.;
    let mut currentRing: libc::c_int = 0i32;
    let mut currentCount: libc::c_int = 0i32;
    let mut perRing: libc::c_int = 3i32;
    setSeed(&mut worldSeed);
    let mut angle: libc::c_double = nextDouble(&mut worldSeed) * 3.141592653589793f64 * 2.0f64;
    if mcversion >= MC_1_9 as libc::c_int {
        if maxSH <= 0i32 {
            maxSH = 128i32
        }
        i = 0i32;
        while i < maxSH {
            distance = 4.0f64 * 32.0f64
                + 6.0f64 * currentRing as libc::c_double * 32.0f64
                + (nextDouble(&mut worldSeed) - 0.5f64) * 32i32 as libc::c_double * 2.5f64;
            if 0 != maxRadius && distance * 16i32 as libc::c_double > maxRadius as libc::c_double {
                return i;
            } else {
                x = round(cos(angle) * distance) as libc::c_int;
                z = round(sin(angle) * distance) as libc::c_int;
                *locations.offset(i as isize) = findBiomePosition(
                    mcversion,
                    *g,
                    cache,
                    (x << 4i32) + 8i32,
                    (z << 4i32) + 8i32,
                    112i32,
                    validStrongholdBiomes,
                    &mut worldSeed,
                    0 as *mut libc::c_int,
                );
                angle += 2i32 as libc::c_double * 3.141592653589793f64 / perRing as libc::c_double;
                currentCount += 1;
                if currentCount == perRing {
                    // Current ring is complete, move to next ring.
                    currentRing += 1;
                    currentCount = 0i32;
                    perRing = perRing + 2i32 * perRing / (currentRing + 1i32);
                    if perRing > 128i32 - i {
                        perRing = 128i32 - i
                    }
                    angle = angle + nextDouble(&mut worldSeed) * 3.141592653589793f64 * 2.0f64
                }
                i += 1
            }
        }
    } else {
        if maxSH <= 0i32 {
            maxSH = 3i32
        }
        i = 0i32;
        while i < maxSH {
            distance = (1.25f64 + nextDouble(&mut worldSeed)) * 32.0f64;
            if 0 != maxRadius && distance * 16i32 as libc::c_double > maxRadius as libc::c_double {
                return i;
            } else {
                x = round(cos(angle) * distance) as libc::c_int;
                z = round(sin(angle) * distance) as libc::c_int;
                *locations.offset(i as isize) = findBiomePosition(
                    mcversion,
                    *g,
                    cache,
                    (x << 4i32) + 8i32,
                    (z << 4i32) + 8i32,
                    112i32,
                    validStrongholdBiomes,
                    &mut worldSeed,
                    0 as *mut libc::c_int,
                );
                angle += 2i32 as libc::c_double * 3.141592653589793f64 / 3.0f64;
                i += 1
            }
        }
    }
    return maxSH;
}
//==============================================================================
// Finding Strongholds and Spawn
//==============================================================================
#[no_mangle]
pub unsafe extern "C" fn getValidStrongholdBiomes() -> *mut libc::c_int {
    static mut validStrongholdBiomes: [libc::c_int; 256] = unsafe { [0; 256] };
    if 0 == validStrongholdBiomes[plains as libc::c_int as usize] {
        let mut id: libc::c_int = 0;
        id = 0i32;
        while id < 256i32 {
            if 0 != biomeExists(id) && biomes[id as usize].height > 0.0f64 {
                validStrongholdBiomes[id as usize] = 1i32
            }
            id += 1
        }
    }
    return validStrongholdBiomes.as_mut_ptr();
}
/* Finds the spawn point in the world.
 * Warning: Slow, and may be inaccurate because the world spawn depends on
 * grass blocks!
 *
 * @mcversion : Minecraft version (changed in 1.7, 1.13)
 * @g         : generator layer stack [worldSeed should be applied before call!]
 * @cache     : biome buffer, set to NULL for temporary allocation
 * @worldSeed : world seed used for the generator
 */
#[no_mangle]
pub unsafe extern "C" fn getSpawn(
    mcversion: libc::c_int,
    mut g: *mut LayerStack_0,
    mut cache: *mut libc::c_int,
    mut worldSeed: int64_t,
) -> Pos_0 {
    let mut n7: libc::c_int = 0;
    let mut isSpawnBiome: *const libc::c_int = getValidSpawnBiomes();
    let mut spawn: Pos_0 = Pos { x: 0, z: 0 };
    let mut found: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    setSeed(&mut worldSeed);
    spawn = findBiomePosition(
        mcversion,
        *g,
        cache,
        0i32,
        0i32,
        256i32,
        isSpawnBiome,
        &mut worldSeed,
        &mut found,
    );
    if 0 == found {
        //printf("Unable to find spawn biome.\n");
        spawn.z = 8i32;
        spawn.x = spawn.z
    }
    if mcversion >= MC_1_13 as libc::c_int {
        // TODO: The 1.13 section may need further checking!
        let mut n2: libc::c_int = 0i32;
        let mut n3: libc::c_int = 0i32;
        let mut n4: libc::c_int = 0i32;
        let mut n5: libc::c_int = -1i32;
        i = 0i32;
        while i < 1024i32 {
            if n2 > -16i32 && n2 <= 16i32 && n3 > -16i32 && n3 <= 16i32 {
                let mut cx: libc::c_int = (spawn.x >> 4i32) + n2 << 4i32;
                let mut cz: libc::c_int = (spawn.z >> 4i32) + n3 << 4i32;
                let mut i2: libc::c_int = cx;
                while i2 <= cx + 15i32 {
                    let mut i3: libc::c_int = cz;
                    while i3 <= cz + 15i32 {
                        let mut pos: Pos_0 = Pos { x: i2, z: i3 };
                        if 0 != canCoordinateBeSpawn(g, cache, pos) {
                            return pos;
                        } else {
                            i3 += 1
                        }
                    }
                    i2 += 1
                }
            }
            if n2 == n3 || n2 < 0i32 && n2 == -n3 || n2 > 0i32 && n2 == 1i32 - n3 {
                n7 = n4;
                n4 = -n5;
                n5 = n7
            }
            n2 += n4;
            n3 += n5;
            i += 1
        }
    } else {
        i = 0i32;
        while i < 1000i32 && 0 == canCoordinateBeSpawn(g, cache, spawn) {
            spawn.x += nextInt(&mut worldSeed, 64i32) - nextInt(&mut worldSeed, 64i32);
            spawn.z += nextInt(&mut worldSeed, 64i32) - nextInt(&mut worldSeed, 64i32);
            i += 1
        }
    }
    return spawn;
}
/* TODO: Estimate whether the given positions could be spawn based on biomes. */
unsafe extern "C" fn canCoordinateBeSpawn(
    mut g: *mut LayerStack_0,
    mut cache: *mut libc::c_int,
    mut pos: Pos_0,
) -> libc::c_int {
    return 1i32;
}
unsafe extern "C" fn getValidSpawnBiomes() -> *mut libc::c_int {
    static mut isSpawnBiome: [libc::c_int; 256] = unsafe { [0; 256] };
    let mut i: libc::c_uint = 0;
    if 0 == isSpawnBiome[biomesToSpawnIn[0usize] as usize] {
        i = 0i32 as libc::c_uint;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_int; 7]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        {
            isSpawnBiome[biomesToSpawnIn[i as usize] as usize] = 1i32;
            i = i.wrapping_add(1)
        }
    }
    return isSpawnBiome.as_mut_ptr();
}
//==============================================================================
// Validating Structure Positions
//==============================================================================
/* *********************** Biome Checks for Structures **************************
 *
 * Scattered features only do a simple check of the biome at the block position
 * of the structure origin (i.e. the north-west corner). Before 1.13 the type of
 * structure was determined by the biome, while in 1.13 the scattered feature
 * positions are calculated separately for each type. However, the biome
 * requirements remain the same:
 *
 *  Desert Pyramid: desert or desertHills
 *  Igloo         : icePlains or coldTaiga
 *  Jungle Pyramid: jungle or jungleHills
 *  Swamp Hut     : swampland
 *
 * Similarly, Ocean Ruins and Shipwrecks require any oceanic biome at their
 * block position.
 *
 * Villages, Monuments and Mansions on the other hand require a certain area to
 * be of a valid biome and the check is performed at a 1:4 scale instead of 1:1.
 * (Actually the area for villages has a radius zero, which means it is a simple
 * biome check at a 1:4 scale.)
 */
/* These functions perform a biome check at the specified block coordinates to
 * determine whether the corresponding structure would spawn there. You can get
 * the block positions using the appropriate getXXXPos() function.
 *
 * @g              : generator layer stack [set seed using applySeed()]
 * @cache          : biome buffer, set to NULL for temporary allocation
 * @blockX, blockZ : block coordinates
 *
 * In the case of isViableFeaturePos() the 'type' argument specifies the type of
 * scattered feature (as an enum) for which the check is performed.
 *
 * The return value is non-zero if the position is valid.
 */
#[no_mangle]
pub unsafe extern "C" fn isViableFeaturePos(
    structureType: libc::c_int,
    g: LayerStack_0,
    mut cache: *mut libc::c_int,
    blockX: libc::c_int,
    blockZ: libc::c_int,
) -> libc::c_int {
    let mut map: *mut libc::c_int = if !cache.is_null() {
        cache
    } else {
        allocCache(
            &mut *g.layers.offset((g.layerNum - 1i32) as isize),
            1i32,
            1i32,
        )
    };
    genArea(
        &mut *g.layers.offset((g.layerNum - 1i32) as isize),
        map,
        blockX,
        blockZ,
        1i32,
        1i32,
    );
    let mut biomeID: libc::c_int = *map.offset(0isize);
    if cache.is_null() {
        free(map as *mut libc::c_void);
    }
    match structureType {
        0 => {
            return (biomeID == desert as libc::c_int || biomeID == desertHills as libc::c_int)
                as libc::c_int
        }
        1 => {
            return (biomeID == icePlains as libc::c_int || biomeID == coldTaiga as libc::c_int)
                as libc::c_int
        }
        2 => {
            return (biomeID == jungle as libc::c_int || biomeID == jungleHills as libc::c_int)
                as libc::c_int
        }
        3 => return (biomeID == swampland as libc::c_int) as libc::c_int,
        5 | 6 => return isOceanic(biomeID),
        _ => {
            fprintf(
                stderr,
                b"Structure type is not valid for the scattered feature biome check.\n\x00"
                    as *const u8 as *const libc::c_char,
            );
            exit(1i32);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn isViableVillagePos(
    g: LayerStack_0,
    mut cache: *mut libc::c_int,
    blockX: libc::c_int,
    blockZ: libc::c_int,
) -> libc::c_int {
    static mut isVillageBiome: [libc::c_int; 256] = unsafe { [0; 256] };
    if 0 == isVillageBiome[villageBiomeList[0usize] as usize] {
        let mut i: libc::c_uint = 0;
        i = 0i32 as libc::c_uint;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_int; 4]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        {
            isVillageBiome[villageBiomeList[i as usize] as usize] = 1i32;
            i = i.wrapping_add(1)
        }
    }
    return areBiomesViable(g, cache, blockX, blockZ, 0i32, isVillageBiome.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn isViableOceanMonumentPos(
    g: LayerStack_0,
    mut cache: *mut libc::c_int,
    blockX: libc::c_int,
    blockZ: libc::c_int,
) -> libc::c_int {
    static mut isWaterBiome: [libc::c_int; 256] = unsafe { [0; 256] };
    static mut isDeepOcean: [libc::c_int; 256] = unsafe { [0; 256] };
    if 0 == isWaterBiome[oceanMonumentBiomeList1[1usize] as usize] {
        let mut i: libc::c_uint = 0;
        i = 0i32 as libc::c_uint;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_int; 12]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        {
            isWaterBiome[oceanMonumentBiomeList1[i as usize] as usize] = 1i32;
            i = i.wrapping_add(1)
        }
        i = 0i32 as libc::c_uint;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_int; 5]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        {
            isDeepOcean[oceanMonumentBiomeList2[i as usize] as usize] = 1i32;
            i = i.wrapping_add(1)
        }
    }
    return (0 != areBiomesViable(g, cache, blockX, blockZ, 16i32, isDeepOcean.as_mut_ptr())
        && 0 != areBiomesViable(g, cache, blockX, blockZ, 29i32, isWaterBiome.as_mut_ptr()))
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn isViableMansionPos(
    g: LayerStack_0,
    mut cache: *mut libc::c_int,
    blockX: libc::c_int,
    blockZ: libc::c_int,
) -> libc::c_int {
    static mut isMansionBiome: [libc::c_int; 256] = unsafe { [0; 256] };
    if 0 == isMansionBiome[mansionBiomeList[0usize] as usize] {
        let mut i: libc::c_uint = 0;
        i = 0i32 as libc::c_uint;
        while (i as libc::c_ulong)
            < (::std::mem::size_of::<[libc::c_int; 2]>() as libc::c_ulong)
                .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
        {
            isMansionBiome[mansionBiomeList[i as usize] as usize] = 1i32;
            i = i.wrapping_add(1)
        }
    }
    return areBiomesViable(g, cache, blockX, blockZ, 32i32, isMansionBiome.as_mut_ptr());
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
/* Checks if the village in the given region would be infested by zombies.
 * (Minecraft 1.10+)
 */
#[no_mangle]
pub unsafe extern "C" fn isZombieVillage(
    mcversion: libc::c_int,
    worldSeed: int64_t,
    regionX: libc::c_int,
    regionZ: libc::c_int,
) -> libc::c_int {
    let mut pos: Pos_0 = Pos { x: 0, z: 0 };
    let mut seed: int64_t = worldSeed;
    if mcversion < MC_1_10 as libc::c_int {
        printf(
            b"Warning: Zombie villages were only introduced in MC 1.10.\n\x00" as *const u8
                as *const libc::c_char,
        );
    }
    // get the chunk position of the village
    seed = regionX as libc::c_long * 341873128712i64
        + regionZ as libc::c_long * 132897987541i64
        + seed
        + VILLAGE_CONFIG.seed;
    // & ((1LL << 48) - 1);
    seed = (seed as libc::c_longlong ^ 0x5deece66di64) as int64_t;
    seed = (seed as libc::c_longlong * 0x5deece66di64 + 0xbi64
        & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
    pos.x = ((seed >> 17i32) % VILLAGE_CONFIG.chunkRange as libc::c_long) as libc::c_int;
    seed = (seed as libc::c_longlong * 0x5deece66di64 + 0xbi64
        & 0xffffffffffffi64 as libc::c_longlong) as int64_t;
    pos.z = ((seed >> 17i32) % VILLAGE_CONFIG.chunkRange as libc::c_long) as libc::c_int;
    pos.x += regionX * VILLAGE_CONFIG.regionSize;
    pos.z += regionZ * VILLAGE_CONFIG.regionSize;
    // jump to the random number check that determines whether this is village
    // is zombie infested
    let mut rnd: int64_t = chunkGenerateRnd(worldSeed, pos.x, pos.z);
    // TODO: check for versions <= 1.11
    skipNextN(
        &mut rnd,
        if mcversion >= MC_1_13 as libc::c_int {
            10i32
        } else {
            11i32
        },
    );
    return (nextInt(&mut rnd, 50i32) == 0i32) as libc::c_int;
}
/* Checks if the village in the given region would generate as a baby zombie
 * village. (The fact that these exist could be regarded as a bug.)
 * (Minecraft 1.12+)
 */
#[no_mangle]
pub unsafe extern "C" fn isBabyZombieVillage(
    mcversion: libc::c_int,
    worldSeed: int64_t,
    regionX: libc::c_int,
    regionZ: libc::c_int,
) -> libc::c_int {
    if 0 == isZombieVillage(mcversion, worldSeed, regionX, regionZ) {
        return 0i32;
    } else {
        // Whether the zombie is a child or not is dependent on the world random
        // object which is not reset for villages. The last reset is instead
        // performed during the positioning of Mansions.
        let mut rnd: int64_t = worldSeed;
        rnd = regionX as libc::c_long * 341873128712i64
            + regionZ as libc::c_long * 132897987541i64
            + rnd
            + MANSION_CONFIG.seed;
        setSeed(&mut rnd);
        skipNextN(&mut rnd, 5i32);
        let mut isChild: libc::c_int =
            ((nextFloat(&mut rnd) as libc::c_double) < 0.05f64) as libc::c_int;
        //int mountNearbyChicken = nextFloat(&rnd) < 0.05;
        //int spawnNewChicken = nextFloat(&rnd) < 0.05;
        return isChild;
    };
}
//==============================================================================
// Seed Filters
//==============================================================================
/* Looks through the seeds in 'seedsIn' and copies those for which all
 * temperature categories are present in the 3x3 area centred on the specified
 * coordinates into 'seedsOut'. The map scale at this layer is 1:1024.
 *
 * @g            : generator layer stack, (NOTE: seed will be modified)
 * @cache        : biome buffer, set to NULL for temporary allocation
 * @seedsIn      : list of seeds to check
 * @seedsOut     : output buffer for the candidate seeds
 * @seedCnt      : number of seeds in 'seedsIn'
 * qcentX, centZ : search origin centre (in 1024 block units)
 *
 * Returns the number of found candidates.
 */
#[no_mangle]
pub unsafe extern "C" fn filterAllTempCats(
    mut g: *mut LayerStack_0,
    mut cache: *mut libc::c_int,
    mut seedsIn: *const int64_t,
    mut seedsOut: *mut int64_t,
    seedCnt: int64_t,
    centX: libc::c_int,
    centZ: libc::c_int,
) -> int64_t {
    let pX: libc::c_int = centX - 1i32;
    /* We require all temperature categories, including the special variations
     * in order to get all main biomes. This gives 8 required values:
     * Oceanic, Warm, Lush, Cold, Freezing,
     * Special Warm, Special Lush, Special Cold
     * These categories generate at Layer 13: Edge, Special.
     *
     * Note: The scale at this layer is 1:1024 and each element can "leak" its
     * biome values up to 1024 blocks outwards into the negative coordinates
     * (due to the Zoom layers).
     *
     * The plan is to check if the 3x3 area contains all 8 temperature types.
     * For this, we can check even earlier at Layer 10: Add Island, that each of
     * the Warm, Cold and Freezing categories are present.
     */
    /* Edit:
     * All the biomes that are generated by a simple Cold climate can actually
     * be generated later on. So I have commented out the Cold requirements.
     */
    let pZ: libc::c_int = centZ - 1i32;
    let sX: libc::c_int = 3i32;
    let sZ: libc::c_int = 3i32;
    let mut map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut lFilterSnow: *mut Layer_0 =
        &mut *(*g).layers.offset(L_ADD_SNOW_1024 as libc::c_int as isize) as *mut Layer_0;
    let mut lFilterSpecial: *mut Layer_0 =
        &mut *(*g).layers.offset(L_SPECIAL_1024 as libc::c_int as isize) as *mut Layer_0;
    map = if !cache.is_null() {
        cache
    } else {
        allocCache(lFilterSpecial, sX, sZ)
    };
    // Construct a dummy Edge,Special layer.
    let mut layerSpecial: Layer_0 = Layer {
        baseSeed: 0,
        worldSeed: 0,
        chunkSeed: 0,
        oceanRnd: 0 as *mut OceanRnd_0,
        scale: 0,
        getMap: None,
        p: 0 as *mut Layer_0,
        p2: 0 as *mut Layer_0,
    };
    setupLayer(1024i32, &mut layerSpecial, 0 as *mut Layer_0, 3i32, None);
    let mut sidx: int64_t = 0;
    let mut hits: int64_t = 0;
    let mut seed: int64_t = 0;
    let mut types: [libc::c_int; 9] = [0; 9];
    let mut specialCnt: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    hits = 0i32 as int64_t;
    sidx = 0i32 as int64_t;
    while sidx < seedCnt {
        seed = *seedsIn.offset(sidx as isize);
        /* **  Pre-Generation Checks  ***/
        // We require at least 3 special temperature categories which can be
        // tested for without going through the previous layers. (We'll get
        // false positives due to Oceans, but this works fine to rule out some
        // seeds early on.)
        setWorldSeed(&mut layerSpecial, seed);
        specialCnt = 0i32;
        i = 0i32;
        while i < sX {
            j = 0i32;
            while j < sZ {
                setChunkSeed(&mut layerSpecial, (i + pX) as int64_t, (j + pZ) as int64_t);
                if mcNextInt(&mut layerSpecial, 13i32) == 0i32 {
                    specialCnt += 1
                }
                j += 1
            }
            i += 1
        }
        if !(specialCnt < 3i32) {
            /* **  Cold/Warm Check  ***/
            // Continue by checking if enough cold and warm categories are present.
            setWorldSeed(lFilterSnow, seed);
            genArea(lFilterSnow, map, pX, pZ, sX, sZ);
            memset(
                types.as_mut_ptr() as *mut libc::c_void,
                0i32,
                ::std::mem::size_of::<[libc::c_int; 9]>() as libc::c_ulong,
            );
            i = 0i32;
            while i < sX * sZ {
                types[*map.offset(i as isize) as usize] += 1;
                i += 1
            }
            // 1xOcean needs to be present
            // 4xWarm need to turn into Warm, Lush, Special Warm and Special Lush
            // 1xFreezing that needs to stay Freezing
            // 3x(Cold + Freezing) for Cold, Special Cold and Freezing
            if !(types[Ocean as libc::c_int as usize] < 1i32
                || types[Warm as libc::c_int as usize] < 4i32
                || types[Freezing as libc::c_int as usize] < 1i32
                || types[Cold as libc::c_int as usize] + types[Freezing as libc::c_int as usize]
                    < 2i32)
            {
                /* **  Complete Temperature Category Check  ***/
                // Check that all temperature variants are present.
                setWorldSeed(lFilterSpecial, seed);
                genArea(lFilterSpecial, map, pX, pZ, sX, sZ);
                memset(
                    types.as_mut_ptr() as *mut libc::c_void,
                    0i32,
                    ::std::mem::size_of::<[libc::c_int; 9]>() as libc::c_ulong,
                );
                i = 0i32;
                while i < sX * sZ {
                    types[(if *map.offset(i as isize) > 4i32 {
                              (*map.offset(i as isize) & 0xfi32) + 4i32
                          } else {
                              *map.offset(i as isize)
                          }) as usize] += 1;
                    i += 1
                }
                if !(types[Ocean as libc::c_int as usize] < 1i32
                    || types[Warm as libc::c_int as usize] < 1i32
                    || types[Lush as libc::c_int as usize] < 1i32
                    || types[Freezing as libc::c_int as usize] < 1i32
                    || types[(Warm as libc::c_int + 4i32) as usize] < 1i32
                    || types[(Lush as libc::c_int + 4i32) as usize] < 1i32
                    || types[(Cold as libc::c_int + 4i32) as usize] < 1i32)
                {
                    /*types[Cold] < 1   ||*/
                    /*
        for (i = 0; i < sX*sZ; i++)
        {
            printf("%c%d ", " s"[cache[i] > 4], cache[i]&0xf);
            if (i % sX == sX-1) printf("\n");
        }
        printf("\n");*/
                    // Save the candidate.
                    *seedsOut.offset(hits as isize) = seed;
                    hits += 1
                }
            }
        }
        sidx += 1
    }
    if cache.is_null() {
        free(map as *mut libc::c_void);
    }
    return hits;
}
/* Looks through the list of seeds in 'seedsIn' and copies those that have all
 * major overworld biomes in the specified area into 'seedsOut'. These checks
 * are done at a scale of 1:256.
 *
 * @g           : generator layer stack, (NOTE: seed will be modified)
 * @cache       : biome buffer, set to NULL for temporary allocation
 * @seedsIn     : list of seeds to check
 * @seedsOut    : output buffer for the candidate seeds
 * @seedCnt     : number of seeds in 'seedsIn'
 * @pX, pZ      : search starting coordinates (in 256 block units)
 * @sX, sZ      : size of the searching area (in 256 block units)
 *
 * Returns the number of seeds found.
 */
#[no_mangle]
pub unsafe extern "C" fn filterAllMajorBiomes(
    mut g: *mut LayerStack_0,
    mut cache: *mut libc::c_int,
    mut seedsIn: *const int64_t,
    mut seedsOut: *mut int64_t,
    seedCnt: int64_t,
    pX: libc::c_int,
    pZ: libc::c_int,
    sX: libc::c_uint,
    sZ: libc::c_uint,
) -> int64_t {
    let mut lFilterMushroom: *mut Layer_0 = &mut *(*g)
        .layers
        .offset(L_ADD_MUSHROOM_ISLAND_256 as libc::c_int as isize)
        as *mut Layer_0;
    let mut lFilterBiomes: *mut Layer_0 =
        &mut *(*g).layers.offset(L_BIOME_256 as libc::c_int as isize) as *mut Layer_0;
    let mut map: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut sidx: int64_t = 0;
    let mut seed: int64_t = 0;
    let mut hits: int64_t = 0;
    let mut i: libc::c_uint = 0;
    let mut id: libc::c_uint = 0;
    let mut hasAll: libc::c_uint = 0;
    let mut types: [libc::c_int; 51] = [0; 51];
    map = if !cache.is_null() {
        cache
    } else {
        allocCache(lFilterBiomes, sX as libc::c_int, sZ as libc::c_int)
    };
    hits = 0i32 as int64_t;
    sidx = 0i32 as int64_t;
    while sidx < seedCnt {
        /* We can use the Mushroom layer both to check for mushroomIsland biomes
         * and to make sure all temperature categories are present in the area.
         */
        seed = *seedsIn.offset(sidx as isize);
        setWorldSeed(lFilterMushroom, seed);
        genArea(
            lFilterMushroom,
            map,
            pX,
            pZ,
            sX as libc::c_int,
            sZ as libc::c_int,
        );
        memset(
            types.as_mut_ptr() as *mut libc::c_void,
            0i32,
            ::std::mem::size_of::<[libc::c_int; 51]>() as libc::c_ulong,
        );
        i = 0i32 as libc::c_uint;
        while i < sX.wrapping_mul(sZ) {
            id = *map.offset(i as isize) as libc::c_uint;
            if id >= BIOME_NUM as libc::c_int as libc::c_uint {
                id = (id & 0xfi32 as libc::c_uint).wrapping_add(4i32 as libc::c_uint)
            }
            types[id as usize] += 1;
            i = i.wrapping_add(1)
        }
        if !(types[Ocean as libc::c_int as usize] < 1i32
            || types[Warm as libc::c_int as usize] < 1i32
            || types[Lush as libc::c_int as usize] < 1i32
            || types[Freezing as libc::c_int as usize] < 1i32
            || types[(Warm as libc::c_int + 4i32) as usize] < 1i32
            || types[(Lush as libc::c_int + 4i32) as usize] < 1i32
            || types[(Cold as libc::c_int + 4i32) as usize] < 1i32
            || types[mushroomIsland as libc::c_int as usize] < 1i32)
        {
            /* types[Cold] < 1   || */
            /* **  Find all major biomes  ***/
            setWorldSeed(lFilterBiomes, seed);
            genArea(
                lFilterBiomes,
                map,
                pX,
                pZ,
                sX as libc::c_int,
                sZ as libc::c_int,
            );
            memset(
                types.as_mut_ptr() as *mut libc::c_void,
                0i32,
                ::std::mem::size_of::<[libc::c_int; 51]>() as libc::c_ulong,
            );
            i = 0i32 as libc::c_uint;
            while i < sX.wrapping_mul(sZ) {
                types[*map.offset(i as isize) as usize] += 1;
                i = i.wrapping_add(1)
            }
            hasAll = 1i32 as libc::c_uint;
            i = 0i32 as libc::c_uint;
            while (i as libc::c_ulong)
                < (::std::mem::size_of::<[libc::c_int; 18]>() as libc::c_ulong)
                    .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            {
                // plains, taiga and deepOcean can be generated in later layers.
                // Also small islands of Forests can be generated in deepOcean
                // biomes, but we are going to ignore those.
                if !(majorBiomes[i as usize] == plains as libc::c_int
                    || majorBiomes[i as usize] == taiga as libc::c_int
                    || majorBiomes[i as usize] == deepOcean as libc::c_int)
                {
                    if types[majorBiomes[i as usize] as usize] < 1i32 {
                        hasAll = 0i32 as libc::c_uint;
                        break;
                    }
                }
                i = i.wrapping_add(1)
            }
            if !(0 == hasAll) {
                *seedsOut.offset(hits as isize) = seed;
                hits += 1
            }
        }
        sidx += 1
    }
    if cache.is_null() {
        free(map as *mut libc::c_void);
    }
    return hits;
}
#[no_mangle]
pub static mut majorBiomes: [libc::c_int; 18] = unsafe {
    [
        ocean as libc::c_int,
        plains as libc::c_int,
        desert as libc::c_int,
        extremeHills as libc::c_int,
        forest as libc::c_int,
        taiga as libc::c_int,
        swampland as libc::c_int,
        icePlains as libc::c_int,
        mushroomIsland as libc::c_int,
        jungle as libc::c_int,
        deepOcean as libc::c_int,
        birchForest as libc::c_int,
        roofedForest as libc::c_int,
        coldTaiga as libc::c_int,
        megaTaiga as libc::c_int,
        savanna as libc::c_int,
        mesaPlateau_F as libc::c_int,
        mesaPlateau as libc::c_int,
    ]
};
/* Searches for the optimal AFK position given four structures at positions 'p',
 * each of volume (ax,ay,az).
 *
 * Returned is the number of spawning spaces within reach.
 */
#[no_mangle]
pub unsafe extern "C" fn countBlocksInSpawnRange(
    mut p: *mut Pos_0,
    ax: libc::c_int,
    ay: libc::c_int,
    az: libc::c_int,
) -> libc::c_int {
    let mut minX: libc::c_int = 30000000.0f64 as libc::c_int;
    let mut minZ: libc::c_int = 30000000.0f64 as libc::c_int;
    let mut maxX: libc::c_int = -30000000.0f64 as libc::c_int;
    let mut maxZ: libc::c_int = -30000000.0f64 as libc::c_int;
    let mut best: libc::c_int = 0;
    // Find corners
    let mut i: libc::c_int = 0i32;
    while i < 4i32 {
        if (*p.offset(i as isize)).x < minX {
            minX = (*p.offset(i as isize)).x
        }
        if (*p.offset(i as isize)).z < minZ {
            minZ = (*p.offset(i as isize)).z
        }
        if (*p.offset(i as isize)).x > maxX {
            maxX = (*p.offset(i as isize)).x
        }
        if (*p.offset(i as isize)).z > maxZ {
            maxZ = (*p.offset(i as isize)).z
        }
        i += 1
    }
    // assume that the search area is bound by the inner corners
    maxX += ax;
    maxZ += az;
    best = 0i32;
    let mut thsq: libc::c_double = 128.0f64 * 128.0f64 - (az * az) as libc::c_double / 4.0f64;
    let mut x: libc::c_int = minX;
    while x < maxX {
        let mut z: libc::c_int = minZ;
        while z < maxZ {
            let mut inrange: libc::c_int = 0i32;
            let mut i_0: libc::c_int = 0i32;
            while i_0 < 4i32 {
                let mut dx: libc::c_double =
                    (*p.offset(i_0 as isize)).x as libc::c_double - (x as libc::c_double + 0.5f64);
                let mut dz: libc::c_double =
                    (*p.offset(i_0 as isize)).z as libc::c_double - (z as libc::c_double + 0.5f64);
                let mut px: libc::c_int = 0i32;
                while px < ax {
                    let mut pz: libc::c_int = 0i32;
                    while pz < az {
                        let mut ddx: libc::c_double = px as libc::c_double + dx;
                        let mut ddz: libc::c_double = pz as libc::c_double + dz;
                        inrange += (ddx * ddx + ddz * ddz <= thsq) as libc::c_int;
                        pz += 1
                    }
                    px += 1
                }
                i_0 += 1
            }
            if inrange > best {
                best = inrange
            }
            z += 1
        }
        x += 1
    }
    return best;
}
