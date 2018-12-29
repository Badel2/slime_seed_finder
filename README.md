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
cargo install --features="clap"
```

Now you should have the `slime_seed_finder` executable in your $PATH.

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

Ideally the program should only output one seed, but we can see this is not the case. To improve it, we can also specify non slime chunks: chunks that can't spawn slimes, with the -n flag. Since it is easy to miss one chunk, there are also options to leave an error margin: -f for slime chunks and -m for non slime chunks.

If you already have a list of possible 48-bit seeds, put them in a file one seed per line:

```
slime_seed_finder --candidate-seeds candidates.txt -c chunks.txt
```

To convert 48-bit seeds into 64-bit seeds (-j flag), put the 48-bit seeds in candidates.txt
and run the program with an empty chunks.txt file:

```
slime_seed_finder --candidate-seeds candidates.txt -c empty_chunks.txt -j
```

Run `slime_seed_finder --help` for full details about the usage.

### Building WebAssemblyy demo

```sh
cargo install cargo-web
cargo +nightly web build --target=wasm32-unknown-unknown --bin wasm_gui --features="stdweb serde1"
cp target/wasm32-unknown-unknown/release/wasm_gui.* static/
# and just open static/index.html with a web browser
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
