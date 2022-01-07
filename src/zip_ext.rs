//! Extensions to the zip crate
use std::ffi::OsStr;
use std::io::Read;
use std::io::Seek;
use std::path::Path;
use zip::ZipArchive;

pub enum FindFileInZipError {
    FoundMoreThanOne,
    NotFound,
}

impl std::fmt::Display for FindFileInZipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let static_str = match self {
            FindFileInZipError::FoundMoreThanOne => "found more than one",
            FindFileInZipError::NotFound => "not found",
        };
        write!(f, "{}", static_str)
    }
}

pub fn find_file_in_zip_exactly_once<'a, R, T>(
    zip_archive: &'a mut ZipArchive<R>,
    target_file: T,
) -> Result<&'a str, FindFileInZipError>
where
    &'a OsStr: PartialEq<T>,
    R: Read + Seek,
    T: std::fmt::Display,
{
    let mut iter = find_file_in_zip(zip_archive, target_file);
    let found_file = iter.next().ok_or(FindFileInZipError::NotFound)?;

    // Keep searching after finding the first file, to make sure there is only one
    // level.dat file
    if iter.next().is_some() {
        return Err(FindFileInZipError::FoundMoreThanOne);
    }

    Ok(found_file)
}

pub fn find_file_in_zip<'a, R, T>(
    zip_archive: &'a mut ZipArchive<R>,
    target_file: T,
) -> impl Iterator<Item = &'a str>
where
    &'a OsStr: PartialEq<T>,
    R: Read + Seek,
{
    zip_archive
        .file_names()
        .filter_map(move |unsanitized_full_path| {
            // full_path may contain invalid directory names such as "../../../etc/passwd", but we will
            // not decompress this file so we don't care
            let full_path = Path::new(unsanitized_full_path);
            // file_name() returns None when the path ends with "/.."
            // we handle that case by returning a ".." filename
            let file_name = full_path.file_name().unwrap_or_else(|| OsStr::new(".."));
            if file_name == target_file {
                Some(unsanitized_full_path)
            } else {
                None
            }
        })
}
