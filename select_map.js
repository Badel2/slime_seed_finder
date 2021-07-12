// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// Original file from:
// https://github.com/mozdevs/gamedev-js-tiles
// Modified version can be found at:
// https://github.com/Badel2/inf-proc-gen-tilemap

let map = {
    tsize: 1,
    // 2 layers
    layers: Array(2).fill(new Map()),
    getTile: function(layer, col, row) {
        // We must use a string as a key because two arrays
        // with are same elements are not equal according to js
        // [0, 0] != [0, 0]
        let k = col + "," + row;
        return this.layers[layer].get(k);
    },
    setTile: function(layer, col, row, value) {
        let k = col + "," + row;
        if (value == 0) {
            // No need to store "empty" tiles
            this.layers[layer].delete(k);
        } else {
            this.layers[layer].set(k, value);
        }
    },
};

function Camera(map, width, height) {
    this.x = 0;
    this.y = 0;
    this.width = width;
    this.height = height;
    this.scale = 32.0;
    this.tsize = map.tsize * this.scale;
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
    this.scale = Math.max(this.scale, 0.1);
    this.tsize = map.tsize * this.scale;
    // Move camera so that center stays constant
    this.centerAt(old_center_x, old_center_y);
};

Camera.prototype.centerAt = function(x, y) {
    this.x = (x + 0.5) * this.tsize - this.width / 2;
    this.y = (y + 0.5) * this.tsize - this.height / 2;
};

Game.load = function() {
    return [
        //Loader.loadImage('tiles', '../assets/tiles.png'),
    ];
};

// args are currently ignored
Game.init = function(tsize, canvasW, canvasH) {
    map.tsize = 1;
    this.tileAtlas = Loader.getImage("tiles");
    this.camera = new Camera(map, 512, 512);
    this.showGrid = true;
    this.centerAt(0, 0);
    // Dirty flag: only render if true, remember to set it when changing state
    this.dirty = true;
};

Game.update = function(delta) {
    // maybe scroll here?
};

Game._drawTile = function(x, y, v) {
    let colors = ["white", "green", "red"];
    if (v < 3) {
        this.ctx.fillStyle = colors[v];
        this.ctx.fillRect(
            Math.round(x), // target x
            Math.round(y), // target y
            this.camera.tsize, // target width
            this.camera.tsize // target height
        );
    }
};

Game._drawLayer = function(layer) {
    // + 1 because when the width is not a multiple of tsize things get weird
    let startCol = Math.floor(this.camera.x / this.camera.tsize);
    let endCol = startCol + this.camera.width / this.camera.tsize + 1;
    let startRow = Math.floor(this.camera.y / this.camera.tsize);
    let endRow = startRow + this.camera.height / this.camera.tsize + 1;
    let offsetX = -this.camera.x + startCol * this.camera.tsize;
    let offsetY = -this.camera.y + startRow * this.camera.tsize;

    //console.log([startCol, endCol, startRow, endRow, offsetX, offsetY]);

    // Few elements set: iterate over the layers[layer] Map
    // Many elements set: iterate over each tile
    if (map.layers[layer].size < (endCol - startCol) * (endRow - startRow)) {
        map.layers[layer].forEach((v, k) => {
            let xy = k.split(",").map(a => Number.parseInt(a));
            let c = xy[0];
            let r = xy[1];
            if (c >= startCol && c <= endCol && r >= startRow && r <= endRow) {
                //console.log(tile);
                let x = (c - startCol) * this.camera.tsize + offsetX;
                let y = (r - startRow) * this.camera.tsize + offsetY;
                this._drawTile(x, y, v);
            }
        });
    } else {
        for (let c = startCol; c <= endCol; c++) {
            for (let r = startRow; r <= endRow; r++) {
                let v = map.getTile(layer, c, r);
                //console.log(tile);
                let x = (c - startCol) * this.camera.tsize + offsetX;
                let y = (r - startRow) * this.camera.tsize + offsetY;
                if (v != undefined) {
                    // undefined => empty tile
                    this._drawTile(x, y, v);
                }
            }
        }
    }

    // Draw grid lines
    if (this.showGrid) {
        this.ctx.strokeStyle = "#AAA";
        this.ctx.lineWidth = 1;
        for (let c = startCol; c <= endCol; c++) {
            let x = (c - startCol) * this.camera.tsize + offsetX;
            x = Math.round(x);
            this.ctx.beginPath();
            this.ctx.moveTo(x, 0);
            this.ctx.lineTo(x, 512);
            this.ctx.stroke();
        }
        for (let r = startRow; r <= endRow; r++) {
            let y = (r - startRow) * this.camera.tsize + offsetY;
            y = Math.round(y);
            this.ctx.beginPath();
            this.ctx.moveTo(0, y);
            this.ctx.lineTo(512, y);
            this.ctx.stroke();
        }
    }
};

Game.render = function() {
    if (!this.dirty) {
        return;
    }
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
    let tx = (x + this.camera.x) / this.camera.tsize;
    let ty = (y + this.camera.y) / this.camera.tsize;
    return [tx, ty];
};

Game.mouse_coords_to_game_coords = function(x, y) {
    let txty = this.mouse_coords_to_game_coords_float(x, y);
    let tx = txty[0];
    let ty = txty[1];
    tx = Math.floor(tx);
    ty = Math.floor(ty);
    return [tx, ty];
};

Game.clickTile = function(x, y) {
    this.dirty = true;
    let txty = this.mouse_coords_to_game_coords(x, y);
    let tx = txty[0];
    let ty = txty[1];
    /*
    console.log("Clicked " + x + "," + y);
    console.log("Which is: " + tx + "," + ty);
    console.log(this.camera);
    */
    let a = map.getTile(0, tx, ty);
    if (a == undefined) {
        a = 0;
    }
    a += 1;
    if (a >= 3) {
        a = 0;
    }
    map.setTile(0, tx, ty, a);
};

// Get all the (x, y) pairs from a layer with the given value
Game.getSelection = function(layer, value) {
    // Iterators in JS dont have .filter()
    let s = [];
    map.layers[layer].forEach((v, k) => {
        let xy = k.split(",").map(a => Number.parseInt(a));
        //console.log(k + " => " + v);
        //console.log(layer_x_y);
        let x = xy[0];
        let y = xy[1];
        if (v == value) {
            s.push([x, y]);
        }
    });
    return s;
};

Game.clearSelection = function(layer) {
    this.dirty = true;
    map.layers[layer].clear();
};

Game.setSelection = function(layer, value, keys) {
    this.dirty = true;
    keys.forEach(k => {
        map.setTile(layer, k[0], k[1], value);
    });
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
