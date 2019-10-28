// common.js
//
// start up function
//

window.onload = function() {
    let pos_div = document.getElementById("position_info");
    let center_butt = document.getElementById("center_button");
    center_butt.onclick = function() {
        let x = document.getElementById("extraBiomeX").value;
        let z = document.getElementById("extraBiomeZ").value;
        Game.centerAt(x, z);
    };
    let elem = document.getElementById("demo");
    fitToContainer(elem);
    let elemLeft = elem.offsetLeft;
    let elemTop = elem.offsetTop;
    let context = elem.getContext("2d");
    let elements = [];
    let dragging = null;

    // Add event listener for `click` events.
    // TODO: touchstart for mobile support
    // https://stackoverflow.com/a/16284281
    let pointerEventToXY = function(e) {
        let out = { x: 0, y: 0 };
        if (
            e.type == "touchstart" ||
            e.type == "touchmove" ||
            e.type == "touchend" ||
            e.type == "touchcancel"
        ) {
            let touch = e.touches[0] || e.changedTouches[0];
            out.x = touch.pageX;
            out.y = touch.pageY;
        } else if (
            e.type == "mousedown" ||
            e.type == "mouseup" ||
            e.type == "mousemove" ||
            e.type == "mouseover" ||
            e.type == "mouseout" ||
            e.type == "mouseenter" ||
            e.type == "mouseleave"
        ) {
            out.x = e.pageX;
            out.y = e.pageY;
        }
        return out;
    };

    ["touchstart", "mousedown"].forEach(function(n) {
        elem.addEventListener(
            n,
            function(e) {
                //console.log('elem mousedown');
                if (n == "touchstart") {
                    e.preventDefault();
                }
                let pointer = pointerEventToXY(e);
                let x = pointer.x - elemLeft,
                    y = pointer.y - elemTop;

                dragging = { x: x, y: y, actuallyScrolling: false };
            },
            false
        );
    });
    ["touchmove", "mousemove"].forEach(function(n) {
        elem.addEventListener(
            n,
            function(e) {
                //console.log('elem mousemove');
                let pointer = pointerEventToXY(e);
                let x = pointer.x - elemLeft,
                    y = pointer.y - elemTop;
                let txty = Game.mouse_coords_to_game_coords_float(x, y);
                let tx = txty[0];
                let ty = txty[1];
                pos_div.innerHTML =
                    "Block x: " + Math.floor(tx) + ", z: " + Math.floor(ty);
            },
            false
        );
    });

    ["touchmove", "mousemove"].forEach(function(n) {
        window.addEventListener(
            n,
            function(e) {
                if (dragging) {
                    let pointer = pointerEventToXY(e);
                    let x = pointer.x - elemLeft,
                        y = pointer.y - elemTop;
                    if (
                        dragging.actuallyScrolling == false &&
                        (Math.abs(dragging.x - x) > 10 ||
                            Math.abs(dragging.y - y) > 10)
                    ) {
                        // Moving more than 10 pixels from the initial position starts the scrolling

                        dragging.actuallyScrolling = true;
                    }
                    let tool = document.getElementById("toolSelector").value;
                    let scrollingEnabled =
                        tool == "click" ||
                        tool == "move" ||
                        tool == "bucket_river" ||
                        tool == "bucket_land" ||
                        tool == "bucket_ocean";
                    if (scrollingEnabled && dragging.actuallyScrolling) {
                        Game.scrollBy(dragging.x - x, dragging.y - y);
                        dragging.x = x;
                        dragging.y = y;
                    }
                    if (!scrollingEnabled && dragging.actuallyScrolling) {
                        if (tool == "pencil_river") {
                            Game.setTile(x, y, 2);
                        } else if (tool == "pencil_land") {
                            Game.setTile(x, y, 1);
                        } else if (tool == "pencil_ocean") {
                            Game.setTile(x, y, 0);
                        } else {
                            console.error("Unhandled tool type");
                        }
                        dragging.x = x;
                        dragging.y = y;

                        update_selection();
                        // Draw rendered map
                        //drawVoronoi();
                        drawTreasureMap();
                    }
                }
            },
            false
        );
    });

    ["touchend", "mouseup"].forEach(function(n) {
        elem.addEventListener(
            n,
            function(e) {
                //console.log('elem mouseup');
                if (n == "touchend") {
                    e.preventDefault();
                }
                let pointer = pointerEventToXY(e);
                let x = pointer.x - elemLeft,
                    y = pointer.y - elemTop;

                if (dragging == null) {
                    // The window event handler was executed first, gg
                    console.error(
                        "BUG: The window event handler was executed before the elem event handler for event mouseup"
                    );
                }
                if (dragging && dragging.actuallyScrolling == false) {
                    let tool = document.getElementById("toolSelector").value;
                    if (tool == "click") {
                        Game.clickTile(x, y);
                    } else if (tool == "pencil_river") {
                        Game.setTile(x, y, 2);
                    } else if (tool == "pencil_ocean") {
                        Game.setTile(x, y, 0);
                    } else if (tool == "pencil_land") {
                        Game.setTile(x, y, 1);
                    } else if (tool == "bucket_river") {
                        Game.bucketTool(x, y, 2);
                    } else if (tool == "bucket_ocean") {
                        Game.bucketTool(x, y, 0);
                    } else if (tool == "bucket_land") {
                        Game.bucketTool(x, y, 1);
                    }

                    update_selection();
                    // Draw rendered map
                    //drawVoronoi();
                    drawTreasureMap();
                }
                dragging = null;
            },
            false
        );
    });

    ["touchend", "mouseup"].forEach(function(n) {
        window.addEventListener(
            n,
            function(e) {
                if (n == "touchend") {
                    // This breaks the page
                    //e.preventDefault();
                }
                dragging = null;
            },
            false
        );
    });

    // Update selection textarea
    update_selection();

    let tsize = 256;
    let canvasW = elem.width;
    let canvasH = elem.height;
    Game.run(context, tsize, canvasW, canvasH, 0);
};

// end common.js

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
let seedInfo = { version: minecraft_version };
let l42AreaC = null;
let foundSeeds = [];
let workers = [];
let selection_history = [];

document
    .getElementById("minecraftVersion")
    .addEventListener("input", function(event) {
        minecraft_version = document.getElementById("minecraftVersion").value;
        seedInfo.version = minecraft_version;
        update_selection();
    });

document
    .getElementById("showLayer1Overlay")
    .addEventListener("input", function(event) {
        let showLayer1 = document.getElementById("showLayer1Overlay").checked;
        console.log("showLayer1: " + showLayer1);
        Game.showLayer1 = showLayer1;
        Game.dirty = true;
    });

function version_map(s) {
    if (
        s == "1.7" ||
        s == "1.8" ||
        s == "1.9" ||
        s == "1.10" ||
        s == "1.11" ||
        s == "1.12"
    ) {
        return "1.7";
    } else if (s == "1.13") {
        return "1.13";
    } else if (s == "1.14") {
        return "1.14";
    } else {
        console.error("Unknown minecraft version: " + s);
        return "";
    }
}

// From seedInfo textarea to seedInfo and map
function load_selection() {
    let seltextarea = document.getElementById("selection_output");
    try {
        let x = JSON.parse(seltextarea.value);
        seedInfo = x;
        Game.clearSelection(0);
        let firstTreasureMap = seedInfo.treasureMaps[0];
        Game.setArea(0, 0, 128, 0, 128, firstTreasureMap.map);
        drawVoronoi();
        document.getElementById("minecraftVersion").value = version_map(
            seedInfo.version
        );
        document.getElementById("fragmentX").value = firstTreasureMap.fragmentX;
        document.getElementById("fragmentZ").value = firstTreasureMap.fragmentZ;
    } catch (e) {
        // Invalid JSON
    }
}
// From map to seedInfo and seedInfo textarea
function update_selection() {
    // Update selection textarea
    let seltextarea = document.getElementById("selection_output");
    selection_history.push(seltextarea.value);
    let firstTreasureMap = {};
    firstTreasureMap.fragmentX = document.getElementById("fragmentX").value | 0;
    firstTreasureMap.fragmentZ = document.getElementById("fragmentZ").value | 0;
    firstTreasureMap.map = Game.getArea(0, 0, 128, 0, 128);
    seedInfo.treasureMaps = [firstTreasureMap];
    seltextarea.value = stringify(seedInfo, { maxLength: 20 });
}

function undo_selection() {
    let x = selection_history.pop();
    if (x != undefined) {
        let seltextarea = document.getElementById("selection_output");
        seltextarea.value = x;
        load_selection();
    }
}

function runWorkers(numWorkers, seedInfo) {
    foundSeeds = [];
    let outta = document.getElementById("output_textarea");
    outta.value = "Calculating...";
    let startedX = 0;
    let x = 0;
    let limit = 1 << 25;
    let range = 1 << 17;
    //let range = (limit / numWorkers);
    for (let workerId = 0; workerId < numWorkers; workerId++) {
        let myWorker = new Worker("treasure_worker.js");
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
            // Draw sample image to canvas
            if (l42AreaC && workerId == 0) {
                Rust.wasm_gui.then(function(wasmgui) {
                    let seltextarea = document.getElementById(
                        "selection_output"
                    );
                    let seedInfo = JSON.parse(seltextarea.value);
                    let minecraft_version = seedInfo.version;
                    let r = wasmgui.generate_rivers_candidate(
                        JSON.stringify({
                            version: minecraft_version,
                            seed: "" + startedX,
                            area: l42AreaC,
                        })
                    );
                    drawMapToCanvas(
                        document.getElementById("mapLayer42Candidate"),
                        r.l42,
                        r.l42Area
                    );
                });
            }
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
    let seedInfo = JSON.parse(seltextarea.value);
    if (window.Worker) {
        let maxWorkers = navigator.hardwareConcurrency || 4;
        runWorkers(maxWorkers, seedInfo);
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

function drawVoronoi() {
    Rust.wasm_gui.then(function(wasmgui) {
        let r = wasmgui.draw_rivers(JSON.stringify({ seedInfo: seedInfo }));
        drawMapToCanvas(
            document.getElementById("mapLayer43"),
            r.l43,
            r.l43Area
        );
        drawMapToCanvas(
            document.getElementById("mapLayer42"),
            r.l42,
            r.l42Area
        );
        l42AreaC = r.l42Area;
    });
}

function drawMapToCanvas(canvas, map, mapArea) {
    console.log(mapArea);
    console.log("w * h * 4 = " + mapArea.w * mapArea.h * 4);
    console.log("map.length = " + map.length);

    let c = canvas;
    c.width = mapArea.w;
    c.height = mapArea.h;
    //c.style.width = "512px";
    //c.style.height = "512px";
    c.style.imageRendering = "pixelated";
    let ctx = c.getContext("2d");
    // Generate fragment
    let imageData = ctx.createImageData(mapArea.w, mapArea.h);
    //imageData.data = rvec; // please
    for (let i = 0; i < map.length; i++) {
        imageData.data[i] = map[i];
    }
    ctx.putImageData(imageData, 0, 0);
}

// format!("#{:02X}{:02X}{:02X}{:02X}", a, r, b, g)
function convertIntsToColor(r, g, b, a) {
    function convertIntToHex2(x) {
        if (x > 0 && x <= 255) {
            let h = x.toString(16);
            if (h.length < 2) {
                h = "0" + h;
            }
            return h;
        } else {
            throw "`" + x + "` is not a valid color value";
        }
    }

    r = convertIntToHex2(r);
    g = convertIntToHex2(g);
    b = convertIntToHex2(b);
    a = convertIntToHex2(a);

    return "#" + r + g + b;
}

function drawTreasureMap() {
    Rust.wasm_gui.then(function(wasmgui) {
        console.log("draw_treasure_map");
        let q = wasmgui.draw_treasure_map(
            JSON.stringify({ seedInfo: seedInfo })
        );
        drawMapToCanvas(document.getElementById("treasureCanvas"), q, {
            w: 128,
            h: 128,
        });
        let r4 = [];
        for (let i = 0; i < 128 * 128 * 4; i += 4) {
            let r = q[i];
            let g = q[i + 1];
            let b = q[i + 2];
            let a = q[i + 3];
            let rgb = convertIntsToColor(r, g, b, a);
            r4.push(rgb);
        }
        Game.setArea(1, 0, 128, 0, 128, r4);
    });
}
// Extra biomes
function addExtraBiome() {
    let x = document.getElementById("extraBiomeX").value | 0;
    let z = document.getElementById("extraBiomeZ").value | 0;
    let biomeId = document.getElementById("extraBiomeId").value;

    Game.setTile(x, z, biomeId);
}

load_selection();
drawTreasureMap();
