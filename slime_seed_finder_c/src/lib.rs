use slime_seed_finder::anvil;
use slime_seed_finder::biome_layers;
use slime_seed_finder::seed_info::MinecraftVersion;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::OsString;
use std::mem;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr;
use std::slice;

// Error handling: instead of Result<(), Error> we use *mut c_char, which is null when the result
// is Ok, and it is a c string of the error message when the result is Err.
// This function is used to build errors from the Rust side, and it is expected that the C code
// will call to free_error_msg after handling the error.
fn c_err<T>(e: T) -> *mut c_char
where
    T: Into<Vec<u8>>,
{
    let msg = match CString::new(e) {
        Ok(x) => x,
        Err(nul_error) => CString::new(format!("Malformed error string: {:?}", nul_error)).unwrap(),
    };

    msg.into_raw()
}

/// # Safety
///
/// The input `err` must have been created using the `c_err` function, and it must have not been
/// modified in any way.
#[no_mangle]
pub unsafe extern "C" fn free_error_msg(err: *mut c_char) {
    if err != ptr::null_mut() {
        let c_string = CString::from_raw(err);
        mem::drop(c_string);
    }
}

#[no_mangle]
pub extern "C" fn read_seed_from_mc_world(
    input_zip_path: *const c_char,
    mc_version: *const c_char,
    seed: *mut i64,
) -> *mut c_char {
    // TODO: I tried to make the API as simple as possible, but I failed. File paths are not
    // encoded as UTF8 in windows, which means that we would need to add an additional input
    // parameter "input_zip_path_len", and then cast the input_zip_path to an OsString.
    // So either we do that, or we only support NULL-terminated UTF8 string, which is the current
    // solution, and that implies an extra call to strlen, UTF8 validation, and an extra
    // allocation.
    let input_zip_path = unsafe { CStr::from_ptr(input_zip_path) };
    let input_zip_path = match input_zip_path.to_str() {
        Ok(x) => x,
        Err(e) => return c_err(format!("input_zip_path error: {}", e)),
    };
    let input_zip_path = OsString::from(input_zip_path);
    let input_zip_path: &Path = input_zip_path.as_ref();

    let mc_version = unsafe { CStr::from_ptr(mc_version) };
    let mc_version = match mc_version.to_str() {
        Ok(x) => x,
        Err(e) => return c_err(format!("mc_version error: {}", e)),
    };
    let version: Option<MinecraftVersion> = mc_version.parse().ok();

    let world_seed = match anvil::read_seed_from_level_dat_zip(input_zip_path, version) {
        Ok(x) => x,
        Err(e) => return c_err(format!("Error reading seed from zip file: {}", e)),
    };

    unsafe {
        *seed = world_seed;
    }

    ptr::null_mut()
}

#[repr(C)]
pub struct Map {
    pub x: i64,
    pub z: i64,
    pub w: u64,
    pub h: u64,
    pub a: *mut i32,
}

/// # Safety
///
/// The pointer `map.a` must be the same as when this `Map` was initialized, but it may have been
/// modified. The values `map.w` and `map.h` must be the same as when this `Map` was initialized.
#[no_mangle]
pub unsafe extern "C" fn free_map(map: Map) {
    if map.a != ptr::null_mut() {
        let ptr = map.a;
        let len = (map.w * map.h) as usize;
        let boxed_slice = Box::from_raw(slice::from_raw_parts_mut(ptr, len));
        mem::drop(boxed_slice);
    }
}

#[no_mangle]
pub extern "C" fn read_biome_map_from_mc_world(
    input_zip_path: *const c_char,
    mc_version: *const c_char,
    biome_map: *mut Map,
) -> *mut c_char {
    let input_zip_path = unsafe { CStr::from_ptr(input_zip_path) };
    let input_zip_path = match input_zip_path.to_str() {
        Ok(x) => x,
        Err(e) => return c_err(format!("input_zip_path error: {}", e)),
    };
    let input_zip_path = OsString::from(input_zip_path);
    let input_zip_path: &Path = input_zip_path.as_ref();

    let mc_version = unsafe { CStr::from_ptr(mc_version) };
    let mc_version = match mc_version.to_str() {
        Ok(x) => x,
        Err(e) => return c_err(format!("mc_version error: {}", e)),
    };
    let version: MinecraftVersion = match mc_version.parse() {
        Ok(x) => x,
        Err(e) => return c_err(format!("mc_version parse error: {}", e)),
    };

    match version {
        MinecraftVersion::Java1_15
        | MinecraftVersion::Java1_16_1
        | MinecraftVersion::Java1_16
        | MinecraftVersion::Java1_17 => {}
        _ => return c_err(format!("unsupported version: {:?}", version)),
    }

    let mut chunk_provider = anvil::ZipChunkProvider::file(input_zip_path).unwrap();
    let points = anvil::get_all_biomes_1_15(&mut chunk_provider);
    let area = biome_layers::Area::from_coords4(points.iter().map(|(_biome_id, point)| *point));

    // -1 is the unknown biome id, which will be ignored during comparison
    let mut biomes_arr = vec![-1; (area.w * area.h) as usize].into_boxed_slice();
    for (biome_id, point) in points {
        let idx = (point.z - area.z) as usize * area.w as usize + (point.x - area.x) as usize;
        biomes_arr[idx] = biome_id.0;
    }

    let c_biomes_arr = biomes_arr.as_mut_ptr();
    mem::forget(biomes_arr);

    let c_map = Map {
        x: area.x,
        z: area.z,
        w: area.w,
        h: area.h,
        a: c_biomes_arr,
    };

    unsafe {
        *biome_map = c_map;
    }

    ptr::null_mut()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
