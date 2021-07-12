use structopt::StructOpt;
use std::path::PathBuf;
use slime_seed_finder::*;
use slime_seed_finder::anvil::ZipChunkProvider;
use slime_seed_finder::biome_info::biome_id;
use slime_seed_finder::biome_layers;
use slime_seed_finder::biome_layers::Area;
use slime_seed_finder::biome_layers::Map;
use slime_seed_finder::chunk::Chunk;
use slime_seed_finder::chunk::Point;
use slime_seed_finder::slime::generate_slime_chunks_and_not;
use slime_seed_finder::slime::seed_from_slime_chunks;
use slime_seed_finder::slime::seed_from_slime_chunks_and_candidates;
use slime_seed_finder::seed_info::biomes_from_map;
use slime_seed_finder::seed_info::BiomeId;
use slime_seed_finder::seed_info::MinecraftVersion;
use slime_seed_finder::seed_info::SeedInfo;
use slime_seed_finder::java_rng::JavaRng;
use slime_seed_finder::population::MossyFloor;
use std::fs::File;
use std::fs;
use std::path::Path;
use std::io::Write;
use std::ffi::OsStr;
use std::thread;
use std::convert::{TryFrom, TryInto};
use std::sync::Arc;
use std::time::Instant;
use log::*;
#[cfg(feature = "rand")]
use rand::{thread_rng, Rng as _};

// This is needed because the getrandom crate uses a different version of wasi
// https://github.com/bytecodealliance/wasi/issues/37
#[cfg(not(feature = "rand"))]
use wasm_rand::*;
#[cfg(not(feature = "rand"))]
mod wasm_rand {
    pub struct MockRng;

    pub fn thread_rng() -> MockRng {
        MockRng
    }

    impl MockRng {
        pub fn gen<T>(&mut self) -> T {
            panic!("Random number generator not available in this platform")
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "slime_seed_finder", rename_all = "kebab-case")]
enum Opt {
    /// Generate a SeedInfo.
    /// Used for testing.
    #[structopt(name = "generate")]
    Generate {
        /// The seed for which to generate the SeedInfo.
        /// To avoid problems with negative seeds, use the following syntax:
        /// -s=-1234 or --seed=-1234
        /// If left blank, will generate a random seed and print it on stderr.
        #[structopt(short = "s", long)]
        seed: Option<i64>,
        /// When creating a Minecraft world, if the seed field is left blank,
        /// the seed will be generated randomly using (new Random()).nextLong(),
        /// which uses only 48 bits of entropy. With this flag, 64 bits of
        /// entropy will be used, allowing to explore the complete seed space.
        #[structopt(long, conflicts_with = "seed")]
        seed_not_from_java_next_long: bool,
        #[structopt(long, default_value = "0")]
        num_slime_chunks: usize,
        #[structopt(long, default_value = "0")]
        num_non_slime_chunks: usize,
        /// Output file. If unspecified, defaults to stdout so the output of
        /// this program can be saved into a file.
        #[structopt(short = "o", long, parse(from_os_str))]
        output_file: Option<PathBuf>,
        /// Add biome information to generated SeedInfo.
        /// This option controls the size of the map. The top-left corner of
        /// the map will be 0,0 unless changed by using the biome_map_x and
        /// biome_map_z options.
        #[structopt(long, default_value = "0")]
        biome_map_size: u64,
        /// Lowest x coordinate in the biome map.
        #[structopt(long, default_value = "0")]
        biome_map_x: i64,
        /// Lowest z coordinate in the biome map.
        #[structopt(long, default_value = "0")]
        biome_map_z: i64,
        /// Minecraft version to use (Java edition).
        /// Supported values: from 1.3 to 1.16
        #[structopt(long)]
        mc_version: String,
    },

    #[structopt(name = "interactive")]
    Interactive {
        /// File containing the SeedInfo
        #[structopt(short = "i", long, parse(from_os_str))]
        input_file: Option<PathBuf>,
    },

    /// Use slime chunks to find the seed. In the future this will use a
    /// combination of all the methods, but not yet.
    #[structopt(name = "find")]
    Find {
        /// File containing the SeedInfo
        #[structopt(short = "i", long, parse(from_os_str))]
        input_file: PathBuf,
        /// File containing a JSON array of all the candidate seeds: so instead
        /// of bruteforcing all the possible seeds we only try the ones from
        /// this file.
        #[structopt(long, parse(from_os_str))]
        candidate_seeds: Option<PathBuf>,
        /// Where to write the found seeds as a JSON array
        #[structopt(short = "o", long, parse(from_os_str))]
        output_file: Option<PathBuf>,
    },

    /// Use rivers and biomes to find the seed
    #[structopt(name = "rivers")]
    Rivers {
        /// File containing the SeedInfo
        #[structopt(short = "i", long, parse(from_os_str))]
        input_file: PathBuf,
        /// Where to write the found seeds as a JSON array
        #[structopt(short = "o", long, parse(from_os_str))]
        output_file: Option<PathBuf>,
    },

    /// Use rivers from an unexplored treasure map to find the seed
    #[structopt(name = "treasure-rivers")]
    TreasureRivers {
        /// File containing the SeedInfo
        #[structopt(short = "i", long, parse(from_os_str))]
        input_file: PathBuf,
        /// Where to write the found seeds as a JSON array
        #[structopt(short = "o", long, parse(from_os_str))]
        output_file: Option<PathBuf>,
    },

    #[structopt(name = "extend48")]
    Extend48 {
        /// File containing the list of 48-bit seeds as a JSON array
        #[structopt(short = "i", long, parse(from_os_str))]
        input_file: PathBuf,
        /// Where to write the extended seeds as a JSON array
        #[structopt(short = "o", long, parse(from_os_str))]
        output_file: Option<PathBuf>,
    },

    /// Generate a biome map
    #[structopt(name = "rendermap")]
    RenderMap {
        /// The seed for which to generate the biome map.
        /// To avoid problems with negative seeds, use the following syntax:
        /// -s=-1234 or --seed=-1234
        #[structopt(short = "s", long)]
        seed: i64,
        /// x position of the top-left coordinate of the map
        /// To avoid problems with negative coordinates, use the following
        /// syntax: -x=-2
        #[structopt(short = "x", default_value = "0")]
        x: i64,
        /// z position of the top-left coordinate of the map
        #[structopt(short = "z", default_value = "0")]
        z: i64,
        /// width
        #[structopt(short = "w", long, default_value = "1024")]
        width: u32,
        /// height
        #[structopt(short = "h", long, default_value = "640")]
        height: u32,
        /// Output filename. Defaults to biome_map_<seed>_x_z_wxh.png.
        /// Supported image formats: jpeg, png, ico, pnm, bmp and tiff.
        #[structopt(short = "o", long, parse(from_os_str))]
        output_file: Option<PathBuf>,
        /// Minecraft version to use (Java edition).
        /// Supported values: from 1.3 to 1.16
        #[structopt(long)]
        mc_version: String,
        /// The last layer to generate. Defaults to the latest one (full
        /// resolution biome map).
        #[structopt(long)]
        last_layer: Option<u32>,
    },

    /// Generate an unexplored treasure map, but without the treasure marker.
    #[structopt(name = "treasure")]
    Treasure {
        /// The seed for which to generate the treasure map.
        /// To avoid problems with negative seeds, use the following syntax:
        /// -s=-1234 or --seed=-1234
        #[structopt(short = "s", long)]
        seed: i64,
        /// x position of the map as "fragment" coordinate.
        /// The formula to convert between fragment coordinates and
        /// top-left-corner-of-the-map coordinates is:
        /// x = fragment_x * 256 - 64
        /// To avoid problems with negative coordinates, use the following
        /// syntax: -x=-2 or --fragment-x=-2
        #[structopt(short = "x", long)]
        fragment_x: i64,
        /// z position of the map as "fragment" coordinate.
        #[structopt(short = "z", long)]
        fragment_z: i64,
        /// Output filename. Defaults to treasure_map_<seed>_x_z.png.
        /// Supported image formats: jpeg, png, ico, pnm, bmp and tiff.
        #[structopt(short = "o", long, parse(from_os_str))]
        output_file: Option<PathBuf>,
        /// Minecraft version to use (Java edition).
        /// Supported values: from 1.3 to 1.16
        #[structopt(long)]
        mc_version: String,
    },

    /// Read a minecraft region file and try to find its seed
    #[structopt(name = "anvil")]
    Anvil {
        /// Path to "minecraft_saved_world/region"
        #[structopt(short = "i", long, parse(from_os_str))]
        input_dir: PathBuf,
        /// Where to write the found seeds as a JSON array
        #[structopt(short = "o", long, parse(from_os_str))]
        output_file: Option<PathBuf>,
        /// Number of threads to use. By default, same as number of CPUs
        #[structopt(short = "j", long, default_value = "0")]
        threads: usize,
        /// Center x coordinate around which to look for rivers
        #[structopt(long, default_value = "0")]
        center_x: i64,
        /// Center z coordinate around which to look for rivers
        #[structopt(long, default_value = "0")]
        center_z: i64,
        /// Minecraft version to use (Java edition).
        /// Supported values: from 1.3 to 1.16
        #[structopt(long)]
        mc_version: String,
    },

    /// Read a minecraft region file and try to find its seed
    #[structopt(name = "anvil-zip")]
    AnvilZip {
        /// Path to "minecraft_saved_world.zip"
        #[structopt(short = "i", long, parse(from_os_str))]
        input_zip: PathBuf,
        /// Where to write the found seeds as a JSON array
        #[structopt(short = "o", long, parse(from_os_str))]
        output_file: Option<PathBuf>,
        /// Number of threads to use. By default, same as number of CPUs
        #[structopt(short = "j", long, default_value = "0")]
        threads: usize,
        /// Center x coordinate around which to look for rivers
        #[structopt(long, default_value = "0")]
        center_x: i64,
        /// Center z coordinate around which to look for rivers
        #[structopt(long, default_value = "0")]
        center_z: i64,
        /// Minecraft version to use (Java edition).
        /// Supported values: from 1.3 to 1.16
        #[structopt(long)]
        mc_version: String,
    },

    /// Bruteforce world seed hash
    #[structopt(name = "bruteforce-seed-hash")]
    BruteforceSeedHash {
        /// Seed hash as 64-bit signed integer.
        /// To avoid problems with negative seeds, use the following
        /// syntax: --seed-hash=-2
        #[structopt(long)]
        seed_hash: i64,
        /// When creating a Minecraft world, if the seed field is left blank,
        /// the seed will be generated randomly using (new Random()).nextLong(),
        /// which uses only 48 bits of entropy. With this flag, 64 bits of
        /// entropy will be used, allowing to explore the complete seed space.
        #[structopt(long)]
        seed_not_from_java_next_long: bool,
        /// Path to file containing a list of 26-bit candidates
        #[structopt(long)]
        candidates_file: Option<PathBuf>,
    },

    /// Given a seed, calculate the seed hash
    #[structopt(name = "seed-hash")]
    SeedHash {
        /// Seed as 64-bit signed integer.
        /// To avoid problems with negative seeds, use the following
        /// syntax: --seed=-2
        #[structopt(long)]
        seed: i64,
    },

    /// Given the coordinates and floor of a dungeon, calculate the corresponding dungeon seed.
    /// The benefit of this command is that it works on any minecraft version since dungeons were
    /// first introduced.
    /// The dungeon seed is not the world seed, you will need to use dungeon-seed-to-world-seed
    /// with the output of this command.
    #[structopt(name = "dungeon-seed")]
    DungeonSeed {
        #[structopt(short="x", long)]
        spawner_x: i64,
        #[structopt(short="y", long)]
        spawner_y: i64,
        /// Coordinates of the dungeon spawner.
        ///
        /// Use the "looking at block" section from the F3 screen. To avoid problems with negative
        /// coordinates, use the following syntax: --spawner-z=-2
        #[structopt(short="z", long)]
        spawner_z: i64,
        /// Block layout of the dungeon floor. The orientation should be the most negative
        /// coordinate at the top right.
        ///
        /// C: cobblestone
        /// M: mossy cobblestone
        /// A: air
        /// ?: unknown
        /// ; next row
        ///
        /// Remember to include the floor that is below the walls, or
        /// add a margin of "?".
        ///
        /// Example:
        ///
        /// "MMMMCMM;M?????M;C?????M;M?????M;C?????M;M?????C;CMMMMMM;"
        ///
        #[structopt(short="f", long)]
        floor: String,
        /// Number of threads to use. By default, same as number of CPUs
        #[structopt(short = "j", long, default_value = "0")]
        threads: usize,
    },

    /// Given 3 dungeon seeds in the format "159,23,-290,982513219448", find the world seed.
    /// To avoid problems with negative seeds, use -- before the arguments: slime_seed_finder
    /// dungeon-seed-to-world-seed -- "159,23,-290,982513219448"
    #[structopt(name = "dungeon-seed-to-world-seed")]
    DungeonSeedToWorldSeed {
        /// Maximum number of calls to rng.previous(). Try increasing this value if the seed could
        /// not be found. You can also set a different limit for one particular dungeon by
        /// appending the limit to the dungeon seed: "159,23,-290,982513219448,1500" will have l=1500
        #[structopt(short = "l", long, default_value = "128")]
        limit_steps_back: u32,
        dungeon_seeds: Vec<String>,
    },

    /// Read a minecraft world, read its seed, generate biome map using the
    /// same seed, and compare both worlds
    #[structopt(name = "test-generation")]
    TestGeneration {
        /// Path to "minecraft_saved_world.zip"
        #[structopt(short = "i", long, parse(from_os_str))]
        input_zip: PathBuf,
        /// Minecraft version to use (Java edition).
        /// Supported values: from 1.3 to 1.16
        #[structopt(long)]
        mc_version: String,
        /// Render biome map from the biomes according to the saved world
        #[structopt(long)]
        draw_biome_map: bool,
    },

    /// Read a minecraft world and find all the already generated dungeons
    #[structopt(name = "read-dungeons")]
    ReadDungeons {
        /// Path to "minecraft_saved_world.zip"
        #[structopt(short = "i", long, parse(from_os_str))]
        input_zip: PathBuf,
        /// Minecraft version to use (Java edition).
        /// Supported values: 1.16
        #[structopt(long)]
        mc_version: String,
    },

    /// Read a list of candidate seeds from a file and a list of biomes from a seedInfo and write
    /// the matching seeds to a file
    #[structopt(name = "filter-biomes")]
    FilterBiomes {
        /// File containing the SeedInfo
        #[structopt(short = "i", long, parse(from_os_str))]
        input_file: PathBuf,
        /// File containing a JSON array of all the candidate seeds: so instead
        /// of bruteforcing all the possible seeds we only try the ones from
        /// this file.
        #[structopt(long, parse(from_os_str))]
        candidate_seeds: PathBuf,
        /// Where to write the found seeds as a JSON array
        #[structopt(short = "o", long, parse(from_os_str))]
        output_file: Option<PathBuf>,
    }
}

fn main() {
    pretty_env_logger::init();

    match Opt::from_args() {
        Opt::Generate {
            seed,
            mut seed_not_from_java_next_long,
            num_slime_chunks,
            num_non_slime_chunks,
            output_file,
            biome_map_size,
            biome_map_x,
            biome_map_z,
            mc_version,
        } => {
            if let Some(seed) = seed {
                // Sorry for the double negation here
                if !seed_not_from_java_next_long {
                    if JavaRng::create_from_long(seed as u64).is_none() {
                        eprintln!("Warning: this seed cannot be generated with Java Random nextLong");
                        seed_not_from_java_next_long = true;
                    }
                }
            }

            let seed = seed.unwrap_or_else(|| {
                if seed_not_from_java_next_long {
                    thread_rng().gen()
                } else {
                    JavaRng::with_seed(thread_rng().gen()).next_long()
                }
            });

            eprintln!("Seed: {}", seed);

            let (c, nc) = generate_slime_chunks_and_not(seed, num_slime_chunks, num_non_slime_chunks);
            let area = Area { x: biome_map_x, z: biome_map_z, w: biome_map_size, h: biome_map_size };
            let biome_map = biome_layers::generate(mc_version.parse().unwrap(), area, seed);

            let mut seed_info = SeedInfo::default();
            seed_info.version = mc_version.to_string();
            seed_info.options.not_from_java_next_long = seed_not_from_java_next_long;
            seed_info.positive.slime_chunks = c;
            seed_info.negative.slime_chunks = nc;
            seed_info.biomes = biomes_from_map(&biome_map);

            let buf = serde_json::to_string(&seed_info).expect("Serialization fail");

            println!("{}", buf);
            if let Some(output_file) = output_file {
                // TODO: proper error handling
                let mut w = File::create(output_file).unwrap();
                write!(w, "{}", buf).unwrap();
            }
        }
        Opt::Interactive {
            ..
        } => {
            println!("Some day you will be able to specify all the options here interactively");
            println!("But not today, sorry");
            unimplemented!()
        }

        Opt::Find {
            input_file,
            candidate_seeds,
            output_file,
        } => {
            let seed_info = SeedInfo::read(input_file).expect("Error reading seed info");
            let c = seed_info.positive.slime_chunks;
            let nc = seed_info.negative.slime_chunks;
            let false_c = seed_info.options.error_margin_slime_chunks as usize;
            let false_nc = seed_info.options.error_margin_slime_chunks_negative as usize;

            if c.is_empty() && nc.is_empty() {
                // Can't find seed without slime chunks
                println!("Not enough slime chunks");
                return;
            }

            // All possible 48 bit seeds
            let seeds = if let Some(path) = candidate_seeds {
                let candidates = read_seeds_from_file(path).expect("Error reading candidates");
                seed_from_slime_chunks_and_candidates(&c, false_c, &nc, false_nc, candidates)
            } else {
                seed_from_slime_chunks(&c, false_c, &nc, false_nc)
            };
            println!("Found {} 48-bit seeds:\n{}", seeds.len(), serde_json::to_string(&seeds).unwrap());

            let java = !seed_info.options.not_from_java_next_long;

            if java {
                // Display only 64 bit seeds that could be generated by java
                // (when the seed box is left empty)
                let mut java_seeds: Vec<i64> = seeds
                    .iter()
                    .flat_map(|&s| JavaRng::extend_long_48(s))
                    .map(|s| s as i64)
                    .collect();

                java_seeds.sort_unstable();
                println!("Java seeds: {}\n{:#?}", java_seeds.len(), java_seeds);
                if let Some(of) = output_file {
                    // TODO: proper error handling
                    write_seeds_to_file(&java_seeds, of).expect("Error writing seeds to file");
                }
            } else {
                if let Some(of) = output_file {
                    // TODO: proper error handling
                    write_seeds_to_file(&seeds.into_iter().map(|x| x as i64).collect::<Vec<_>>(), of).expect("Error writing seeds to file");
                }
            }
        }

        Opt::Rivers {
            input_file,
            output_file,
        } => {
            let seed_info = SeedInfo::read(input_file).expect("Error reading seed info");
            let version = seed_info.version.parse().unwrap();
            // TODO: integrate the river seed finder into the "find" subcommand
            let extra_biomes: Vec<_> = seed_info.biomes.iter().flat_map(|(id, vec_xz)| {
                if *id == BiomeId(biome_id::river) {
                    vec![]
                } else {
                    vec_xz.iter().map(|p| (*id, *p)).collect()
                }
            }).collect();

            // All possible 64 bit seeds
            let seeds = if let Some(rivers) = seed_info.biomes.get(&BiomeId(biome_id::river)) {
                biome_layers::river_seed_finder(rivers, &extra_biomes, version)
            } else {
                error!("No rivers in seedInfo");
                vec![]
            };

            println!("Found {} 64-bit seeds:\n{}", seeds.len(), serde_json::to_string(&seeds).unwrap());

            if let Some(of) = output_file {
                write_seeds_to_file(&seeds, of).expect("Error writing seeds to file");
            }
        }

        Opt::TreasureRivers {
            input_file,
            output_file,
        } => {
            let seed_info = SeedInfo::read(input_file).expect("Error reading seed info");
            let version = seed_info.version.parse().unwrap();
            // TODO: integrate the treasure map river seed finder into the "find" subcommand
            let first_treasure_map = &seed_info.treasure_maps[0];

            let mut pmap = Map::new(Area { x: (-64 + 256 * first_treasure_map.fragment_x) / 2, z: (-64 + 256 * first_treasure_map.fragment_z) / 2, w: 128, h: 128 });
            for (i, v) in first_treasure_map.map.iter().enumerate() {
                let (x, z) = (i % 128, i / 128);
                pmap.a[(x, z)] = match v {
                    0 => biome_id::ocean,
                    1 => biome_id::plains,
                    2 => biome_id::river,
                    // Unknown biome
                    255 => 255,
                    _ => panic!("Invalid id: {}", v),
                };
            }

            // All possible 26 bit seeds
            let seeds = biome_layers::treasure_map_river_seed_finder(&pmap, version, 0, 1 << 24);
            println!("Found {} 26-bit seeds:\n{}", seeds.len(), serde_json::to_string(&seeds).unwrap());

            if let Some(of) = output_file {
                write_seeds_to_file(&seeds, of).expect("Error writing seeds to file");
            }
        }

        Opt::Extend48 {
            input_file,
            output_file,
        } => {
            let seeds = read_seeds_from_file(input_file).expect("Error reading input file");
            let mut r = vec![];
            for s in seeds {
                if !(s < (1u64 << 48)) {
                    panic!("Input must be lower than 2^48");
                };

                let rr = JavaRng::extend_long_48(s);
                r.extend(rr.into_iter().map(|seed| seed as i64));
            }

            println!("{}", serde_json::to_string_pretty(&r).unwrap());

            if let Some(output_file) = output_file {
                write_seeds_to_file(&r, output_file).expect("Error writing seeds to file");
            }
        }

        Opt::RenderMap {
            seed,
            x,
            z,
            width,
            height,
            output_file,
            mc_version,
            last_layer,
        } => {
            let output_file = output_file.unwrap_or_else(|| {
                format!("biome_map_{}_{}_{}_{}_{}x{}.png", mc_version, seed, x, z, width, height).into()
            });
            let version: MinecraftVersion = mc_version.parse().unwrap();
            let last_layer = last_layer.unwrap_or_else(|| version.num_layers());
            let area = Area { x, z, w: width as u64, h: height as u64 };
            let vec_rgba = biome_layers::generate_image_up_to_layer(version, area, seed, last_layer);
            assert_eq!(vec_rgba.len(), (width * height * 4) as usize);
            image::save_buffer(output_file.clone(), &vec_rgba, width, height, image::ColorType::Rgba8).unwrap();
            println!("Saved image to {}", output_file.to_string_lossy());
        }

        Opt::Treasure {
            seed,
            fragment_x,
            fragment_z,
            output_file,
            mc_version,
        } => {
            let mc_version: MinecraftVersion = mc_version.parse().unwrap();
            let output_file = output_file.unwrap_or_else(|| {
                format!("treasure_map_{}_{}_{}.png", seed, fragment_x, fragment_z).into()
            });
            let vec_rgba = biome_layers::generate_image_treasure_map_at(mc_version, seed, fragment_x, fragment_z);
            assert_eq!(vec_rgba.len(), 128 * 128 * 4);
            image::save_buffer(output_file.clone(), &vec_rgba, 128, 128, image::ColorType::Rgba8).unwrap();
            println!("Saved image to {}", output_file.to_string_lossy());
        }

        Opt::Anvil {
            input_dir,
            output_file,
            threads,
            center_x,
            center_z,
            mc_version,
        } => {
            if input_dir.file_name() != Some(OsStr::new("region")) {
                println!(r#"Error: input dir must end with "/region""#);
                return;
            }
            let version = mc_version.parse().unwrap();

            let (rivers, extra_biomes) = anvil::get_rivers_and_some_extra_biomes_folder(&input_dir, Point { x: center_x, z: center_z });
            let rivers = Arc::new(rivers);
            let extra_biomes = Arc::new(extra_biomes);
            let num_threads = if threads == 0 { num_cpus::get() } else { threads };

            let total_range = 1u32 << 24;
            let thread_range = total_range / u32::try_from(num_threads).unwrap();

            let seeds: Vec<String> = run_threads(num_threads, move |thread_id| {
                let range_lo = thread_range * u32::try_from(thread_id).unwrap();
                let range_hi = if thread_id + 1 == num_threads {
                    total_range
                } else {
                    thread_range * u32::try_from(thread_id + 1).unwrap()
                };
                debug!("Spawning thread {} from {:X} to {:X}", thread_id, range_lo, range_hi);
                let r = biome_layers::river_seed_finder_range(&rivers, &extra_biomes, version, range_lo, range_hi);
                debug!("Thread {} finished", thread_id);

                r
            }).unwrap().into_iter().flat_map(|x| x).map(|seed| format!("{:016X}", seed)).collect();
            println!("Found {} 64-bit seeds:\n{}", seeds.len(), serde_json::to_string(&seeds).unwrap());

            if let Some(of) = output_file {
                write_candidates_to_file(&seeds, of).expect("Error writing seeds to file");
            }
        }

        Opt::AnvilZip {
            input_zip,
            output_file,
            threads,
            center_x,
            center_z,
            mc_version,
        } => {
            let version = mc_version.parse().unwrap();

            if version >= MinecraftVersion::Java1_15 {
                let (rivers, _extra_biomes) = anvil::get_rivers_and_some_extra_biomes_zip_1_15(&input_zip, Point { x: center_x, z: center_z });

                {
                    // Save the extracted data as a SeedInfo
                    // So we can use it later for tests
                    let mut seed_info = SeedInfo::default();
                    seed_info.biomes_quarter_scale.insert(BiomeId(7), rivers.clone());
                    seed_info.version = mc_version.to_string();
                    seed_info.options.not_from_java_next_long = false;

                    // TODO: proper error handling
                    let buf = serde_json::to_string(&seed_info).expect("Serialization fail");
                    fs::write("seedinfo_latest.json", buf).expect("Failed to write seedinfo");
                }
                let rivers = Arc::new(rivers);
                let num_threads = if threads == 0 { num_cpus::get() } else { threads };

                let total_range = 1u32 << 24;
                let thread_range = total_range / u32::try_from(num_threads).unwrap();

                let seeds: Vec<String> = run_threads(num_threads, move |thread_id| {
                    let range_lo = thread_range * u32::try_from(thread_id).unwrap();
                    let range_hi = if thread_id + 1 == num_threads {
                        total_range
                    } else {
                        thread_range * u32::try_from(thread_id + 1).unwrap()
                    };
                    debug!("Spawning thread {} from {:X} to {:X}", thread_id, range_lo, range_hi);
                    let r = biome_layers::river_seed_finder_26_range(&rivers, range_lo, range_hi);
                    debug!("Thread {} finished", thread_id);

                    r
                }).unwrap().into_iter().flat_map(|x| x).map(|seed| format!("{:07X}", seed)).collect();
                // TODO: candidates and seeds should always be serialized in hex, as JSON does not
                // support 64-bit integers
                println!("Found {} 26-bit candidates:\n{}", seeds.len(), serde_json::to_string(&seeds).unwrap());

                if let Some(of) = output_file {
                    // TODO: define structure of candidates file
                    write_candidates_to_file(&seeds, of).expect("Error writing seeds to file");
                }

                println!("You can now use the seed hash to bruteforce the remaining bits");
                return;
            }

            let (rivers, extra_biomes) = anvil::get_rivers_and_some_extra_biomes_zip(&input_zip, Point { x: center_x, z: center_z });

            // TODO: this logic is duplicated in slime_seed_finder_web/src/main.rs
            {
                // Save the extracted data as a SeedInfo
                // So we can use it later for tests
                let mut seed_info = SeedInfo::default();
                seed_info.biomes.insert(BiomeId(7), rivers.clone());
                seed_info.version = mc_version.to_string();
                seed_info.options.not_from_java_next_long = false;

                for (b_id, b_coords) in extra_biomes.iter().cloned() {
                    // Adding more rivers here breaks bounding box detection...
                    if b_id != BiomeId(7) {
                        seed_info.biomes.entry(b_id).or_default().push(b_coords);
                    }
                }

                // TODO: proper error handling
                let buf = serde_json::to_string(&seed_info).expect("Serialization fail");
                fs::write("seedinfo_latest.json", buf).expect("Failed to write seedinfo");
            }

            let rivers = Arc::new(rivers);
            let extra_biomes = Arc::new(extra_biomes);
            let num_threads = if threads == 0 { num_cpus::get() } else { threads };

            let total_range = 1u32 << 24;
            let thread_range = total_range / u32::try_from(num_threads).unwrap();

            let seeds: Vec<String> = run_threads(num_threads, move |thread_id| {
                let range_lo = thread_range * u32::try_from(thread_id).unwrap();
                let range_hi = if thread_id + 1 == num_threads {
                    total_range
                } else {
                    thread_range * u32::try_from(thread_id + 1).unwrap()
                };
                debug!("Spawning thread {} from {:X} to {:X}", thread_id, range_lo, range_hi);
                let r = biome_layers::river_seed_finder_range(&rivers, &extra_biomes, version, range_lo, range_hi);
                debug!("Thread {} finished", thread_id);

                r
            }).unwrap().into_iter().flat_map(|x| x).map(|seed| format!("{:016X}", seed as u64)).collect();
            println!("Found {} 64-bit seeds:\n{}", seeds.len(), serde_json::to_string(&seeds).unwrap());

            if let Some(of) = output_file {
                write_candidates_to_file(&seeds, of).expect("Error writing seeds to file");
            }
        }

        Opt::BruteforceSeedHash {
            seed_hash,
            seed_not_from_java_next_long,
            candidates_file,
        } => {
            fn print_progress_since(start: &Instant, iter_done: u64, iter_total: u64, tried_seeds: u64) {
                let duration = start.elapsed();
                let eta = duration.as_secs_f64() / (iter_done as f64) * ((iter_total - iter_done) as f64);
                let eta_hours = (eta / 3600.0).round();
                let eta_msg = if eta_hours < 2.0 {format!("{} minutes", (eta / 60.0).round())} else {format!("{} hours", eta_hours)};
                let mut msg = format!("Not found. Tried {} seeds. ETA {}", tried_seeds, eta_msg);
                msg.push_str("                                                  ");
                msg.truncate(70);
                print!("{}\r", msg);
            }

            if let Some(candidates_path) = candidates_file {
                let candidates = read_candidates_from_file(candidates_path).unwrap();
                let start = Instant::now();
                let iter = 0..=u32::max_value();
                let total_iter_len = iter.size_hint().1.unwrap_or(u64::max_value() as usize);
                for range_lo in iter {
                    //let found_seed = biome_layers::seed_hash_bruteforce_26_range(seed_hash, &candidates, range_lo, range_lo);
                    let found_seed = if seed_not_from_java_next_long {
                        biome_layers::seed_hash_bruteforce_26_range(seed_hash, &candidates, range_lo, range_lo)
                    } else {
                        biome_layers::seed_hash_bruteforce_26_java_range(seed_hash, &candidates, range_lo, range_lo)
                    };
                    if let Some(seed) = found_seed {
                        println!("\nFound seed: {}", seed);
                        return;
                    }

                    if (range_lo & ((1 << if seed_not_from_java_next_long { 10 } else { 18 }) - 1)) == 0 {
                        let tried_seeds = (range_lo as u64) * (1 << 6) * (candidates.len() as u64);
                        print_progress_since(&start, range_lo as u64, total_iter_len as u64, tried_seeds);
                    }
                }

                println!("\nZero seeds found");
            } else {
                println!("Warning: trying to bruteforce seed from hash without candidates");
                println!("This will take a few years...");

                let start = Instant::now();
                let iter = 0..=u64::max_value();
                let total_iter_len = iter.size_hint().1.unwrap_or(u64::max_value() as usize);
                for range_lo in iter {
                    if !seed_not_from_java_next_long {
                        // if seed_from_java_next_long
                        if JavaRng::create_from_long(range_lo).is_none() {
                            if (range_lo & ((1 << 28) - 1)) == 0 {
                                print_progress_since(&start, range_lo as u64, total_iter_len as u64, range_lo as u64);
                            }
                            continue;
                        }
                    }
                    let seed = range_lo as i64;
                    if biome_layers::sha256_long_to_long(seed) == seed_hash {
                        println!("\nFound seed: {}", seed);
                        return;
                    }

                    if (range_lo & ((1 << if seed_not_from_java_next_long { 20 } else { 28 }) - 1)) == 0 {
                        print_progress_since(&start, range_lo as u64, total_iter_len as u64, range_lo as u64);
                    }
                }

                println!("\nZero seeds found");
            };
        }

        Opt::SeedHash {
            seed,
        } => {
            println!("{}", biome_layers::sha256_long_to_long(seed));
        }

        Opt::DungeonSeed {
            spawner_x,
            spawner_y,
            spawner_z,
            floor,
            threads,
        } => {
            let floor = MossyFloor::parse(&floor).expect("error parsing floor");
            let (wx, wy, wz) = (spawner_x, spawner_y, spawner_z);
            let (x, y, z) = population::spawner_coordinates_to_next_int(wx, wy, wz);
            let (top_left, top_right, bottom_left, bottom_right) = floor.corner_coords(wx, wy, wz);

            println!("Please double check that the entered data is correct:");
            println!("The coordinates of the spawner are x: {}, y: {}, z: {}", spawner_x, spawner_y, spawner_z);
            println!("When standing on the floor, the y coordinate of the player should be {}", wy);
            println!("This is the dungeon floor, and the coordinates of each corner are:");
            println!("{:?} ::: {:?}", top_left, top_right);
            println!("");
            println!("    {}", floor.to_pretty_string().replace("\n", "\n    "));
            println!("{:?} ::: {:?}", bottom_left, bottom_right);
            println!("");
            let num_threads = if threads == 0 { num_cpus::get() } else { threads };
            println!("Started brutefroce using {} threads. Estimated time: around {} minutes", num_threads, 240 / num_threads);

            let total_range = 1u64 << 40;
            let thread_range = total_range / u64::try_from(num_threads).unwrap();
            let floor = Arc::new(floor);
            let seeds: Vec<String> = run_threads(num_threads, move |thread_id| {
                let range_lo = thread_range * u64::try_from(thread_id).unwrap();
                let range_hi = if thread_id + 1 == num_threads {
                    total_range
                } else {
                    thread_range * u64::try_from(thread_id + 1).unwrap()
                };
                let range_hi = u64::try_from(range_hi).unwrap();
                debug!("Spawning thread {} from {:X} to {:X}", thread_id, range_lo, range_hi);
                let dungeon_rngs = population::dungeon_rng_bruteforce_range((x, y, z), &floor, range_lo, range_hi);
                debug!("Thread {} finished", thread_id);

                dungeon_rngs
            }).unwrap().into_iter().flat_map(|x| x).map(|dungeon_rng| format!("{},{},{},{}", spawner_x, spawner_y, spawner_z, dungeon_rng.get_seed())).collect();

            let num_candidates = seeds.len();

            println!("Found {} dungeon seeds:\n{}", seeds.len(), serde_json::to_string(&seeds).unwrap());

            match num_candidates {
                0 => println!("Help: try to double check that the coordinates and the floor are correct"),
                1 => println!("Help: once you have the seeds of 3 different dungeons, you can use the dungeon-seed-to-world-seed command to find the world seed"),
                _ => println!("Help: you can try to reduce the number of candidates by removing '?' from the dungeon floor"),
            }
        }

        Opt::DungeonSeedToWorldSeed {
            limit_steps_back,
            dungeon_seeds,
        } => {
            /// Parse the string output of the dungeon-seed command into (dungeon_seed, chunk_x,
            /// chunk_z, limit_steps_back).
            ///
            /// Example: `"159,23,-290,982513219448"` is parsed into `(159, 23, -290, 982513219448, default_l)`
            // Actually, l is only an argument because the syntax to convert a tuple with 4
            // elements into a tuple of 5 elements is super ugly, so it looks better to just return
            // a 5 element tuple here
            fn parse_dungeon_seed(s: &str, default_l: u32) -> Result<(i64, i64, i64, u64, u32), ()> {
                let mut parts = s.split(',');
                let x = parts.next().ok_or(())?;
                let y = parts.next().ok_or(())?;
                let z = parts.next().ok_or(())?;
                let seed = parts.next().ok_or(())?;
                let l = parts.next();
                if parts.next().is_some() {
                    // Trailing ','
                    return Err(());
                }

                let x = x.parse().map_err(|_| ())?;
                let y = y.parse().map_err(|_| ())?;
                let z = z.parse().map_err(|_| ())?;
                let seed = seed.parse().map_err(|_| ())?;
                let l = if let Some(l) = l {
                    l.parse().map_err(|_| ())?
                } else {
                    default_l
                };

                Ok((x, y, z, seed, l))
            }

            let dungeon_seeds: Vec<_> = dungeon_seeds.into_iter().map(|d| {
                parse_dungeon_seed(&d, limit_steps_back).map(|(x, _y, z, seed, l)| {
                    let Chunk { x: chunk_x, z: chunk_z } = population::spawner_coordinates_to_chunk(x, z);
                    (seed, chunk_x, chunk_z, l)
                }).unwrap_or_else(|_| {
                    panic!("Error parsing \"{}\": dungeon seed should follow the format \"{}\"`", d, "159,23,-290,982513219448");
                })
            }).collect();
            if dungeon_seeds.len() < 3 {
                println!("Need at least 3 dungeon seeds");
                return;
            }
            println!("{:?}", dungeon_seeds);

            let i1 = dungeon_seeds[0];
            let i2 = dungeon_seeds[1];
            let i3 = dungeon_seeds[2];
            let world_seeds = population::dungeon_seed_to_world_seed_any_version(i1, i2, i3);
            println!("Found {} world seeds:", world_seeds.len());
            println!("{:?}", world_seeds);
        }

        Opt::TestGeneration {
            input_zip,
            mc_version,
            draw_biome_map,
        } => {
            let version: MinecraftVersion = mc_version.parse().unwrap();
            match version {
                MinecraftVersion::Java1_3 | MinecraftVersion::Java1_7 | MinecraftVersion::Java1_9 | MinecraftVersion::Java1_11 | MinecraftVersion::Java1_13 | MinecraftVersion::Java1_14 => {
                    let world_seed = anvil::read_seed_from_level_dat_zip(&input_zip, Some(version)).unwrap();
                    if JavaRng::create_from_long(world_seed as u64).is_none() {
                        println!("Warning: this seed cannot be generated with Java Random nextLong");
                    }
                    println!("Seed from level.dat {}", world_seed);
                    let mut chunk_provider = ZipChunkProvider::file(input_zip).unwrap();
                    let biomes = anvil::get_all_biomes_1_14(&mut chunk_provider);
                    println!("Got {} biomes", biomes.len());

                    let points = biomes.iter().map(|(_biome_id, p)| Point { x: p.x, z: p.z });
                    let area = Area::from_coords(points);
                    println!("Area: {:?}", area);

                    if draw_biome_map {
                        println!("Drawing biome map");
                        let mut map = Map::from_area_fn(area, |(_, _)| biome_info::UNKNOWN_BIOME_ID);
                        for (expected_biome_id, p) in &biomes {
                            map.set(p.x, p.z, expected_biome_id.0);
                        }
                        let map_image = biome_layers::draw_map_image(&map);
                        let x = area.x;
                        let z = area.z;
                        let width = area.w.try_into().unwrap();
                        let height = area.h.try_into().unwrap();
                        let output_file = format!("biome_map_mc_{}_{}_{}_{}_{}x{}.png", mc_version, world_seed, x, z, width, height);
                        image::save_buffer(output_file.clone(), &map_image, width, height, image::ColorType::Rgba8).unwrap();
                        println!("Saved image to {}", output_file);
                    }

                    // Generate area with 1:1 resolution
                    let map = biome_layers::generate(version, area, world_seed);

                    // Compare maps :D
                    for (expected_biome_id, p) in biomes {
                        let b = map.get(p.x, p.z);
                        if b != expected_biome_id.0 {
                            panic!("Mismatch at ({}, {}): expected {} generated {}", p.x, p.z, expected_biome_id.0, b);
                        }
                    }

                    println!("All biomes match");
                }
                MinecraftVersion::Java1_15 | MinecraftVersion::Java1_16_1 | MinecraftVersion::Java1_16 | MinecraftVersion::Java1_17 => {
                    let world_seed = anvil::read_seed_from_level_dat_zip(&input_zip, Some(version)).unwrap();
                    if JavaRng::create_from_long(world_seed as u64).is_none() {
                        println!("Warning: this seed cannot be generated with Java Random nextLong");
                    }
                    println!("Seed from level.dat {}", world_seed);
                    let mut chunk_provider = ZipChunkProvider::file(input_zip).unwrap();
                    let biomes = anvil::get_all_biomes_1_15(&mut chunk_provider);
                    println!("Got {} biomes", biomes.len());
                    let points = biomes.iter().map(|(_biome_id, p)| Point { x: p.x, z: p.z });
                    let area = Area::from_coords(points);
                    println!("Area: {:?}", area);

                    if draw_biome_map {
                        println!("Drawing biome map");
                        let mut map = Map::from_area_fn(area, |(_, _)| biome_info::UNKNOWN_BIOME_ID);
                        for (expected_biome_id, p) in &biomes {
                            map.set(p.x, p.z, expected_biome_id.0);
                        }
                        let map_image = biome_layers::draw_map_image(&map);
                        let x = area.x;
                        let z = area.z;
                        let width = area.w.try_into().unwrap();
                        let height = area.h.try_into().unwrap();
                        let output_file = format!("biome_map_mc_{}_{}_{}_{}_{}x{}.png", mc_version, world_seed, x, z, width, height);
                        image::save_buffer(output_file.clone(), &map_image, width, height, image::ColorType::Rgba8).unwrap();
                        println!("Saved image to {}", output_file);
                    }

                    // Generate area with 1:4 resolution
                    let map = biome_layers::generate_up_to_layer_1_15(area, world_seed, version.num_layers() - 1, version);

                    // Compare maps :D
                    for (expected_biome_id, p) in biomes {
                        let b = map.get(p.x, p.z);
                        if b != expected_biome_id.0 {
                            panic!("Mismatch at ({}, {}): expected {} generated {}", p.x, p.z, expected_biome_id.0, b);
                        }
                    }

                    println!("All biomes match");
                }
                _ => {
                    unimplemented!("Version {} is not supported", mc_version);
                }
            }
        }

        Opt::ReadDungeons {
            input_zip,
            mc_version,
        } => {
            let mut chunk_provider = ZipChunkProvider::file(input_zip).unwrap();
            let version: MinecraftVersion = mc_version.parse().unwrap();
            // TODO: implement other versions
            assert!(version > MinecraftVersion::Java1_15, "only version 1.16 is supported");
            let dungeons = anvil::find_dungeons(&mut chunk_provider).unwrap();
            // Convert DungeonKind to string in order to serialize it
            let dungeons: Vec<_> = dungeons.into_iter().map(|((x, y, z), kind, floor)| ((x, y, z), kind.to_string(), floor)).collect();
            let dungeons_json = serde_json::to_string(&dungeons).unwrap();
            println!("{}", dungeons_json);
        }

        Opt::FilterBiomes {
            input_file,
            candidate_seeds,
            output_file,
        } => {
            let seed_info = SeedInfo::read(input_file).expect("Error reading seed info");
            let extra_biomes: Vec<_> = seed_info.biomes.iter().flat_map(|(id, vec_xz)| {
                vec_xz.iter().map(move |p| (*id, *p))
            }).collect();
            let version: MinecraftVersion = seed_info.version.parse().expect("Error parsing version");

            // Candidates should be 64-bit seeds
            let candidates = read_seeds_from_file_i64(candidate_seeds).expect("Error reading candidates");

            let seeds = biome_layers::filter_seeds_using_biomes(&candidates, &extra_biomes, version);

            println!("Found {} 64-bit seeds:\n{}", seeds.len(), serde_json::to_string(&seeds).unwrap());

            if let Some(of) = output_file {
                // TODO: proper error handling
                write_seeds_to_file(&seeds.into_iter().map(|x| x as i64).collect::<Vec<_>>(), of).expect("Error writing seeds to file");
            }
        }
    }
}

// Create a new file and write all the found seeds to it
// If the file already exists, it gets overwritten
fn read_seeds_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<u64>, std::io::Error> {
    let file = File::open(path)?;
    let s = serde_json::from_reader(file)?;

    Ok(s)
}

// Create a new file and write all the found seeds to it
// If the file already exists, it gets overwritten
fn read_seeds_from_file_i64<P: AsRef<Path>>(path: P) -> Result<Vec<i64>, std::io::Error> {
    let file = File::open(path)?;
    let s = serde_json::from_reader(file)?;

    Ok(s)
}

// Create a new file and write all the found seeds to it
// If the file already exists, it gets overwritten
fn write_seeds_to_file<P: AsRef<Path>>(s: &[i64], path: P) -> Result<(), std::io::Error> {
    let w = File::create(path)?;
    serde_json::to_writer(w, s)?;

    Ok(())
}

#[derive(Debug)]
enum ReadCandidateError {
    Io(std::io::Error),
    ParseInt(std::num::ParseIntError),
    SerdeJson(serde_json::Error),
}

// Create a new file and write all the found seeds to it
// If the file already exists, it gets overwritten
fn read_candidates_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<u64>, ReadCandidateError> {
    let file = File::open(path).map_err(ReadCandidateError::Io)?;
    let s: Vec<String> = serde_json::from_reader(file).map_err(ReadCandidateError::SerdeJson)?;
    let s: Result<Vec<u64>, _> = s.into_iter().map(|x| u64::from_str_radix(&x, 16)).collect();

    s.map_err(ReadCandidateError::ParseInt)
}

// Create a new file and write all the found seeds to it
// If the file already exists, it gets overwritten
fn write_candidates_to_file<P: AsRef<Path>>(s: &[String], path: P) -> Result<(), std::io::Error> {
    let w = File::create(path)?;
    serde_json::to_writer(w, s)?;

    Ok(())
}

// Spawn n threads and wait for them to finish, returning a vector of the results
// Optimization: when n is 1 do not spawn any threads and run the computation on the current thread
fn run_threads<F, T>(num_threads: usize, f: F) -> Result<Vec<T>, Box<dyn std::any::Any + Send>>
where
    F: FnOnce(usize) -> T,
    F: Clone + Send + 'static,
    T: Send + 'static,
{
    if num_threads == 1 {
        return Ok(vec![f(0)]);
    }

    let mut threads = vec![];
    for thread_id in 0..num_threads {
        let ff = f.clone();
        let handle = thread::spawn(move || {
            ff(thread_id)
        });
        threads.push(handle);
    }

    let mut r = vec![];
    for h in threads {
        r.push(h.join()?);
    }

    Ok(r)
}
