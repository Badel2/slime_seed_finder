//! Extra functionality for fastanvil crate
//! Ideally this would be merged upstream

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
    regions: HashMap<(i32, i32), Option<fastanvil::Region<S>>>,
    // (chunk_x, chunk_z) => chunk
    // Invariant: the boxed slice must live as long as the Chunk, and it must be impossible for a
    // caller to obtain access to the Chunk<'static>. Access to a Chunk<'a> is fine
    chunks: HashMap<(i32, i32), (fastanvil::Chunk<'static>, Box<[u8]>)>,
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

            let region = fastanvil::Region::new(file);
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
        let pinned_chunk_bytes = chunk_bytes.into_boxed_slice();
        let static_ref_to_chunk_bytes: &'static [u8] = unsafe {
            // Trust me, pinned_chunk_bytes will be valid for as long as "chunk"
            let len = pinned_chunk_bytes.len();
            let raw_pointer = &*pinned_chunk_bytes as *const [u8];
            std::slice::from_raw_parts(raw_pointer as *const u8, len)
        };

        let chunk: fastanvil::Chunk = fastnbt::de::from_bytes(&static_ref_to_chunk_bytes).expect("Failed to deserialize chunk");
        // Overwrite chunks that did already exist
        self.chunks.insert((chunk_x, chunk_z), (chunk, pinned_chunk_bytes));
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
    pub fn get_block<'a>(&'a mut self, x: i64, y: i64, z: i64) -> Option<&'a fastanvil::Block<'a>> {
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

        let (chunk, _) = self.chunks.get_mut(&(chunk_x, chunk_z)).unwrap();

        chunk.block(usize::from(block_x), usize::try_from(y).unwrap(), usize::from(block_z))
    }

    /// Iterate over all the chunks in this dimension. Iteration order is undefined.
    pub fn iter_chunks<'a>(&'a self) -> impl Iterator<Item = &'a fastanvil::Chunk<'a>> {
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
    pub fn iter_blocks<'a>(&'a self) -> impl Iterator<Item = &'a fastanvil::Block<'a>> {
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
