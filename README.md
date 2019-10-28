# slime_seed_finder

This program finds minecraft world generation seeds given enough slime chunks.
Since slime chunks are 10% of all the chunks and there are 2^64 minecraft
seeds, one would think that with 20 chunks you can find the seed, however
thinks get interesting...

### Web demo
There is a work in progress WebAssembly demo available at:

<https://badel2.github.io/slime_seed_finder/>

Also, a biome viewer (like AMIDST but without structures):
<https://badel2.github.io/slime_seed_finder/biomes.html>

And a slime map which can be used to compare multiple seeds:
<https://badel2.github.io/slime_seed_finder/slime_map.html>

### Local instalation
To build this project you need to install the Rust programming language. Follow the instructions on https://rustup.rs
Then, run the following commands
```
git clone https://github.com/badel2/slime_seed_finder
cd slime_seed_finder
cargo install --path . --features="main"
```

Now you should have the `slime_seed_finder` executable in your $PATH.

### Usage
```
slime_seed_finder find -i seedinfo.json -o seeds.json
```

Run `slime_seed_finder --help` for full details about the usage,
and `slime_seed_finder <subcommand> --help` for detailed help about a
subcommand.


If you don't have a list of slime chunks and want to try this program, use the generate option to generate slime chunks.
You can choose a numerical seed, or leave it blank to generate a random seed.
You can specify how many slime chunks to generate with `--num-slime-chunks` (the default is 0):
```
# Seed 1234
slime_seed_finder generate -s 1234 -o seedinfo_1234.json --num-slime-chunks 40
# Random seed
slime_seed_finder generate -o seedinfo_random_seed.json --num-slime-chunks 40
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

In this case each 48-bit seed has 1 corresponding 64-bit seed, but it can be 0, 1 or 2 seeds.
This has the implications that it is impossible to create a new Minecraft world with seed
1 or 2 unless you manually set the seed to that number.

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

Each slime chunk reduces the expected runtime by 50%, until colisions start to
dominate. Non-slime chunks don't reduce the expected runtime, but are needed to
remove all the false positives.

Java Random uses 48 bits, so we can only find the lower 48 bits of the seed
using slime chunks.
But instead of bruteforcing 2^48, we bruteforce 2^18 + 2^30.
That's because those two lines are equivalent:

```c
i % 10 == 0
(i % 2 == 0) && (i % 5 == 0)
```

And the parity of the output of `Random.nextInt(10)` depends only on the
lower 18 bits, so if the parity is odd, we discard this 18-bit combination.

### See also
https://github.com/pruby/slime-seed - A project with the same goal and similar optimizations
