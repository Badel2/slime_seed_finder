importScripts("wasm_gui.js");

onmessage = function(e) {
  console.log('Message received from main script');
  Rust.wasm_gui.then( function( wasmgui ) {
      let workerResult = wasmgui.slime_seed_finder( {seedInfo: e.data.seedInfo} );
      console.log('Posting message back to main script');
      postMessage(workerResult);
  }, function( err ) {
      console.error(err);
  });
}
