importScripts("wasm_gui.js");

onmessage = function(e) {
  console.log('Message received from main script');
  Rust.wasm_gui.then( function( wasmgui ) {
      var workerResult = wasmgui.slime_seed_finder( {chunks: e.data.chunks, no_chunks: e.data.no_chunks} );
      console.log('Posting message back to main script');
      postMessage(workerResult);
  }, function( err ) {
      console.error(err);
  });
}
