#!/bin/sh
cargo web build --target=wasm32-unknown-unknown --bin wasm_gui --features="wasm" --release
cp target/wasm32-unknown-unknown/release/wasm_gui.* static/
cp -rf static target/deploy
cargo doc --features="main"
cp -rf target/doc target/deploy/doc
