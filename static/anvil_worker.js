importScripts("wasm_gui.js");

onmessage = function(e) {
    console.log("Message received from main script");
    Rust.wasm_gui.then(
        function(wasmgui) {
            console.log("Calling Rust code...");
            let workerResult = wasmgui.anvil_region_seed_finder(
                e.data.region,
                JSON.stringify({
                    range: e.data.range,
                    version: e.data.minecraftVersion,
                })
            );
            console.log("Posting message back to main script");
            postMessage(workerResult);
        },
        function(err) {
            console.error(err);
        }
    );
};
