// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// Original file from:
// https://github.com/mozdevs/gamedev-js-tiles
// Modified version can be found at:
// https://github.com/Badel2/inf-proc-gen-tilemap

//
// Asset loader
//

let Loader = {
    images: {},
};

Loader.loadImage = function(key, src) {
    let img = new Image();

    let d = new Promise(
        function(resolve, reject) {
            img.onload = function() {
                this.images[key] = img;
                resolve(img);
            }.bind(this);

            img.onerror = function() {
                reject("Could not load image: " + src);
            };
        }.bind(this)
    );

    img.src = src;
    return d;
};

Loader.getImage = function(key) {
    return key in this.images ? this.images[key] : null;
};

//
// Game object
//

let Game = {};

Game.run = function(context, tsize, canvasW, canvasH, activeLayer) {
    this.ctx = context;
    this._previousElapsed = 0;

    let p = this.load();
    Promise.all(p).then(
        function(loaded) {
            this.init(tsize, canvasW, canvasH, activeLayer);
            window.requestAnimationFrame(this.tick);
        }.bind(this)
    );
};

Game.tick = function(elapsed) {
    window.requestAnimationFrame(this.tick);
    // compute delta time in seconds -- also cap it
    let delta = (elapsed - this._previousElapsed) / 1000.0;
    delta = Math.min(delta, 0.25); // maximum delta of 250 ms
    this._previousElapsed = elapsed;

    this.update(delta);
    this.render();
}.bind(Game);

// override these methods to create the demo
Game.init = function() {};
Game.update = function(delta) {};
Game.render = function() {};

//
// start up function
//

//window.onload = function () {
function startGame(lastLayer, callbackPixelColor = null) {
    let pos_div = document.getElementById("position_info");
    let center_butt = document.getElementById("center_button");
    center_butt.onclick = function() {
        let x = document.getElementById("center_x").value;
        let z = document.getElementById("center_z").value;
        // Center at block if supported, otherwise center at chunk/fragment
        if (Game.centerAtBlock) {
            Game.centerAtBlock(x, z);
        } else {
            Game.centerAt(x, z);
        }
    };
    let elem = document.getElementById("demo");
    let elemLeft = elem.offsetLeft;
    let elemTop = elem.offsetTop;
    let context = elem.getContext("2d");
    let elements = [];
    let dragging = null;
    let prev_impetus = [];
    let prev_mousemove = [];

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

                dragging = { x: null, y: null, actuallyScrolling: false };
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
                prev_mousemove = [x, y];
                let txty = Game.mouse_coords_to_game_coords_float(x, y);
                let tx = txty[0];
                let ty = txty[1];
                if (map.getFragment) {
                    pos_div.innerHTML =
                        "Fragment x: " +
                        Math.floor(tx) +
                        ", z: " +
                        Math.floor(ty);
                    pos_div.innerHTML +=
                        " --- Block x: " +
                        Math.floor(tx * FRAG_SIZE * map.currentScale) +
                        ", z: " +
                        Math.floor(ty * FRAG_SIZE * map.currentScale);
                } else {
                    pos_div.innerHTML =
                        "Chunk x: " + Math.floor(tx) + ", z: " + Math.floor(ty);
                    pos_div.innerHTML +=
                        " --- Block x: " +
                        Math.floor(tx * 16) +
                        ", z: " +
                        Math.floor(ty * 16);
                }
                if (callbackPixelColor) {
                    let pixelColor = Game.getPixelAtGameCoords(tx, ty);
                    callbackPixelColor(pixelColor);
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

                if (dragging && dragging.actuallyScrolling == false) {
                    Game.clickTile(x, y);

                    // Update selection textarea
                    let seltextarea = document.getElementById(
                        "selection_output"
                    );
                    if (seltextarea) {
                        seltextarea.value = stringify(
                            {
                                version: "1.7",
                                seedInfo: "0.1",
                                slimeChunks: Game.getSelection(0, 1),
                                negative: {
                                    slimeChunks: Game.getSelection(0, 2),
                                },
                            },
                            { maxLength: 20 }
                        );
                    }
                    // This disables scrolling until the next "mousedown" event
                    // It is needed to ensure that scrolling will not start after the mousedown
                    // because of momentum
                    dragging.actuallyScrolling = null;
                }
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
                if (dragging && dragging.actuallyScrolling === false) {
                    dragging.actuallyScrolling = null;
                }
            },
            false
        );
    });

    // Zoom inspired by
    // https://stackoverflow.com/a/5526721
    ["DOMMouseScroll", "mousewheel"].forEach(function(n) {
        elem.addEventListener(
            n,
            function(e) {
                let delta = e.wheelDelta
                    ? e.wheelDelta / 40
                    : e.detail
                    ? -e.detail
                    : 0;
                if (delta) {
                    let factor = Math.pow(1.05, delta);
                    let canvasW = elem.clientWidth;
                    let canvasH = elem.clientHeight;
                    if (prev_mousemove.length == 2 && canvasW && canvasH) {
                        let tx = prev_mousemove[0] - canvasW / 2;
                        let ty = prev_mousemove[1] - canvasH / 2;
                        Game.scrollBy(tx, ty);
                        Game.zoomBy(factor);
                        Game.scrollBy(-tx, -ty);
                    } else {
                        // Zoom around center
                        Game.zoomBy(factor);
                    }
                }
                return e.preventDefault() && false;
            },
            false
        );
    });
    var zoom = function(clicks) {
        var pt = ctx.transformedPoint(lastX, lastY);
        ctx.translate(pt.x, pt.y);
        var factor = Math.pow(scaleFactor, clicks);
        ctx.scale(factor, factor);
        ctx.translate(-pt.x, -pt.y);
        redraw();
    };

    // Update selection textarea
    let seltextarea = document.getElementById("selection_output");
    if (seltextarea && seltextarea.value == "") {
        seltextarea.value = stringify(
            {
                version: "1.7",
                seedInfo: "0.1",
                slimeChunks: Game.getSelection(0, 1),
                negative: {
                    slimeChunks: Game.getSelection(0, 2),
                },
            },
            { maxLength: 20 }
        );
    }

    Game.impetus = new Impetus({
        source: elem,
        friction: 0.92,
        update: function(imp_x, imp_y) {
            if (dragging && dragging.actuallyScrolling === false) {
                if (dragging.x == null && dragging.y == null) {
                    dragging.x = imp_x;
                    dragging.y = imp_y;
                }
                // Start scrolling after moving 15 units away from the starting point
                if (
                    Math.abs(dragging.x - imp_x) > 15 ||
                    Math.abs(dragging.y - imp_y) > 15
                ) {
                    dragging.actuallyScrolling = true;
                }
            }
            let x = imp_x - prev_impetus[0];
            let y = imp_y - prev_impetus[1];
            if (prev_impetus.length == 2 && dragging.actuallyScrolling) {
                // Movement threshold set to 0.1 units, moving 0.0 units should not cause a redraw
                if (!(Math.abs(x) < 0.1 && Math.abs(y) < 0.1)) {
                    // Maximum movement set to 500 units
                    let scale = 1;
                    if (x > 500 && x >= y) {
                        scale = 500 / x;
                    } else if (y > 500) {
                        scale = 500 / y;
                    }

                    Game.scrollBy(x * -scale, y * -scale);
                }
            }
            prev_impetus = [imp_x, imp_y];
        },
    });

    let tsize = 256;
    // TODO: canvasW and canvasH are not used
    let canvasW = elem.style.width;
    let canvasH = elem.style.height;
    Game.run(context, tsize, canvasW, canvasH, lastLayer);
}

function load_selection() {
    let seltextarea = document.getElementById("selection_output");
    let x = JSON.parse(seltextarea.value);
    Game.clearSelection(0);
    Game.setSelection(0, 1, x.slimeChunks);
    Game.setSelection(0, 2, x.negative.slimeChunks);
}
