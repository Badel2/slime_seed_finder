use crate::chunk::Chunk;
use crate::biome_layers::Area;
use crate::biome_layers::Map;
use std::collections::HashMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer };
use serde_json;

// TODO: use real types
pub type Point = (i64, i64);
pub type BiomeId = i32;
pub type MinecraftVersion = String;

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SeedStructures {
    #[serde(default, skip_serializing_if = "is_default")]
    pub slime_chunks: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub mineshafts: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub nether_forts: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub desert_temples: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub jungle_temples: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub witch_huts: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub strongholds: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub villages: Vec<Chunk>,
}

#[derive(Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct SeedInfo {
    pub version: MinecraftVersion,
    #[serde(default, skip_serializing_if = "is_default")]
    pub biomes: HashMap<BiomeId, Vec<Point>>,
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

fn area_from_coords<'a, I>(c: I) -> Area
where
    I: IntoIterator<Item = &'a Point>
{
    let mut c = c.into_iter();
    let c0 = c.next();
    if c0.is_none() {
        // On empty coords, return empty area
        return Area { x: 0, z: 0, w: 0, h: 0 }
    }

    let c0 = c0.unwrap();
    let (mut x_min, mut z_min) = c0;
    let (mut x_max, mut z_max) = c0;

    for &(x, z) in c {
        use std::cmp::{min, max};
        x_min = min(x_min, x);
        z_min = min(z_min, z);
        x_max = max(x_max, x);
        z_max = max(z_max, z);
    }

    let area = Area { x: x_min, z: z_min, w: (x_max - x_min + 1) as u64, h: (z_max - z_min + 1) as u64 };
    area
}

pub fn biomes_to_map<I>(biomes: I) -> Map
where
    I: IntoIterator<Item = (BiomeId, Vec<Point>)>
{
    let h: Vec<_> = biomes.into_iter().flat_map(|(k, v)| v.into_iter().map(move |x| (x, k))).collect();
    let area = area_from_coords(h.iter().map(|x| &x.0));
    let mut m = Map::new(area);
    for ((x, z), biome_id) in h {
        m.a[((x - area.x) as usize, (z - area.z) as usize)] = biome_id;
    }
    m
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

    #[test]
    fn biome_map() {
        let json = r#"{
            "version": "1.7",
            "biomes": {
                "7": [[0, 0], [2, 2]]
            }
        }"#;

        let seed_info: SeedInfo = serde_json::from_str(json).unwrap();
        assert_eq!(seed_info.version, "1.7".to_string());
        assert_eq!(seed_info.biomes[&7], vec![(0, 0), (2, 2)]);
    }
}
