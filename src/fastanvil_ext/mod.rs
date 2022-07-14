//! Extra functionality for fastanvil crate
//! Ideally this would be merged upstream

use crate::strict_parse_int::strict_parse_i32;
use fastanvil::Chunk;
use fastanvil::RCoord;
use fastanvil::RegionFileLoader;
use fastanvil::RegionLoader;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::convert::TryInto;
use std::fs::OpenOptions;
use std::io;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::path::PathBuf;

pub use compound_tag::read_gzip_compound_tag;
pub use compound_tag::CompoundTag;
pub use compound_tag::CompoundTagError;
pub use zip_chunk_provider::ZipChunkProvider;

mod compound_tag;
mod zip_chunk_provider;

/// A single dimesion of a minecraft world
pub struct Dimension<S: Read + Seek> {
    // (region_x, region_z) => region
    regions: HashMap<(i32, i32), Option<fastanvil::Region<S>>>,
    // (chunk_x, chunk_z) => chunk
    chunks: HashMap<(i32, i32), fastanvil::JavaChunk>,
}

impl Dimension<std::fs::File> {
    /// Read all the mca files from this folder
    pub fn from_folder(path: PathBuf) -> Result<Self, std::io::Error> {
        let region_coords = find_all_region_mca(path.clone())?;
        let mut regions = HashMap::with_capacity(region_coords.len());

        for (region_x, region_z) in region_coords {
            let region_path = format!("{}/r.{}.{}.mca", path.to_string_lossy(), region_x, region_z);
            let file = OpenOptions::new()
                .write(false)
                .read(true)
                .create(false)
                .open(region_path)
                .unwrap();

            let region = fastanvil::Region::from_stream(file).unwrap();
            regions.insert((region_x, region_z), Some(region));
        }

        Ok(Self {
            regions,
            chunks: HashMap::new(),
        })
    }
}

impl<S: Read + Seek> Dimension<S> {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            regions: HashMap::new(),
        }
    }

    pub fn from_chunks<I, R>(r: I) -> Result<Self, String>
    where
        I: IntoIterator<Item = ((i32, i32), R)>,
        R: Read,
    {
        let mut s = Self {
            regions: HashMap::new(),
            chunks: HashMap::new(),
        };

        for ((chunk_x, chunk_z), mut reader) in r.into_iter() {
            s.add_chunk(chunk_x, chunk_z, &mut reader)?;
        }

        Ok(s)
    }

    /// Deserialize this chunk and add it to this dimension
    pub fn add_chunk<R>(&mut self, chunk_x: i32, chunk_z: i32, reader: &mut R) -> Result<(), String>
    where
        R: Read,
    {
        let mut chunk_bytes = vec![];
        reader
            .read_to_end(&mut chunk_bytes)
            .expect("Failed to read");
        let chunk =
            fastanvil::JavaChunk::from_bytes(&chunk_bytes).expect("Failed to deserialize chunk");
        // Overwrite chunks that did already exist
        self.chunks.insert((chunk_x, chunk_z), chunk);
        // If the region did not exist, insert None to indicate that some chunks from this region
        // exist
        let (region_x, region_z) = chunk_coords_to_region_coords(chunk_x, chunk_z);
        self.regions.entry((region_x, region_z)).or_default();

        Ok(())
    }

    pub fn has_chunk(&self, chunk_x: i32, chunk_z: i32) -> bool {
        self.chunks.contains_key(&(chunk_x, chunk_z))
    }

    /// Get the block at this coordinates
    // TODO: remove need to use mutable reference to self?
    pub fn get_block<'a>(&'a mut self, x: i64, y: i64, z: i64) -> Option<&'a fastanvil::Block> {
        let block_x = u8::try_from(x & 0xF).unwrap();
        let block_z = u8::try_from(z & 0xF).unwrap();

        let chunk_x = i32::try_from(x >> 4).unwrap();
        let chunk_z = i32::try_from(z >> 4).unwrap();

        if !self.chunks.contains_key(&(chunk_x, chunk_z)) {
            let (region_x, region_z) = chunk_coords_to_region_coords(chunk_x, chunk_z);
            let (region_chunk_x, region_chunk_z) = chunk_coords_inside_region(chunk_x, chunk_z);

            let region = self.regions.get_mut(&(region_x, region_z))?.as_mut()?;
            // TODO: second expect may not be an error? If chunk does not exist we can just write
            // an empty array?
            let chunk_bytes = region
                .read_chunk(usize::from(region_chunk_x), usize::from(region_chunk_z))
                .expect("Failed to read chunk")
                .expect("Chunk does not exist");
            self.add_chunk(chunk_x, chunk_z, &mut Cursor::new(chunk_bytes))
                .expect("Failed to add chunk");
        }

        let chunk = self.chunks.get_mut(&(chunk_x, chunk_z)).unwrap();

        chunk.block(
            usize::from(block_x),
            isize::try_from(y).unwrap(),
            usize::from(block_z),
        )
    }

    /// Iterate over all the chunks in this dimension. Iteration order is undefined.
    pub fn iter_chunks<'a>(&'a self) -> impl Iterator<Item = &'a fastanvil::JavaChunk> {
        // TODO: iterate over regions, and lazily load them into memory
        // Note that it is possible to add chunks that do not belong to any region. In that case,
        // the region will have a None entry, and this iterator must check all the possible chunks.

        // https://github.com/rust-lang/rust/issues/36375
        if true {
            unimplemented!()
        } else {
            std::iter::empty()
        }
    }

    /// Iterate over all the blocks in this dimension. Iteration order is undefined.
    pub fn iter_blocks<'a>(&'a self) -> impl Iterator<Item = &'a fastanvil::Block> {
        // TODO: use iter_chunks to implement this

        // https://github.com/rust-lang/rust/issues/36375
        if true {
            unimplemented!()
        } else {
            std::iter::empty()
        }
    }
}

// Find all the region files in the current folder
fn find_all_region_mca(path: PathBuf) -> Result<Vec<(i32, i32)>, std::io::Error> {
    let mut r = vec![];

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        let filename = path.file_name().and_then(|x| x.to_str());
        if filename.is_none() {
            continue;
        }

        if let Some(coords) = parse_region_file_name(&filename.unwrap()) {
            r.push(coords);
        }
    }

    Ok(r)
}

// Copy of fastanvil::Region::for_each_chunk that ignores errors
pub fn region_for_each_chunk<S>(
    region: &mut fastanvil::Region<S>,
    mut f: impl FnMut(usize, usize, &Vec<u8>),
) -> fastanvil::Result<()>
where
    S: Seek + Read,
{
    for chunk_data in region.iter() {
        match chunk_data {
            Ok(chunk_data) => f(chunk_data.x, chunk_data.z, &chunk_data.data),
            Err(e) => {
                // Ignore errors
                // TODO: it would be nice to get the x, z coordinated of the chunk that failed
                log::error!("Error loading chunk: {:?}", e);
            }
        }
    }

    Ok(())
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RegionAndOffset {
    pub region_x: i32,
    pub region_z: i32,
    pub region_chunk_x: u8,
    pub region_chunk_z: u8,
}

pub fn chunk_coords_to_region_coords(chunk_x: i32, chunk_z: i32) -> (i32, i32) {
    (chunk_x >> 5, chunk_z >> 5)
}

pub fn chunk_coords_inside_region(chunk_x: i32, chunk_z: i32) -> (u8, u8) {
    ((chunk_x & 0x1F) as u8, (chunk_z & 0x1F) as u8)
}

impl RegionAndOffset {
    pub fn from_chunk(chunk_x: i32, chunk_z: i32) -> Self {
        let (region_x, region_z) = chunk_coords_to_region_coords(chunk_x, chunk_z);
        let (region_chunk_x, region_chunk_z) = chunk_coords_inside_region(chunk_x, chunk_z);

        Self {
            region_x,
            region_z,
            region_chunk_x,
            region_chunk_z,
        }
    }

    pub fn to_chunk_coords(&self) -> (i32, i32) {
        (
            (self.region_x << 5) + i32::from(self.region_chunk_x),
            (self.region_z << 5) + i32::from(self.region_chunk_z),
        )
    }
}

/// Parse "r.1.2.mca" into (1, 2)
pub fn parse_region_file_name(s: &str) -> Option<(i32, i32)> {
    let mut iter = s.as_bytes().split(|x| *x == b'.');
    if iter.next() != Some(b"r") {
        return None;
    }
    let x = strict_parse_i32(iter.next()?)?;
    let z = strict_parse_i32(iter.next()?)?;
    if iter.next() != Some(b"mca") {
        return None;
    }

    if iter.next() != None {
        // Trailing dots
        return None;
    }

    Some((x, z))
}

/// Possible errors while loading the chunk.
#[derive(Debug)]
pub enum ChunkLoadError {
    /// Region at specified coordinates not found.
    RegionNotFound { region_x: i32, region_z: i32 },
    /// Chunk at specified coordinates inside region not found.
    ChunkNotFound { chunk_x: u8, chunk_z: u8 },
    /*
    /// Chunk length overlaps declared maximum.
    ///
    /// This should not occur under normal conditions.
    ///
    /// Region file are corrupted.
    LengthExceedsMaximum {
        /// Chunk length.
        length: u32,
        /// Chunk maximum expected length.
        maximum_length: u32,
    },
    /// Currently are only 2 types of compression: Gzip and Zlib.
    ///
    /// This should not occur under normal conditions.
    ///
    /// Region file are corrupted or was introduced new compression type.
    UnsupportedCompressionScheme {
        /// Compression scheme type id.
        compression_scheme: u8,
    },
    */
    /// I/O Error which happened while were reading chunk data from region file.
    ReadError { io_error: io::Error },
    /*
    /// Error while decoding binary data to NBT tag.
    ///
    /// This should not occur under normal conditions.
    ///
    /// Region file are corrupted or a developer error in the NBT library.
    TagDecodeError { tag_decode_error: TagDecodeError },
    */
}

impl From<io::Error> for ChunkLoadError {
    fn from(io_error: io::Error) -> Self {
        ChunkLoadError::ReadError { io_error }
    }
}

pub trait ReadAndSeek: Read + Seek {}
impl<T: Read + Seek> ReadAndSeek for T {}

pub trait AnvilChunkProvider {
    fn get_region(
        &mut self,
        region_x: i32,
        region_z: i32,
    ) -> Result<Box<dyn ReadAndSeek + '_>, ChunkLoadError>;
    fn load_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> Result<Vec<u8>, ChunkLoadError>;
    fn list_chunks(&mut self) -> Result<Vec<(i32, i32)>, ChunkLoadError>;
    fn list_regions(&mut self) -> Result<Vec<(i32, i32)>, ChunkLoadError>;
}

pub struct FolderChunkProvider {
    inner: RegionFileLoader,
}

impl FolderChunkProvider {
    pub fn new(region_dir: PathBuf) -> Self {
        Self {
            inner: RegionFileLoader::new(region_dir),
        }
    }
}

impl AnvilChunkProvider for FolderChunkProvider {
    fn get_region(
        &mut self,
        region_x: i32,
        region_z: i32,
    ) -> Result<Box<dyn ReadAndSeek + '_>, ChunkLoadError> {
        let region = self
            .inner
            .region(
                RCoord(region_x.try_into().unwrap()),
                RCoord(region_z.try_into().unwrap()),
            )
            .ok_or(ChunkLoadError::RegionNotFound { region_x, region_z })?;

        Ok(Box::new(region.into_inner().unwrap()))
    }

    fn load_chunk(&mut self, chunk_x: i32, chunk_z: i32) -> Result<Vec<u8>, ChunkLoadError> {
        let RegionAndOffset {
            region_x,
            region_z,
            region_chunk_x,
            region_chunk_z,
        } = RegionAndOffset::from_chunk(chunk_x, chunk_z);

        let region_bytes = self.get_region(region_x, region_z)?;
        let mut region = fastanvil::Region::from_stream(region_bytes).unwrap();
        let chunk_bytes = region
            .read_chunk(region_chunk_x.into(), region_chunk_z.into())
            .unwrap()
            .ok_or(ChunkLoadError::ChunkNotFound {
                chunk_x: region_chunk_x,
                chunk_z: region_chunk_z,
            })?;

        Ok(chunk_bytes)
    }

    fn list_chunks(&mut self) -> Result<Vec<(i32, i32)>, ChunkLoadError> {
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

    fn list_regions(&mut self) -> Result<Vec<(i32, i32)>, ChunkLoadError> {
        Ok(self
            .inner
            .list()
            .unwrap()
            .into_iter()
            .map(|rcoords| {
                (
                    i32::try_from(rcoords.0 .0).unwrap(),
                    i32::try_from(rcoords.1 .0).unwrap(),
                )
            })
            .collect())
    }
}
