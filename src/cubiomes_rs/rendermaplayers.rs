#![allow(
    dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals,
    unused_mut
)]
use libc;
extern "C" {
    pub type _IO_FILE_plus;
    #[no_mangle]
    fn strtol(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
    ) -> libc::c_long;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void) -> ();
    #[no_mangle]
    fn exit(_: libc::c_int) -> !;
    #[no_mangle]
    fn system(__command: *const libc::c_char) -> libc::c_int;
    //==============================================================================
    // Essentials
    //==============================================================================
    #[no_mangle]
    static mut biomes: [Biome_0; 256];
    /* initBiomes() has to be called before any of the generators can be used */
    #[no_mangle]
    fn initBiomes() -> ();
    #[no_mangle]
    fn initBiomeColours(biomeColours: *mut [libc::c_uchar; 3]) -> ();
    #[no_mangle]
    fn setupGeneratorMC17UpTo(l: libc::c_int) -> LayerStack_0;
    #[no_mangle]
    fn setupGeneratorMC17() -> LayerStack_0;
    /* Cleans up and frees the generator layers */
    #[no_mangle]
    fn freeGenerator(g: LayerStack_0) -> ();
    /* Allocates an amount of memory required to generate an area of dimensions
     * 'sizeX' by 'sizeZ' for the magnification of the current top layer.
     */
    #[no_mangle]
    fn allocCache(layer: *mut Layer_0, sizeX: libc::c_int, sizeZ: libc::c_int) -> *mut libc::c_int;
    /* Sets the world seed for the generator */
    #[no_mangle]
    fn applySeed(g: *mut LayerStack_0, seed: int64_t) -> ();
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
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    #[no_mangle]
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    #[no_mangle]
    fn fprintf(_: *mut FILE, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn printf(_: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, ...) -> libc::c_int;
    #[no_mangle]
    fn snprintf(_: *mut libc::c_char, _: libc::c_ulong, _: *const libc::c_char, ...)
        -> libc::c_int;
    #[no_mangle]
    fn fwrite(__ptr: *const libc::c_void, __size: size_t, __n: size_t, __s: *mut FILE) -> size_t;
    #[no_mangle]
    static mut sys_nerr: libc::c_int;
    #[no_mangle]
    static sys_errlist: [*const libc::c_char; 0];
    #[no_mangle]
    fn strncpy(_: *mut libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
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
    //==============================================================================
    // Multi-Structure-Base Checks
    //==============================================================================
    /* Calls the correct quad-base finder for the structure config, if available.
     * (Exits program otherwise.)
     */
    #[no_mangle]
    fn isQuadBase(sconf: StructureConfig, seed: int64_t, qual: int64_t) -> libc::c_int;
    //==============================================================================
    // Finding Structure Positions
    //==============================================================================
    /* Fast implementation for finding the block position at which the structure
     * generation attempt will occur within the specified region.
     * This function applies for scattered-feature structureSeeds and villages.
     */
    #[no_mangle]
    fn getStructurePos(
        config: StructureConfig,
        seed: int64_t,
        regionX: libc::c_int,
        regionZ: libc::c_int,
    ) -> Pos;
    /* Fast implementation for finding the block position at which the ocean
     * monument or woodland mansion generation attempt will occur within the
     * specified region.
     */
    #[no_mangle]
    fn getLargeStructurePos(
        config: StructureConfig,
        seed: int64_t,
        regionX: libc::c_int,
        regionZ: libc::c_int,
    ) -> Pos;
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
    fn findStrongholds(
        mcversion: libc::c_int,
        g: *mut LayerStack_0,
        cache: *mut libc::c_int,
        locations: *mut Pos,
        worldSeed: int64_t,
        maxSH: libc::c_int,
        maxRadius: libc::c_int,
    ) -> libc::c_int;
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
    fn getSpawn(
        mcversion: libc::c_int,
        g: *mut LayerStack_0,
        cache: *mut libc::c_int,
        worldSeed: int64_t,
    ) -> Pos;
    #[no_mangle]
    fn isViableVillagePos(
        g: LayerStack_0,
        cache: *mut libc::c_int,
        blockX: libc::c_int,
        blockZ: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn isViableOceanMonumentPos(
        g: LayerStack_0,
        cache: *mut libc::c_int,
        blockX: libc::c_int,
        blockZ: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn isViableMansionPos(
        g: LayerStack_0,
        cache: *mut libc::c_int,
        blockX: libc::c_int,
        blockZ: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn __errno_location() -> *mut libc::c_int;
    #[no_mangle]
    static mut optarg: *mut libc::c_char;
    #[no_mangle]
    static mut optind: libc::c_int;
    #[no_mangle]
    static mut opterr: libc::c_int;
    #[no_mangle]
    static mut optopt: libc::c_int;
    #[no_mangle]
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn pow(_: libc::c_double, _: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sqrt(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn ceil(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn floor(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn round(_: libc::c_double) -> libc::c_double;
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
pub type StructureConfig = StructureConfig_0;
pub type Pos = Pos_0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Pos_0 {
    pub x: libc::c_int,
    pub z: libc::c_int,
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
pub struct StructureConfig_0 {
    pub seed: int64_t,
    pub regionSize: libc::c_int,
    pub chunkRange: libc::c_int,
    pub properties: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct MapOptions {
    pub seed: int64_t,
    pub ppmfn: [libc::c_char; 256],
    pub pngfn: [libc::c_char; 256],
    pub width: libc::c_int,
    pub height: libc::c_int,
    pub imageScale: libc::c_int,
    pub iconScale: libc::c_int,
    pub hutScale: libc::c_int,
    pub desertScale: libc::c_int,
    pub iglooScale: libc::c_int,
    pub jungleScale: libc::c_int,
    pub mansionScale: libc::c_int,
    pub monumentScale: libc::c_int,
    pub spawnScale: libc::c_int,
    pub strongholdScale: libc::c_int,
    pub villageScale: libc::c_int,
    pub oceanRuinScale: libc::c_int,
    pub shipwreckScale: libc::c_int,
    pub use_1_12: libc::c_int,
    pub oneBiome: libc::c_int,
    pub highlightSpecial: libc::c_int,
    pub highlightMutated: libc::c_int,
    pub highlightSearched: libc::c_int,
    pub highlightNewOceans: libc::c_int,
    pub highlightIcons: libc::c_int,
    pub chunkGrid: libc::c_int,
    pub centerAtHuts: libc::c_int,
}
pub type unnamed = libc::c_uint;
pub const MC_1_13: unnamed = 6;
pub const MC_1_12: unnamed = 5;
pub const MC_1_11: unnamed = 4;
pub const MC_1_10: unnamed = 3;
pub const MC_1_9: unnamed = 2;
pub const MC_1_8: unnamed = 1;
pub const MC_1_7: unnamed = 0;
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
/*
#[no_mangle]
pub static mut biomeGroupNames: [*const libc::c_char; 14] =
    unsafe { [0 as *const libc::c_char; 14] };
#[no_mangle]
pub static mut biomeNames: [*const libc::c_char; 256] = unsafe { [0 as *const libc::c_char; 256] };
*/
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
pub unsafe extern "C" fn usage() -> () {
    fprintf(
        stderr,
        b"Options:\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --help\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --seed=<integer>\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --filename=<string>\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --width=<integer>\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --height=<integer>\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --image_scale=<integer>\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --icon_scale=<integer>\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --hut_scale=<integer>\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --mansion_scale=<integer>\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --monument_scale=<integer>\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --spawn_scale=<integer>\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --stronghold_scale=<integer>\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --village_scale=<integer>\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --ocean_ruin_scale=<integer>\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --shipwreck_scale=<integer>\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --use_1_12\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --one_biome=<integer>\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --highlight_special\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --highlight_mutated\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --highlight_searched\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --highlight_new_oceans\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --highlight_icons\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --chunk_grid\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"    --center_at_huts\n\x00" as *const u8 as *const libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn int64Arg(
    mut val: *const libc::c_char,
    mut name: *const libc::c_char,
) -> int64_t {
    let mut endptr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: int64_t = strtol(val, &mut endptr, 10i32);
    if *__errno_location() != 0i32 {
        fprintf(
            stderr,
            b"%s must be an integer\n\x00" as *const u8 as *const libc::c_char,
            name,
        );
        usage();
        exit(-1i32);
    } else {
        return ret;
    };
}
#[no_mangle]
pub unsafe extern "C" fn intArg(
    mut val: *const libc::c_char,
    mut name: *const libc::c_char,
) -> libc::c_int {
    return int64Arg(val, name) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn parseOptions(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> MapOptions {
    let mut c: libc::c_int = 0;
    let mut opts: MapOptions =
        MapOptions{seed: 0i32 as int64_t,
                   ppmfn:
                       *::std::mem::transmute::<&[u8; 256],
                                                &mut [libc::c_char; 256]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                   pngfn:
                       *::std::mem::transmute::<&[u8; 256],
                                                &mut [libc::c_char; 256]>(b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00"),
                   width: 3840i32 * 3i32,
                   height: 3840i32 * 3i32 * 9i32 / 16i32,
                   imageScale: 1i32,
                   iconScale: 1i32,
                   hutScale: -1i32,
                   desertScale: -1i32,
                   iglooScale: -1i32,
                   jungleScale: -1i32,
                   mansionScale: -1i32,
                   monumentScale: -1i32,
                   spawnScale: -1i32,
                   strongholdScale: -1i32,
                   villageScale: -1i32,
                   oceanRuinScale: -1i32,
                   shipwreckScale: -1i32,
                   use_1_12: 0i32,
                   oneBiome: -1i32,
                   highlightSpecial: 0i32,
                   highlightMutated: 0i32,
                   highlightSearched: 0i32,
                   highlightNewOceans: 0i32,
                   highlightIcons: 0i32,
                   chunkGrid: 0i32,
                   centerAtHuts: 0i32,};
    loop {
        static mut longOptions: [option; 27] = unsafe {
            [
                option {
                    name: b"help\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'h' as i32,
                },
                option {
                    name: b"seed\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 's' as i32,
                },
                option {
                    name: b"filename\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'f' as i32,
                },
                option {
                    name: b"width\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'x' as i32,
                },
                option {
                    name: b"height\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'z' as i32,
                },
                option {
                    name: b"image_scale\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'X' as i32,
                },
                option {
                    name: b"icon_scale\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'i' as i32,
                },
                option {
                    name: b"desert_scale\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'D' as i32,
                },
                option {
                    name: b"igloo_scale\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'I' as i32,
                },
                option {
                    name: b"jungle_scale\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'J' as i32,
                },
                option {
                    name: b"hut_scale\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'H' as i32,
                },
                option {
                    name: b"mansion_scale\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'W' as i32,
                },
                option {
                    name: b"monument_scale\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'M' as i32,
                },
                option {
                    name: b"spawn_scale\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'S' as i32,
                },
                option {
                    name: b"stronghold_scale\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'T' as i32,
                },
                option {
                    name: b"village_scale\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'V' as i32,
                },
                option {
                    name: b"ocean_ruin_scale\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'O' as i32,
                },
                option {
                    name: b"shipwreck_scale\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'K' as i32,
                },
                option {
                    name: b"use_1_12\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '2' as i32,
                },
                option {
                    name: b"one_biome\x00" as *const u8 as *const libc::c_char,
                    has_arg: 1i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '1' as i32,
                },
                option {
                    name: b"highlight_special\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '5' as i32,
                },
                option {
                    name: b"highlight_mutated\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '6' as i32,
                },
                option {
                    name: b"highlight_searched\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '7' as i32,
                },
                option {
                    name: b"highlight_new_oceans\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '8' as i32,
                },
                option {
                    name: b"highlight_icons\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: '9' as i32,
                },
                option {
                    name: b"chunk_grid\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'g' as i32,
                },
                option {
                    name: b"center_at_huts\x00" as *const u8 as *const libc::c_char,
                    has_arg: 0i32,
                    flag: 0 as *const libc::c_int as *mut libc::c_int,
                    val: 'c' as i32,
                },
            ]
        };
        let mut index: libc::c_int = 0i32;
        c = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"hs:f:x:z:X:i:D:I:H:W:M:S:T:V:O:K:3256789gc\x00" as *const u8 as *const libc::c_char,
            longOptions.as_mut_ptr(),
            &mut index,
        );
        if c == -1i32 {
            break;
        }
        match c {
            104 => {
                usage();
                exit(0i32);
            }
            115 => opts.seed = int64Arg(optarg, longOptions[index as usize].name),
            102 => {
                if strlen(optarg) > 250i32 as libc::c_ulong {
                    fprintf(
                        stderr,
                        b"Output filename too long.\x00" as *const u8 as *const libc::c_char,
                    );
                    exit(-1i32);
                } else {
                    snprintf(
                        opts.ppmfn.as_mut_ptr(),
                        256i32 as libc::c_ulong,
                        b"%s.ppm\x00" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                    snprintf(
                        opts.pngfn.as_mut_ptr(),
                        256i32 as libc::c_ulong,
                        b"%s.png\x00" as *const u8 as *const libc::c_char,
                        optarg,
                    );
                }
            }
            120 => opts.width = intArg(optarg, longOptions[index as usize].name),
            122 => opts.height = intArg(optarg, longOptions[index as usize].name),
            88 => opts.imageScale = intArg(optarg, longOptions[index as usize].name),
            105 => opts.iconScale = intArg(optarg, longOptions[index as usize].name),
            68 => opts.desertScale = intArg(optarg, longOptions[index as usize].name),
            73 => opts.iglooScale = intArg(optarg, longOptions[index as usize].name),
            74 => opts.jungleScale = intArg(optarg, longOptions[index as usize].name),
            72 => opts.hutScale = intArg(optarg, longOptions[index as usize].name),
            87 => opts.mansionScale = intArg(optarg, longOptions[index as usize].name),
            77 => opts.monumentScale = intArg(optarg, longOptions[index as usize].name),
            83 => opts.spawnScale = intArg(optarg, longOptions[index as usize].name),
            84 => opts.strongholdScale = intArg(optarg, longOptions[index as usize].name),
            86 => opts.villageScale = intArg(optarg, longOptions[index as usize].name),
            79 => opts.oceanRuinScale = intArg(optarg, longOptions[index as usize].name),
            75 => opts.shipwreckScale = intArg(optarg, longOptions[index as usize].name),
            50 => opts.use_1_12 = 1i32,
            49 => opts.oneBiome = intArg(optarg, longOptions[index as usize].name),
            53 => opts.highlightSpecial = 1i32,
            54 => opts.highlightMutated = 1i32,
            55 => opts.highlightSearched = 1i32,
            56 => opts.highlightNewOceans = 1i32,
            57 => opts.highlightIcons = 1i32,
            103 => opts.chunkGrid = 1i32,
            99 => opts.centerAtHuts = 1i32,
            _ => {
                exit(-1i32);
            }
        }
    }
    if 0 == opts.seed {
        fprintf(
            stderr,
            b"Seed is required (0 is not a valid MC seed).\n\x00" as *const u8
                as *const libc::c_char,
        );
        usage();
        exit(-1i32);
    } else {
        if 0 == strlen(opts.ppmfn.as_mut_ptr()) {
            //fprintf(stderr, "Filename is required.\n");
            //usage();
            //exit(-1);
            sprintf(
                opts.ppmfn.as_mut_ptr(),
                b"images/%ld.ppm\x00" as *const u8 as *const libc::c_char,
                opts.seed,
            );
            sprintf(
                opts.pngfn.as_mut_ptr(),
                b"images/%ld.png\x00" as *const u8 as *const libc::c_char,
                opts.seed,
            );
        }
        if opts.desertScale == -1i32 {
            opts.desertScale = opts.iconScale * 2i32
        }
        if opts.iglooScale == -1i32 {
            opts.iglooScale = opts.iconScale * 2i32
        }
        if opts.jungleScale == -1i32 {
            opts.jungleScale = opts.iconScale * 2i32
        }
        if opts.hutScale == -1i32 {
            opts.hutScale = opts.iconScale * 2i32
        }
        if opts.mansionScale == -1i32 {
            opts.mansionScale = opts.iconScale * 4i32
        }
        if opts.monumentScale == -1i32 {
            opts.monumentScale = opts.iconScale * 2i32
        }
        if opts.spawnScale == -1i32 {
            opts.spawnScale = opts.iconScale * 3i32
        }
        if opts.strongholdScale == -1i32 {
            opts.strongholdScale = opts.iconScale * 3i32
        }
        if opts.villageScale == -1i32 {
            opts.villageScale = opts.iconScale * 2i32
        }
        if opts.oceanRuinScale == -1i32 {
            opts.oceanRuinScale = opts.iconScale * 1i32
        }
        if opts.shipwreckScale == -1i32 {
            opts.shipwreckScale = opts.iconScale * 1i32
        }
        return opts;
    };
}
// Standard values from IEC 61966-2-1
// NOTE: A gamma of 2.2 approximates sRGB, but the actual sRGB curve is a
// piecewise function of a linear and power component. The power component of
// that curve has an expontent of 2.4.
#[no_mangle]
pub unsafe extern "C" fn sRGBToLinear(mut c: libc::c_int) -> libc::c_float {
    let mut c1: libc::c_float = (c as libc::c_float as libc::c_double / 255.0f64) as libc::c_float;
    let mut ret: libc::c_float = 0.;
    if c1 as libc::c_double <= 0.04045f64 {
        ret = (c1 as libc::c_double / 12.92f64) as libc::c_float
    } else {
        ret = pow(
            (c1 as libc::c_double + 0.055f64) / (1i32 as libc::c_double + 0.055f64),
            2.4f64,
        ) as libc::c_float
    }
    if (ret as libc::c_double) < 0.0f64 {
        return 0.0f64 as libc::c_float;
    } else if ret as libc::c_double > 1.0f64 {
        return 1.0f64 as libc::c_float;
    } else {
        return ret;
    };
}
#[no_mangle]
pub unsafe extern "C" fn linearTosRGB(mut c: libc::c_float) -> libc::c_int {
    let mut c1: libc::c_float = 0.;
    if c as libc::c_double <= 0.04045f64 / 12.92f64 {
        c1 = (c as libc::c_double * 12.92f64) as libc::c_float
    } else {
        c1 = ((1i32 as libc::c_double + 0.055f64) * pow(c as libc::c_double, 1.0f64 / 2.4f64)
            - 0.055f64) as libc::c_float
    }
    let mut ret: libc::c_int = round(255.0f64 * c1 as libc::c_double) as libc::c_int;
    if ret < 0i32 {
        return 0i32;
    } else if ret > 255i32 {
        return 255i32;
    } else {
        return ret;
    };
}
#[no_mangle]
pub unsafe extern "C" fn biomesToColors(
    mut opts: MapOptions,
    mut biomeColors: *mut [libc::c_uchar; 3],
    mut biomes_0: *mut libc::c_int,
    mut pixels: *mut libc::c_uchar,
    mut left: libc::c_int,
    mut z: libc::c_int,
) -> () {
    let mut i: libc::c_int = 0i32;
    while i < opts.width {
        if *biomes_0.offset(i as isize) > 255i32 {
            //fprintf(stderr, "Invalid biome.\n");
            //exit(-1);
            *biomes_0.offset(i as isize) &= 0xffi32
        }
        let mut r: libc::c_int = 0;
        let mut g: libc::c_int = 0;
        let mut b: libc::c_int = 0;
        let mut x: libc::c_int = left + i;
        let mut id: libc::c_int = *biomes_0.offset(i as isize);
        if id < 128i32 {
            r = (*biomeColors.offset(id as isize))[0usize] as libc::c_int;
            g = (*biomeColors.offset(id as isize))[1usize] as libc::c_int;
            b = (*biomeColors.offset(id as isize))[2usize] as libc::c_int
        } else {
            r = (*biomeColors.offset(id as isize))[0usize] as libc::c_int + 40i32;
            r = if r > 0xffi32 { 0xffi32 } else { r };
            g = (*biomeColors.offset(id as isize))[1usize] as libc::c_int + 40i32;
            g = if g > 0xffi32 { 0xffi32 } else { g };
            b = (*biomeColors.offset(id as isize))[2usize] as libc::c_int + 40i32;
            b = if b > 0xffi32 { 0xffi32 } else { b }
        }
        if 0 != opts.highlightSpecial
            || 0 != opts.highlightSearched
            || 0 != opts.highlightMutated
            || 0 != opts.highlightNewOceans
            || 0 != opts.highlightIcons
        {
            let mut highlighted: libc::c_int = 0i32;
            if (0 != opts.highlightSpecial || 0 != opts.highlightSearched)
                && (id == jungle as libc::c_int
                    || id == jungleHills as libc::c_int
                    || id == jungleEdge as libc::c_int
                    || id == jungle as libc::c_int + 128i32
                    || id == jungleEdge as libc::c_int + 128i32
                    || id == megaTaiga as libc::c_int
                    || id == megaTaigaHills as libc::c_int
                    || id == megaTaiga as libc::c_int + 128i32
                    || id == megaTaigaHills as libc::c_int + 128i32
                    || id == mesa as libc::c_int
                    || id == mesaPlateau_F as libc::c_int
                    || id == mesaPlateau as libc::c_int
                    || id == mesa as libc::c_int + 128i32
                    || id == mesaPlateau_F as libc::c_int + 128i32
                    || id == mesaPlateau as libc::c_int + 128i32
                    || id == mushroomIsland as libc::c_int
                    || id == mushroomIslandShore as libc::c_int)
            {
                highlighted = 1i32
            }
            if 0 != opts.highlightMutated && id >= 128i32 {
                highlighted = 1i32
            }
            if 0 != opts.highlightSearched
                && (id == forest as libc::c_int + 128i32
                    || id == plains as libc::c_int + 128i32
                    || id == icePlains as libc::c_int + 128i32
                    || id == mesa as libc::c_int + 128i32)
            {
                highlighted = 1i32
            }
            if 0 != opts.highlightNewOceans
                && (id == frozenOcean as libc::c_int
                    || id == frozenDeepOcean as libc::c_int
                    || id == coldOcean as libc::c_int
                    || id == coldDeepOcean as libc::c_int
                    || id == lukewarmOcean as libc::c_int
                    || id == lukewarmDeepOcean as libc::c_int
                    || id == warmOcean as libc::c_int
                    || id == warmDeepOcean as libc::c_int)
            {
                highlighted = 1i32
            }
            if 0 == highlighted {
                // I think I'm probably a tool for making this colometrically
                // correct. I should probably make the multiplier a command
                // line option.
                let mut fr: libc::c_float = sRGBToLinear(r);
                let mut fg: libc::c_float = sRGBToLinear(g);
                let mut fb: libc::c_float = sRGBToLinear(b);
                let mut a: libc::c_float = fr + fg + fb;
                r = linearTosRGB(
                    ((fr + 2i32 as libc::c_float * a) as libc::c_double * 0.005f64)
                        as libc::c_float,
                );
                g = linearTosRGB(
                    ((fg + 2i32 as libc::c_float * a) as libc::c_double * 0.005f64)
                        as libc::c_float,
                );
                b = linearTosRGB(
                    ((fb + 2i32 as libc::c_float * a) as libc::c_double * 0.005f64)
                        as libc::c_float,
                )
            }
        }
        if 0 != opts.chunkGrid {
            // Acts like a flooring div by 16. Regular
            let mut chunkx: libc::c_int = x >> 4i32;
            // divide is toward zero. Bit shift is signed.
            let mut chunkz: libc::c_int = z >> 4i32;
            let mut offx: libc::c_int = chunkx & 31i32;
            let mut offz: libc::c_int = chunkz & 31i32;
            let mut offr: libc::c_float = 0.;
            let mut offg: libc::c_float = 0.;
            let mut offb: libc::c_float = 0.;
            if offx < 24i32 && offz < 24i32 {
                // Witch hut colors
                offr = 0.35f64 as libc::c_float;
                offg = 0.2f64 as libc::c_float;
                offb = 0.4f64 as libc::c_float
            } else if offx < 27i32 && offz < 27i32 {
                // Ocean monument colors
                offr = 0.25f64 as libc::c_float;
                offg = 0.45f64 as libc::c_float;
                offb = 0.55f64 as libc::c_float
            } else {
                // Nothing colors
                offr = 0.15f64 as libc::c_float;
                offg = 0.15f64 as libc::c_float;
                offb = 0.15f64 as libc::c_float
            }
            if 0 != chunkx + chunkz & 1i32 {
                offr = (offr as libc::c_double * 0.5f64) as libc::c_float;
                offg = (offg as libc::c_double * 0.5f64) as libc::c_float;
                offb = (offb as libc::c_double * 0.5f64) as libc::c_float
            }
            r = linearTosRGB(
                (sRGBToLinear(r) as libc::c_double * 0.1f64 + offr as libc::c_double * 0.9f64)
                    as libc::c_float,
            );
            g = linearTosRGB(
                (sRGBToLinear(g) as libc::c_double * 0.1f64 + offg as libc::c_double * 0.9f64)
                    as libc::c_float,
            );
            b = linearTosRGB(
                (sRGBToLinear(b) as libc::c_double * 0.1f64 + offb as libc::c_double * 0.9f64)
                    as libc::c_float,
            )
        }
        *pixels.offset((i * 3i32 + 0i32) as isize) = r as libc::c_uchar;
        *pixels.offset((i * 3i32 + 1i32) as isize) = g as libc::c_uchar;
        *pixels.offset((i * 3i32 + 2i32) as isize) = b as libc::c_uchar;
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn writePPMHeader(
    mut fp: *mut FILE,
    mut width: libc::c_int,
    mut height: libc::c_int,
) -> () {
    fprintf(
        fp,
        b"P6\n%d %d\n255\n\x00" as *const u8 as *const libc::c_char,
        width,
        height,
    );
}
unsafe extern "C" fn min(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a < b { a } else { b };
}
unsafe extern "C" fn max(mut a: libc::c_int, mut b: libc::c_int) -> libc::c_int {
    return if a > b { a } else { b };
}
unsafe extern "C" fn dist(mut spawn: Pos, mut x: libc::c_int, mut z: libc::c_int) -> libc::c_int {
    let mut dx: libc::c_int = spawn.x - x;
    let mut dz: libc::c_int = spawn.z - z;
    return round(sqrt((dx * dx + dz * dz) as libc::c_double)) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn findQuadRegions(mut seed: int64_t, mut radius: libc::c_int) -> Pos {
    let mut rX: libc::c_int = 0;
    let mut rZ: libc::c_int = 0;
    rZ = -radius - 1i32;
    while rZ < radius {
        rX = -radius - 1i32;
        while rX < radius {
            let mut translated: int64_t = moveStructure(seed, -rX, -rZ);
            if 0 != isQuadBase(SWAMP_HUT_CONFIG, translated, 1i32 as int64_t) {
                return Pos_0 { x: rX, z: rZ };
            } else {
                rX += 1
            }
        }
        rZ += 1
    }
    return Pos_0 { x: 0i32, z: 0i32 };
}
#[no_mangle]
pub unsafe extern "C" fn findCenterFromRegion(mut quad: Pos) -> Pos {
    return Pos_0 {
        x: ((quad.x + 1i32) * 32i32 - 4i32) * 16i32,
        z: ((quad.z + 1i32) * 32i32 - 4i32) * 16i32,
    };
}
#[no_mangle]
pub unsafe extern "C" fn findQuadCenter(mut seed: int64_t, mut radius: libc::c_int) -> Pos {
    let mut quad: Pos = findQuadRegions(seed, radius);
    return findCenterFromRegion(quad);
}
#[no_mangle]
pub unsafe extern "C" fn writeMap(
    mut opts: MapOptions,
    mut g: *mut LayerStack_0,
    mut fp: *mut FILE,
) -> () {
    let mut biomeColors: [[libc::c_uchar; 3]; 256] = [[0; 3]; 256];
    initBiomeColours(biomeColors.as_mut_ptr());
    let mut fullRes: *mut Layer_0 =
        &mut *(*g).layers.offset(((*g).layerNum - 1i32) as isize) as *mut Layer_0;
    let mut cache: *mut libc::c_int = allocCache(fullRes, opts.width, 256i32);
    let vla = (opts.width * 3i32) as usize;
    let mut pixelBuf: Vec<libc::c_uchar> = ::std::vec::from_elem(0, vla);
    let mut spawn: Pos = getSpawn(
        if 0 != opts.use_1_12 {
            MC_1_12 as libc::c_int
        } else {
            MC_1_13 as libc::c_int
        },
        g,
        cache,
        opts.seed,
    );
    let mut distances: [libc::c_int; 256] = [0; 256];
    let mut i: libc::c_int = 0i32;
    while i < 256i32 {
        distances[i as usize] = 2147483647i32;
        i += 1
    }
    writePPMHeader(fp, opts.width, opts.height);
    let mut center: Pos = Pos_0 { x: 0i32, z: 0i32 };
    let mut distfrom: Pos = spawn;
    if 0 != opts.centerAtHuts {
        center = findQuadCenter(opts.seed, 10240i32 / 16i32 / 32i32);
        distfrom = center
    }
    let mut left: libc::c_int = center.x - opts.width / 2i32;
    let mut minZ: libc::c_int = center.z - opts.height / 2i32;
    let mut maxZ: libc::c_int = center.z + opts.height / 2i32;
    let mut top: libc::c_int = minZ;
    while top < maxZ {
        let mut rows: libc::c_int = maxZ - top;
        rows = if rows > 256i32 { 256i32 } else { rows };
        genArea(fullRes, cache, left, top, opts.width, rows);
        let mut row: libc::c_int = 0i32;
        while row < rows {
            let mut z: libc::c_int = top + row;
            let mut i_0: libc::c_int = 0i32;
            while i_0 < opts.width {
                let mut b: libc::c_int = *cache.offset((row * opts.width + i_0) as isize);
                if b < 256i32 {
                    distances[b as usize] =
                        min(distances[b as usize], dist(distfrom, i_0 + left, z))
                }
                i_0 += 1
            }
            //else
            //    fprintf(stderr, "INVALID BIOME!");
            biomesToColors(
                opts,
                biomeColors.as_mut_ptr(),
                cache.offset((row * opts.width) as isize),
                pixelBuf.as_mut_ptr(),
                left,
                z,
            );
            fwrite(
                pixelBuf.as_mut_ptr() as *const libc::c_void,
                3i32 as size_t,
                opts.width as size_t,
                fp,
            );
            row += 1
        }
        top += 256i32
    }
    free(cache as *mut libc::c_void);
}
/*
    fprintf(stderr, "Distances to biomes:\n");
    for (int i=0; i<256; i++) {
        if (distances[i] < INT_MAX)
            fprintf(stderr, "    %23s: %5d\n", biomeNames[i], distances[i]);
    }
    fprintf(stderr, "======================================="
            "======================================\n");
    */
#[no_mangle]
pub unsafe extern "C" fn addIcon(
    mut icon: *mut libc::c_char,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut imageScale: libc::c_int,
    mut center: Pos,
    mut pos: Pos,
    mut iconWidth: libc::c_int,
    mut iconHeight: libc::c_int,
    mut scale: libc::c_int,
) -> libc::c_int {
    // Setting scale to 0 can be used to hide an icon category.
    if 0 == scale {
        return 0i32;
    } else {
        let mut iconW: libc::c_int = iconWidth * scale;
        let mut iconH: libc::c_int = iconHeight * scale;
        let mut realX: libc::c_int = imageScale * (pos.x - center.x + width / 2i32) - iconW / 2i32;
        let mut realZ: libc::c_int = imageScale * (pos.z - center.z + height / 2i32) - iconH / 2i32;
        // Just ignore icons that are off the edge of the map.
        if realX < -iconW
            || realZ < -iconH
            || realX > width * imageScale
            || realZ > height * imageScale
        {
            return 0i32;
        } else {
            printf(
                b"    \\( \"icon/%s.png\" -resize %d00%% \\) -geometry +%d+%d -composite \\\n\x00"
                    as *const u8 as *const libc::c_char,
                icon,
                scale,
                realX,
                realZ,
            );
            return 1i32;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn regionify(
    mut center: Pos,
    mut width: libc::c_int,
    mut height: libc::c_int,
    mut regionSize: libc::c_int,
    mut tl: *mut Pos,
    mut br: *mut Pos,
) -> () {
    (*tl).x = floor(
        ((center.x - width / 2i32) as libc::c_float
            / 16i32 as libc::c_float
            / regionSize as libc::c_float) as libc::c_double,
    ) as libc::c_int;
    (*tl).z = floor(
        ((center.z - height / 2i32) as libc::c_float
            / 16i32 as libc::c_float
            / regionSize as libc::c_float) as libc::c_double,
    ) as libc::c_int;
    (*br).x = ceil(
        ((center.x + width / 2i32) as libc::c_float
            / 16i32 as libc::c_float
            / regionSize as libc::c_float) as libc::c_double,
    ) as libc::c_int;
    (*br).z = ceil(
        ((center.z + height / 2i32) as libc::c_float
            / 16i32 as libc::c_float
            / regionSize as libc::c_float) as libc::c_double,
    ) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn getBiomeAt(
    mut g: *const LayerStack_0,
    pos: Pos,
    mut buf: *mut libc::c_int,
) -> libc::c_int {
    genArea(
        &mut *(*g).layers.offset(((*g).layerNum - 1i32) as isize),
        buf,
        pos.x,
        pos.z,
        1i32,
        1i32,
    );
    return *buf.offset(0isize);
}
#[no_mangle]
pub unsafe extern "C" fn printCompositeCommand(
    mut opts: MapOptions,
    mut g: *mut LayerStack_0,
) -> () {
    let mut fullRes: *mut Layer_0 =
        &mut *(*g).layers.offset(((*g).layerNum - 1i32) as isize) as *mut Layer_0;
    let mut cache: *mut libc::c_int = allocCache(fullRes, 256i32, 256i32);
    let mut center: Pos = Pos_0 { x: 0i32, z: 0i32 };
    let mut huts: Pos = findQuadRegions(opts.seed, 10240i32 / 16i32 / 32i32);
    let mut hutCenter: Pos = findCenterFromRegion(huts);
    if 0 != opts.centerAtHuts {
        center = hutCenter
    }
    fprintf(
        stderr,
        b"Interesting structures:\n\x00" as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"     Quad hut center: %6d, %6d\n\x00" as *const u8 as *const libc::c_char,
        hutCenter.x,
        hutCenter.z,
    );
    printf(
        b"convert \"%s\" -filter Point \\\n\x00" as *const u8 as *const libc::c_char,
        opts.ppmfn.as_mut_ptr(),
    );
    if opts.imageScale != 1i32 {
        printf(
            b"    -resize %d00%% \\\n\x00" as *const u8 as *const libc::c_char,
            opts.imageScale,
        );
    }
    let mut spawn: Pos = getSpawn(
        if 0 != opts.use_1_12 {
            MC_1_12 as libc::c_int
        } else {
            MC_1_13 as libc::c_int
        },
        g,
        cache,
        opts.seed,
    );
    fprintf(
        stderr,
        b"               Spawn: %6d, %6d\n\x00" as *const u8 as *const libc::c_char,
        spawn.x,
        spawn.z,
    );
    addIcon(
        b"spawn\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        opts.width,
        opts.height,
        opts.imageScale,
        center,
        spawn,
        20i32,
        20i32,
        opts.spawnScale,
    );
    let mut desertPyramid: StructureConfig = StructureConfig_0 {
        seed: 0,
        regionSize: 0,
        chunkRange: 0,
        properties: 0,
    };
    let mut igloo: StructureConfig = StructureConfig_0 {
        seed: 0,
        regionSize: 0,
        chunkRange: 0,
        properties: 0,
    };
    let mut junglePyramid: StructureConfig = StructureConfig_0 {
        seed: 0,
        regionSize: 0,
        chunkRange: 0,
        properties: 0,
    };
    let mut swampHut: StructureConfig = StructureConfig_0 {
        seed: 0,
        regionSize: 0,
        chunkRange: 0,
        properties: 0,
    };
    if 0 != opts.use_1_12 {
        swampHut = FEATURE_CONFIG;
        junglePyramid = swampHut;
        igloo = junglePyramid;
        desertPyramid = igloo
    } else {
        desertPyramid = DESERT_PYRAMID_CONFIG;
        igloo = IGLOO_CONFIG;
        junglePyramid = JUNGLE_PYRAMID_CONFIG;
        swampHut = SWAMP_HUT_CONFIG
    }
    let mut tl: Pos = Pos_0 { x: 0, z: 0 };
    let mut br: Pos = Pos_0 { x: 0, z: 0 };
    regionify(
        center,
        opts.width,
        opts.height,
        FEATURE_CONFIG.regionSize,
        &mut tl,
        &mut br,
    );
    let mut biomeAt: libc::c_int = 0;
    let mut pos: Pos = Pos_0 { x: 0, z: 0 };
    let mut z: libc::c_int = tl.z;
    while z <= br.z {
        let mut x: libc::c_int = tl.x;
        while x <= br.x {
            pos = getLargeStructurePos(MONUMENT_CONFIG, opts.seed, x, z);
            if 0 != isViableOceanMonumentPos(*g, cache, pos.x, pos.z) {
                addIcon(
                    b"ocean_monument\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    opts.width,
                    opts.height,
                    opts.imageScale,
                    center,
                    pos,
                    20i32,
                    20i32,
                    opts.monumentScale,
                );
                if (z == huts.z || z == huts.z + 1i32) && (x == huts.x || x == huts.x + 1i32) {
                    fprintf(
                        stderr,
                        b"     Nearby monument: %6d, %6d (%d, %2d, %2d)\n\x00" as *const u8
                            as *const libc::c_char,
                        pos.x,
                        pos.z,
                        dist(spawn, pos.x, pos.z),
                        pos.x >> 4i32 & 31i32,
                        pos.z >> 4i32 & 31i32,
                    );
                }
            }
            pos = getStructurePos(VILLAGE_CONFIG, opts.seed, x, z);
            if 0 != isViableVillagePos(*g, cache, pos.x, pos.z) {
                addIcon(
                    b"village\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    opts.width,
                    opts.height,
                    opts.imageScale,
                    center,
                    pos,
                    20i32,
                    26i32,
                    opts.villageScale,
                );
            }
            pos = getStructurePos(desertPyramid, opts.seed, x, z);
            biomeAt = getBiomeAt(g, pos, cache);
            if biomeAt == desert as libc::c_int || biomeAt == desertHills as libc::c_int {
                addIcon(
                    b"desert\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    opts.width,
                    opts.height,
                    opts.imageScale,
                    center,
                    pos,
                    20i32,
                    20i32,
                    opts.desertScale,
                );
            }
            pos = getStructurePos(igloo, opts.seed, x, z);
            biomeAt = getBiomeAt(g, pos, cache);
            if biomeAt == icePlains as libc::c_int || biomeAt == coldTaiga as libc::c_int {
                addIcon(
                    b"igloo\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    opts.width,
                    opts.height,
                    opts.imageScale,
                    center,
                    pos,
                    20i32,
                    20i32,
                    opts.iglooScale,
                );
            }
            pos = getStructurePos(junglePyramid, opts.seed, x, z);
            biomeAt = getBiomeAt(g, pos, cache);
            if biomeAt == jungle as libc::c_int || biomeAt == jungleHills as libc::c_int {
                addIcon(
                    b"jungle\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    opts.width,
                    opts.height,
                    opts.imageScale,
                    center,
                    pos,
                    20i32,
                    20i32,
                    opts.jungleScale,
                );
            }
            pos = getStructurePos(swampHut, opts.seed, x, z);
            biomeAt = getBiomeAt(g, pos, cache);
            if biomeAt == swampland as libc::c_int {
                addIcon(
                    b"witch\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                    opts.width,
                    opts.height,
                    opts.imageScale,
                    center,
                    pos,
                    20i32,
                    26i32,
                    opts.hutScale,
                );
                if (z == huts.z || z == huts.z + 1i32) && (x == huts.x || x == huts.x + 1i32) {
                    fprintf(
                        stderr,
                        b"      Quad witch hut: %6d, %6d (%d, %2d, %2d)\n\x00" as *const u8
                            as *const libc::c_char,
                        pos.x,
                        pos.z,
                        dist(spawn, pos.x, pos.z),
                        pos.x >> 4i32 & 31i32,
                        pos.z >> 4i32 & 31i32,
                    );
                }
            }
            x += 1
        }
        z += 1
    }
    if 0 == opts.use_1_12 {
        regionify(
            center,
            opts.width,
            opts.height,
            OCEAN_RUIN_CONFIG.regionSize,
            &mut tl,
            &mut br,
        );
        let mut z_0: libc::c_int = tl.z;
        while z_0 <= br.z {
            let mut x_0: libc::c_int = tl.x;
            while x_0 <= br.x {
                pos = getStructurePos(OCEAN_RUIN_CONFIG, opts.seed, x_0, z_0);
                biomeAt = getBiomeAt(g, pos, cache);
                if 0 != isOceanic(biomeAt) {
                    addIcon(
                        b"ocean_ruins\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        opts.width,
                        opts.height,
                        opts.imageScale,
                        center,
                        pos,
                        20i32,
                        20i32,
                        opts.oceanRuinScale,
                    );
                }
                x_0 += 1
            }
            z_0 += 1
        }
        regionify(
            center,
            opts.width,
            opts.height,
            SHIPWRECK_CONFIG.regionSize,
            &mut tl,
            &mut br,
        );
        let mut z_1: libc::c_int = tl.z;
        while z_1 <= br.z {
            let mut x_1: libc::c_int = tl.x;
            while x_1 <= br.x {
                pos = getStructurePos(SHIPWRECK_CONFIG, opts.seed, x_1, z_1);
                biomeAt = getBiomeAt(g, pos, cache);
                if 0 != isOceanic(biomeAt) {
                    addIcon(
                        b"shipwreck\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                        opts.width,
                        opts.height,
                        opts.imageScale,
                        center,
                        pos,
                        20i32,
                        20i32,
                        opts.shipwreckScale,
                    );
                }
                x_1 += 1
            }
            z_1 += 1
        }
    }
    regionify(
        center,
        opts.width,
        opts.height,
        MANSION_CONFIG.regionSize,
        &mut tl,
        &mut br,
    );
    let mut z_2: libc::c_int = tl.z;
    while z_2 <= br.z {
        let mut x_2: libc::c_int = tl.x;
        while x_2 <= br.x {
            pos = getLargeStructurePos(MANSION_CONFIG, opts.seed, x_2, z_2);
            if 0 != isViableMansionPos(*g, cache, pos.x, pos.z) {
                addIcon(
                    b"woodland_mansion\x00" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    opts.width,
                    opts.height,
                    opts.imageScale,
                    center,
                    pos,
                    20i32,
                    26i32,
                    opts.mansionScale,
                );
                fprintf(
                    stderr,
                    b"    Woodland mansion: %6d, %6d (%d)\n\x00" as *const u8
                        as *const libc::c_char,
                    pos.x,
                    pos.z,
                    dist(spawn, pos.x, pos.z),
                );
            }
            x_2 += 1
        }
        z_2 += 1
    }
    let mut strongholds: [Pos; 128] = [Pos_0 { x: 0, z: 0 }; 128];
    findStrongholds(
        if 0 != opts.use_1_12 {
            MC_1_12 as libc::c_int
        } else {
            MC_1_13 as libc::c_int
        },
        g,
        cache,
        strongholds.as_mut_ptr(),
        opts.seed,
        0i32,
        0i32,
    );
    let mut i: libc::c_int = 0i32;
    while i < 128i32 {
        pos = strongholds[i as usize];
        if 0 != addIcon(
            b"stronghold\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            opts.width,
            opts.height,
            opts.imageScale,
            center,
            pos,
            20i32,
            20i32,
            opts.strongholdScale,
        ) {
            fprintf(
                stderr,
                b"          Stronghold: %6d, %6d (%d)\n\x00" as *const u8 as *const libc::c_char,
                pos.x,
                pos.z,
                dist(spawn, pos.x, pos.z),
            );
        }
        i += 1
    }
    printf(
        b"    \"%s\"\n\x00" as *const u8 as *const libc::c_char,
        opts.pngfn.as_mut_ptr(),
    );
    fprintf(
        stderr,
        b"=============================================================================\n\x00"
            as *const u8 as *const libc::c_char,
    );
    free(cache as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mapFake(
    mut l: *mut Layer_0,
    mut out: *mut libc::c_int,
    mut areaX: libc::c_int,
    mut areaZ: libc::c_int,
    mut areaWidth: libc::c_int,
    mut areaHeight: libc::c_int,
) -> () {
    let mut i: libc::c_int = 0i32;
    while i < areaWidth * areaHeight {
        *out.offset(i as isize) = (*l).baseSeed as libc::c_int;
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn setupFakeGenerator(mut biome: libc::c_int) -> LayerStack_0 {
    let mut g: LayerStack_0 = setupGeneratorMC17();
    let mut i: libc::c_int = 0i32;
    while i < g.layerNum {
        (*g.layers.offset(i as isize)).baseSeed = biome as int64_t;
        let ref mut fresh0 = (*g.layers.offset(i as isize)).getMap;
        *fresh0 = Some(mapFake);
        i += 1
    }
    return g;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut opts: MapOptions = parseOptions(argc, argv);
    fprintf(
        stderr,
        b"=============================================================================\n\x00"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"Writing %dx%d map for seed %ld...\n\x00" as *const u8 as *const libc::c_char,
        opts.width,
        opts.height,
        opts.seed,
    );
    fprintf(
        stderr,
        b"=============================================================================\n\x00"
            as *const u8 as *const libc::c_char,
    );
    initBiomes();
    let mut g: LayerStack_0 = LayerStack {
        layers: 0 as *mut Layer_0,
        layerNum: 0,
    };
    /*
    if (opts.oneBiome != -1) {
        g = setupFakeGenerator(opts.oneBiome);
    } else if (opts.use_1_12) {
        g = setupGeneratorMC17();
    } else {
        g = setupGeneratorMC113();
    }
    */
    let mut filename_noext: [libc::c_char; 400] = [0; 400];
    strncpy(
        filename_noext.as_mut_ptr(),
        opts.ppmfn.as_mut_ptr(),
        400i32 as libc::c_ulong,
    );
    let mut sl: libc::c_int = strlen(filename_noext.as_mut_ptr()) as libc::c_int;
    // Remove ".ppm"
    sl -= 4i32;
    filename_noext[sl as usize] = '\u{0}' as i32 as libc::c_char;
    let mut i: libc::c_int = 0;
    i = 44i32;
    while i > 0i32 {
        printf(
            b"Generating map for layer %d\n\x00" as *const u8 as *const libc::c_char,
            i,
        );
        let mut fp: *mut FILE = fopen(
            opts.ppmfn.as_mut_ptr(),
            b"w\x00" as *const u8 as *const libc::c_char,
        );
        if fp.is_null() {
            fprintf(
                stderr,
                b"Could not open file %s for writing.\n\x00" as *const u8 as *const libc::c_char,
                opts.ppmfn.as_mut_ptr(),
            );
            exit(-1i32);
        } else {
            g = setupGeneratorMC17UpTo(i);
            applySeed(&mut g, opts.seed);
            // Write the base map as a PPM file
            writeMap(opts, &mut g, fp);
            fclose(fp);
            // Convert to png to save space
            // Assumes the "pnmtopng" program is installed
            // TODO: The filename needs to be escaped
            let mut cmd: [libc::c_char; 800] = [0; 800];
            snprintf(
                cmd.as_mut_ptr(),
                800i32 as libc::c_ulong,
                b"pnmtopng \'%s\' > \'%s_%02d.png\' && rm \'%s\'\x00" as *const u8
                    as *const libc::c_char,
                opts.ppmfn.as_mut_ptr(),
                filename_noext.as_mut_ptr(),
                i - 1i32,
                opts.ppmfn.as_mut_ptr(),
            );
            system(cmd.as_mut_ptr());
            freeGenerator(g);
            i -= 1
        }
    }
    // Write a imagemagick command to compose the map with icons and convert
    // to a .png file.
    //printCompositeCommand(opts, &g);
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
