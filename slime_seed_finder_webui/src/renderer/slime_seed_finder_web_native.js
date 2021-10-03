"use strict";

if (typeof Rust === "undefined") {
    var Rust = {};
}

Rust.slime_seed_finder_web = new Promise((resolve, reject) => {
    setTimeout(() => {
        let rust_addon = window.electron.Rust.slime_seed_finder_web;
        console.log(
            "Loaded rust addon in slime_seed_finder_web_native.js:",
            rust_addon
        );
        let logs_path = window.electron.getLogsPath();
        rust_addon.init(logs_path);
        resolve(rust_addon);
    }, 0);
});
//Rust.slime_seed_finder_web = Promise.resolve(
//    rust_addon
//)
