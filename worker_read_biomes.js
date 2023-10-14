importScripts("slime_seed_finder_web.js");

// Simulate lengthy calculation or an async call
function doCalculation(slime_seed_finder_web, data, cb) {
    let err = null;
    /*
    console.log('Message received from main script');
    console.log(data);
    */
    let version = data.version;
    let region = data.region;
    let fx = data.fx;
    let fy = data.fy;
    let FRAG_SIZE = data.FRAG_SIZE;
    let y_offset = data.y_offset;
    let rvec = slime_seed_finder_web.read_fragment_biome_map(
        region,
        version,
        fx,
        fy,
        FRAG_SIZE,
        y_offset
    );
    let result = { rvec: rvec };
    cb(err, result);
}

// Handle incoming messages
self.onmessage = function(msg) {
    const { id, payload } = msg.data;

    Rust.slime_seed_finder_web.then(function(slime_seed_finder_web) {
        doCalculation(slime_seed_finder_web, payload, function(err, result) {
            const msg = {
                id,
                err,
                payload: result,
            };
            self.postMessage(msg);
        });
    });
};
