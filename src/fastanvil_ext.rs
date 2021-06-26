//! Extra functionality for fastanvil crate
//! Ideally this would be merged upstream

use fastanvil::Chunk;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::fs::OpenOptions;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::path::PathBuf;

/// A single dimesion of a minecraft world
pub struct Dimension<S: Read + Seek> {
    // (region_x, region_z) => region
    regions: HashMap<(i32, i32), Option<fastanvil::RegionBuffer<S>>>,
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

            let region = fastanvil::RegionBuffer::new(file);
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
    where I: IntoIterator<Item = ((i32, i32), R)>, R: Read
    {
        let mut s = Self { regions: HashMap::new(), chunks: HashMap::new() };

        for ((chunk_x, chunk_z), mut reader) in r.into_iter() {
            s.add_chunk(chunk_x, chunk_z, &mut reader)?;
        }

        Ok(s)
    }

    /// Deserialize this chunk and add it to this dimension
    pub fn add_chunk<R>(&mut self, chunk_x: i32, chunk_z: i32, reader: &mut R) -> Result<(), String>
    where R: Read {
        let mut chunk_bytes = vec![];
        reader.read_to_end(&mut chunk_bytes).expect("Failed to read");
        let chunk: fastanvil::JavaChunk = fastnbt::de::from_bytes(&chunk_bytes).expect("Failed to deserialize chunk");
        // Overwrite chunks that did already exist
        self.chunks.insert((chunk_x, chunk_z), chunk);
        // If the region did not exist, insert None to indicate that some chunks from this region
        // exist
        let (region_x, region_z) = anvil_region::chunk_coords_to_region_coords(chunk_x, chunk_z);
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
            let (region_x, region_z) = anvil_region::chunk_coords_to_region_coords(chunk_x, chunk_z);
            let (region_chunk_x, region_chunk_z) = anvil_region::chunk_coords_inside_region(chunk_x, chunk_z);

            let region = self.regions.get_mut(&(region_x, region_z))?.as_mut()?;
            let chunk_bytes = region.load_chunk(usize::from(region_chunk_x), usize::from(region_chunk_z)).expect("Failed to load chunk");
            self.add_chunk(chunk_x, chunk_z, &mut Cursor::new(chunk_bytes)).expect("Failed to add chunk");
        }

        let chunk = self.chunks.get_mut(&(chunk_x, chunk_z)).unwrap();

        chunk.block(usize::from(block_x), isize::try_from(y).unwrap(), usize::from(block_z))
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

        if let Some(coords) = anvil_region::parse_region_file_name(&filename.unwrap()) {
            r.push(coords);
        }
    }

    Ok(r)
}

// Copy of fastanvil::Region::for_each_chunk that ignores errors
pub fn region_for_each_chunk<S>(region: &mut fastanvil::RegionBuffer<S>, mut f: impl FnMut(usize, usize, &Vec<u8>)) -> fastanvil::Result<()>
where S: Seek + Read
{
        let mut offsets = Vec::<fastanvil::ChunkLocation>::new();

        // Build list of existing chunks
        for x in 0..32 {
            for z in 0..32 {
                match region.chunk_location(x, z) {
                    Ok(loc) => {
                        // 0,0 chunk location means the chunk isn't present.
                        // cannot decide if this means we should return an error from chunk_location() or not.
                        if loc.begin_sector != 0 && loc.sector_count != 0 {
                            offsets.push(loc);
                        }
                    }
                    Err(e) => {
                        // Ignore errors
                        log::error!("Error getting chunk location of chunk {:?}: {:?}", (x, z), e);
                    }
                }
            }
        }

        // sort so we linearly seek through the file.
        // might make things easier on a HDD [citation needed]
        offsets.sort_by(|o1, o2| o2.begin_sector.cmp(&o1.begin_sector));

        for offset in offsets {
            let chunk = region.load_chunk(offset.x, offset.z)?;
            f(offset.x, offset.z, &chunk);
        }

        Ok(())
}
