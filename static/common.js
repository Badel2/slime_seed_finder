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

var Loader = {
    images: {}
};

Loader.loadImage = function (key, src) {
    var img = new Image();

    var d = new Promise(function (resolve, reject) {
        img.onload = function () {
            this.images[key] = img;
            resolve(img);
        }.bind(this);

        img.onerror = function () {
            reject('Could not load image: ' + src);
        };
    }.bind(this));

    img.src = src;
    return d;
};

Loader.getImage = function (key) {
    return (key in this.images) ? this.images[key] : null;
};

//
// Game object
//

var Game = {};

Game.run = function (context) {
    this.ctx = context;
    this._previousElapsed = 0;

    var p = this.load();
    Promise.all(p).then(function (loaded) {
        this.init();
        window.requestAnimationFrame(this.tick);
    }.bind(this));
};

Game.tick = function (elapsed) {
    window.requestAnimationFrame(this.tick);
    // compute delta time in seconds -- also cap it
    var delta = (elapsed - this._previousElapsed) / 1000.0;
    delta = Math.min(delta, 0.25); // maximum delta of 250 ms
    this._previousElapsed = elapsed;

    this.update(delta);
    this.render();
}.bind(Game);

// override these methods to create the demo
Game.init = function () {};
Game.update = function (delta) {};
Game.render = function () {};

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
                if (dragging.actuallyScrolling) {
                    Game.scrollBy(dragging.x - x, dragging.y - y);
                    dragging.x = x;
                    dragging.y = y;
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
                Game.clickTile(x, y);

                // Update selection textarea
                var seltextarea = document.getElementById('selection_output');
                seltextarea.value = stringify({
                    slime_chunks: Game.getSelection(0, 1),
                    slime_chunks_not: Game.getSelection(0, 2)
                }, { maxLength: 20 });
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
            slime_chunks: Game.getSelection(0, 1),
            slime_chunks_not: Game.getSelection(0, 2)
        }, { maxLength: 20 });
    }

    Game.run(context);
};

function load_selection() {
    var seltextarea = document.getElementById('selection_output');
    var x = JSON.parse(seltextarea.value);
    Game.clearSelection(0);
    Game.setSelection(0, 1, x.slime_chunks);
    Game.setSelection(0, 2, x.slime_chunks_not);
}
