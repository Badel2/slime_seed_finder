#!/bin/sh
cargo web build --target=wasm32-unknown-unknown --release -p slime_seed_finder_web
cp target/wasm32-unknown-unknown/release/slime_seed_finder_web.* static/
cp -rf static target/deploy
# Generate local documentation
#cargo doc --features="main"
#cp -rf target/doc target/deploy/doc
