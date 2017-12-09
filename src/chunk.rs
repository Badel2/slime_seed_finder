#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Chunk {
    pub x: i32,
    pub z: i32,
}

impl Chunk {
    pub fn new(x: i32, z: i32) -> Chunk {
        Chunk { x, z }
    }
    // TODO this is untested
    pub fn from_coordinates(x: i32, z: i32) -> Chunk {
        Chunk {
            x: (x + 16) / 16 - 1,
            z: (z + 16) / 16 - 1,
        }
    }
}
