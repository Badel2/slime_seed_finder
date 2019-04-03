// Checkbox change should update canvas
[].forEach(function(a) {
    document.getElementById(a).addEventListener("change", (event) => {
        render();
    })
});
// Reset grid on grid size change
["gridSizeX", "numColors"].forEach(function(a) {
    document.getElementById(a).addEventListener("change", (event) => {
        initPoints();
        render();
    })
});

document.getElementById("genRandom").addEventListener("click", randomGen);
document.getElementById("genClear").addEventListener("click", clearGen);

document.addEventListener('keydown', (event) => {
    if (event.code == "KeyR") {
        randomGen();
    }
    if (event.code == "KeyT") {
        center1Gen();
    }
    if (event.code == "KeyY") {
        centerGen();
    }
});

function constrain(x, a, b) {
    return Math.min(Math.max(x, a), b);
}

var c = document.getElementById('voronoi');
var elem = c;
var ctx = c.getContext("2d");
var c2 = document.getElementById('voronoi2');
var ctx2 = c2.getContext("2d");

var gridSizeX, size;

// Positions of the generated points
var genx, geny, dragging, vcolor;
var baseColors = ["#000070", "#8db360", "#606060", "#056621", "#0b6659", "#fa9418", "#000000", "#008800", "#000088", "#dddd00", "#00FFFF", "#FF00FF"];
var totalColors = 2;

function nextColor(c) {
    return baseColors[(baseColors.findIndex((x) => x == c) + 1) % totalColors];
}

initPoints();

function initPoints() {
    gridSizeX = document.getElementById("gridSizeX").value | 0;
    document.getElementById("gridSizeXValue").innerHTML = gridSizeX;
    totalColors = constrain(document.getElementById("numColors").value | 0, 1, baseColors.length);
    document.getElementById("numColorsValue").innerHTML = totalColors;
    var c_width = document.getElementById("canvasSizeLol").value | 0;
    if (c_width == 0) {
        c_width = 720;
    }
    c.width = c_width;
    c.height = c.width;
    c2.width = c.width;
    c2.height = c.width;
    size = c.width / gridSizeX; // small pixel size
    vcolor = new Array(gridSizeX * gridSizeX);

    randomGen();
}

function randomGen() {
    for (var ix = 0; ix < gridSizeX; ix++) {
        for (var iy = 0; iy < gridSizeX; iy++) {
            vcolor[iy * gridSizeX + ix] = baseColors[Math.floor(Math.random() * totalColors)];
        }
    }
    render();
}

function clearGen() {
    for (var ix = 0; ix < gridSizeX; ix++) {
        for (var iy = 0; iy < gridSizeX; iy++) {
            vcolor[iy * gridSizeX + ix] = baseColors[0];
        }
    }
    render();
}

// https://stackoverflow.com/a/16284281
var pointerEventToXY = function(e) {
    var out = {
        x: 0,
        y: 0
    };
    if (e.type == 'touchstart' || e.type == 'touchmove' || e.type == 'touchend' || e.type == 'touchcancel') {
        var touch = e.touches[0] || e.changedTouches[0];
        out.x = touch.pageX;
        out.y = touch.pageY;
    } else if (e.type == 'mousedown' || e.type == 'mouseup' || e.type == 'mousemove' || e.type == 'mouseover' || e.type == 'mouseout' || e.type == 'mouseenter' || e.type == 'mouseleave') {
        out.x = e.pageX;
        out.y = e.pageY;
    }
    return out;
};

['touchstart', 'mousedown'].forEach(function(n) {
    elem.addEventListener(n, function(e) {
        if (n == 'touchstart') {
            e.preventDefault();
        }

        //console.log("click:", x, ",", y);

    }, false)
});

['touchmove', 'mousemove'].forEach(function(n) {
    elem.addEventListener(n, function(e) {
        if (n == 'touchstart') {
            e.preventDefault();
        }

    }, false)
});

['touchend', 'mouseup'].forEach(function(n) {
    window.addEventListener(n, function(e) {
        if (n == 'touchend') {
            // This breaks the page
            //e.preventDefault();
        }
        var pointer = pointerEventToXY(e);
        var elemLeft = elem.offsetLeft,
            elemTop = elem.offsetTop;
        var x = Math.floor((pointer.x - elemLeft) / size),
            y = Math.floor((pointer.y - elemTop) / size);

        if (x >= 0 && x < gridSizeX && y >= 0 && y < gridSizeX) {
            var current = vcolor[y * gridSizeX + x];
            vcolor[y * gridSizeX + x] = nextColor(current);
            render();
        }
    }, false)
});


function render() {
    var w = c.width,
        h = c.height;

    // Clear canvas
    ctx.fillStyle = "white";
    ctx.fillRect(0, 0, w, h);

    // Draw colored tiles
    for (var ix = 0; ix < gridSizeX; ix++) {
        for (var iy = 0; iy < gridSizeX; iy++) {
            ctx.fillStyle = vcolor[iy * gridSizeX + ix];
            ctx.fillRect(size * ix, size * iy, size, size);
        }
    }

    render2();
}

function render2() {

    var w = c2.width,
        h = c2.height;

    // Clear canvas
    ctx2.fillStyle = "#EEEEEE";
    ctx2.fillRect(0, 0, w, h);

    // Draw colored tiles
    for (var ix = 0; ix < gridSizeX - 2; ix++) {
        for (var iy = 0; iy < gridSizeX - 2; iy++) {
            var v11 = vcolor[(iy + 1) * gridSizeX + (ix + 1)];
            var v10 = vcolor[(iy + 0) * gridSizeX + (ix + 1)];
            var v01 = vcolor[(iy + 1) * gridSizeX + (ix + 0)];
            var v12 = vcolor[(iy + 2) * gridSizeX + (ix + 1)];
            var v21 = vcolor[(iy + 1) * gridSizeX + (ix + 2)];
            if ( v10 == v12 && v01 == v21 ) {
                // Random choice, 50%
                // Represented as a checker pattern
                ctx2.fillStyle = v10;
                ctx2.fillRect(size * (ix + 1.0), size * (iy + 1.0), size/2, size/2);
                ctx2.fillRect(size * (ix + 1.5), size * (iy + 1.5), size/2, size/2);
                ctx2.fillStyle = v01;
                ctx2.fillRect(size * (ix + 1.5), size * (iy + 1.0), size/2, size/2);
                ctx2.fillRect(size * (ix + 1.0), size * (iy + 1.5), size/2, size/2);
            } else {
                if ( v10 == v12 ) {
                    v11 = v10;
                } else if ( v01 == v21 ) {
                    v11 = v01;
                }
                ctx2.fillStyle = v11;
                ctx2.fillRect(size * (ix + 1), size * (iy + 1), size, size);
            }
        }
    }
}

