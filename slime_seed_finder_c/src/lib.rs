use slime_seed_finder::anvil;
use slime_seed_finder::biome_layers;
use slime_seed_finder::biome_layers::Area3D;
use slime_seed_finder::seed_info::MinecraftVersion;
use std::convert::TryInto;
use std::ffi::CStr;
use std::ffi::CString;
use std::ffi::OsString;
use std::mem;
use std::os::raw::c_char;
use std::path::Path;
use std::ptr;
use std::slice;

// TODO: Panicking across a FFI boundary is undefined behavior.
// Solution: either ensure that the exported functions never panic or use panic="abort".
// Problem: I don't know how to set panic="abort" for this crate only, cargo complains and says
// that it must be set at the workspace root. Should be as simple as adding
// [profile.release] panic = "abort"

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
pub struct Map3D {
    pub x: i64,
    pub y: i64,
    pub z: i64,
    pub sx: u64,
    pub sy: u64,
    pub sz: u64,
    pub a: *mut i32,
}

/// # Safety
///
/// The pointer `map.a` must be the same as when this `Map3D` was initialized. The contents of the
/// array `map.a` may have been modified. The values `map.sx`, `map.sy`, and `map.sz` must be the
/// same as when this `Map3D` was initialized.
#[no_mangle]
pub unsafe extern "C" fn free_map(map: Map3D) {
    if map.a != ptr::null_mut() {
        let ptr = map.a;
        let len = (map.sx * map.sy * map.sz) as usize;
        let boxed_slice = Box::from_raw(slice::from_raw_parts_mut(ptr, len));
        mem::drop(boxed_slice);
    }
}

#[no_mangle]
pub extern "C" fn read_biome_map_from_mc_world(
    input_zip_path: *const c_char,
    mc_version: *const c_char,
    biome_map: *mut Map3D,
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

    let mut chunk_provider = anvil::ZipChunkProvider::file(input_zip_path).unwrap();

    let mut biomes_arr;
    let area;
    match version {
        MinecraftVersion::Java1_3
        | MinecraftVersion::Java1_7
        | MinecraftVersion::Java1_9
        | MinecraftVersion::Java1_11
        | MinecraftVersion::Java1_13
        | MinecraftVersion::Java1_14 => {
            let points = anvil::get_all_biomes_1_14(&mut chunk_provider);
            let area2d =
                biome_layers::Area::from_coords(points.iter().map(|(_biome_id, point)| *point));
            area = Area3D {
                x: area2d.x,
                y: 0,
                z: area2d.z,
                sx: area2d.w,
                sy: 1,
                sz: area2d.h,
            };

            // -1 is the unknown biome id, which will be ignored during comparison
            biomes_arr = vec![-1; (area.sx * area.sz) as usize].into_boxed_slice();
            for (biome_id, point) in points {
                let idx =
                    (point.z - area.z) as usize * area.sx as usize + (point.x - area.x) as usize;
                biomes_arr[idx] = biome_id.0;
            }
        }
        MinecraftVersion::Java1_15
        | MinecraftVersion::Java1_16_1
        | MinecraftVersion::Java1_16
        | MinecraftVersion::Java1_17 => {
            let points = anvil::get_all_biomes_1_15(&mut chunk_provider);
            let area2d =
                biome_layers::Area::from_coords4(points.iter().map(|(_biome_id, point)| *point));
            area = Area3D {
                x: area2d.x,
                y: 0,
                z: area2d.z,
                sx: area2d.w,
                sy: 1,
                sz: area2d.h,
            };

            // -1 is the unknown biome id, which will be ignored during comparison
            biomes_arr = vec![-1; (area.sx * area.sz) as usize].into_boxed_slice();
            for (biome_id, point) in points {
                let idx =
                    (point.z - area.z) as usize * area.sx as usize + (point.x - area.x) as usize;
                biomes_arr[idx] = biome_id.0;
            }
        }
        MinecraftVersion::Java1_18 => {
            let points = anvil::get_all_biomes_1_18(&mut chunk_provider);
            area = biome_layers::Area3D::from_coords4(
                points.iter().map(|(_biome_id, point3d)| *point3d),
            );

            // -1 is the unknown biome id, which will be ignored during comparison
            biomes_arr = vec![-1; (area.sx * area.sy * area.sz) as usize].into_boxed_slice();
            for (biome_id, point) in points {
                let idx = (point.y - area.y) as usize * (area.sz as usize * area.sx as usize)
                    + (point.z - area.z) as usize * area.sx as usize
                    + (point.x - area.x) as usize;
                biomes_arr[idx] = biome_id.0;
            }
        }
        _ => return c_err(format!("unsupported version: {:?}", version)),
    }

    let c_biomes_arr = biomes_arr.as_mut_ptr();
    mem::forget(biomes_arr);

    let c_map = Map3D {
        x: area.x,
        y: area.y,
        z: area.z,
        sx: area.sx,
        sy: area.sy,
        sz: area.sz,
        a: c_biomes_arr,
    };

    unsafe {
        *biome_map = c_map;
    }

    ptr::null_mut()
}

#[cfg(feature = "image")]
#[no_mangle]
pub extern "C" fn draw_map3d_image_to_file(
    biome_map: *const Map3D,
    output_file_path: *const c_char,
) -> *mut c_char {
    pub fn draw_map_image(map: &Map3D) -> Vec<u8> {
        let dims = (map.sx * map.sy * map.sz) as usize;
        let mut v = vec![0; dims * 4];
        for i in 0..dims {
            let color =
                biome_layers::biome_to_color(unsafe { *map.a.offset(i.try_into().unwrap()) });
            v[i * 4 + 0] = color[0];
            v[i * 4 + 1] = color[1];
            v[i * 4 + 2] = color[2];
            v[i * 4 + 3] = color[3];
        }

        v
    }

    let map = unsafe { biome_map.as_ref().expect("biome_map is null") };
    let map_image = draw_map_image(map);
    let width = map.sx.try_into().unwrap();
    let height = (map.sz * map.sy).try_into().unwrap();
    let output_file_path = unsafe { CStr::from_ptr(output_file_path) };
    let output_file_path = match output_file_path.to_str() {
        Ok(x) => x,
        Err(e) => return c_err(format!("input_zip_path error: {}", e)),
    };
    let output_file_path = OsString::from(output_file_path);
    let output_file_path: &Path = output_file_path.as_ref();
    match image::save_buffer(
        output_file_path,
        &map_image,
        width,
        height,
        image::ColorType::Rgba8,
    ) {
        Ok(()) => {}
        Err(e) => return c_err(format!("error writing image: {}", e)),
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
