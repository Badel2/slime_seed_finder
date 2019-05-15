            // common.js
//
// start up function
//

window.onload = function () {
    var context = document.getElementById('demo').getContext('2d');
    var pos_div = document.getElementById('position_info');
    var center_butt = document.getElementById('center_button');
    center_butt.onclick = function() {
        var x = document.getElementById('center_x').value;
        var z = document.getElementById('center_z').value;
        // Center at block if supported, otherwise center at chunk/fragment
        if(Game.centerAtBlock) {
            Game.centerAtBlock(x, z);
        } else {
            Game.centerAt(x, z);
        }
    };
    var elem = document.getElementById('demo'),
    elemLeft = elem.offsetLeft,
    elemTop = elem.offsetTop,
    context = elem.getContext('2d'),
    elements = [];
    var dragging = null;

    // Add event listener for `click` events.
    // TODO: touchstart for mobile support
    // https://stackoverflow.com/a/16284281
    var pointerEventToXY = function(e){
      var out = {x:0, y:0};
      if(e.type == 'touchstart' || e.type == 'touchmove' || e.type == 'touchend' || e.type == 'touchcancel'){
        var touch = e.touches[0] || e.changedTouches[0];
        out.x = touch.pageX;
        out.y = touch.pageY;
      } else if (e.type == 'mousedown' || e.type == 'mouseup' || e.type == 'mousemove' || e.type == 'mouseover'|| e.type=='mouseout' || e.type=='mouseenter' || e.type=='mouseleave') {
        out.x = e.pageX;
        out.y = e.pageY;
      }
      return out;
    };

    ['touchstart', 'mousedown'].forEach(function(n) {
        elem.addEventListener(n, function(e) {
            //console.log('elem mousedown');
            if (n == 'touchstart') {
                e.preventDefault();
            }
            var pointer = pointerEventToXY(e);
            var x = pointer.x - elemLeft,
                y = pointer.y - elemTop;

            dragging = {x: x, y: y, actuallyScrolling: false};
        }, false)
    });
    ['touchmove', 'mousemove'].forEach(function(n) {
        elem.addEventListener(n, function(e) {
            //console.log('elem mousemove');
            var pointer = pointerEventToXY(e);
            var x = pointer.x - elemLeft,
                y = pointer.y - elemTop;
            var txty = Game.mouse_coords_to_game_coords_float(x, y);
            var tx = txty[0];
            var ty = txty[1];
            if (map.getFragment) {
                pos_div.innerHTML = "Fragment x: " + Math.floor(tx) + ", z: " + Math.floor(ty);
                pos_div.innerHTML += " --- Block x: " + Math.floor(tx*FRAG_SIZE) + ", z: " + Math.floor(ty*FRAG_SIZE);
            } else {
                pos_div.innerHTML = "Chunk x: " + Math.floor(tx) + ", z: " + Math.floor(ty);
                pos_div.innerHTML += " --- Block x: " + Math.floor(tx*16) + ", z: " + Math.floor(ty*16);
            }
        }, false)
    });

    ['touchmove', 'mousemove'].forEach(function(n) {
        window.addEventListener(n, function(e) {
            if (dragging) {
                var pointer = pointerEventToXY(e);
                var x = pointer.x - elemLeft,
                    y = pointer.y - elemTop;
                if (dragging.actuallyScrolling == false && (Math.abs(dragging.x - x) > 10 || Math.abs(dragging.y - y) > 10)) {
                    // Moving more than 10 pixels from the initial position starts the scrolling

                    dragging.actuallyScrolling = true;
                }
                var tool = document.getElementById('toolSelector').value;
                var scrollingEnabled = tool == 'click' || tool == 'move' || tool == 'bucket' || tool == 'bucket_erase';
                if (scrollingEnabled && dragging.actuallyScrolling) {
                    Game.scrollBy(dragging.x - x, dragging.y - y);
                    dragging.x = x;
                    dragging.y = y;
                }
                if (!scrollingEnabled && dragging.actuallyScrolling) {
                    if (tool == 'pencil') {
                        Game.setTile(x, y, 1);
                    } else if (tool == 'pencil_erase') {
                        Game.setTile(x, y, 0);
                    } else {
                        console.error('Unhandled tool type');
                    }
                    dragging.x = x;
                    dragging.y = y;

                    update_selection();
                    // Draw rendered map
                    drawVoronoi();
                }
            }
        }, false)
    });

    ['touchend', 'mouseup'].forEach(function(n) {
        elem.addEventListener(n, function(e) {
            //console.log('elem mouseup');
            if (n == 'touchend') {
                e.preventDefault();
            }
            var pointer = pointerEventToXY(e);
            var x = pointer.x - elemLeft,
                y = pointer.y - elemTop;

            if (dragging == null) {
                // The window event handler was executed first, gg
                console.error('BUG: The window event handler was executed before the elem event handler for event mouseup');
            }
            if (dragging && dragging.actuallyScrolling == false) {
                var tool = document.getElementById('toolSelector').value;
                if (tool == 'click') {
                    Game.clickTile(x, y);
                } else if(tool == 'pencil') {
                    Game.setTile(x, y, 1);
                } else if(tool == 'pencil_erase') {
                    Game.setTile(x, y, 0);
                } else if(tool == 'bucket') {
                    Game.bucketTool(x, y, 1);
                } else if(tool == 'bucket_erase') {
                    Game.bucketTool(x, y, 0);
                }

                update_selection();
                // Draw rendered map
                drawVoronoi();
            }
            dragging = null;
        }, false)
    });

    ['touchend', 'mouseup'].forEach(function(n) {
        window.addEventListener(n, function(e) {
            if (n == 'touchend') {
                // This breaks the page
                //e.preventDefault();
            }
            dragging = null;
        }, false)
    });

    // Update selection textarea
    var seltextarea = document.getElementById('selection_output');
    if (seltextarea && seltextarea.value == "") {
        seltextarea.value = stringify({
            version: "1.7",
            biomes: {
                7: Game.getSelection(0, 1),
            },
        }, { maxLength: 20 });
    }

    var tsize = 256;
    fitToContainer(elem);
    var canvasW = elem.width;
    var canvasH = elem.height;
    Game.run(context, tsize, canvasW, canvasH);
};

// end common.js

function showProgressBar(currentValue, maxValue) {
    let x = document.getElementById("progressBarLabel");
    x.dataset.label = currentValue + " / " + maxValue;
    let percent = currentValue * 100 / maxValue;
    let y = document.getElementById("progressBarSpan");
    y.style.width = percent + "%";
}
function fitToContainer(canvas){
    // Make it visually fill the positioned parent
   canvas.style.width ='100%';
   //canvas.style.height='100%';
   // ...then set the internal size to match
   canvas.width  = canvas.offsetWidth;
   //canvas.height = canvas.offsetHeight;
}

let seedInfo = {"version": "1.7"};
var l42AreaC = null;
let foundSeeds = [];
let workers = [];
var selection_history = []

// From seedInfo textarea to seedInfo and map
function load_selection() {
    var seltextarea = document.getElementById('selection_output');
    try {
        var x = JSON.parse(seltextarea.value);
        seedInfo = x;
        Game.clearSelection(0);
        Game.setSelection(0, 1, x.biomes[7]);
        drawVoronoi();
    } catch(e) {
        // Invalid JSON
    }
}
// From map to seedInfo and seedInfo textarea
function update_selection() {
    // Update selection textarea
    var seltextarea = document.getElementById('selection_output');
    selection_history.push(seltextarea.value);
    if (!seedInfo.biomes) {
        seedInfo.biomes = {};
    }
    seedInfo.biomes["7"] = Game.getSelection(0, 1);
    seltextarea.value = stringify(seedInfo, { maxLength: 20 });
}

function undo_selection() {
    var x = selection_history.pop();
    if (x != undefined) {
        var seltextarea = document.getElementById('selection_output');
        seltextarea.value = x;
        load_selection();
    }
}

function runWorkers(numWorkers, seedInfo) {
    foundSeeds = [];
    let outta = document.getElementById('output_textarea');
    outta.value = "Calculating...";
    let startedX = 0;
    let x = 0;
    let limit = 1 << 25;
    let range = 1 << 17;
    //let range = (limit / numWorkers);
    for(let workerId = 0; workerId < numWorkers; workerId++) {
            let myWorker = new Worker("rivers_worker.js");
            workers.push(myWorker);
            showProgressBar(x / range, limit / range);
            myWorker.postMessage(JSON.stringify({ seedInfo: seedInfo, range: [startedX, startedX + range] }));
            startedX += range;
            console.log('Message posted to worker');
            myWorker.onmessage = function(e) {
                console.log('Got message from worker ' + workerId);
                x += range;
                showProgressBar(x / range, limit / range);
                foundSeeds = [...foundSeeds, ...e.data];
                outta.value = stringify({ foundSeeds: foundSeeds }, {maxLength: 20 });
                // Draw sample image to canvas
                if(l42AreaC && workerId == 0) {
                    Rust.wasm_gui.then( function( wasmgui ) {
                        let seltextarea = document.getElementById('selection_output');
                        let seedInfo = JSON.parse(seltextarea.value);
                        var r = wasmgui.generate_rivers_candidate(JSON.stringify({ seed: ""+startedX, area: l42AreaC}));
                        drawMapToCanvas(document.getElementById("mapLayer42Candidate"), r.l42, r.l42Area);
                    });
                }
                if(startedX < limit) {
                    myWorker.postMessage(JSON.stringify({ seedInfo: seedInfo, range: [startedX, startedX + range] }));
                    startedX += range;
                } else if(x < limit) {
                    // Waiting for other workers to finish
                } else {
                    searchFinished = true;
                    workers = [];
                    alert("Done! Found " + foundSeeds.length + " seeds");
                }
            };
    }
}
function runGui() {
    let seltextarea = document.getElementById('selection_output');
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
    Rust.wasm_gui.then( function( wasmgui ) {
        let outta = document.getElementById('num_candidates');
        outta.value = wasmgui.count_rivers(JSON.stringify({ seedInfo: seedInfo }));
    }, function( err ) {
        console.error(err);
    });
}

function drawVoronoi() {
    Rust.wasm_gui.then( function( wasmgui ) {
        var r = wasmgui.draw_rivers(JSON.stringify({ seedInfo: seedInfo }));
        drawMapToCanvas(document.getElementById("mapLayer43"), r.l43, r.l43Area);
        drawMapToCanvas(document.getElementById("mapLayer42"), r.l42, r.l42Area);
        l42AreaC = r.l42Area;
    });
}

function drawMapToCanvas(canvas, map, mapArea) {
    console.log(mapArea);
    console.log("w * h * 4 = " + mapArea.w * mapArea.h * 4);
    console.log("map.length = " + map.length);

    var c = canvas;
    c.width = mapArea.w;
    c.height = mapArea.h;
    //c.style.width = "512px";
    //c.style.height = "512px";
    c.style.imageRendering = "pixelated"
    var ctx = c.getContext('2d');
    // Generate fragment
    var imageData = ctx.createImageData(mapArea.w, mapArea.h);
    //imageData.data = rvec; // please
    for(var i=0; i<map.length; i++) {
        imageData.data[i] = map[i];
    }
    ctx.putImageData(imageData, 0, 0);
}
// Extra biomes
function addExtraBiome() {
    var x = document.getElementById("extraBiomeX").value|0;
    var z = document.getElementById("extraBiomeZ").value|0;
    var biomeId = document.getElementById("extraBiomeId").value;
    if (!seedInfo.biomes) { seedInfo.biomes = {}; }
    if (!seedInfo.biomes[biomeId]) {
        seedInfo.biomes[biomeId] = [];
    }
    seedInfo.biomes[biomeId].push([x, z]);

    let seltextarea = document.getElementById('selection_output');
    seltextarea.value = stringify(seedInfo, {maxLength: 20});
    if(biomeId == 7) { // on rivers load river map
        load_selection();
    }
}

load_selection();

