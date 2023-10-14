// Checkbox change should update canvas
[
    "showVoronoi",
    "showGrid",
    "showGrid2",
    "showGridSmall",
    "showGridSmall2",
    "showResult",
    "showResult2",
    "showNegativeCircles",
    "showPositiveCircles",
].forEach(function(a) {
    document.getElementById(a).addEventListener("change", event => {
        render();
    });
});
// Reset grid on grid size change
["gridSizeX"].forEach(function(a) {
    document.getElementById(a).addEventListener("change", event => {
        initPoints();
        render();
    });
});

document.getElementById("genRandom").addEventListener("click", randomGen);
document.getElementById("genCenter1").addEventListener("click", center1Gen);
document.getElementById("genCenter").addEventListener("click", centerGen);

document.addEventListener("keydown", event => {
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

let c = document.getElementById("voronoi");
let elem = c;
let ctx = c.getContext("2d");
let c2 = document.getElementById("voronoi2");
let ctx2 = c2.getContext("2d");

let gridSizeX, size;

// Positions of the generated points
let genx, geny, dragging, vcolor;
let baseColors = [
    "#aa00aa",
    "#880000",
    "#00FF00",
    "#0000FF",
    "#000000",
    "#008800",
    "#000088",
    "#dddd00",
    "#00FFFF",
    "#FF00FF",
];

initPoints();

function initPoints() {
    gridSizeX = document.getElementById("gridSizeX").value | 0;
    document.getElementById("gridSizeXValue").innerHTML = gridSizeX;
    let c_width = document.getElementById("canvasSizeLol").value | 0;
    if (c_width == 0) {
        c_width = 720;
    }
    c.width = c_width;
    c.height = c.width;
    c2.width = c.width;
    c2.height = c.width;
    size = c.width / (gridSizeX + 1) / 4; // small pixel size
    genx = new Array(gridSizeX * gridSizeX);
    geny = new Array(gridSizeX * gridSizeX);
    dragging = new Array(gridSizeX * gridSizeX);
    vcolor = new Array(gridSizeX * gridSizeX);

    for (let x = 0; x < gridSizeX; x++) {
        for (let y = 0; y < gridSizeX; y++) {
            dragging[y * gridSizeX + x] = false;
            vcolor[y * gridSizeX + x] =
                baseColors[Math.floor(Math.random() * 8)];
        }
    }

    if (gridSizeX <= 3) {
        vcolor = baseColors;
    }

    center1Gen();
}

function randomGen() {
    for (let ix = 0; ix < gridSizeX; ix++) {
        for (let iy = 0; iy < gridSizeX; iy++) {
            genx[iy * gridSizeX + ix] =
                4.0 +
                4.0 * ix +
                (Math.floor(Math.random() * 1024) / 1024.0 - 0.5) * 3.6;
            geny[iy * gridSizeX + ix] =
                4.0 +
                4.0 * iy +
                (Math.floor(Math.random() * 1024) / 1024.0 - 0.5) * 3.6;
        }
    }
    render();
}

function centerGen() {
    for (let ix = 0; ix < gridSizeX; ix++) {
        for (let iy = 0; iy < gridSizeX; iy++) {
            genx[iy * gridSizeX + ix] =
                4.0 + 4.0 * ix + (512 / 1024.0 - 0.5) * 3.6;
            geny[iy * gridSizeX + ix] =
                4.0 + 4.0 * iy + (512 / 1024.0 - 0.5) * 3.6;
        }
    }
    render();
}

function center1Gen() {
    for (let ix = 0; ix < gridSizeX; ix++) {
        for (let iy = 0; iy < gridSizeX; iy++) {
            genx[iy * gridSizeX + ix] =
                4.0 + 4.0 * ix + (511 / 1024.0 - 0.5) * 3.6;
            geny[iy * gridSizeX + ix] =
                4.0 + 4.0 * iy + (511 / 1024.0 - 0.5) * 3.6;
        }
    }
    render();
}

// https://stackoverflow.com/a/16284281
let pointerEventToXY = function(e) {
    let out = {
        x: 0,
        y: 0,
    };
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
            if (n == "touchstart") {
                e.preventDefault();
            }
            let pointer = pointerEventToXY(e);
            let elemLeft = elem.offsetLeft,
                elemTop = elem.offsetTop;
            let x = (pointer.x - elemLeft) / size,
                y = (pointer.y - elemTop) / size;

            for (let ix = 0; ix < gridSizeX; ix++) {
                for (let iy = 0; iy < gridSizeX; iy++) {
                    if (
                        x >= 2.2 + 4.0 * ix &&
                        x <= 2.2 + 3.6 + 4.0 * ix &&
                        y >= 2.2 + 4.0 * iy &&
                        y <= 2.2 + 3.6 + 4.0 * iy
                    ) {
                        genx[iy * gridSizeX + ix] = constrain(
                            x,
                            2.2 + 4.0 * ix,
                            2.2 + 3.6 + 4.0 * ix
                        );
                        geny[iy * gridSizeX + ix] = constrain(
                            y,
                            2.2 + 4.0 * iy,
                            2.2 + 3.6 + 4.0 * iy
                        );
                        dragging[iy * gridSizeX + ix] = true;
                        render();
                    } else {
                        dragging[iy * gridSizeX + ix] = false;
                    }
                }
            }
        },
        false
    );
});

["touchmove", "mousemove"].forEach(function(n) {
    elem.addEventListener(
        n,
        function(e) {
            if (n == "touchstart") {
                e.preventDefault();
            }
            let pointer = pointerEventToXY(e);

            let elemLeft = elem.offsetLeft,
                elemTop = elem.offsetTop;
            let x = (pointer.x - elemLeft) / size,
                y = (pointer.y - elemTop) / size;

            for (let ix = 0; ix < gridSizeX; ix++) {
                for (let iy = 0; iy < gridSizeX; iy++) {
                    if (dragging[iy * gridSizeX + ix]) {
                        genx[iy * gridSizeX + ix] = constrain(
                            x,
                            2.2 + 4.0 * ix,
                            2.2 + 3.6 + 4.0 * ix
                        );
                        geny[iy * gridSizeX + ix] = constrain(
                            y,
                            2.2 + 4.0 * iy,
                            2.2 + 3.6 + 4.0 * iy
                        );
                        render();
                    }
                }
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
            for (let ix = 0; ix < gridSizeX; ix++) {
                for (let iy = 0; iy < gridSizeX; iy++) {
                    dragging[iy * gridSizeX + ix] = false;
                }
            }
        },
        false
    );
});

function render() {
    let w = c.width,
        h = c.height;

    // Clear canvas
    ctx.fillStyle = "white";
    ctx.fillRect(0, 0, w, h);
    // Draw background tiles
    let backgroundPalette = ["#222222", "#EEEEEE"];
    for (let ix = 0; ix < gridSizeX + 1; ix++) {
        ctx.fillStyle = backgroundPalette[ix % 2];
        ctx.fillRect(4 * size * ix, 0, size * 4, size * 4);
    }
    for (let iy = 0; iy < gridSizeX + 1; iy++) {
        ctx.fillStyle = backgroundPalette[iy % 2];
        ctx.fillRect(0, 4 * size * iy, size * 4, size * 4);
    }

    // Draw colored tiles
    for (let ix = 0; ix < gridSizeX; ix++) {
        for (let iy = 0; iy < gridSizeX; iy++) {
            ctx.fillStyle = vcolor[iy * gridSizeX + ix];
            ctx.fillRect(
                4.0 * size * (ix + 1),
                4.0 * size * (iy + 1),
                size * 4,
                size * 4
            );
        }
    }

    if (document.getElementById("showResult").checked) {
        for (let ix = 0; ix < gridSizeX - 1; ix++) {
            for (let iy = 0; iy < gridSizeX - 1; iy++) {
                // Calculate distance from (i, j) to each point
                for (let i = 0; i < 4; i++) {
                    for (let j = 0; j < 4; j++) {
                        let da =
                            (j - geny[iy * gridSizeX + ix] + 4 * (iy + 1)) *
                                (j - geny[iy * gridSizeX + ix] + 4 * (iy + 1)) +
                            (i - genx[iy * gridSizeX + ix] + 4 * (ix + 1)) *
                                (i - genx[iy * gridSizeX + ix] + 4 * (ix + 1));
                        let db =
                            (j - geny[iy * gridSizeX + ix + 1] + 4 * (iy + 1)) *
                                (j -
                                    geny[iy * gridSizeX + ix + 1] +
                                    4 * (iy + 1)) +
                            (i - genx[iy * gridSizeX + ix + 1] + 4 * (ix + 1)) *
                                (i -
                                    genx[iy * gridSizeX + ix + 1] +
                                    4 * (ix + 1));
                        let dc =
                            (j -
                                geny[(iy + 1) * gridSizeX + ix] +
                                4 * (iy + 1)) *
                                (j -
                                    geny[(iy + 1) * gridSizeX + ix] +
                                    4 * (iy + 1)) +
                            (i -
                                genx[(iy + 1) * gridSizeX + ix] +
                                4 * (ix + 1)) *
                                (i -
                                    genx[(iy + 1) * gridSizeX + ix] +
                                    4 * (ix + 1));
                        let dd =
                            (j -
                                geny[(iy + 1) * gridSizeX + ix + 1] +
                                4 * (iy + 1)) *
                                (j -
                                    geny[(iy + 1) * gridSizeX + ix + 1] +
                                    4 * (iy + 1)) +
                            (i -
                                genx[(iy + 1) * gridSizeX + ix + 1] +
                                4 * (ix + 1)) *
                                (i -
                                    genx[(iy + 1) * gridSizeX + ix + 1] +
                                    4 * (ix + 1));

                        let draw = vcolor[(iy + 1) * gridSizeX + ix + 1];
                        if (da < db && da < dc && da < dd) {
                            draw = vcolor[iy * gridSizeX + ix];
                        } else if (db < da && db < dc && db < dd) {
                            draw = vcolor[iy * gridSizeX + ix + 1];
                        } else if (dc < da && dc < db && dc < dd) {
                            draw = vcolor[(iy + 1) * gridSizeX + ix];
                        }

                        ctx.beginPath();
                        ctx.strokeStyle = "#ffffff";
                        ctx.fillStyle = draw + "ff";
                        ctx.rect(
                            (4.0 * (ix + 1) + i) * size,
                            (4.0 * (iy + 1) + j) * size,
                            size,
                            size
                        );
                        ctx.fill();
                        if (document.getElementById("showGridSmall").checked) {
                            ctx.stroke();
                        }
                    }
                }
            }
        }
    }

    document.getElementById("genInfo").innerHTML = "";
    for (let ix = 0; ix < gridSizeX; ix++) {
        for (let iy = 0; iy < gridSizeX; iy++) {
            document.getElementById("genInfo").innerHTML +=
                "x: " +
                ix +
                ", y: " +
                iy +
                " -- " +
                genx[iy * gridSizeX + ix] +
                ", " +
                geny[iy * gridSizeX + ix] +
                "<br>";
        }
    }

    if (document.getElementById("showGrid").checked) {
        ctx.beginPath();
        for (let ix = 0; ix < gridSizeX; ix++) {
            for (let iy = 0; iy < gridSizeX; iy++) {
                ctx.rect(
                    size * (4.0 * (ix + 1) - 1.8),
                    size * (4.0 * (iy + 1) - 1.8),
                    size * 3.6,
                    size * 3.6
                );
            }
        }
        ctx.strokeStyle = "#caca00";
        ctx.stroke();
    }

    ctx.strokeStyle = "#ffffff";
    for (let ix = 0; ix < gridSizeX; ix++) {
        for (let iy = 0; iy < gridSizeX; iy++) {
            ctx.beginPath();
            ctx.rect(
                (genx[iy * gridSizeX + ix] - 0.1) * size,
                (geny[iy * gridSizeX + ix] - 0.1) * size,
                size * 0.2,
                size * 0.2
            );
            ctx.fillStyle = vcolor[iy * gridSizeX + ix];
            ctx.fill();
            ctx.stroke();
        }
    }

    if (document.getElementById("showVoronoi").checked) {
        delaunay();
    }
    render2();
}

function delaunay() {
    function edgesOfTriangle(t) {
        return [3 * t, 3 * t + 1, 3 * t + 2];
    }

    function triangleOfEdge(e) {
        return Math.floor(e / 3);
    }

    function nextHalfedge(e) {
        return e % 3 === 2 ? e - 2 : e + 1;
    }

    function prevHalfedge(e) {
        return e % 3 === 0 ? e + 2 : e - 1;
    }

    function edgesOfTriangle(t) {
        return [3 * t, 3 * t + 1, 3 * t + 2];
    }

    function pointsOfTriangle(delaunay, t) {
        return edgesOfTriangle(t).map(e => delaunay.triangles[e]);
    }

    function triangleCenter(points, delaunay, t) {
        const vertices = pointsOfTriangle(delaunay, t).map(p => points[p]);
        return circumcenter(vertices[0], vertices[1], vertices[2]);
    }

    function circumcenter(a, b, c) {
        const ad = a[0] * a[0] + a[1] * a[1];
        const bd = b[0] * b[0] + b[1] * b[1];
        const cd = c[0] * c[0] + c[1] * c[1];
        const D =
            2 *
            (a[0] * (b[1] - c[1]) +
                b[0] * (c[1] - a[1]) +
                c[0] * (a[1] - b[1]));
        return [
            (1 / D) *
                (ad * (b[1] - c[1]) + bd * (c[1] - a[1]) + cd * (a[1] - b[1])),
            (1 / D) *
                (ad * (c[0] - b[0]) + bd * (a[0] - c[0]) + cd * (b[0] - a[0])),
        ];
    }

    function forEachTriangleEdge(points, delaunay, callback) {
        for (let e = 0; e < delaunay.triangles.length; e++) {
            if (e > delaunay.halfedges[e]) {
                const p = points[delaunay.triangles[e]];
                const q = points[delaunay.triangles[nextHalfedge(e)]];
                callback(e, p, q);
            }
        }
    }

    function forEachVoronoiEdge(points, delaunay, callback) {
        for (let e = 0; e < delaunay.triangles.length; e++) {
            if (e < delaunay.halfedges[e]) {
                const p = triangleCenter(points, delaunay, triangleOfEdge(e));
                const q = triangleCenter(
                    points,
                    delaunay,
                    triangleOfEdge(delaunay.halfedges[e])
                );
                callback(e, p, q);
            }
        }
    }
    /*
    forEachTriangleEdge(points, delaunay, function(e, p, q) {
      ctx.strokeStyle = "#FFFF00";
      ctx.beginPath();
      ctx.moveTo(p[0] * size, p[1] * size);
      ctx.lineTo(q[0] * size, q[1] * size);
      ctx.stroke();
    });
    */

    for (let ix = 0; ix < gridSizeX - 1; ix++) {
        for (let iy = 0; iy < gridSizeX - 1; iy++) {
            let points = new Array(4 + 2);
            points[0] = [
                genx[(iy + 0) * gridSizeX + ix + 0],
                geny[(iy + 0) * gridSizeX + ix + 0],
            ];
            points[1] = [
                genx[(iy + 0) * gridSizeX + ix + 1],
                geny[(iy + 0) * gridSizeX + ix + 1],
            ];
            points[2] = [
                genx[(iy + 1) * gridSizeX + ix + 0],
                geny[(iy + 1) * gridSizeX + ix + 0],
            ];
            points[3] = [
                genx[(iy + 1) * gridSizeX + ix + 1],
                geny[(iy + 1) * gridSizeX + ix + 1],
            ];
            points[4] = [1000, 1000];
            points[5] = [-1000, -1000];

            const delaunay = Delaunator.from(points);

            ctx.save();
            ctx.rect(
                4.0 * size * (ix + 1),
                4.0 * size * (iy + 1),
                4.0 * size,
                4.0 * size
            );
            ctx.stroke();
            ctx.clip();
            forEachVoronoiEdge(points, delaunay, function(e, p, q) {
                ctx.strokeStyle = "#FFFFFF";
                ctx.beginPath();
                ctx.moveTo(p[0] * size, p[1] * size);
                ctx.lineTo(q[0] * size, q[1] * size);
                ctx.stroke();
            });
            ctx.restore();
        }
    }
}

function render2() {
    let w = c2.width,
        h = c2.height;

    // Clear canvas
    ctx2.fillStyle = "#EEEEEE";
    ctx2.fillRect(0, 0, w, h);

    /*
      for (let ix=0; ix<gridSizeX; ix++) {
          for (let iy=0; iy<gridSizeX; iy++) {
              ctx2.fillStyle = vcolor[iy * gridSizeX + ix];
              ctx2.fillRect(4.0 * size * (ix + 1), 4.0 * size * (iy + 1), size * 4, size * 4);
          }
      }
    */

    if (document.getElementById("showResult2").checked) {
        for (let ix = 0; ix < gridSizeX - 1; ix++) {
            for (let iy = 0; iy < gridSizeX - 1; iy++) {
                // Calculate distance from (i, j) to each point
                for (let i = 0; i < 4; i++) {
                    for (let j = 0; j < 4; j++) {
                        let da =
                            (j - geny[iy * gridSizeX + ix] + 4 * (iy + 1)) *
                                (j - geny[iy * gridSizeX + ix] + 4 * (iy + 1)) +
                            (i - genx[iy * gridSizeX + ix] + 4 * (ix + 1)) *
                                (i - genx[iy * gridSizeX + ix] + 4 * (ix + 1));
                        let db =
                            (j - geny[iy * gridSizeX + ix + 1] + 4 * (iy + 1)) *
                                (j -
                                    geny[iy * gridSizeX + ix + 1] +
                                    4 * (iy + 1)) +
                            (i - genx[iy * gridSizeX + ix + 1] + 4 * (ix + 1)) *
                                (i -
                                    genx[iy * gridSizeX + ix + 1] +
                                    4 * (ix + 1));
                        let dc =
                            (j -
                                geny[(iy + 1) * gridSizeX + ix] +
                                4 * (iy + 1)) *
                                (j -
                                    geny[(iy + 1) * gridSizeX + ix] +
                                    4 * (iy + 1)) +
                            (i -
                                genx[(iy + 1) * gridSizeX + ix] +
                                4 * (ix + 1)) *
                                (i -
                                    genx[(iy + 1) * gridSizeX + ix] +
                                    4 * (ix + 1));
                        let dd =
                            (j -
                                geny[(iy + 1) * gridSizeX + ix + 1] +
                                4 * (iy + 1)) *
                                (j -
                                    geny[(iy + 1) * gridSizeX + ix + 1] +
                                    4 * (iy + 1)) +
                            (i -
                                genx[(iy + 1) * gridSizeX + ix + 1] +
                                4 * (ix + 1)) *
                                (i -
                                    genx[(iy + 1) * gridSizeX + ix + 1] +
                                    4 * (ix + 1));

                        let draw = vcolor[(iy + 1) * gridSizeX + ix + 1];
                        if (da < db && da < dc && da < dd) {
                            draw = vcolor[iy * gridSizeX + ix];
                        } else if (db < da && db < dc && db < dd) {
                            draw = vcolor[iy * gridSizeX + ix + 1];
                        } else if (dc < da && dc < db && dc < dd) {
                            draw = vcolor[(iy + 1) * gridSizeX + ix];
                        }

                        ctx2.beginPath();
                        ctx2.strokeStyle = "#ffffff";
                        ctx2.fillStyle = draw + "ff";
                        ctx2.rect(
                            (4.0 * (ix + 1) + i) * size,
                            (4.0 * (iy + 1) + j) * size,
                            size,
                            size
                        );
                        ctx2.fill();
                        if (document.getElementById("showGridSmall2").checked) {
                            ctx2.stroke();
                        }
                    }
                }
            }
        }
    }

    for (let ix = 0; ix < gridSizeX - 1; ix++) {
        for (let iy = 0; iy < gridSizeX - 1; iy++) {
            // Calculate distance from (i, j) to each point
            for (let i = 0; i < 4; i++) {
                for (let j = 0; j < 4; j++) {
                    let da =
                        (j - geny[iy * gridSizeX + ix] + 4 * (iy + 1)) *
                            (j - geny[iy * gridSizeX + ix] + 4 * (iy + 1)) +
                        (i - genx[iy * gridSizeX + ix] + 4 * (ix + 1)) *
                            (i - genx[iy * gridSizeX + ix] + 4 * (ix + 1));
                    let db =
                        (j - geny[iy * gridSizeX + ix + 1] + 4 * (iy + 1)) *
                            (j - geny[iy * gridSizeX + ix + 1] + 4 * (iy + 1)) +
                        (i - genx[iy * gridSizeX + ix + 1] + 4 * (ix + 1)) *
                            (i - genx[iy * gridSizeX + ix + 1] + 4 * (ix + 1));
                    let dc =
                        (j - geny[(iy + 1) * gridSizeX + ix] + 4 * (iy + 1)) *
                            (j -
                                geny[(iy + 1) * gridSizeX + ix] +
                                4 * (iy + 1)) +
                        (i - genx[(iy + 1) * gridSizeX + ix] + 4 * (ix + 1)) *
                            (i -
                                genx[(iy + 1) * gridSizeX + ix] +
                                4 * (ix + 1));
                    let dd =
                        (j -
                            geny[(iy + 1) * gridSizeX + ix + 1] +
                            4 * (iy + 1)) *
                            (j -
                                geny[(iy + 1) * gridSizeX + ix + 1] +
                                4 * (iy + 1)) +
                        (i -
                            genx[(iy + 1) * gridSizeX + ix + 1] +
                            4 * (ix + 1)) *
                            (i -
                                genx[(iy + 1) * gridSizeX + ix + 1] +
                                4 * (ix + 1));

                    let nearest = 3;
                    let draw = vcolor[(iy + 1) * gridSizeX + ix + 1];
                    if (da < db && da < dc && da < dd) {
                        nearest = 0;
                        draw = vcolor[iy * gridSizeX + ix];
                    } else if (db < da && db < dc && db < dd) {
                        nearest = 1;
                        draw = vcolor[iy * gridSizeX + ix + 1];
                    } else if (dc < da && dc < db && dc < dd) {
                        nearest = 2;
                        draw = vcolor[(iy + 1) * gridSizeX + ix];
                    }

                    // Draw circle from small grid point to edge of nearest
                    function circle(ctx, x, y, r) {
                        ctx.beginPath();
                        ctx.arc(x, y, r, 0, 2 * Math.PI, false);
                        ctx.lineWidth = 3;
                        ctx.strokeStyle = "#FF0000";
                        ctx.stroke();
                        ctx.fillStyle = draw + "40";
                        ctx.fill();
                        ctx.lineWidth = 1;
                    }

                    function circle_green_clipped(
                        ctx,
                        x,
                        y,
                        r,
                        clipx,
                        clipy,
                        clipw,
                        cliph
                    ) {
                        ctx.save();
                        ctx.beginPath();
                        ctx.rect(clipx, clipy, clipw, cliph);
                        ctx.clip();
                        ctx.beginPath();
                        ctx.arc(x, y, r, 0, 2 * Math.PI, false);
                        ctx.lineWidth = 3;
                        ctx.strokeStyle = "#00FF00";
                        ctx.stroke();
                        ctx.fillStyle = draw + "20";
                        ctx.fill();
                        ctx.lineWidth = 1;
                        ctx.restore();
                    }

                    function distance_point_vline(px, py, linex) {
                        return Math.abs(px - linex);
                    }

                    function distance_point_hline(px, py, liney) {
                        return Math.abs(py - liney);
                    }

                    function distance_points(ax, ay, bx, by) {
                        return Math.sqrt(
                            Math.pow(ax - bx, 2) + Math.pow(ay - by, 2)
                        );
                    }

                    function distance_point_nearest_line(i, j, nearest) {
                        // Quadrant 0: nearest lines are x = 2.2 (n=1) and y = 2.2 (n=1)
                        if (i <= 1 && j <= 1) {
                            if (nearest == 0) return 0;
                            if (nearest == 1)
                                return distance_point_vline(i, j, 2.2);
                            if (nearest == 2)
                                return distance_point_hline(i, j, 2.2);
                            if (nearest == 3)
                                return distance_points(i, j, 2.2, 2.2);
                        }
                        // Quadrant 1: nearest lines are y = 1.8 (n=0) and x = 2.2 (n=3)
                        if (i == 3 && j <= 1) {
                            if (nearest == 0)
                                return distance_point_vline(i, j, 1.8);
                            if (nearest == 1) return 0;
                            if (nearest == 2)
                                return distance_points(i, j, 1.8, 2.2);
                            if (nearest == 3)
                                return distance_point_hline(i, j, 2.2);
                        }
                        // Quadrant 2: nearest lines are x = 1.8 (n=0) and y = 2.2 (n=3)
                        if (i <= 1 && j == 3) {
                            if (nearest == 0)
                                return distance_point_hline(i, j, 1.8);
                            if (nearest == 1)
                                return distance_points(i, j, 2.2, 1.8);
                            if (nearest == 2) return 0;
                            if (nearest == 3)
                                return distance_point_vline(i, j, 2.2);
                        }
                        // Quadrant 3: nearest lines are x = 1.8 (n=2) and y = 1.8 (n=1)
                        if (i == 3 && j == 3) {
                            if (nearest == 0)
                                return distance_points(i, j, 1.8, 1.8);
                            if (nearest == 1)
                                return distance_point_hline(i, j, 1.8);
                            if (nearest == 2)
                                return distance_point_vline(i, j, 1.8);
                            if (nearest == 3) return 0;
                        }
                        // Middle grounds 0-1
                        if (i == 2 && j <= 1) {
                            if (nearest == 0)
                                return 0 * distance_point_vline(i, j, 1.8);
                            if (nearest == 1)
                                return 0 * distance_point_vline(i, j, 2.2);
                            if (nearest == 2)
                                return distance_points(i, j, 1.8, 2.2);
                            if (nearest == 3)
                                return distance_points(i, j, 2.2, 2.2);
                        }
                        // Middle grounds 0-2
                        if (i <= 1 && j == 2) {
                            if (nearest == 0)
                                return 0 * distance_point_hline(i, j, 1.8);
                            if (nearest == 1)
                                return distance_points(i, j, 2.2, 1.8);
                            if (nearest == 2)
                                return 0 * distance_point_hline(i, j, 2.2);
                            if (nearest == 3)
                                return distance_points(i, j, 2.2, 2.2);
                        }
                        // Middle grounds 2-3
                        if (i == 2 && j == 3) {
                            if (nearest == 0)
                                return distance_points(i, j, 1.8, 1.8);
                            if (nearest == 1)
                                return distance_points(i, j, 2.2, 1.8);
                            if (nearest == 2)
                                return 0 * distance_point_vline(i, j, 1.8);
                            if (nearest == 3)
                                return 0 * distance_point_vline(i, j, 2.2);
                        }
                        // Middle grounds 1-3
                        if (i == 3 && j == 2) {
                            if (nearest == 0)
                                return distance_points(i, j, 1.8, 1.8);
                            if (nearest == 1)
                                return 0 * distance_point_hline(i, j, 1.8);
                            if (nearest == 2)
                                return distance_points(i, j, 1.8, 2.2);
                            if (nearest == 3)
                                return 0 * distance_point_hline(i, j, 2.2);
                        }
                        // Middle grounds 0-1-2-3
                        if (i == 2 && j == 2) {
                            if (nearest == 0)
                                return 0 * distance_points(i, j, 1.8, 1.8);
                            if (nearest == 1)
                                return 0 * distance_points(i, j, 2.2, 1.8);
                            if (nearest == 2)
                                return 0 * distance_points(i, j, 1.8, 2.2);
                            if (nearest == 3)
                                return 0 * distance_points(i, j, 2.2, 2.2);
                        }
                        console.error("Didnt enter any branch");
                    }

                    function distance_point_furthest_edge(i, j, nearest) {
                        // Quadrant 0: furthest edge is x = -1.8, y = -1.8
                        if (i <= 1 && j <= 1) {
                            if (nearest == 0) return 0;
                            if (nearest == 1)
                                return distance_points(i, j, -1.8, -1.8);
                            if (nearest == 2)
                                return distance_points(i, j, -1.8, -1.8);
                            if (nearest == 3)
                                return distance_points(i, j, -1.8, -1.8);
                        }
                        // Quadrant 1: furthest edge is x = 5.8, y = -1.8
                        if (i == 3 && j <= 1) {
                            if (nearest == 0)
                                return distance_points(i, j, 5.8, -1.8);
                            if (nearest == 1) return 0;
                            if (nearest == 2)
                                return distance_points(i, j, 5.8, -1.8);
                            if (nearest == 3)
                                return distance_points(i, j, 5.8, -1.8);
                        }
                        // Quadrant 2: furthest edge is x = -1.8, y = 5.8
                        if (i <= 1 && j == 3) {
                            if (nearest == 0)
                                return distance_points(i, j, -1.8, 5.8);
                            if (nearest == 1)
                                return distance_points(i, j, -1.8, 5.8);
                            if (nearest == 2) return 0;
                            if (nearest == 3)
                                return distance_points(i, j, -1.8, 5.8);
                        }
                        // Quadrant 3: furthest edge is x = 5.8, y = 5.8
                        if (i == 3 && j == 3) {
                            if (nearest == 0)
                                return distance_points(i, j, 5.8, 5.8);
                            if (nearest == 1)
                                return distance_points(i, j, 5.8, 5.8);
                            if (nearest == 2)
                                return distance_points(i, j, 5.8, 5.8);
                            if (nearest == 3) return 0;
                        }
                        // Middle grounds 0-1
                        if (i == 2 && j <= 1) {
                            if (nearest == 0) return 0;
                            if (nearest == 1) return 0;
                            if (nearest == 2)
                                return distance_points(i, j, -1.8, -1.8);
                            if (nearest == 3)
                                return distance_points(i, j, -1.8, -1.8);
                        }
                        // Middle grounds 0-2
                        if (i <= 1 && j == 2) {
                            if (nearest == 0) return 0;
                            if (nearest == 2) return 0;
                            if (nearest == 1)
                                return distance_points(i, j, -1.8, -1.8);
                            if (nearest == 3)
                                return distance_points(i, j, -1.8, -1.8);
                        }
                        // Middle grounds 2-3
                        if (i == 2 && j == 3) {
                            if (nearest == 0)
                                return distance_points(i, j, 5.8, 5.8);
                            if (nearest == 1)
                                return distance_points(i, j, 5.8, 5.8);
                            if (nearest == 2) return 0;
                            if (nearest == 3) return 0;
                        }
                        // Middle grounds 1-3
                        if (i == 3 && j == 2) {
                            if (nearest == 0)
                                return distance_points(i, j, 5.8, 5.8);
                            if (nearest == 1) return 0;
                            if (nearest == 2)
                                return distance_points(i, j, 5.8, 5.8);
                            if (nearest == 3) return 0;
                        }
                        // Middle grounds 0-1-2-3
                        if (i == 2 && j == 2) {
                            return 0;
                        }
                        console.error("Didnt enter any branch");
                    }
                    let px = 4.0 * (ix + 1) + i;
                    let py = 4.0 * (iy + 1) + j;
                    if (
                        document.getElementById("showNegativeCircles").checked
                    ) {
                        circle(
                            ctx2,
                            px * size,
                            py * size,
                            distance_point_nearest_line(i, j, nearest) * size
                        );
                    }
                    if (
                        document.getElementById("showPositiveCircles").checked
                    ) {
                        circle_green_clipped(
                            ctx2,
                            px * size,
                            py * size,
                            distance_point_furthest_edge(i, j, nearest) * size,
                            size * (4.0 * (ix + (nearest % 2) + 1) - 1.8),
                            size *
                                (4.0 * (iy + Math.floor(nearest / 2) + 1) -
                                    1.8),
                            size * 3.6,
                            size * 3.6
                        );
                    }
                }
            }
        }
    }

    if (document.getElementById("showGrid2").checked) {
        ctx2.beginPath();
        for (let ix = 0; ix < gridSizeX; ix++) {
            for (let iy = 0; iy < gridSizeX; iy++) {
                ctx2.rect(
                    size * (4.0 * (ix + 1) - 1.8),
                    size * (4.0 * (iy + 1) - 1.8),
                    size * 3.6,
                    size * 3.6
                );
            }
        }
        ctx2.strokeStyle = "#caca00";
        ctx2.stroke();
    }

    /*
      ctx2.strokeStyle = "#ffffff";
      for (let ix=0; ix<gridSizeX; ix++) {
          for (let iy=0; iy<gridSizeX; iy++) {
              ctx2.beginPath();
              ctx2.rect((genx[iy * gridSizeX + ix] - 0.1) * size, (geny[iy * gridSizeX + ix] - 0.1) * size, size * 0.2, size * 0.2);
              ctx2.fillStyle = vcolor[iy * gridSizeX + ix];
              ctx2.fill();
              ctx2.stroke();
          }
      }
    */
}
