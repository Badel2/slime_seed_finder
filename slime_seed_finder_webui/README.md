## slime\_seed\_finder\_webui

This is an electron app that should provide the same functionality as the web demo.

Currently the code is duplicated here and in `../static/`, but that does not
scale so it should be unified somewhere. The only page implemented so far is
`read_biomes.html`, and the two versions are slightly different. The plan is to
make the two versions as similar as possible, perhaps use some environment flag
to implement electron-specific functionality, and later implement all other
pages.

### Build

```
npm install
npm run dist
```

This will create a file like `dist/slime_seed_finder_webui-0.1.0.AppImage`
which can be executed using

```
./dist/slime_seed_finder_webui-0.1.0.AppImage
```

If that doesn't work and results in error "The SUID sandbox helper binary was
found, but is not configured correctly. Rather than run without sandboxing I'm
aborting now.", try running this command to fix it:

```
sudo sysctl kernel.unprivileged_userns_clone=1
```

Because of the usage of native code, this `AppImage` can only be used on the
same architecture and operating system where it was compiled. But you can use
docker to cross-compile.

### Development mode

```
# Start the application
npm run start
```

And to test changes to Rust code:

```
npm run compile-rust && npm run copy-rust
```
