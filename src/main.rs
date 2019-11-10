use structopt::StructOpt;
use std::path::PathBuf;
use slime_seed_finder::*;
use slime_seed_finder::biome_layers;
use slime_seed_finder::biome_layers::Area;
use slime_seed_finder::biome_layers::biome_id;
use slime_seed_finder::biome_layers::Map;
use slime_seed_finder::slime::seed_from_slime_chunks;
use slime_seed_finder::slime::seed_from_slime_chunks_and_candidates;
use slime_seed_finder::seed_info::biomes_from_map;
use slime_seed_finder::seed_info::MinecraftVersion;
use slime_seed_finder::seed_info::SeedInfo;
use slime_seed_finder::java_rng::JavaRng;
use std::fs::File;
use std::path::Path;
use std::io::Write;
use std::ffi::OsStr;
use std::thread;
use std::convert::TryFrom;
use std::sync::Arc;
use rand::{thread_rng, Rng as _};
use log::*;

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
    },
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
            // TODO: integrate the river seed finder into the "find" subcommand
            let extra_biomes: Vec<_> = seed_info.biomes.iter().flat_map(|(id, vec_xz)| {
                if *id == biome_id::river {
                    vec![]
                } else {
                    vec_xz.iter().map(|(x, z)| (*id, *x, *z)).collect()
                }
            }).collect();

            // All possible 64 bit seeds
            let seeds = if let Some(rivers) = seed_info.biomes.get(&biome_id::river) {
                biome_layers::river_seed_finder(rivers, &extra_biomes)
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
            image::save_buffer(output_file.clone(), &vec_rgba, width, height, image::ColorType::RGBA(8)).unwrap();
            println!("Saved image to {}", output_file.to_string_lossy());
        }

        Opt::Treasure {
            seed,
            fragment_x,
            fragment_z,
            output_file,
        } => {
            let output_file = output_file.unwrap_or_else(|| {
                format!("treasure_map_{}_{}_{}.png", seed, fragment_x, fragment_z).into()
            });
            let vec_rgba = biome_layers::generate_image_treasure_map_at(seed, fragment_x, fragment_z);
            assert_eq!(vec_rgba.len(), 128 * 128 * 4);
            image::save_buffer(output_file.clone(), &vec_rgba, 128, 128, image::ColorType::RGBA(8)).unwrap();
            println!("Saved image to {}", output_file.to_string_lossy());
        }

        Opt::Anvil {
            input_dir,
            output_file,
            threads,
            center_x,
            center_z,
        } => {
            if input_dir.file_name() != Some(OsStr::new("region")) {
                println!(r#"Error: input dir must end with "/region""#);
                return;
            }

            let (rivers, extra_biomes) = anvil::get_rivers_and_some_extra_biomes(&input_dir, (center_x, center_z));
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
                    let r = biome_layers::river_seed_finder_range(&rivers, &extra_biomes, range_lo, range_hi);
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

