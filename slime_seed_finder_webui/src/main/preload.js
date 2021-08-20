const { contextBridge } = require("electron");
const ssf = require("../../rust-dist");

const slime_seed_finder_web = {};

// No idea why, but without this copy the functions are not exported
for (let name of Object.getOwnPropertyNames(ssf)) {
    slime_seed_finder_web[name] = ssf[name];
}

contextBridge.exposeInMainWorld("electron", {
    Rust: {
        slime_seed_finder_web: slime_seed_finder_web,
    },
});
