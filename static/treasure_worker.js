importScripts("wasm_gui.js");

onmessage = function(e) {
    console.log("Message received from main script");
    Rust.wasm_gui.then(
        function(slime_seed_finder_web) {
            console.log("Calling Rust code...");
            let workerResult = slime_seed_finder_web.draw_treasure_map(e.data);
            console.log("Posting message back to main script");
            postMessage(workerResult);
        },
        function(err) {
            console.error(err);
        }
    );
};
