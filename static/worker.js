importScripts("wasm_gui.js");

onmessage = function(e) {
    console.log("Message received from main script");
    Rust.wasm_gui.then(
        function(slime_seed_finder_web) {
            let workerResult = slime_seed_finder_web.slime_seed_finder({
                seedInfo: e.data.seedInfo,
            });
            console.log("Posting message back to main script");
            postMessage(workerResult);
        },
        function(err) {
            console.error(err);
        }
    );
};
