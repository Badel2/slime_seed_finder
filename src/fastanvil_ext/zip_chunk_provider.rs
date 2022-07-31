use crate::fastanvil_ext::parse_region_file_name;
use crate::fastanvil_ext::{AnvilChunkProvider, ChunkLoadError, ReadAndSeek, RegionAndOffset};
use crate::weak_alloc::ArcRef;
use crate::weak_alloc::WeakRef;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Cursor, Read, Seek};
use std::path::Path;
use zip::ZipArchive;

pub use zip::result::ZipError;

/// The chunks are read from a zip file
#[derive(Debug)]
pub struct ZipChunkProvider<R: Read + Seek> {
    zip_archive: ZipArchive<R>,
    // Prefix for the region folder. Must end with "/".
    // For example: "region/", "world/region/" or "saves/world/region/"
    region_prefix: String,
    // Cache (region_x, region_z) to uncompressed file, so each region file is
    // only uncompressed once
    cache: HashMap<(i32, i32), WeakRef<Vec<u8>>>,
}

// Workaround wrapper type because ArcRef<Vec<u8>> does not implement AsRef<[u8]>, which is needed
// for Cursor::new
struct CachedFile(ArcRef<Vec<u8>>);

impl AsRef<[u8]> for CachedFile {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[derive(Debug)]
pub enum ZipProviderError {
    Io(io::Error),
    Zip(ZipError),
    RegionFolderNotFound,
    MoreThanOneRegionFolder,
}

impl From<io::Error> for ZipProviderError {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

impl From<ZipError> for ZipProviderError {
    fn from(e: ZipError) -> Self {
        Self::Zip(e)
    }
}

// Find the path of the region folder inside the zip archive.
// For example: "region/", "world/region/" or "saves/world/region/"
// Returns error if no region folder is found
// Returns error if more than one folder is found
fn find_region_folder_path<R: Read + Seek>(
    zip_archive: &mut ZipArchive<R>,
    dimension: Option<&str>,
) -> Result<String, ZipProviderError> {
    let mut region_prefix = String::from("/");
    let mut found_region_count = 0;
    let mut found_region_folder_depth = usize::MAX;
    for unsanitized_full_path in zip_archive.file_names() {
        // full_path may contain invalid directory names such as "../../../etc/passwd", but we will
        // not decompress this file so we don't care
        let full_path = Path::new(&unsanitized_full_path);
        // file_name() returns None when the path ends with "/.."
        // we handle that case by returning a ".." filename
        let folder_name = full_path.file_name().unwrap_or(OsStr::new(".."));
        if folder_name == "region" {
            match dimension {
                Some(dimension) => {
                    if let Some(parent) = full_path.parent() {
                        let parent_file_name = parent.file_name().unwrap_or_default();
                        if parent_file_name != dimension {
                            continue;
                        }
                    } else {
                        // No parent folder means that the parent folder cannot be "DIM1"
                        continue;
                    }
                }
                None => {
                    // If no dimension is provided we will try to find the overworld.
                    // This dimension is special because it is not inside a "DIM1" or "DIM-1"
                    // folder, instead it is at the same level as these other folders.
                    // So the algorithm to find the overworld is: the "region" folder with
                    // minimum folder depth. If there is more than one match at the end of the
                    // process, this function will return an error.
                    let depth = full_path.iter().count();
                    if depth < found_region_folder_depth {
                        // Found region folder at a lower depth that the best match, reset
                        // number of matches
                        found_region_count = 0;
                        found_region_folder_depth = depth;
                    } else if depth > found_region_folder_depth {
                        // Skip region folders that are "deeper" than the current match
                        continue;
                    }
                }
            }
            found_region_count += 1;
            //region_prefix = full_path.parent().and_then(|p| p.to_str()).map(|p| p.to_string()).unwrap_or_else(|| "/".to_string());
            region_prefix = full_path
                .to_str()
                .map(|p| p.to_string())
                .unwrap_or_else(|| "/".to_string());
            // Keep searching after finding the first folder, to make sure
            // there is only one region/ folder
        }
    }
    if found_region_count == 0 {
        return Err(ZipProviderError::RegionFolderNotFound);
    }
    if found_region_count > 1 {
        return Err(ZipProviderError::MoreThanOneRegionFolder);
    }

    Ok(region_prefix)
}

fn find_all_region_mca<R: Read + Seek>(
    zip_archive: &mut ZipArchive<R>,
    region_prefix: &str,
) -> Vec<(i32, i32)> {
    let mut r = vec![];
    for unsanitized_full_path in zip_archive.file_names() {
        // full_path may contain invalid directory names such as "../../../etc/passwd", but we will
        // not decompress this file so we don't care
        let full_path = Path::new(&unsanitized_full_path);
        let folder_name = full_path.parent().unwrap_or_else(|| Path::new("/"));
        if folder_name != Path::new(region_prefix) {
            continue;
        }
        let mca_name = full_path.file_name().and_then(|x| x.to_str());
        if mca_name.is_none() {
            continue;
        }
        if let Some(coords) = parse_region_file_name(&mca_name.unwrap()) {
            r.push(coords);
        }
    }

    r
}

impl<R: Read + Seek> ZipChunkProvider<R> {
    pub fn new(reader: R) -> Result<Self, ZipProviderError> {
        Self::new_with_dimension(reader, None)
    }

    pub fn new_with_dimension(
        reader: R,
        dimension: Option<&str>,
    ) -> Result<Self, ZipProviderError> {
        let mut zip_archive = ZipArchive::new(reader)?;
        let region_prefix = find_region_folder_path(&mut zip_archive, dimension)?;
        let cache = HashMap::new();

        Ok(ZipChunkProvider {
            zip_archive,
            region_prefix,
            cache,
        })
    }

    fn region_path(&self, region_x: i32, region_z: i32) -> String {
        format!("{}r.{}.{}.mca", self.region_prefix, region_x, region_z)
    }

    fn load_region_into_cache(
        &mut self,
        region_x: i32,
        region_z: i32,
    ) -> Result<ArcRef<Vec<u8>>, ChunkLoadError> {
        let arc_ref = self
            .cache
            .get(&(region_x, region_z))
            .and_then(|w| w.upgrade());

        if let Some(arc_ref) = arc_ref {
            Ok(arc_ref)
        } else {
            let region_path = self.region_path(region_x, region_z);

            let mut region_file = match self.zip_archive.by_name(&region_path) {
                Ok(x) => x,
                Err(ZipError::FileNotFound) => {
                    return Err(ChunkLoadError::RegionNotFound { region_x, region_z })
                }
                Err(ZipError::Io(io_error)) => return Err(ChunkLoadError::ReadError { io_error }),
                Err(e) => panic!("Unhandled zip error: {}", e),
            };

            let uncompressed_size = region_file.size();
            let mut buf = Vec::with_capacity(uncompressed_size as usize);
            region_file.read_to_end(&mut buf)?;

            // Insert into cache
            let arc_ref = crate::weak_alloc::give_and_upgrade(buf);
            self.cache
                .insert((region_x, region_z), ArcRef::downgrade(&arc_ref));

            Ok(arc_ref)
        }
    }

    pub fn load_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> Result<Vec<u8>, ChunkLoadError> {
        let RegionAndOffset {
            region_x,
            region_z,
            region_chunk_x,
            region_chunk_z,
        } = RegionAndOffset::from_chunk(chunk_x, chunk_z);

        let arc_ref = self.load_region_into_cache(region_x, region_z)?;

        let buf = &*arc_ref;
        let mut region = fastanvil::Region::from_stream(Cursor::new(buf)).unwrap();
        let chunk_bytes = region
            .read_chunk(region_chunk_x.into(), region_chunk_z.into())
            .unwrap()
            .ok_or(ChunkLoadError::ChunkNotFound {
                chunk_x: region_chunk_x,
                chunk_z: region_chunk_z,
            })?;

        Ok(chunk_bytes)
    }

    pub fn list_chunks(&mut self) -> Result<Vec<(i32, i32)>, ChunkLoadError> {
        let regions = self.list_regions()?;
        let mut chunks = vec![];

        for (region_x, region_z) in regions {
            let region_bytes = self.get_region(region_x, region_z)?;
            let region = fastanvil::Region::from_stream(region_bytes);
            let mut region = match region {
                Ok(x) => x,
                Err(e) => {
                    log::warn!("Failed to read region {:?}: {:?}", (region_x, region_z), e);
                    continue;
                }
            };

            for chunk_data in region.iter() {
                let chunk_data = chunk_data.unwrap();
                let coords = RegionAndOffset {
                    region_x,
                    region_z,
                    region_chunk_x: u8::try_from(chunk_data.x).unwrap(),
                    region_chunk_z: u8::try_from(chunk_data.z).unwrap(),
                };
                let (chunk_x, chunk_z) = coords.to_chunk_coords();
                chunks.push((chunk_x, chunk_z));
            }
        }

        Ok(chunks)
    }
}

impl ZipChunkProvider<File> {
    pub fn file<P: AsRef<Path>>(path: P) -> Result<Self, ZipProviderError> {
        let file = OpenOptions::new()
            .write(false)
            .read(true)
            .create(false)
            .open(path)?;

        Self::new(file)
    }
}

impl<R: Read + Seek> AnvilChunkProvider for ZipChunkProvider<R> {
    fn get_region(
        &mut self,
        region_x: i32,
        region_z: i32,
    ) -> Result<Box<dyn ReadAndSeek + '_>, ChunkLoadError> {
        let arc_ref = self.load_region_into_cache(region_x, region_z)?;

        Ok(Box::new(Cursor::new(CachedFile(arc_ref))))
    }
    fn load_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> Result<Vec<u8>, ChunkLoadError> {
        self.load_chunk(chunk_x, chunk_z)
    }
    fn list_chunks(&mut self) -> Result<Vec<(i32, i32)>, ChunkLoadError> {
        self.list_chunks()
    }
    fn list_regions(&mut self) -> Result<Vec<(i32, i32)>, ChunkLoadError> {
        let regions = find_all_region_mca(&mut self.zip_archive, &self.region_prefix);
        Ok(regions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_empty_buffer_as_zip() {
        // Try to read an empty buffer as a zip file
        let zip = b"";

        let z = ZipChunkProvider::new(Cursor::new(zip));

        match z.err().unwrap() {
            ZipProviderError::Zip(ZipError::InvalidArchive("Invalid zip header")) => {}
            e => panic!("Expected `Zip` but got `{:?}`", e),
        }
    }

    #[test]
    fn read_small_valid_zip() {
        // Smallest possible valid zip file:
        let zip = b"\x50\x4B\x05\x06\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";

        // Reading works but since it has zero entries, the region/ folder
        // does not exist
        let z = ZipChunkProvider::new(Cursor::new(zip));

        match z {
            Err(ZipProviderError::RegionFolderNotFound) => {}
            e => panic!("Expected `RegionFolderNotFound` but got `{:?}`", e),
        }
    }
}
