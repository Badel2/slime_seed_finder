extern crate slime_seed_finder;
extern crate clap;
use clap::{App, Arg};
use slime_seed_finder::*;
use slime_seed_finder::slime::seed_from_slime_chunks_and_candidates;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

fn main() {
    let matches = App::new("slime_seed_finder")
        .arg(Arg::with_name("chunks_file")
             .help("The file containing the slime chunks")
             .short("c")
             .long("chunks")
             .takes_value(true)
        )
        .arg(Arg::with_name("no_chunks_file")
             .help("The file containing the non slime chunks")
             .short("n")
             .long("no-chunks")
             .takes_value(true)
             .required(false)
        )
        .arg(Arg::with_name("output_file")
             .help("Write all the found 48-bit seeds to this file")
             .short("o")
             .long("output")
             .takes_value(true)
             .required(false)
        )
        .arg(Arg::with_name("false_chunks")
             .help("The max number of chunks that may not actually be slime chunks")
             .short("f")
             .long("false-chunks")
             .default_value("0")
             .takes_value(true)
             .required(false)
        )
        .arg(Arg::with_name("false_no_chunks")
             .help("The max number of non slime chunks that may actually be slime chunks")
             .short("m")
             .long("false-no-chunks")
             .default_value("0")
             .takes_value(true)
             .required(false)
        )
        .arg(Arg::with_name("java")
             .help("Assume the seed was generated using java nextLong(), and write \
                    all the possible 64 bit seeds to the output file")
             .short("j")
             .long("java-long")
        )
        .arg(Arg::with_name("generate_seed")
             .help("Output a list of slime chunks from this seed, for testing purposes")
             .short("s")
             .long("seed")
             .takes_value(true)
             .required(false)
        )
        .arg(Arg::with_name("generate_seed_no")
             .help("Output a list of non slime chunks from this seed, for testing purposes")
             .long("seed-no")
             .takes_value(true)
             .required(false)
             .conflicts_with("generate_seed")
        )
        .arg(Arg::with_name("generate_chunks")
             .help("Control how many chunks to generate")
             .short("g")
             .long("generate-chunks")
             .default_value("40")
             .takes_value(true)
             .required(false)
        )
        .arg(Arg::with_name("candidate-seeds")
             .help("Only try the seeds supplied by this file")
             .long("candidate-seeds")
             .takes_value(true)
             .required(false)
        )
        .get_matches();

    let of = matches.value_of("output_file");

    if let Some(generate_seed) = matches.value_of("generate_seed") {
        let s = generate_seed.parse().unwrap();
        let n = matches.value_of("generate_chunks").unwrap().parse().unwrap_or(40);
        let c = generate_slime_chunks(s, n);
        let mut buf = String::new();
        println!("{} slime chunks for seed {}:", n, s);

        for x in &c {
            buf.push_str(&format!("{},{}\n", x.x, x.z));
        }

        println!("{}", buf);
        if let Some(of) = of {
            // TODO: proper error handling
            let mut w = File::create(of).unwrap();
            write!(w, "{}", buf).unwrap();
        }
        return;
    }

    if let Some(generate_seed) = matches.value_of("generate_seed_no") {
        let s = generate_seed.parse().unwrap();
        let n = matches.value_of("generate_chunks").unwrap().parse().unwrap_or(40);
        let c = generate_no_slime_chunks(s, n);
        let mut buf = String::new();
        println!("{} no slime chunks for seed {}:", n, s);

        for x in &c {
            buf.push_str(&format!("{},{}\n", x.x, x.z));
        }

        println!("{}", buf);
        if let Some(of) = of {
            // TODO: proper error handling
            let mut w = File::create(of).unwrap();
            write!(w, "{}", buf).unwrap();
        }
        return;
    }

    let ncf = matches.value_of("no_chunks_file");
    let false_c = matches.value_of("false_chunks").unwrap_or("0").parse().unwrap_or(0);
    let false_nc = matches.value_of("false_no_chunks").unwrap_or("0").parse().unwrap_or(0);
    let java = matches.is_present("java");
    let candidate_seeds = matches.value_of("candidate-seeds");

    if let Some(ref cf) = matches.value_of("chunks_file") {
        let c = read_chunks_from_file(cf);
        let nc = if let Some(ncf) = ncf {
            read_chunks_from_file(ncf)
        } else {
            Ok(vec![])
        };

        if let (Ok(c), Ok(nc)) = (c, nc) {
            // All possible 48 bit seeds
            let seeds = if let Some(path) = candidate_seeds {
                let candidates = read_seeds_from_file(path).expect("Error reading seeds from file");
                seed_from_slime_chunks_and_candidates(&c, false_c, &nc, false_nc, candidates)
            } else {
                seed_from_slime_chunks(&c, false_c, &nc, false_nc)
            };
            println!("Found {} seeds:\n{:#?}", seeds.len(), seeds);

            if java {
                // Display only 64 bit seeds that could be generated by java
                // (when the seed box is left empty)
                let mut java_seeds: Vec<i64> = seeds
                    .iter()
                    .flat_map(|&s| Rng::extend_long_48(s))
                    .map(|s| s as i64)
                    .collect();

                java_seeds.sort();
                println!("Java seeds: {}\n{:#?}", java_seeds.len(), java_seeds);
                if let Some(of) = of {
                    // TODO: proper error handling
                    write_seeds_to_file(&java_seeds.into_iter().map(|x| x as u64).collect::<Vec<_>>(), of).unwrap();
                }
            } else {
                if let Some(of) = of {
                    // TODO: proper error handling
                    write_seeds_to_file(&seeds, of).unwrap();
                }
            }
        }
    } else {
        println!("Usage: slime_seed_finder -c chunks_file.txt\n\
                  For more information try --help");
    }

}

// Create a new file and write all the found seeds to it
// If the file already exists, it gets overwritten
fn write_seeds_to_file(s: &[u64], path: &str) -> Result<(), std::io::Error> {
    let mut w = File::create(path)?;
    for &i in s {
        write!(w, "{}\n", i as i64)?;
    }

    Ok(())
}

// Create a new file and write all the found seeds to it
// If the file already exists, it gets overwritten
fn read_seeds_from_file(path: &str) -> Result<Vec<u64>, std::io::Error> {
    let f = File::open(path)?;
    let file = BufReader::new(&f);
    let mut s = vec![];
    for seed in file.lines() {
        let x: i64 = seed.unwrap().parse().unwrap();
        s.push(x as u64);
    }

    Ok(s)
}
