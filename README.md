# slime_seed_finder

This program finds minecraft world generation seeds given enough slime chunks.
Since slime chunks are 10% of all the chunks and there are 2^64 minecraft
seeds, one would think that with 20 chunks you can find the seed, however
thinks get interesting...

### Web demo
There is a work in progress WebAssembly demo available at:

<https://badel2.github.io/slime_seed_finder/>

Also, a biome viewer (like [Amidst](https://github.com/toolbox4minecraft/amidst) but without structures):
<https://badel2.github.io/slime_seed_finder/biomes.html>

And a slime map which can be used to compare multiple seeds:
<https://badel2.github.io/slime_seed_finder/slime_map.html>

### Local instalation
To build this project you need to install the Rust programming language. Follow the instructions on https://rustup.rs
Then, run the following commands
```
git clone https://github.com/badel2/slime_seed_finder
cd slime_seed_finder
cargo install --path . --features="main" -f
```

Now you should have the `slime_seed_finder` executable in your $PATH.

To update to the latest version:

```
git pull origin master
cargo install --path . --features="main" -f
```

### Usage
```
slime_seed_finder find -i seedinfo.json -o seeds.json
```

Run `slime_seed_finder --help` for full details about the usage,
and `slime_seed_finder <subcommand> --help` for detailed help about a
subcommand.

The [SeedInfo](https://github.com/Badel2/slime_seed_finder/blob/master/docs/seedinfo.md)
is a JSON with information about the world that can be useful to find the seed.

If you don't have a list of slime chunks and want to try this program, use the generate option to generate slime chunks.
You can choose a numerical seed, or leave it blank to generate a random seed.
You can specify how many slime chunks to generate with `--num-slime-chunks` (the default is 0):
```
# Random seed
slime_seed_finder generate -o seedinfo_random_seed.json --num-slime-chunks 40
# Seed 1234
slime_seed_finder generate -s 1234 -o seedinfo_1234.json --num-slime-chunks 40
# Finding seed 1234
slime_seed_finder find -i seedinfo_1234.json -o 1234_and_some_more.json
```

If you already have a list of possible 48-bit seeds, put them in a file as a JSON array:

```
slime_seed_finder find --candidate-seeds candidates.json -i seedinfo.json
```

#### extend48

To convert 48-bit seeds into 64-bit seeds (by assuming that the seed was generated
using Java Random nextLong), put the 48-bit seeds in candidates.json as a JSON array
and run the extend48 subcommand. For example, to extend the seeds 1 and 2:

```
echo '[1, 2]' > candidates.json
slime_seed_finder extend48 -i candidates.json
```

Output:

```
[
  8897424013823836161,
  -651896046061879294
]
```

It will become more clear if we convert those seeds to hexadecimal:

```
0x7b7a000000000001
0xf6f4000000000002
```

In this case each 48-bit seed has 1 corresponding 64-bit seed, but it can be 0,
1 or 2 seeds. This has the implications that the only way to create a new
Minecraft world with seed 1 or 2 is to manually set the seed to that number.

### Building WebAssembly demo

In order to locally test the web demos:

```
# Install cargo web
cargo install cargo-web
# And run this after each change
./ci/build_demo.sh
```

You need to run a local web server at the `static/` dir. You can use the provided
`server.py` file, which starts a server at http://localhost:8000

```
cd static
python2 server.py
```

### Theory

[RNG Basics](https://github.com/Badel2/slime_seed_finder/blob/master/docs/rng_basics.md)
[Slime Chunks](https://github.com/Badel2/slime_seed_finder/blob/master/docs/slime_chunks.md)

### See also
https://github.com/pruby/slime-seed - A project with the same goal and similar optimizations
