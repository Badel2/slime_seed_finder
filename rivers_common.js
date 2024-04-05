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

Game.run = function(context, tsize, canvasW, canvasH) {
    this.ctx = context;
    this._previousElapsed = 0;

    let p = this.load();
    Promise.all(p).then(
        function(loaded) {
            this.init(tsize, canvasW, canvasH);
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
