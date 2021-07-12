let region = null;

document.getElementById("filepicker").addEventListener(
    "change",
    function() {
        let reader = new FileReader();
        reader.onload = function() {
            let arrayBuffer = this.result;
            let array = new Uint8Array(arrayBuffer);
            region = array;
            console.log("Loaded file. Size:", array.length);
        };
        reader.readAsArrayBuffer(this.files[0]);
    },
    false
);

function findBlock() {
    Rust.slime_seed_finder_web.then(function(slime_seed_finder_web) {
        let blockName = document.getElementById("string_query").value;
        let local_found_blocks = slime_seed_finder_web.nbt_search(
            region,
            blockName
        );
        console.log("Found following blocks:");
        console.log(local_found_blocks);
        let outputTextarea = document.getElementById("output_textarea");
        outputTextarea.value = stringify(local_found_blocks, { maxLength: 20 });
        document.getElementById(
            "how_many_found"
        ).innerHTML = `Found ${local_found_blocks.length} entries`;
    });
}
