# slime_seed_finder

This program finds seeds (minecraft world generation seeds) with some given properties. Essentially it focuses on slime chunks, which are chunks with a special property: they can spawn slimes. Since slime chunks are 10% of the chunks and there are 2^64 minecraft seeds, one would think that with 20 chunks you can find the seed, however thinks get interesting...

### Instalation
To build this project you need to install the Rust programming language. Follow the instructions on https://rustup.rs
Then, run the following commands
```
git clone https://github.com/badel2/slime_seed_finder
cd slime_seed_finder
cargo install --features="clap"
```

Now you should have the slime_seed_finder executable in your $PATH.

### Usage
Put the slime chunks in a text file, one chunk per line, with the x and z chunk coordinates separated by a comma, and then run the following command to save the found seeds in the file seeds.txt
```
slime_seed_finder -c chunks_file.txt -o seeds.txt
```

If you don't have a list of slime chunks and want to try this program, use the generate option to generate slime chunks, just choose a numerical seed:
```
slime_seed_finder -s 1234 -o 1234.txt
slime_seed_finder -c 1234.txt -o 1234_and_some_more.txt
```

Ideally the program should only output one seed, but we can see this is not the case. To improve it, we can also specify non slime chunks: chunks that can't spawn slimes, with the -n flag. Since it is easy to miss one chunk, there are also options to leave an error margin: -f for slime chunks and -m for non slime chunks. There is also a --help option, which explains the command line usage.


### See also
https://github.com/pruby/slime-seed - A project with the same goal and similar optimizations
