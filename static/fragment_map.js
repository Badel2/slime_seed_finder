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
    let a = x % m;
    if (a < 0) {
        a += m;
    }
    return a;
}

//let FRAG_SIZE = 256;
let canvas = document.getElementById("demo");
let CANVAS_W = canvas.width | 0;
let CANVAS_H = canvas.height | 0;
let NUM_LAYERS = 244;

function array_filled_with(length, what) {
    let a = Array(length);
    for (let i = 0; i < length; i++) {
        a[i] = what();
    }
    return a;
}

let map = {
    tsize: null,
    // 2 layers
    layers: array_filled_with(NUM_LAYERS, function() {
        return new Map();
    }),
    generating: array_filled_with(NUM_LAYERS, function() {
        return new Set();
    }),
    getFragment: function(layer, fx, fy) {
        let k = fx + "," + fy;
        let frag = this.layers[layer].get(k);
        if (frag == undefined) {
            // Check if we are already generating this fragment...
            if (!this.generating[layer].has(k)) {
                this.generating[layer].add(k);
                let this_layers_layer = this.layers[layer];
                let this_generating_layer = this.generating[layer];
                this.generateFragment(layer, fx, fy).then(
                    function(value) {
                        //console.log(value); // Success!
                        //console.log("Finished generating fragment: " + fx + ", " + fy);
                        this_layers_layer.set(k, value);
                        Game.dirty = true;
                    },
                    function(reason) {
                        this_generating_layer.delete(k);
                        //Game.dirty = true;
                        console.error(reason); // Error!
                    }
                );
            }
        }
        return frag;
    },
    generateFragment: null,
    currentScale: 1,
};

function Camera(map, width, height) {
    this.x = 0;
    this.y = 0;
    this.width = width;
    this.height = height;
    this.scale = 1.0;
    // If tsize is not integer, white lines appear between fragments
    this.tsize = Math.round(map.tsize * this.scale);
}

Camera.SPEED = 256; // pixels per second

Camera.prototype.move = function(delta, dirx, diry) {
    // move camera
    this.x += dirx * Camera.SPEED * delta * this.scale;
    this.y += diry * Camera.SPEED * delta * this.scale;
};

Camera.prototype.moveRaw = function(dirx, diry) {
    // move camera
    this.x += dirx;
    this.y += diry;
};

Camera.prototype.zoom = function(newF) {
    let old_center_x = (this.x + this.width / 2) / this.tsize - 0.5;
    let old_center_y = (this.y + this.height / 2) / this.tsize - 0.5;
    this.scale *= newF;
    this.scale = Math.max(this.scale, 0.01);
    this.scale = Math.min(this.scale, 2000);
    this.tsize = Math.round(map.tsize * this.scale);
    // Move camera so that center stays constant
    this.centerAt(old_center_x, old_center_y);
};

Camera.prototype.centerAt = function(x, y) {
    this.x = (x + 0.5) * this.tsize - this.width / 2;
    this.y = (y + 0.5) * this.tsize - this.height / 2;
};

Camera.prototype.centerAtBlock = function(x, y) {
    this.x = ((x + 0.5) / map.tsize) * this.tsize - this.width / 2;
    this.y = ((y + 0.5) / map.tsize) * this.tsize - this.height / 2;
};

Camera.prototype.blockAtCenter = function() {
    let x = ((this.x + this.width / 2) / this.tsize) * map.tsize - 0.5;
    let y = ((this.y + this.height / 2) / this.tsize) * map.tsize - 0.5;
    return [x, y];
};

Camera.prototype.resolutionChange = function(f) {
    // The map resolution changes by factor f, meaning 0,0 is still 0,0
    // but 100,100 becomes 100*f,100*f
    let old_xy = this.blockAtCenter();
    this.zoom(f);
    this.centerAtBlock(
        (old_xy[0] + 0.5) / f - 0.5,
        (old_xy[1] + 0.5) / f - 0.5
    );
};

Game.load = function() {
    return [
        //Loader.loadImage('tiles', '../assets/tiles.png'),
    ];
};

Game.init = function(tsize, canvasH, canvasW, activeLayer) {
    map.tsize = tsize;
    this.tileAtlas = Loader.getImage("tiles");
    this.camera = new Camera(map, CANVAS_W, CANVAS_H);
    this.showGrid = true;
    let gs = document.getElementById("gridSize");
    if (gs) {
        this.gridSize = Math.round(Math.pow(2, gs.value));
        gs.oninput = function() {
            Game.gridSize = Math.round(Math.pow(2, gs.value));
            Game.dirty = true;
        };
    } else {
        this.gridSize = map.tsize;
    }
    this.centerAtBlock(0, 0);
    this.layerCanvas = map.layers.map(function() {
        let c = document.createElement("canvas");
        c.width = CANVAS_W;
        c.height = CANVAS_H;
        return c;
    });
    // Disable smoothing, we want a sharp pixelated image
    this.ctx.imageSmoothingEnabled = false;
    // Dirty flag: only render if true, remember to set it when changing state
    this.dirty = true;
    this.activeLayer = activeLayer;
    // Never have more than 3 elements in the cache
    this.lru_cache_size = 3;
    this.maxNumFragmentsOnScreen = 400;
    if (this.afterInit) {
        this.afterInit();
    }
};

Game.update = function(delta) {
    // maybe scroll here?
};

Game._limitZoomToMaxNumFragments = function() {
    // TODO: instead of doing this, just get a rough estimate of the zoom factor needed to draw
    // this.maxNumFragmentsOnScreen, and if the current zoom factor is greater than that, limit
    // it. This is the cause of the stuttering when zooming out to the zoom limit

    // + 1 because when the width is not a multiple of tsize things get weird
    let startCol = Math.floor(this.camera.x / this.camera.tsize);
    let endCol = startCol + this.camera.width / this.camera.tsize + 1;
    let startRow = Math.floor(this.camera.y / this.camera.tsize);
    let endRow = startRow + this.camera.height / this.camera.tsize + 1;
    let offsetX = -this.camera.x + startCol * this.camera.tsize;
    let offsetY = -this.camera.y + startRow * this.camera.tsize;

    //console.log([startCol, endCol, startRow, endRow, offsetX, offsetY]);

    let numFragments = (endCol - startCol + 1) * (endRow - startRow + 1);
    while (numFragments > this.maxNumFragmentsOnScreen) {
        // Change zoom level and retry
        this.zoomBy(1.01);
        startCol = Math.floor(this.camera.x / this.camera.tsize);
        endCol = startCol + this.camera.width / this.camera.tsize + 1;
        startRow = Math.floor(this.camera.y / this.camera.tsize);
        endRow = startRow + this.camera.height / this.camera.tsize + 1;
        offsetX = -this.camera.x + startCol * this.camera.tsize;
        offsetY = -this.camera.y + startRow * this.camera.tsize;
        numFragments = (endCol - startCol + 1) * (endRow - startRow + 1);
    }
};

Game._drawLayer = function(layer) {
    this._limitZoomToMaxNumFragments();
    // + 1 because when the width is not a multiple of tsize things get weird
    let startCol = Math.floor(this.camera.x / this.camera.tsize);
    let endCol = startCol + this.camera.width / this.camera.tsize + 1;
    let startRow = Math.floor(this.camera.y / this.camera.tsize);
    let endRow = startRow + this.camera.height / this.camera.tsize + 1;
    let offsetX = -this.camera.x + startCol * this.camera.tsize;
    let offsetY = -this.camera.y + startRow * this.camera.tsize;

    //console.log([startCol, endCol, startRow, endRow, offsetX, offsetY]);
    let i = 0;
    for (let c = startCol; c <= endCol; c++) {
        for (let r = startRow; r <= endRow; r++) {
            i += 1;
            //console.log(tile);
            let x = (c - startCol) * this.camera.tsize + offsetX;
            let y = (r - startRow) * this.camera.tsize + offsetY;
            let fragmentImage = map.getFragment(
                layer,
                Math.round(c),
                Math.round(r)
            );
            if (fragmentImage != undefined) {
                this.ctx.drawImage(
                    fragmentImage, // image
                    0, // source x
                    0, // source y
                    map.tsize, // source width
                    map.tsize, // source height
                    Math.round(x), // target x
                    Math.round(y), // target y
                    this.camera.tsize, // target width
                    this.camera.tsize // target height
                );
            }
        }
    }
    //console.log("Drawing fragments: " + i);

    //this.ctx.drawImage(fragmentImage, sx, sy, sWidth, sHeight, dx, dy, dWidth, dHeight);

    // Draw grid lines
    if (this.showGrid) {
        // Returns fragment coords
        let mtsize = Math.max(this.gridSize, map.tsize);
        let xyf = this.mouse_coords_to_game_coords_float(0, 0);
        let startCol = Math.floor(xyf[0]) * mtsize;
        let startRow = Math.floor(xyf[1]) * mtsize;
        let xy = this.mouse_coords_to_game_coords_float(CANVAS_W, CANVAS_H);
        let endCol = Math.floor(xy[0] + 1) * mtsize;
        let endRow = Math.floor(xy[1] + 1) * mtsize;

        this.ctx.strokeStyle = "#AAA";
        this.ctx.lineWidth = 1;
        // c and r are world-coordinates
        for (let c = startCol; c <= endCol; c += this.gridSize) {
            // Convert c to fragment-coordinates, and then to mouse/canvas coordinates
            let x = this.game_coords_to_mouse_coords_float(c / mtsize, 0)[0];
            x = Math.round(x);
            this.ctx.beginPath();
            this.ctx.moveTo(x, 0);
            this.ctx.lineTo(x, CANVAS_H);
            this.ctx.stroke();
        }
        for (let r = startRow; r <= endRow; r += this.gridSize) {
            let y = this.game_coords_to_mouse_coords_float(0, r / mtsize)[1];
            y = Math.round(y);
            this.ctx.beginPath();
            this.ctx.moveTo(0, y);
            this.ctx.lineTo(CANVAS_W, y);
            this.ctx.stroke();
        }
    }
};

Game.render = function() {
    if (!this.dirty) {
        return;
    }
    this.dirty = false;
    CANVAS_W = canvas.width | 0;
    CANVAS_H = canvas.height | 0;
    this.camera.width = CANVAS_W;
    this.camera.height = CANVAS_H;
    // Disable smoothing, we want a sharp pixelated image
    this.ctx.imageSmoothingEnabled = false;
    // clear previous frame
    this.ctx.fillStyle = "white";
    this.ctx.fillRect(0, 0, CANVAS_W, CANVAS_H);

    this._drawLayer(this.activeLayer);
};

Game.mouse_coords_to_game_coords_float = function(x, y) {
    let tx = (x + this.camera.x) / this.camera.tsize;
    let ty = (y + this.camera.y) / this.camera.tsize;
    return [tx, ty];
};

Game.game_coords_to_mouse_coords_float = function(tx, ty) {
    let x = tx * this.camera.tsize - this.camera.x;
    let y = ty * this.camera.tsize - this.camera.y;
    return [x, y];
};

Game.mouse_coords_to_game_coords = function(x, y) {
    let txty = this.mouse_coords_to_game_coords_float(x, y);
    let tx = txty[0];
    let ty = txty[1];
    tx = Math.floor(tx);
    ty = Math.floor(ty);
    return [tx, ty];
};

Game.clickTile = function(x, y) {};

// Get all the (x, y) pairs from a layer with the given value
Game.getSelection = function(layer, value) {};

Game.load_layers_from_cache = function(cache_key) {
    if (!this.lru_cache) {
        this.lru_cache = new Map();
    }
    let cached_layers = this.lru_cache.get(cache_key);
    if (cached_layers) {
        map.layers = cached_layers;
    } else {
        map.layers = array_filled_with(NUM_LAYERS, function() {
            return new Map();
        });
    }
    // Hopefully update insertion order to make this the most recent
    this.save_layers_to_cache(cache_key);
};

Game.save_layers_to_cache = function(cache_key) {
    this.lru_cache.set(cache_key, map.layers);
    if (this.lru_cache.size > this.lru_cache_size) {
        this.lru_cache.delete(this.lru_cache.keys().next().value);
    }
};

Game.clearLruCache = function() {
    this.lru_cache = new Map();
};
Game.setLruCacheSize = function(size) {
    this.lru_cache_size = size;
    // If shrinking, delete extra elements
    while (this.lru_cache.size > this.lru_cache_size) {
        this.lru_cache.delete(this.lru_cache.keys().next().value);
    }
};

Game.clear = function(cache_key) {
    if (cache_key) {
        // Load state from the LRU cache using this KEY
        Game.load_layers_from_cache(cache_key);
    } else {
        map.layers = array_filled_with(NUM_LAYERS, function() {
            return new Map();
        });
    }
    map.generating = array_filled_with(NUM_LAYERS, function() {
        return new Set();
    });
    this.dirty = true;
};

Game.setSelection = function(layer, value, keys) {};

Game.scrollBy = function(x, y) {
    this.dirty = true;
    this.camera.moveRaw(x, y);
};

Game.zoomBy = function(f) {
    this.dirty = true;
    this.camera.zoom(f);
    this._limitZoomToMaxNumFragments();
};

Game.centerAt = function(x, y) {
    this.dirty = true;
    this.camera.centerAt(Number.parseInt(x), Number.parseInt(y));
};

Game.centerAtBlock = function(x, y) {
    this.dirty = true;
    this.camera.centerAtBlock(Number.parseInt(x), Number.parseInt(y));
};

Game.setActiveLayer = function(layer) {
    this.dirty = true;
    this.activeLayer = layer;
};

Game.getPixelAtGameCoords = function(x, y) {
    // x and y are fragment coords (float)
    let fragX = Math.floor(x);
    let fragY = Math.floor(y);
    let innerX = Math.floor((x - fragX) * map.tsize);
    let innerY = Math.floor((y - fragY) * map.tsize);
    if (innerX >= map.tsize) {
        innerX = 0;
        fragX += 1;
    }
    if (innerY >= map.tsize) {
        innerY = 0;
        fragY += 1;
    }

    let layer = this.activeLayer;

    let fragmentImage = map.getFragment(layer, fragX, fragY);
    if (!fragmentImage) {
        return undefined;
    }

    let context = fragmentImage.getContext("2d");

    let imgd = context.getImageData(innerX, innerY, 1, 1);
    let data = imgd.data;

    let [r, g, b, a] = data;

    function toHex(x, minLength = 0) {
        let s = x.toString(16);
        while (s.length < minLength) {
            s = "0" + s;
        }
        return s;
    }

    return "#" + toHex(r, 2) + toHex(g, 2) + toHex(b, 2);
};
