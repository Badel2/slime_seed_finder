#!/bin/sh
cargo wasi run --release --features=wasi
# This verifies that the compiled module will run on
# https://webassembly.sh
wasmer ./target/wasm32-wasi/release/slime_seed_finder.wasm
