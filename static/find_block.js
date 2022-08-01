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

function findBlock() {
    if (mainWorker === null) {
        mainWorker = new Worker("worker_generic.js");
    }
    let blockName = document.getElementById("block_name").value;
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
    mainWorker.postMessage({
        command: "find_blocks_in_world",
        args: [region, blockName, searchAround],
    });
    mainWorker.onmessage = function(e) {
        let local_found_blocks = e.data;
        console.log("Found following blocks:");
        console.log(local_found_blocks);
        let outputTextarea = document.getElementById("output_textarea");
        outputTextarea.value = stringify(local_found_blocks, { maxLength: 20 });
        document.getElementById(
            "how_many_found"
        ).innerHTML = `Found ${local_found_blocks.length} blocks with id "${blockName}" in a ${chunkRadius}-chunk radius around ${centerX},${centerY},${centerZ}`;
    };
}
