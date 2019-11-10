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

let minecraft_version = "1.7";
let l42AreaC = null;
let foundSeeds = [];
let workers = [];
let selection_history = [];
let region = [];

document
    .getElementById("minecraftVersion")
    .addEventListener("input", function(event) {
        minecraft_version = document.getElementById("minecraftVersion").value;
    });

function runWorkers(numWorkers) {
    foundSeeds = [];
    let outta = document.getElementById("output_textarea");
    outta.value = "Calculating...";
    let startedX = 0;
    let x = 0;
    let limit = 1 << 24;
    let range = 1 << 17;
    //let range = (limit / numWorkers);
    for (let workerId = 0; workerId < numWorkers; workerId++) {
        let myWorker = new Worker("anvil_worker.js");
        workers.push(myWorker);
        showProgressBar(x / range, limit / range);
        myWorker.postMessage({
            region: region,
            range: [startedX, startedX + range],
            minecraftVersion: minecraft_version,
        });
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
                myWorker.postMessage({
                    region: region,
                    range: [startedX, startedX + range],
                    minecraftVersion: minecraft_version,
                });
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
        runWorkers(maxWorkers);
    } else {
        alert("Version without webworkers not implemented");
    }
}

// Count candidates
function countCandidates() {
    Rust.wasm_gui.then(
        function(wasmgui) {
            let outta = document.getElementById("num_candidates");
            outta.value = wasmgui.count_rivers(
                JSON.stringify({ seedInfo: seedInfo })
            );
        },
        function(err) {
            console.error(err);
        }
    );
}
