use crate::chunk::Chunk;
use std::collections::HashMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer };
use serde_derive::{Deserialize, Serialize};
use serde_json;

// TODO: use real types
type Point = (i64, i64);
type BiomeId = i32;
type MinecraftVersion = String;

#[derive(Default, PartialEq, Deserialize, Serialize)]
pub struct SeedStructures {
    #[serde(default, skip_serializing_if = "is_default", rename = "slimeChunks")]
    pub slime_chunks: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub mineshafts: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default", rename = "netherForts")]
    pub nether_forts: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default", rename = "desertTemples")]
    pub desert_temples: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default", rename = "jungleTemples")]
    pub jungle_temples: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default", rename = "witchHuts")]
    pub witch_huts: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub strongholds: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub villages: Vec<Chunk>,
}

#[derive(Default, PartialEq, Deserialize, Serialize)]
pub struct SeedInfo {
    pub version: MinecraftVersion,
    #[serde(default, skip_serializing_if = "is_default")]
    pub biomes: HashMap<Point, BiomeId>,
    #[serde(flatten)]
    pub positive: SeedStructures,
    // Coords of structures that do not exist, useful to remove duplicates
    #[serde(default, skip_serializing_if = "is_default")]
    pub negative: SeedStructures,
    // Extra data from other versions
    #[serde(default, skip_serializing_if = "is_default")]
    pub and: Vec<SeedInfo>,
}

impl SeedInfo {
    pub fn read(filename: &str) -> Result<SeedInfo, ReadError> {
        use std::fs::File;
        let file = File::open(filename)?;
        let seed_info = serde_json::from_reader(file)?;

        Ok(seed_info)
    }
}

#[derive(Debug)]
pub enum ReadError {
    Io(std::io::Error),
    Serde(serde_json::Error),
}

impl From<std::io::Error> for ReadError {
    fn from(x: std::io::Error) -> Self {
        ReadError::Io(x)
    }
}

impl From<serde_json::Error> for ReadError {
    fn from(x: serde_json::Error) -> Self {
        ReadError::Serde(x)
    }
}

// Manually implement chunk serialization in order to allow different encodings:
//
// "chunks": [{x: 3, z: -5}]
// "chunks": ["3,-5"]
// "chunks": [[3, -5]]
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum PointSerialization {
    Normal { x: i64, z: i64 },
    // TODO: add string representation. It may be useful because javascript
    // doesnt actually support 64 bit integers
    //Str(String),
    Tuple((i64, i64)),
}

impl Into<Chunk> for PointSerialization {
    fn into(self) -> Chunk {
        match self {
            PointSerialization::Normal { x, z } => Chunk { x: x as i32, z: z as i32 },
            PointSerialization::Tuple((x, z)) => Chunk { x: x as i32, z: z as i32 },
        }
    }
}

impl From<Chunk> for PointSerialization {
    fn from(c: Chunk) -> Self {
        PointSerialization::Tuple((c.x as i64, c.z as i64))
    }
}

impl<'de> Deserialize<'de> for Chunk {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(PointSerialization::deserialize(deserializer)?.into())
    }
}

impl Serialize for Chunk {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let x = PointSerialization::from(*self);
        x.serialize(serializer)
    }
}

// https://www.mth.st/blog/skip-default/
fn is_default<T: Default + PartialEq>(t: &T) -> bool {
        t == &T::default()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_default() {
        let seed_info = SeedInfo::default();
        let x = serde_json::to_string(&seed_info).unwrap();
        // Version field must be serialized!
        assert_eq!(x, r#"{"version":""}"#);
    }

    #[test]
    fn deserialize_slime_chunks() {
        let json = r#"{
            "version": "1.7",
            "slimeChunks": [[14, -8], [15, -8]],
            "negative": {
                "slimeChunks": [[16, -8]]
            }
        }"#;
        let seed_info: SeedInfo = serde_json::from_str(json).unwrap();
        assert_eq!(seed_info.version, "1.7".to_string());
        assert_eq!(seed_info.positive.slime_chunks, vec![Chunk { x: 14, z: -8 }, Chunk { x: 15, z: -8 }]);
        assert_eq!(seed_info.negative.slime_chunks, vec![Chunk { x: 16, z: -8 }]);
    }
}
