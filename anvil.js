document.getElementById("filepicker").addEventListener(
    "change",
    function() {
        let reader = new FileReader();
        reader.onload = function() {
            let arrayBuffer = this.result;
            let array = new Uint8Array(arrayBuffer);
            region = array;
            console.log("Loaded region file. Size:", array.length);
        };
        reader.readAsArrayBuffer(this.files[0]);
    },
    false
);

function showProgressBar(currentValue, maxValue) {
    let x = document.getElementById("progressBarLabel");
    x.dataset.label = currentValue + " / " + maxValue;
    let percent = (currentValue * 100) / maxValue;
    let y = document.getElementById("progressBarSpan");
    y.style.width = percent + "%";
}
function fitToContainer(canvas) {
    // Make it visually fill the positioned parent
    canvas.style.width = "100%";
    canvas.style.height = "100%";
    // ...then set the internal size to match
    canvas.width = canvas.offsetWidth;
    canvas.height = canvas.offsetHeight;
    // And finally reset style w/h, because resizing the canvas may have also
    // resized the parent container
    canvas.style.width = "";
    canvas.style.height = "";
}

let l42AreaC = null;
let foundSeeds = [];
let workers = [];
let selection_history = [];
let region = [];

function runWorkers(numWorkers, seedInfo) {
    foundSeeds = [];
    let outta = document.getElementById("output_textarea");
    outta.value = "Calculating...";
    let startedX = 0;
    let x = 0;
    let limit = 1 << 24;
    let range = 1 << 17;
    //let range = (limit / numWorkers);
    for (let workerId = 0; workerId < numWorkers; workerId++) {
        let myWorker = new Worker("rivers_worker.js");
        workers.push(myWorker);
        showProgressBar(x / range, limit / range);
        myWorker.postMessage(
            JSON.stringify({
                seedInfo: seedInfo,
                range: [startedX, startedX + range],
            })
        );
        startedX += range;
        console.log("Message posted to worker");
        myWorker.onmessage = function(e) {
            console.log("Got message from worker " + workerId);
            x += range;
            showProgressBar(x / range, limit / range);
            foundSeeds = [...foundSeeds, ...e.data];
            outta.value = stringify(
                { foundSeeds: foundSeeds },
                { maxLength: 20 }
            );
            if (startedX < limit) {
                myWorker.postMessage(
                    JSON.stringify({
                        seedInfo: seedInfo,
                        range: [startedX, startedX + range],
                    })
                );
                startedX += range;
            } else if (x < limit) {
                // Waiting for other workers to finish
            } else {
                searchFinished = true;
                workers = [];
                alert(
                    "Done! Found " +
                        foundSeeds.length +
                        " seed" +
                        (foundSeeds.length == 1 ? "" : "s")
                );
            }
        };
    }
}
function runGui() {
    let seltextarea = document.getElementById("selection_output");
    if (window.Worker) {
        let maxWorkers = navigator.hardwareConcurrency || 4;
        Rust.slime_seed_finder_web.then(function(slime_seed_finder_web) {
            let isThis115 =
                document.getElementById("minecraftPlayVersion").value == "1.15";
            let seedInfo_str = slime_seed_finder_web.anvil_region_to_river_seed_finder(
                region,
                isThis115
            );
            console.log("Got seedInfo from wasm:");
            console.log(seedInfo_str);
            let seedInfo = JSON.parse(seedInfo_str);
            seedInfo.version = document.getElementById(
                "minecraftGenerateVersion"
            ).value;
            if (isThis115) {
                seedInfo.worldSeedHash = document.getElementById(
                    "worldSeedHash"
                ).value;
            }
            runWorkers(maxWorkers, seedInfo);
        });
    } else {
        alert("Version without webworkers not implemented");
    }
}

// Count candidates
function countCandidates() {
    Rust.slime_seed_finder_web.then(
        function(slime_seed_finder_web) {
            let outta = document.getElementById("num_candidates");
            outta.value = slime_seed_finder_web.count_rivers(
                JSON.stringify({ seedInfo: seedInfo })
            );
        },
        function(err) {
            console.error(err);
        }
    );
}
