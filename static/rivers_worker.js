importScripts("slime_seed_finder_web.js");

onmessage = function(e) {
    console.log("Message received from main script");
    Rust.slime_seed_finder_web.then(
        function(slime_seed_finder_web) {
            console.log("Calling Rust code...");
            if (e.data.command === "anvil_region_to_river_seed_finder") {
                let workerResult = slime_seed_finder_web.anvil_region_to_river_seed_finder(
                    ...e.data.args
                );
                console.log("Posting message back to main script");
                postMessage(workerResult);
            } else {
                let workerResult = slime_seed_finder_web.river_seed_finder(
                    e.data
                );
                console.log("Posting message back to main script");
                postMessage(workerResult);
            }
        },
        function(err) {
            console.error(err);
        }
    );
};
