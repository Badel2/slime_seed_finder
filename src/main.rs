use structopt::StructOpt;
use std::path::PathBuf;
use slime_seed_finder::*;
use slime_seed_finder::biome_layers;
use slime_seed_finder::biome_layers::Area;
use slime_seed_finder::biome_layers::biome_id;
use slime_seed_finder::slime::seed_from_slime_chunks;
use slime_seed_finder::slime::seed_from_slime_chunks_and_candidates;
use slime_seed_finder::seed_info::biomes_from_map;
use slime_seed_finder::seed_info::SeedInfo;
use slime_seed_finder::java_rng::JavaRng;
use std::fs::File;
use std::path::Path;
use std::io::Write;
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
        /// If the seed flag is omitted, it will be generated randomly using
        /// the same method as a default Minecraft server, which has a flaw
        /// and uses only 48 bits of entropy. With this flag, 64 bits of
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
        /// Minecraft Version to use.
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

    #[structopt(name = "extend48")]
    Extend48 {
        /// File containing the list of 48-bit seeds as a JSON array
        #[structopt(short = "i", long, parse(from_os_str))]
        input_file: PathBuf,
        /// Where to write the extended seeds as a JSON array
        #[structopt(short = "o", long, parse(from_os_str))]
        output_file: Option<PathBuf>,
    }
}

fn main() {
    pretty_env_logger::init();

    match Opt::from_args() {
        Opt::Generate {
            seed,
            seed_not_from_java_next_long,
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

