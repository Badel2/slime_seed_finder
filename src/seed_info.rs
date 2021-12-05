use crate::chunk::Chunk;
use crate::chunk::Point;
use crate::chunk::Point4;
use crate::biome_layers::Area;
use crate::biome_layers::Map;
use std::collections::HashMap;
use std::str::FromStr;
use std::path::Path;
use serde::{Deserialize, Deserializer, Serialize, Serializer };
use serde_json;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct BiomeId(pub i32);

impl FromStr for BiomeId {
    type Err = std::num::ParseIntError;

    fn from_str(x: &str) -> Result<Self, Self::Err> {
        Ok(Self(x.parse()?))
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum MinecraftVersion {
    JavaAlpha1_2_5, // Alpha v1.0.4 - Alpha v1.2.5
    JavaBeta, // Alpha v1.2.6 - Beta ???
    Java1_2, // From Beta 1.7 to 1.2
    Java1_3, // From 1.3 to 1.6
    Java1_7, // From 1.7 to 1.8
    Java1_9, // From 1.9 to 1.10
    Java1_11, // From 1.11 to 1.12
    Java1_13,
    Java1_14,
    Java1_15,
    Java1_16_1, // From 1.16 to 1.16.1
    Java1_16, // From 1.16.2 to 1.16.5
    Java1_17,
    Java1_18,
}

impl MinecraftVersion {
    /// Total number of biome layers
    pub fn num_layers(&self) -> u32 {
        match self {
            MinecraftVersion::Java1_3 => 33,
            MinecraftVersion::Java1_7 => 43,
            MinecraftVersion::Java1_9 => 43,
            MinecraftVersion::Java1_11 => 43,
            MinecraftVersion::Java1_13 => 51,
            MinecraftVersion::Java1_14 => 51, // actually 52 but bamboo jungle is inlined...
            MinecraftVersion::Java1_15 => 51, // actually 52 but bamboo jungle is inlined...
            MinecraftVersion::Java1_16_1 => 51, // actually 52 but bamboo jungle is inlined...
            MinecraftVersion::Java1_16 => 51, // actually 52 but bamboo jungle is inlined...
            MinecraftVersion::Java1_17 => 51, // actually 52 but bamboo jungle is inlined...
            MinecraftVersion::Java1_18 => 9, // TODO: may change
            _ => panic!("Biome generator for version {:?} is not implemented", self),
        }
    }
}

impl FromStr for MinecraftVersion {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: this ignores everything starting from the second dot: 1.2.3 is parsed as 1.2
        let x = trim_at_dot(2, s);
        Ok(match x {
            "1.3" | "1.4" | "1.5" | "1.6" => MinecraftVersion::Java1_3,
            "1.7" | "1.8" => MinecraftVersion::Java1_7,
            "1.9" | "1.10" => MinecraftVersion::Java1_9,
            "1.11" | "1.12" => MinecraftVersion::Java1_11,
            "1.13" => MinecraftVersion::Java1_13,
            "1.14" => MinecraftVersion::Java1_14,
            "1.15" => MinecraftVersion::Java1_15,
            "1.16" => {
                // Need to handle special case here: the biome generation was changed in 1.16.2
                // So 1.16 and 1.16.1 use the same as 1.15, but 1.16.2 and later use a new one
                // The default should always be the newest one, so to use the generation from
                // 1.16.1 you must explicitly say 1.16.1
                if trim_at_dot(3, s) == "1.16.1" {
                    MinecraftVersion::Java1_16_1
                } else {
                    MinecraftVersion::Java1_16
                }
            }
            "1.17" => MinecraftVersion::Java1_17,
            "1.18" => MinecraftVersion::Java1_18,
            _ => return Err(s.to_string())
        })
    }
}

fn trim_at_dot(n: u32, x: &str) -> &str {
    let mut count = 0;
    let idx = x.find(|c| {
        if c == '.' {
            count += 1;
            if count == n {
                return true;
            }
        }

        false
    }).unwrap_or(x.len());

    &x[..idx]
}

// Options not necesarly related to the seed or the minecraft world
#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
// Allow unknown fields
//#[serde(deny_unknown_fields)]
pub struct Options {
    // Indicates that the seed was not generated using the nextLong method from
    // Java Random. That method has a flaw of using only 48 bits of entropy, so
    // we can use extend48 to find the full 64 bit seed given only 48 bits.
    #[serde(default, skip_serializing_if = "is_default")]
    pub not_from_java_next_long: bool,
    #[serde(default, skip_serializing_if = "is_default")]
    pub error_margin_slime_chunks: u8,
    #[serde(default, skip_serializing_if = "is_default")]
    pub error_margin_slime_chunks_negative: u8,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct TreasureMap {
    pub fragment_x: i64,
    pub fragment_z: i64,
    //pub map: [u8; 128*128],
    pub map: Vec<u8>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SeedStructures {
    #[serde(default, skip_serializing_if = "is_default")]
    pub slime_chunks: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub mineshafts: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub nether_forts: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub strongholds: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub desert_temples: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub jungle_temples: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub witch_huts: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub villages: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub ocean_monuments: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub igloos: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub woodland_mansions: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub end_cities: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub ocean_ruins: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub shipwrecks: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub buried_treasures: Vec<Chunk>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub pillager_outposts: Vec<Chunk>,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
// Serialization is implemented for the SeedInfoCompat struct in order to
// support multiple versions of SeedInfo
#[serde(from = "SeedInfoCompat")]
#[serde(into = "SeedInfoCompat")]
pub struct SeedInfo {
    /// Minecraft version used to generate the world
    pub version: String,
    /// Seed of the world, if known
    pub world_seed: Option<i64>,
    /// Hashed world seed. Starting from Minecraft 1.15, this is sent by the server
    pub world_seed_hash: Option<i64>,
    /// Human readable description of the seed
    pub description: String,
    // Extra settings for optimizing the search: error margin, use extend48
    pub options: Options,
    pub biomes: HashMap<BiomeId, Vec<Point>>,
    pub biomes_quarter_scale: HashMap<BiomeId, Vec<Point4>>,
    pub end_pillars: Vec<u8>,
    pub treasure_maps: Vec<TreasureMap>,
    pub positive: SeedStructures,
    // Coords of structures that do not exist, useful to remove duplicates
    pub negative: SeedStructures,
    // Extra data from sections of the minecraft world generated using a
    // different minecraft version
    pub and: Vec<SeedInfo>,
}

impl SeedInfo {
    pub fn read<P: AsRef<Path>>(filename: P) -> Result<SeedInfo, ReadError> {
        use std::fs::File;
        let file = File::open(filename)?;
        let seed_info = serde_json::from_reader(file)?;

        Ok(seed_info)
    }

    pub fn version(&self) -> Result<MinecraftVersion, String> {
        self.version.parse()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct SeedInfoV0_1 {
    /// Minecraft version used to generate the world
    pub version: String,
    /// Seed of the world, if known
    #[serde(default, skip_serializing_if = "is_default", with = "opt_string")]
    pub world_seed: Option<i64>,
    /// Hashed world seed. Starting from Minecraft 1.15, this is sent by the server
    #[serde(default, skip_serializing_if = "is_default", with = "opt_string")]
    pub world_seed_hash: Option<i64>,
    /// Human readable description of the seed
    #[serde(default, skip_serializing_if = "is_default")]
    pub description: String,
    // Extra settings for optimizing the search: error margin, use extend48
    #[serde(default, skip_serializing_if = "is_default")]
    pub options: Options,
    #[serde(default, skip_serializing_if = "is_default")]
    #[serde(deserialize_with = "deserialize_biomes")]
    pub biomes: HashMap<BiomeId, Vec<Point>>,
    #[serde(default, skip_serializing_if = "is_default")]
    #[serde(deserialize_with = "deserialize_biomes4")]
    pub biomes_quarter_scale: HashMap<BiomeId, Vec<Point4>>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub end_pillars: Vec<u8>,
    #[serde(default, skip_serializing_if = "is_default")]
    pub treasure_maps: Vec<TreasureMap>,
    #[serde(flatten)]
    pub positive: SeedStructures,
    // Coords of structures that do not exist, useful to remove duplicates
    #[serde(default, skip_serializing_if = "is_default")]
    pub negative: SeedStructures,
    // Extra data from sections of the minecraft world generated using a
    // different minecraft version
    #[serde(default, skip_serializing_if = "is_default")]
    pub and: Vec<SeedInfo>,
}

impl From<SeedInfoV0_1> for SeedInfo {
    fn from(s: SeedInfoV0_1) -> SeedInfo {
        SeedInfo {
            version: s.version,
            world_seed: s.world_seed,
            world_seed_hash: s.world_seed_hash,
            description: s.description,
            options: s.options,
            biomes: s.biomes,
            biomes_quarter_scale: s.biomes_quarter_scale,
            end_pillars: s.end_pillars,
            treasure_maps: s.treasure_maps,
            positive: s.positive,
            negative: s.negative,
            and: s.and,
        }
    }
}

impl From<SeedInfo> for SeedInfoV0_1 {
    fn from(s: SeedInfo) -> SeedInfoV0_1 {
        SeedInfoV0_1 {
            version: s.version,
            world_seed: s.world_seed,
            world_seed_hash: s.world_seed_hash,
            description: s.description,
            options: s.options,
            biomes: s.biomes,
            biomes_quarter_scale: s.biomes_quarter_scale,
            end_pillars: s.end_pillars,
            treasure_maps: s.treasure_maps,
            positive: s.positive,
            negative: s.negative,
            and: s.and,
        }
    }
}

#[derive(Debug, PartialEq, Deserialize, Serialize)]
#[serde(tag = "seedInfo")]
enum SeedInfoCompat {
    #[serde(rename = "0.1")]
    V0_1(SeedInfoV0_1),
}

impl SeedInfoCompat {
    // Consume self by converting it into the latest version of the SeedInfo
    // format
    fn convert_to_latest(mut self) -> SeedInfo {
        loop {
            match self {
                // Latest version: we're done
                Self::V0_1(seed_info) => return seed_info.into(),
                // Older version: incrementally convert to latest
                #[allow(unreachable_patterns)]
                _ => self = self.convert_to_next(),
            }
        }
    }
    // Update the version by one step
    // This allows us to support all the existing SeedInfo versions, while only
    // having to write a migration from the second newest version to the newest
    // version
    fn convert_to_next(self) -> Self {
        match self {
            Self::V0_1(seed_info) => Self::V0_1(seed_info),
        }
    }
}

impl From<SeedInfoCompat> for SeedInfo {
    fn from(s: SeedInfoCompat) -> SeedInfo {
        s.convert_to_latest()
    }
}

impl From<SeedInfo> for SeedInfoCompat {
    fn from(s: SeedInfo) -> SeedInfoCompat {
        SeedInfoCompat::V0_1(s.into())
    }
}

fn deserialize_biomes<'de, D>(d: D) -> Result<HashMap<BiomeId, Vec<Point>>, D::Error> where D: Deserializer<'de> {
    let biomes = HashMap::<String, Vec<Point>>::deserialize(d)?;
    Ok(biomes.into_iter().map(|(k, v)| (k.parse().unwrap(), v)).collect())
}

fn deserialize_biomes4<'de, D>(d: D) -> Result<HashMap<BiomeId, Vec<Point4>>, D::Error> where D: Deserializer<'de> {
    let biomes = HashMap::<String, Vec<Point4>>::deserialize(d)?;
    Ok(biomes.into_iter().map(|(k, v)| (k.parse().unwrap(), v)).collect())
}

pub fn biomes_to_map<I>(biomes: I) -> Map
where
    I: IntoIterator<Item = (BiomeId, Vec<Point>)>
{
    let h: Vec<_> = biomes.into_iter().flat_map(|(k, v)| v.into_iter().map(move |x| (x, k))).collect();
    let area = Area::from_coords(h.iter().map(|x| x.0));
    let mut m = Map::new(area);
    for (Point {x, z}, biome_id) in h {
        m.a[((x - area.x) as usize, (z - area.z) as usize)] = biome_id.0;
    }
    m
}

pub fn biomes_from_map(map: &Map) -> HashMap<BiomeId, Vec<Point>> {
    let mut biomes: HashMap<BiomeId, Vec<Point>> = HashMap::new();

    let area = map.area();
    for z in 0..area.h as usize {
        for x in 0..area.w as usize {
            let biome = BiomeId(map.a[(x, z)]);
            biomes.entry(biome).or_default().push(Point { x: x as i64 + area.x, z: z as i64 + area.z });
        }
    }

    biomes
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

impl Into<Point> for PointSerialization {
    fn into(self) -> Point {
        match self {
            PointSerialization::Normal { x, z } => Point { x, z },
            PointSerialization::Tuple((x, z)) => Point { x, z },
        }
    }
}

impl From<Point> for PointSerialization {
    fn from(c: Point) -> Self {
        PointSerialization::Tuple((c.x, c.z))
    }
}

impl<'de> Deserialize<'de> for Point {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(PointSerialization::deserialize(deserializer)?.into())
    }
}

impl Serialize for Point {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let x = PointSerialization::from(*self);
        x.serialize(serializer)
    }
}

impl Into<Point4> for PointSerialization {
    fn into(self) -> Point4 {
        match self {
            PointSerialization::Normal { x, z } => Point4 { x, z },
            PointSerialization::Tuple((x, z)) => Point4 { x, z },
        }
    }
}

impl From<Point4> for PointSerialization {
    fn from(c: Point4) -> Self {
        PointSerialization::Tuple((c.x, c.z))
    }
}

impl<'de> Deserialize<'de> for Point4 {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(PointSerialization::deserialize(deserializer)?.into())
    }
}

impl Serialize for Point4 {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let x = PointSerialization::from(*self);
        x.serialize(serializer)
    }
}

// https://www.mth.st/blog/skip-default/
fn is_default<T: Default + PartialEq>(t: &T) -> bool {
        t == &T::default()
}

// https://github.com/serde-rs/json/issues/329#issuecomment-305608405
mod opt_string {
    use std::fmt::Display;
    use std::str::FromStr;

    use serde::{de, Serializer, Deserialize, Deserializer};

    pub fn serialize<T, S>(value: &Option<T>, serializer: S) -> Result<S::Ok, S::Error>
        where T: Display,
              S: Serializer
    {
        if let Some(value) = value {
            serializer.collect_str(value)
        } else {
            // Serialize None as empty string
            serializer.collect_str("")
        }
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<T>, D::Error>
        where T: FromStr,
              T::Err: Display,
              D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;

        if s.is_empty() {
            // Deserialize empty string as None
            Ok(None)
        } else {
            s.parse().map_err(de::Error::custom).map(Some)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialize_default() {
        let seed_info = SeedInfo {
            version: "1.7".to_string(),
            ..Default::default()
        };
        let x = serde_json::to_string(&seed_info).unwrap();
        // Version field must be serialized!
        assert_eq!(x, r#"{"seedInfo":"0.1","version":"1.7"}"#);
    }

    #[test]
    fn serialize_compat() {
        let seed_info = SeedInfoCompat::V0_1(SeedInfo {
            version: "1.7".to_string(),
            ..Default::default()
        }.into());
        let x = serde_json::to_string(&seed_info).unwrap();
        // Version field must be serialized!
        assert_eq!(x, r#"{"seedInfo":"0.1","version":"1.7"}"#);
    }

    #[test]
    fn deserialize_no_seed_info_version() {
        let json = r#"{
            "version": "1.7",
            "description": "I have no seedInfo field"
        }"#;
        // This should fail
        assert!(serde_json::from_str::<SeedInfo>(json).is_err());
    }

    #[test]
    fn deserialize_slime_chunks() {
        let json = r#"{
            "seedInfo": "0.1",
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
            "seedInfo": "0.1",
            "version": "1.7",
            "biomes": {
                "7": [[0, 0], [2, 2]]
            }
        }"#;

        let seed_info: SeedInfo = serde_json::from_str(json).unwrap();
        assert_eq!(seed_info.version, "1.7".to_string());
        assert_eq!(seed_info.biomes[&BiomeId(7)], vec![Point { x: 0, z: 0 }, Point { x: 2, z: 2 }]);
    }

    #[test]
    fn world_seed_string() {
        let json = r#"{
            "seedInfo": "0.1",
            "version": "1.7",
            "worldSeed": "1234"
        }"#;

        let seed_info: SeedInfo = serde_json::from_str(json).unwrap();
        assert_eq!(seed_info.world_seed, Some(1234));
    }

    #[test]
    fn world_seed_empty_string() {
        let json = r#"{
            "seedInfo": "0.1",
            "version": "1.7",
            "worldSeed": ""
        }"#;

        let seed_info: SeedInfo = serde_json::from_str(json).unwrap();
        assert_eq!(seed_info.world_seed, None);
    }

    #[test]
    fn trim_version_str() {
        assert_eq!(trim_at_dot(2, ""), "");
        assert_eq!(trim_at_dot(2, "1"), "1");
        assert_eq!(trim_at_dot(2, "1."), "1.");
        assert_eq!(trim_at_dot(2, "1.2"), "1.2");
        assert_eq!(trim_at_dot(2, "1.2.3"), "1.2");
        assert_eq!(trim_at_dot(2, "1.2.."), "1.2");
        assert_eq!(trim_at_dot(2, "1.2..3"), "1.2");

        assert_eq!(trim_at_dot(2, "."), ".");
        assert_eq!(trim_at_dot(2, ".."), ".");
        assert_eq!(trim_at_dot(2, "..."), ".");
    }

    #[test]
    fn parse_1_16_version() {
        assert_eq!(MinecraftVersion::from_str("1.16"), Ok(MinecraftVersion::Java1_16));
        assert_eq!(MinecraftVersion::from_str("1.16.1"), Ok(MinecraftVersion::Java1_16_1));
        assert_eq!(MinecraftVersion::from_str("1.16.2"), Ok(MinecraftVersion::Java1_16));
        assert_eq!(MinecraftVersion::from_str("1.16.3"), Ok(MinecraftVersion::Java1_16));
    }
}
