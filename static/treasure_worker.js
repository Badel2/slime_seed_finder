importScripts("wasm_gui.js");

onmessage = function(e) {
    console.log("Message received from main script");
    Rust.wasm_gui.then(
        function(wasmgui) {
            console.log("Calling Rust code...");
            let workerResult = wasmgui.draw_treasure_map(e.data);
            console.log("Posting message back to main script");
            postMessage(workerResult);
        },
        function(err) {
            console.error(err);
        }
    );
};
