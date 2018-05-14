importScripts("wasm_gui.js");

onmessage = function(e) {
  console.log('Message received from main script');
  Rust.wasm_gui.then( function( wasmgui ) {
      var workerResult = wasmgui.slime_seed_finder(e.data.chunks, e.data.no_chunks);
      console.log('Posting message back to main script');
      postMessage(workerResult);
  });
}
