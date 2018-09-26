importScripts("wasm_gui.js");

// Simulate lengthy calculation or an async call
function doCalculation(wasmgui, data, cb) {
    var err = null;
    /*
    console.log('Message received from main script');
    console.log(data);
    */
    var fx = data.fx;
    var fy = data.fy;
    var seed = data.seed;
    var FRAG_SIZE = data.FRAG_SIZE;
    var rvec = wasmgui.generate_fragment_slime_map(fx, fy, seed, FRAG_SIZE);
    var result = { rvec: rvec };
    cb(err, result);
}

// Handle incoming messages
self.onmessage = function(msg) {
  const {id, payload} = msg.data

  Rust.wasm_gui.then( function( wasmgui ) {
      doCalculation(wasmgui, payload, function(err, result) {
        const msg = {
          id,
          err,
          payload: result
        }
        self.postMessage(msg)
      });
  });
}
