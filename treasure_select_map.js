// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// You can obtain one at http://mozilla.org/MPL/2.0/.

// Original file from:
// https://github.com/mozdevs/gamedev-js-tiles
// Modified version can be found at:
// https://github.com/Badel2/inf-proc-gen-tilemap

let map = {
    tsize: 1,
    // 3 layer
    layers: [new Map(), new Map(), new Map()],
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

// args are currently ignored
Game.init = function(tsize, canvasW, canvasH) {
    console.log("Init game with w:" + canvasW + ", h:" + canvasH);
    map.tsize = 1;
    this.camera = new Camera(map, canvasW, canvasH);
    this.showGrid = false;
    this.centerAt(8, 8);
    // Dirty flag: only render if true, remember to set it when changing state
    this.dirty = true;
    this.showLayer1 = true;
    this.showLayer2 = true;
};

Game.update = function(delta) {
    // maybe scroll here?
};

Game._drawTile = function(x, y, v) {
    // ocean, land, river
    let colors = ["#bbbbff", "#DBC6AC", "#0000FF"];
    if (v == undefined) {
        v = 0;
    }
    if (v <= 3) {
        this._drawRGBTile(x, y, colors[v]);
    } else {
        let unknown = "#ff00ff";
        this._drawRGBTile(x, y, unknown);
    }
};

Game._drawRGBTile = function(x, y, rgb) {
    this.ctx.fillStyle = rgb;
    this.ctx.fillRect(
        Math.round(x) - 1, // target x
        Math.round(y) - 1, // target y
        this.camera.tsize + 2, // target width
        this.camera.tsize + 2 // target height
    );
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

    // Iterate over each tile
    for (let c = startCol; c <= endCol; c++) {
        for (let r = startRow; r <= endRow; r++) {
            if (c >= 0 && c < 128 && r >= 0 && r < 128) {
                let v = map.getTile(layer, c, r);
                //console.log(tile);
                let x = (c - startCol) * this.camera.tsize + offsetX;
                let y = (r - startRow) * this.camera.tsize + offsetY;
                if (layer == 0) {
                    this._drawTile(x, y, v);
                } else if (layer == 1) {
                    this._drawRGBTile(x, y, v);
                } else if (layer == 2) {
                    this._drawRGBTile(x, y, v);
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
            this.ctx.lineTo(x, this.camera.height);
            this.ctx.stroke();
        }
        for (let r = startRow; r <= endRow; r++) {
            let y = (r - startRow) * this.camera.tsize + offsetY;
            y = Math.round(y);
            this.ctx.beginPath();
            this.ctx.moveTo(0, y);
            this.ctx.lineTo(this.camera.width, y);
            this.ctx.stroke();
        }
    }
};

Game._drawVoronoiTiles = function() {
    // + 1 because when the width is not a multiple of tsize things get weird
    let startCol = Math.floor(this.camera.x / this.camera.tsize);
    let endCol = startCol + this.camera.width / this.camera.tsize + 1;
    let startRow = Math.floor(this.camera.y / this.camera.tsize);
    let endRow = startRow + this.camera.height / this.camera.tsize + 1;
    let offsetX = -this.camera.x + startCol * this.camera.tsize;
    let offsetY = -this.camera.y + startRow * this.camera.tsize;

    //console.log([startCol, endCol, startRow, endRow, offsetX, offsetY]);

    for (let c = startCol; c <= endCol; c++) {
        for (let r = startRow; r <= endRow; r++) {
            function mod(n, m) {
                return ((n % m) + m) % m;
            }
            let v = mod(c, 2) == 0 && mod(r, 2) == 0;
            //console.log(tile);
            let x = (c - startCol) * this.camera.tsize + offsetX;
            let y = (r - startRow) * this.camera.tsize + offsetY;
            if (v && c >= 0 && c < 128 && r >= 0 && r < 128) {
                //this._drawTile(x, y, v);
                this.ctx.fillStyle = "#eeffee";
                this.ctx.fillRect(
                    Math.round(x), // target x
                    Math.round(y), // target y
                    this.camera.tsize, // target width
                    this.camera.tsize // target height
                );
            }
        }
    }
};

Game.render = function() {
    if (!this.dirty) {
        return;
    }
    this.dirty = false;
    // clear previous frame
    this.ctx.fillStyle = "black";
    this.ctx.fillRect(0, 0, this.camera.width, this.camera.height);
    // draw map background layer, with voroinoi tiles highlighted in green
    this._drawVoronoiTiles();
    // draw selection layer, with rivers in blue
    this._drawLayer(0);
    // draw treasure map
    if (this.showLayer1) {
        this._drawLayer(1);
    }
    if (this.showLayer2) {
        this._drawLayer(2);
    }
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
    if (tx >= 0 && tx < 128 && ty >= 0 && ty < 128) {
        let a = map.getTile(0, tx, ty);
        if (a == undefined) {
            a = 0;
        }
        a += 1;
        if (a >= 2) {
            a = 0;
        }
        map.setTile(0, tx, ty, a);
    }
};

Game.setTile = function(x, y, value) {
    this.dirty = true;
    let txty = this.mouse_coords_to_game_coords(x, y);
    let tx = txty[0];
    let ty = txty[1];
    if (tx >= 0 && tx < 128 && ty >= 0 && ty < 128) {
        map.setTile(0, tx, ty, value);
    }
};

Game._recursiveBucketTool = function(
    targetValue,
    floorValue,
    recursionCounter,
    stack
) {
    let newStack = [];
    stack.forEach(xy => {
        let x = xy[0];
        let y = xy[1];
        let a = map.getTile(0, x, y);
        if (a == undefined) {
            a = 0;
        }
        if (a != targetValue && a == floorValue) {
            if (x >= 0 && x < 128 && y >= 0 && y < 128) {
                map.setTile(0, x, y, targetValue);
            }
            newStack.push([x + 0, y + 1]);
            newStack.push([x + 0, y - 1]);
            newStack.push([x + 1, y + 0]);
            newStack.push([x - 1, y + 0]);
        }
    });
    if (recursionCounter > 0) {
        this._recursiveBucketTool(
            targetValue,
            floorValue,
            recursionCounter - 1,
            newStack
        );
    }
};

Game.bucketTool = function(x, y, targetValue, recursionLimit = 10) {
    this.dirty = true;
    let txty = this.mouse_coords_to_game_coords(x, y);
    let tx = txty[0];
    let ty = txty[1];
    let floorValue = map.getTile(0, tx, ty) | 0;
    this._recursiveBucketTool(targetValue, floorValue, recursionLimit, [
        [tx, ty],
    ]);
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

Game.getArea = function(layer, x0, x1, y0, y1) {
    let area = [];
    for (let y = y0; y < y1; y++) {
        for (let x = x0; x < x1; x++) {
            let v = map.getTile(layer, x, y) | 0;
            area.push(v);
        }
    }

    return area;
};

Game.setArea = function(layer, x0, x1, y0, y1, area) {
    this.dirty = true;
    for (let y = y0; y < y1; y++) {
        for (let x = x0; x < x1; x++) {
            let v = area[(y - y0) * (x1 - x0) + (x - x0)];
            map.setTile(layer, x, y, v);
        }
    }
};
