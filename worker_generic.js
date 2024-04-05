importScripts("slime_seed_finder_web.js");

function post_progress_message(progress) {
    postMessage({ progress });
}

onmessage = function(e) {
    console.log("Message received from main script");
    Rust.slime_seed_finder_web.then(
        function(slime_seed_finder_web) {
            let workerResult = slime_seed_finder_web[e.data.command](
                ...e.data.args
            );
            console.log("Posting message back to main script");
            postMessage({ result: workerResult });
        },
        function(err) {
            console.error(err);
        }
    );
};
