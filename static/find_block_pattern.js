let region = null;
document.getElementById("filepicker").addEventListener(
    "change",
    function() {
        region = this.files[0];
        console.log("Loaded region file. Size:", region.size);
    },
    false
);

let mainWorker = null;

function parse_y_range(y_range_str) {
    if (y_range_str === "") {
        return null;
    }

    let [yMin, yMax, error] = y_range_str.split(",");
    if (error !== undefined) {
        alert("Invalid coordinates, please follow the format: '1,2'");
    }

    yMin = yMin | 0;
    yMax = yMax | 0;
    return [yMin, yMax];
}

function findBlock() {
    if (mainWorker === null) {
        mainWorker = new Worker("worker_generic.js");
    }
    let centerCoordsString = document.getElementById("center_coords").value;
    let centerX, centerY, centerZ;
    if (centerCoordsString === "") {
        centerX = 0;
        centerY = 0;
        centerZ = 0;
    } else {
        let centerCoordsStringSplit = centerCoordsString.split(",");
        let [x, y, z, centerError] = centerCoordsStringSplit;
        if (centerError !== undefined) {
            alert("Invalid coordinates, please follow the format: '1,2,3'");
        }
        centerX = x | 0;
        centerY = y | 0;
        centerZ = z | 0;
    }
    let chunkRadius = document.getElementById("search_radius").value | 0;
    let searchAround = [{ x: centerX, y: centerY, z: centerZ }, chunkRadius];
    if (chunkRadius === 0) {
        searchAround = null;
    }
    let yRange = parse_y_range(document.getElementById("search_y_range").value);
    let blockPattern = document.getElementById("pattern_textarea").value;
    let dimension = document.getElementById("dimension").value;
    if (dimension == "DIM0") {
        dimension = null;
    }
    mainWorker.postMessage({
        command: "find_block_pattern_in_world",
        args: [
            region,
            {
                pattern: blockPattern,
                centerPositionAndChunkRadius: searchAround,
                dimension: dimension,
                yRange: yRange,
            },
        ],
    });
    mainWorker.onmessage = function(e) {
        let local_found_blocks = e.data;
        console.log("Found following blocks:");
        console.log(local_found_blocks);
        let outputTextarea = document.getElementById("output_textarea");
        outputTextarea.value = stringify(local_found_blocks, { maxLength: 20 });
        document.getElementById(
            "how_many_found"
        ).innerHTML = `Found ${local_found_blocks.length} matches in a ${chunkRadius}-chunk radius around ${centerX},${centerY},${centerZ}`;
    };
}

let templates = {
    bedrock3x3: {
        name: "Bedrock 3x3",
        pattern: {
            palette: {
                b: { block_name: "minecraft:bedrock" },
                n: { not: { block_name: "minecraft:bedrock" } },
            },
            // 3x3 bedrock above 3x3 of anything but bedrock
            map: "bbb,bbb,bbb;nnn,nnn,nnn;",
        },
    },
};

function addTemplateOptions() {
    let template_select = document.getElementById("findTemplate");
    for (let [id, template] of Object.entries(templates)) {
        let opt = document.createElement("option");
        opt.value = id;
        opt.innerHTML = template.name;
        template_select.appendChild(opt);
    }
}

function loadNewPattern(pattern) {
    if (pattern != null) {
        document.getElementById("pattern_textarea").value = stringify(pattern, {
            maxLength: 20,
        });
    }
    pattern = JSON.parse(document.getElementById("pattern_textarea").value);
    document.getElementById("palette_textarea").value = stringify(
        pattern["palette"],
        { maxLength: 20 }
    );
    document.getElementById("map_textarea").value = stringify(pattern["map"], {
        maxLength: 20,
    });
    document.getElementById("pattern_error_div").style.display = "none";
    document.getElementById("pattern_error_message").value = "";
    document.getElementById("everything_else_div").style.display = "block";
}

function handlePatternOptionChange(e) {
    let s = e.target.value;
    if (s === "custom") {
        loadNewPattern(null);
    } else {
        let pattern = templates[s].pattern;
        loadNewPattern(pattern);
    }
}

document.getElementById("findTemplate").onchange = handlePatternOptionChange;

addTemplateOptions();
