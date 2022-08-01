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
    let dimension = document.getElementById("dimension").value;
    if (dimension == "DIM0") {
        dimension = null;
    }
    let ignoreCaveSpiders = document.getElementById("ignore_cave_spiders")
        .checked;
    mainWorker.postMessage({
        command: "find_spawners_in_world",
        args: [
            region,
            {
                center_position_and_chunk_radius: [
                    { x: centerX, y: centerY, z: centerZ },
                    chunkRadius,
                ],
                dimension: dimension,
            },
        ],
    });
    mainWorker.onmessage = function(e) {
        let local_found_blocks = e.data;
        console.log("Found following blocks:");
        console.log(local_found_blocks);
        updateList(local_found_blocks, {
            chunkRadius,
            dimension,
            centerX,
            centerY,
            centerZ,
            ignoreCaveSpiders,
        });
    };
}

function updateList(
    local_found_blocks,
    { chunkRadius, dimension, centerX, centerY, centerZ, ignoreCaveSpiders }
) {
    let outputTextarea = document.getElementById("output_textarea");
    outputTextarea.value = stringify(local_found_blocks, { maxLength: 20 });
    if (ignoreCaveSpiders) {
        local_found_blocks = local_found_blocks.flatMap(function(dungeon) {
            dungeon.spawners = dungeon.spawners.flatMap(function(spawner) {
                if (spawner.kind === "minecraft:cave_spider") {
                    return [];
                } else {
                    return [spawner];
                }
            });

            if (dungeon.spawners.length < 2) {
                // 1 spawner is not a multi-spawner
                return [];
            } else {
                return [dungeon];
            }
        });
    }
    document.getElementById(
        "how_many_found"
    ).innerHTML = `Found ${local_found_blocks.length} multi-spawners in a ${chunkRadius}-chunk radius around ${centerX},${centerY},${centerZ}`;

    let dungeon_list = document.getElementById("dungeon_list");
    dungeon_list.innerHTML = "";
    for (let i = 0; i < local_found_blocks.length; i++) {
        let dungeon = local_found_blocks[i];
        let newDiv = document.createElement("div");
        newDiv.id = `dungeonCard${i}`;
        newDiv.className = "smallCard";
        newDiv.innerHTML += `${dungeon.spawners.length} spawners active at ${dungeon.optimalPosition.x}, ${dungeon.optimalPosition.y}, ${dungeon.optimalPosition.z}<br>`;
        for (let j = 0; j < dungeon.spawners.length; j++) {
            let spawner = dungeon.spawners[j];
            let newNestedDiv = document.createElement("div");
            newNestedDiv.id = `dungeonCard${i}-${j}`;
            newNestedDiv.className = "smallerCard";
            newNestedDiv.innerHTML += `${spawner.position.x}, ${spawner.position.y}, ${spawner.position.z}: ${spawner.kind}`;
            newDiv.appendChild(newNestedDiv);
        }
        dungeon_list.appendChild(newDiv);
        //dungeon_list.innerHTML += `<div id='dungeonCard${i}' class='smallCard' onClick='selectDungeon(${i})'>${JSON.stringify(
        //    dungeon.position
        //)}<br>${dungeon.kind}</div>`;
    }
}

// https://stackoverflow.com/a/16779396
function more(obj, elemId) {
    var content = document.getElementById(elemId);

    if (content.style.display == "none") {
        content.style.display = "";
    } else {
        content.style.display = "none";
    }
}

document.getElementById(
    "advancedOptions"
).style.display = document.getElementById("toggleAdvanced1").checked
    ? ""
    : "none";
