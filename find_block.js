let region = null;

document.getElementById("filepicker").addEventListener(
    "change",
    function() {
        let reader = new FileReader();
        reader.onload = function() {
            let arrayBuffer = this.result;
            let array = new Uint8Array(arrayBuffer);
            region = array;
            console.log("Loaded region file. Size:", array.length);
        };
        reader.readAsArrayBuffer(this.files[0]);
    },
    false
);

function findBlock() {
    Rust.slime_seed_finder_web.then(function(slime_seed_finder_web) {
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
        let local_found_blocks = slime_seed_finder_web.find_blocks_in_world(
            region,
            blockName,
            [{ x: centerX, y: centerY, z: centerZ }, chunkRadius]
        );
        console.log("Found following blocks:");
        console.log(local_found_blocks);
        let outputTextarea = document.getElementById("output_textarea");
        outputTextarea.value = stringify(local_found_blocks, { maxLength: 20 });
        document.getElementById(
            "how_many_found"
        ).innerHTML = `Found ${local_found_blocks.length} blocks with id "${blockName}" in a ${chunkRadius}-chunk radius around ${centerX},${centerY},${centerZ}`;
    });
}
