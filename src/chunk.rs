#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: i64,
    pub z: i64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point2 {
    pub x: i64,
    pub z: i64,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point4 {
    pub x: i64,
    pub z: i64,
}

impl Point {
    pub fn into_quarter_scale(self) -> Option<Point4> {
        fn divide_by_4(x: i64) -> i64 {
            (x - 2) / 4
        }

        if self.x as u8 % 4 == 2 && self.z as u8 % 4 == 2 {
            Some(Point4 { x: divide_by_4(self.x), z: divide_by_4(self.z)})
        } else {
            None
        }
    }
}

impl Point4 {
    pub fn into_full_resolution(self) -> Point {
        fn multiply_by_4(x: i64) -> i64 {
            (x * 4) + 2
        }

        Point { x: multiply_by_4(self.x), z: multiply_by_4(self.z) }
    }
}
