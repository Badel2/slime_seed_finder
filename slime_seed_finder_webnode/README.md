## slime\_seed\_finder\_webnode

This package contains the same interface as the `slime_seed_finder_web` package, but it compiles to a native node module instead of webassembly.
This is used in the `slime_seed_finder_webui` electron app to provide the same features as the web demo, but without all the limitations of webassembly.

### Build

You need the node-bindgen CLI tool to build this package. Install as:

```
cargo install nj-cli
```

And then, cd into this folder and run:

```
nj-cli build --release
```

This creates a node module in `dist/` with entry point `dist/index.node`.
