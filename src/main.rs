use structopt::StructOpt;
use std::path::PathBuf;
use slime_seed_finder::*;
use slime_seed_finder::biome_info::biome_id;
use slime_seed_finder::biome_layers;
use slime_seed_finder::biome_layers::Area;
use slime_seed_finder::biome_layers::Map;
use slime_seed_finder::chunk::Point;
use slime_seed_finder::slime::seed_from_slime_chunks;
use slime_seed_finder::slime::seed_from_slime_chunks_and_candidates;
use slime_seed_finder::seed_info::biomes_from_map;
use slime_seed_finder::seed_info::BiomeId;
use slime_seed_finder::seed_info::MinecraftVersion;
use slime_seed_finder::seed_info::SeedInfo;
use slime_seed_finder::java_rng::JavaRng;
use std::fs::File;
use std::fs;
use std::path::Path;
use std::io::Write;
use std::ffi::OsStr;
use std::thread;
use std::convert::TryFrom;
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
        /// Supported values: 1.7, 1.8, 1.9, 1.10, 1.11, 1.12, 1.13, 1.14.
        #[structopt(long, default_value = "1.14")]
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
        /// Supported values: 1.7, 1.8, 1.9, 1.10, 1.11, 1.12, 1.13, 1.14.
        #[structopt(long, default_value = "1.14")]
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
        /// Supported values: 1.7, 1.8, 1.9, 1.10, 1.11, 1.12, 1.13, 1.14.
        #[structopt(long, default_value = "1.14")]
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
        /// Supported values: 1.7, 1.8, 1.9, 1.10, 1.11, 1.12, 1.13, 1.14.
        #[structopt(long, default_value = "1.14")]
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
        /// Supported values: 1.7, 1.8, 1.9, 1.10, 1.11, 1.12, 1.13, 1.14.
        #[structopt(long, default_value = "1.14")]
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
            // TODO: integrate the treasure map river seed finder into the "find" subcommand
            let first_treasure_map = &seed_info.treasure_maps[0];

            let mut pmap = Map::new(Area { x: (-64 + 256 * first_treasure_map.fragment_x) / 2, z: (-64 + 256 * first_treasure_map.fragment_z) / 2, w: 128, h: 128 });
            for (i, v) in first_treasure_map.map.iter().enumerate() {
                let (x, z) = (i % 128, i / 128);
                pmap.a[(x, z)] = match v {
                    0 => biome_id::ocean,
                    1 => biome_id::plains,
                    2 => biome_id::river,
                    _ => panic!("Invalid id: {}", v),
                };
            }

            // All possible 26 bit seeds
            let seeds = biome_layers::treasure_map_river_seed_finder(&pmap, 0, 1 << 24);
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
            // TODO: implement minecraft version selection
            let _version: MinecraftVersion = mc_version.parse().unwrap();
            let output_file = output_file.unwrap_or_else(|| {
                format!("treasure_map_{}_{}_{}.png", seed, fragment_x, fragment_z).into()
            });
            let vec_rgba = biome_layers::generate_image_treasure_map_at(MinecraftVersion::Java1_13, seed, fragment_x, fragment_z);
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

            let mut threads = vec![];
            let total_range = 1 << 24;
            let thread_range = total_range / num_threads;
            for thread_id in 0..num_threads {
                let rivers = Arc::clone(&rivers);
                let extra_biomes = Arc::clone(&extra_biomes);
                let range_lo = u32::try_from(thread_range * thread_id).unwrap();
                let range_hi = if thread_id + 1 == num_threads {
                    total_range
                } else {
                    thread_range * (thread_id + 1)
                };
                let range_hi = u32::try_from(range_hi).unwrap();

                debug!("Spawning thread {} from {:X} to {:X}", thread_id, range_lo, range_hi);
                let handle = thread::spawn(move || {
                    let r = biome_layers::river_seed_finder_range(&rivers, &extra_biomes, version, range_lo, range_hi);
                    debug!("Thread {} finished", thread_id);

                    r
                });
                threads.push(handle);
            }

            let seeds: Vec<_> = threads.into_iter().flat_map(|h| h.join().unwrap()).collect();
            println!("Found {} 64-bit seeds:\n{}", seeds.len(), serde_json::to_string(&seeds).unwrap());

            if let Some(of) = output_file {
                write_seeds_to_file(&seeds, of).expect("Error writing seeds to file");
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

            if version == MinecraftVersion::Java1_15 {
                let (rivers, extra_biomes) = anvil::get_rivers_and_some_extra_biomes_zip_1_15(&input_zip, Point { x: center_x, z: center_z });

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
                let extra_biomes = Arc::new(extra_biomes);
                let num_threads = if threads == 0 { num_cpus::get() } else { threads };

                let mut threads = vec![];
                let total_range = 1 << 24;
                let thread_range = total_range / num_threads;
                for thread_id in 0..num_threads {
                    let rivers = Arc::clone(&rivers);
                    let _extra_biomes = Arc::clone(&extra_biomes);
                    let range_lo = u32::try_from(thread_range * thread_id).unwrap();
                    let range_hi = if thread_id + 1 == num_threads {
                        total_range
                    } else {
                        thread_range * (thread_id + 1)
                    };
                    let range_hi = u32::try_from(range_hi).unwrap();

                    debug!("Spawning thread {} from {:X} to {:X}", thread_id, range_lo, range_hi);
                    let handle = thread::spawn(move || {
                        let r = biome_layers::river_seed_finder_26_range(&rivers, range_lo, range_hi);
                        debug!("Thread {} finished", thread_id);

                        r
                    });
                    threads.push(handle);
                }

                let seeds: Vec<_> = threads.into_iter().flat_map(|h| h.join().unwrap()).map(|seed| format!("{:07X}", seed)).collect();
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

            let mut threads = vec![];
            let total_range = 1 << 24;
            let thread_range = total_range / num_threads;
            for thread_id in 0..num_threads {
                let rivers = Arc::clone(&rivers);
                let extra_biomes = Arc::clone(&extra_biomes);
                let range_lo = u32::try_from(thread_range * thread_id).unwrap();
                let range_hi = if thread_id + 1 == num_threads {
                    total_range
                } else {
                    thread_range * (thread_id + 1)
                };
                let range_hi = u32::try_from(range_hi).unwrap();

                debug!("Spawning thread {} from {:X} to {:X}", thread_id, range_lo, range_hi);
                let handle = thread::spawn(move || {
                    let r = biome_layers::river_seed_finder_range(&rivers, &extra_biomes, version, range_lo, range_hi);
                    debug!("Thread {} finished", thread_id);

                    r
                });
                threads.push(handle);
            }

            let seeds: Vec<_> = threads.into_iter().flat_map(|h| h.join().unwrap()).collect();
            println!("Found {} 64-bit seeds:\n{}", seeds.len(), serde_json::to_string(&seeds).unwrap());

            if let Some(of) = output_file {
                write_seeds_to_file(&seeds, of).expect("Error writing seeds to file");
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

