"use strict";

if (typeof Rust === "undefined") {
    var Rust = {};
}

Rust.slime_seed_finder_web = new Promise((resolve, reject) => {
    setTimeout(() => {
        let rust_addon = require("../../rust-dist");
        console.log(
            "Loaded rust addon in slime_seed_finder_web_native.js:",
            rust_addon
        );
        // TODO: logging disabled because node-bindgen already sets a logger and it is not possible
        // to unset that logger and set this console.log logger
        //rust_addon.init(function (level, msg, f1, f2, f3) {
        //    console.log("LOG");
        //    //       console.log(msg, f1, f2, f3);
        //});
        resolve(rust_addon);
    }, 0);
});
//Rust.slime_seed_finder_web = Promise.resolve(
//    rust_addon
//)
