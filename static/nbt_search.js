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

    let blockName = document.getElementById("string_query").value;
    mainWorker.postMessage({
        command: "nbt_search",
        args: [region, blockName],
    });
    mainWorker.onmessage = function(e) {
        let local_found_blocks = e.data.result;

        console.log("Found following blocks:");
        console.log(local_found_blocks);
        let outputTextarea = document.getElementById("output_textarea");
        outputTextarea.value = stringify(local_found_blocks, { maxLength: 20 });
        document.getElementById(
            "how_many_found"
        ).innerHTML = `Found ${local_found_blocks.length} entries`;
    };
}
