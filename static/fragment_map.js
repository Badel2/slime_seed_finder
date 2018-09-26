// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// Original file from:
// https://github.com/mozdevs/gamedev-js-tiles
// Modified version can be found at:
// https://github.com/Badel2/inf-proc-gen-tilemap

// In javascript % return the remainder (can be negative)
// We want the modulus, so we force the value to be positive
function mod(x, m) {
    var a = x % m;
    if (a < 0) {
        a += m;
    }
    return a;
}

var FRAG_SIZE = 512;

function generateTile(x, y) {
    //return [mod((x * 123 + y * y * 37), 255), 0, 0, 255];
    var fx = Math.floor(x / FRAG_SIZE);
    var fy = Math.floor(y / FRAG_SIZE);
    // Just some testing pattern where each fragment is easy to distinguish
    var mx = mod(x, 256);
    var my = mod(y, 256);
    var mxy = mod(x+y, 2) * 255;
    var mm = mod(fy, 2) == 0 ? mx : my;
    switch (mod(fx+fy*3, 9)) {
        case 0: case 2: case 6: case 8: return [0, mxy, mm, 255];
        case 1: case 3: case 5: case 7: return [mm, mxy, 0, 255];
        case 4: return [mxy, mm, 0, 255];
    }
}

function generateFragment(fx, fy) {
    return new Promise(
        function(resolve, reject) {
            console.log("Generating fragment: " + fx + ", " + fy);
            // Create off-screen canvas
            var c = document.createElement('canvas');
            c.width = FRAG_SIZE;
            c.height = FRAG_SIZE;
            var ctx = c.getContext('2d');
            // Generate fragment
            var imageData = ctx.createImageData(FRAG_SIZE, FRAG_SIZE);
            for(var x=0; x<FRAG_SIZE; x++) {
                for(var y=0; y<FRAG_SIZE; y++) {
                    var pixel = generateTile(fx * FRAG_SIZE + x, fy * FRAG_SIZE + y);
                    var i = ((y * FRAG_SIZE) + x) * 4;
                    var colorIndices = [i, i+1, i+2, i+3];

                    var redIndex = colorIndices[0];
                    var greenIndex = colorIndices[1];
                    var blueIndex = colorIndices[2];
                    var alphaIndex = colorIndices[3];

                    imageData.data[redIndex] = pixel[0];
                    imageData.data[greenIndex] = pixel[1];
                    imageData.data[blueIndex] = pixel[2];
                    imageData.data[alphaIndex] = pixel[3];
                }
            }
            ctx.putImageData(imageData, 0, 0);
            resolve(c);
        }
    );
}

var map = {
    tsize: FRAG_SIZE,
    // 2 layers
    layers: Array(2).fill(new Map()),
    generating: Array(2).fill(new Set()),
    getFragment: function (layer, fx, fy) {
        var k = fx + "," + fy;
        var frag = this.layers[layer].get(k);
        if (frag == undefined) {
            // Check if we are already generating this fragment...
            if (!this.generating[layer].has(k)) {
                this.generating[layer].add(k);
                var this_layer = this.layers[layer];
                this.generateFragment(fx, fy).then(function(value) {
                    //console.log(value); // Success!
                    console.log("Finished generating fragment: " + fx + ", " + fy);
                    this_layer.set(k, value);
                    Game.dirty = true;
                }, function(reason) {
                    console.error(reason); // Error!
                });
            }
        }
        return frag;
    },
    generateFragment: generateFragment,
};

function Camera(map, width, height) {
    this.x = 0;
    this.y = 0;
    this.width = width;
    this.height = height;
    this.scale = 8.0;
    // If tsize is not integer, white lines appear between fragments
    this.tsize = Math.round(map.tsize * this.scale);
}

Camera.SPEED = 256; // pixels per second

Camera.prototype.move = function (delta, dirx, diry) {
    // move camera
    this.x += (dirx * Camera.SPEED * delta) * this.scale;
    this.y += (diry * Camera.SPEED * delta) * this.scale;
};

Camera.prototype.moveRaw = function (dirx, diry) {
    // move camera
    this.x += dirx;
    this.y += diry;
};

Camera.prototype.zoom = function (newF) {
    var old_center_x = (this.x + this.width / 2) / this.tsize - 0.5;
    var old_center_y = (this.y + this.height / 2) / this.tsize - 0.5;
    this.scale *= newF;
    this.scale = Math.max(this.scale, 0.01);
    this.tsize = Math.round(map.tsize * this.scale);
    // Move camera so that center stays constant
    this.centerAt(old_center_x, old_center_y);
};

Camera.prototype.centerAt = function (x, y) {
    this.x = (x + 0.5) * this.tsize - this.width / 2;
    this.y = (y + 0.5) * this.tsize - this.height / 2;
}

Camera.prototype.centerAtBlock = function (x, y) {
    this.x = ((x + 0.5) / map.tsize) * this.tsize - this.width / 2;
    this.y = ((y + 0.5) / map.tsize) * this.tsize - this.height / 2;
}

Game.load = function () {
    return [
        //Loader.loadImage('tiles', '../assets/tiles.png'),
    ];
};

Game.init = function () {
    this.tileAtlas = Loader.getImage('tiles');
    this.camera = new Camera(map, 512, 512);
    this.showGrid = true;
    var gs = document.getElementById('gridSize');
    if (gs) {
        this.gridSize = Math.round(Math.pow(2, gs.value));
        gs.oninput = function() {
            Game.gridSize = Math.round(Math.pow(2, gs.value));
            Game.dirty = true;
        }
    } else {
        this.gridSize = map.tsize;
    }
    this.centerAt(0, 0);
    this.layerCanvas = map.layers.map(function() {
        var c = document.createElement('canvas');
        c.width = 512;
        c.height = 512;
        return c;
    });
    // Disable smoothing, we want a sharp pixelated image
    this.ctx.imageSmoothingEnabled = false;
    // Dirty flag: only render if true, remember to set it when changing state
    this.dirty = true;
};

Game.update = function (delta) {
    // maybe scroll here?
};

Game._drawLayer = function (layer) {
    // + 1 because when the width is not a multiple of tsize things get weird
    var startCol = Math.floor(this.camera.x / this.camera.tsize);
    var endCol = startCol + (this.camera.width / this.camera.tsize) + 1;
    var startRow = Math.floor(this.camera.y / this.camera.tsize);
    var endRow = startRow + (this.camera.height / this.camera.tsize) + 1;
    var offsetX = -this.camera.x + startCol * this.camera.tsize;
    var offsetY = -this.camera.y + startRow * this.camera.tsize;

    //console.log([startCol, endCol, startRow, endRow, offsetX, offsetY]);

    var i = 0;
    for (var c = startCol; c <= endCol; c++) {
        for (var r = startRow; r <= endRow; r++) {
            i += 1;
            //console.log(tile);
            var x = (c - startCol) * this.camera.tsize + offsetX;
            var y = (r - startRow) * this.camera.tsize + offsetY;
            var fragmentImage = map.getFragment(layer, Math.round(c), Math.round(r));
            if (fragmentImage != undefined) {
                this.ctx.drawImage(
                    fragmentImage, // image
                    0, // source x
                    0, // source y
                    map.tsize, // source width
                    map.tsize, // source height
                    Math.round(x),  // target x
                    Math.round(y), // target y
                    this.camera.tsize, // target width
                    this.camera.tsize // target height
                );
            }
        }
    }
    console.log("Drawing fragments: " + i);

    //this.ctx.drawImage(fragmentImage, sx, sy, sWidth, sHeight, dx, dy, dWidth, dHeight);

    // Draw grid lines
    if (this.showGrid) {
        // Returns fragment coords
        var xy = this.mouse_coords_to_game_coords_float(0, 0);
        var startCol = Math.floor(xy[0]) * map.tsize;
        var startRow = Math.floor(xy[1]) * map.tsize;
        var xy = this.mouse_coords_to_game_coords_float(512, 512);
        var endCol= Math.floor(xy[0] + 1) * map.tsize;
        var endRow = Math.floor(xy[1] + 1) * map.tsize;

        this.ctx.strokeStyle = "#AAA";
        this.ctx.lineWidth = 1;
        // c and r are world-coordinates
        for (var c = startCol; c <= endCol; c += this.gridSize) {
            // Convert c to fragment-coordinates, and then to mouse/canvas coordinates
            var x = this.game_coords_to_mouse_coords_float(c / map.tsize, 0)[0];
            x = Math.round(x);
            this.ctx.beginPath();
            this.ctx.moveTo(x, 0);
            this.ctx.lineTo(x, 512);
            this.ctx.stroke();
        }
        for (var r = startRow; r <= endRow; r += this.gridSize) {
            var y = this.game_coords_to_mouse_coords_float(0, r / map.tsize)[1];
            y = Math.round(y);
            this.ctx.beginPath();
            this.ctx.moveTo(0, y);
            this.ctx.lineTo(512, y);
            this.ctx.stroke();
        }
    }
};

Game.render = function () {
    if (!this.dirty) { return; }
    this.dirty = false;
    // clear previous frame
    this.ctx.fillStyle = "white";
    this.ctx.fillRect(0, 0, 512, 512);
    // draw map background layer
    this._drawLayer(0);
    // draw map top layer
    this._drawLayer(1);
};

Game.mouse_coords_to_game_coords_float = function(x, y) {
    var tx = (x + this.camera.x) / this.camera.tsize;
    var ty = (y + this.camera.y) / this.camera.tsize;
    return [tx, ty];
};

Game.game_coords_to_mouse_coords_float = function(tx, ty) {
    var x = (tx * this.camera.tsize) - this.camera.x;
    var y = (ty * this.camera.tsize) - this.camera.y;
    return [x, y];
};

Game.mouse_coords_to_game_coords = function(x, y) {
    var txty = this.mouse_coords_to_game_coords_float(x, y);
    var tx = txty[0];
    var ty = txty[1];
    tx = Math.floor(tx);
    ty = Math.floor(ty);
    return [tx, ty];
};

Game.clickTile = function(x, y) {
};

// Get all the (x, y) pairs from a layer with the given value
Game.getSelection = function(layer, value) {
};

Game.clear = function(layer) {
    map.layers = Array(2).fill(new Map());
    map.generating = Array(2).fill(new Set());
    this.dirty = true;
};

Game.setSelection = function(layer, value, keys) {
};

Game.scrollBy = function(x, y) {
    this.dirty = true;
    this.camera.moveRaw(x, y);
};

Game.zoomBy = function(f) {
    this.dirty = true;
    this.camera.zoom(f);
};

Game.centerAt = function(x, y) {
    this.dirty = true;
    this.camera.centerAt(Number.parseInt(x), Number.parseInt(y));
};

Game.centerAtBlock = function(x, y) {
    this.dirty = true;
    this.camera.centerAtBlock(Number.parseInt(x), Number.parseInt(y));
}
