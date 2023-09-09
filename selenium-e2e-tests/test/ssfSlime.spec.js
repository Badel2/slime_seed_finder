const assert = require("assert");
const fs = require("fs").promises;
const { Builder, By, until } = require("selenium-webdriver");
const { suite } = require("selenium-webdriver/testing");
const firefox = require("selenium-webdriver/firefox");

const APP_ROOT_URL = process.env.SLIME_SEED_FINDER_DEMO_URL || 'https://badel2.github.io/slime_seed_finder';
const APP_URL = `${APP_ROOT_URL}/slime.html`;

suite(function(env) {
    describe("Slime Seed Finder slime.html", function() {
        let driver;

        before(async function() {
            let options = new firefox.Options();
            driver = await new Builder()
                .setFirefoxOptions(options)
                .forBrowser("firefox")
                .build();
        });

        after(() => driver.quit());

        it("Estimate candidates works on empty chunk list", async function() {
            await driver.get(APP_URL);
            await driver.manage().setTimeouts({ implicit: 500 });
            let numCandidates = await driver.findElement(
                By.id("num_candidates")
            );
            let numCandidatesText = await numCandidates.getAttribute("value");
            assert.equal(numCandidatesText, "");
            let estimateRuntime = await driver.findElement(
                By.id("estimate_runtime")
            );
            await estimateRuntime.click();
            numCandidatesText = await numCandidates.getAttribute("value");
            assert.equal(numCandidatesText, "262144 * 2^30 candidates");
        });

        it("Click on map sets chunk as slime chunk", async function() {
            await driver.get(APP_URL);
            await driver.manage().setTimeouts({ implicit: 500 });
            const actions = driver.actions();
            let selectionOutput = await driver.findElement(
                By.id("selection_output")
            );
            let selectionOutputJson = JSON.parse(
                await selectionOutput.getAttribute("value")
            );
            assert.deepEqual(selectionOutputJson.slimeChunks, []);
            assert.deepEqual(selectionOutputJson.negative.slimeChunks, []);
            let slimeMap = await driver.findElement(By.id("demo"));
            // Click on center of canvas, this should mark chunk 0,0 as slime chunk
            await actions
                .move({ origin: slimeMap, x: 0, y: 0 })
                .click()
                .perform();
            // TODO: we must clear actions to allow defining a new action below, without this clear
            // the next perform would perform this action and also the next action. With the clear
            // it will only perform the next action.
            await actions.clear();
            selectionOutputJson = JSON.parse(
                await selectionOutput.getAttribute("value")
            );
            assert.deepEqual(selectionOutputJson.slimeChunks, [[0, 0]]);
            assert.deepEqual(selectionOutputJson.negative.slimeChunks, []);
            // Click on center of canvas again, this should mark chunk 0,0 as non slime chunk
            await actions
                .move({ origin: slimeMap, x: 0, y: 0 })
                .click()
                .perform();
            await actions.clear();
            selectionOutputJson = JSON.parse(
                await selectionOutput.getAttribute("value")
            );
            assert.deepEqual(selectionOutputJson.slimeChunks, []);
            assert.deepEqual(selectionOutputJson.negative.slimeChunks, [
                [0, 0],
            ]);
            // Click on center of canvas again, this should remove chunk 0,0 from the non slime chunk list
            await actions
                .move({ origin: slimeMap, x: 0, y: 0 })
                .click()
                .perform();
            await actions.clear();
            selectionOutputJson = JSON.parse(
                await selectionOutput.getAttribute("value")
            );
            assert.deepEqual(selectionOutputJson.slimeChunks, []);
            assert.deepEqual(selectionOutputJson.negative.slimeChunks, []);
        });

        it("Center on chunk moves map center to that chunk", async function() {
            await driver.get(APP_URL);
            await driver.manage().setTimeouts({ implicit: 500 });
            const actions = driver.actions();
            let selectionOutput = await driver.findElement(
                By.id("selection_output")
            );
            let selectionOutputJson = JSON.parse(
                await selectionOutput.getAttribute("value")
            );
            assert.deepEqual(selectionOutputJson.slimeChunks, []);
            assert.deepEqual(selectionOutputJson.negative.slimeChunks, []);
            let centerButton = await driver.findElement(By.id("center_button"));
            let centerX = await driver.findElement(By.id("center_x"));
            let centerZ = await driver.findElement(By.id("center_z"));
            let slimeMap = await driver.findElement(By.id("demo"));
            await centerX.sendKeys("10");
            await centerZ.sendKeys("-20");
            await centerButton.click();
            // Click on center of canvas, this should mark chunk 10,-20 as slime chunk
            await actions
                .move({ origin: slimeMap, x: 0, y: 0 })
                .click()
                .perform();
            // TODO: we must clear actions to allow defining a new action below, without this clear
            // the next perform would perform this action and also the next action. With the clear
            // it will only perform the next action.
            await actions.clear();
            selectionOutputJson = JSON.parse(
                await selectionOutput.getAttribute("value")
            );
            assert.deepEqual(selectionOutputJson.slimeChunks, [[10, -20]]);
            assert.deepEqual(selectionOutputJson.negative.slimeChunks, []);
            // Click on center of canvas again, this should mark chunk 10,-20 as non slime chunk
            await actions
                .move({ origin: slimeMap, x: 0, y: 0 })
                .click()
                .perform();
            await actions.clear();
            selectionOutputJson = JSON.parse(
                await selectionOutput.getAttribute("value")
            );
            assert.deepEqual(selectionOutputJson.slimeChunks, []);
            assert.deepEqual(selectionOutputJson.negative.slimeChunks, [
                [10, -20],
            ]);
            // Click on center of canvas again, this should remove chunk 10,-20 from the non slime chunk list
            await actions
                .move({ origin: slimeMap, x: 0, y: 0 })
                .click()
                .perform();
            await actions.clear();
            selectionOutputJson = JSON.parse(
                await selectionOutput.getAttribute("value")
            );
            assert.deepEqual(selectionOutputJson.slimeChunks, []);
            assert.deepEqual(selectionOutputJson.negative.slimeChunks, []);
        });

        it("Load JSON updates the map", async function() {
            let seedInfoFile = await fs.readFile(
                "../seedinfo_tests/slime_1234.json"
            );
            let seedInfo = seedInfoFile.toString();
            await driver.get(APP_URL);
            await driver.manage().setTimeouts({ implicit: 500 });
            const actions = driver.actions();
            let selectionOutput = await driver.findElement(
                By.id("selection_output")
            );
            let selectionOutputJson = JSON.parse(
                await selectionOutput.getAttribute("value")
            );
            assert.deepEqual(selectionOutputJson.slimeChunks, []);
            assert.deepEqual(selectionOutputJson.negative.slimeChunks, []);
            let centerButton = await driver.findElement(By.id("center_button"));
            let centerX = await driver.findElement(By.id("center_x"));
            let centerZ = await driver.findElement(By.id("center_z"));
            let loadJsonButton = await driver.findElement(By.id("load_json"));
            let slimeMap = await driver.findElement(By.id("demo"));
            // TODO: this should be element.setValue but that does not exist
            await driver.executeScript(
                'document.getElementById("selection_output").value = ' +
                    JSON.stringify(seedInfo)
            );
            await loadJsonButton.click();
            selectionOutputJson = JSON.parse(
                await selectionOutput.getAttribute("value")
            );
            assert.deepEqual(selectionOutputJson.slimeChunks.length, 40);
            // negative is undefined
            assert.deepEqual(
                selectionOutputJson?.negative?.slimeChunks || [],
                []
            );
            await centerX.sendKeys("1");
            await centerZ.sendKeys("6");
            await centerButton.click();
            // Click on center of canvas, this should mark chunk 1,6 as non slime chunk
            await actions
                .move({ origin: slimeMap, x: 0, y: 0 })
                .click()
                .perform();
            // TODO: we must clear actions to allow defining a new action below, without this clear
            // the next perform would perform this action and also the next action. With the clear
            // it will only perform the next action.
            await actions.clear();
            selectionOutputJson = JSON.parse(
                await selectionOutput.getAttribute("value")
            );
            assert.deepEqual(selectionOutputJson.slimeChunks.length, 39);
            assert.deepEqual(selectionOutputJson.negative.slimeChunks, [
                [1, 6],
            ]);
            // Click on center of canvas again, this should remove chunk 1,6 from the non slime chunk list
            await actions
                .move({ origin: slimeMap, x: 0, y: 0 })
                .click()
                .perform();
            await actions.clear();
            selectionOutputJson = JSON.parse(
                await selectionOutput.getAttribute("value")
            );
            assert.deepEqual(selectionOutputJson.slimeChunks.length, 39);
            // negative is no longer undefined
            assert.deepEqual(selectionOutputJson.negative.slimeChunks, []);
        });

        it("Load JSON estimates 1 candidate", async function() {
            let seedInfoFile = await fs.readFile(
                "../seedinfo_tests/slime_1234.json"
            );
            let seedInfo = seedInfoFile.toString();
            await driver.get(APP_URL);
            await driver.manage().setTimeouts({ implicit: 500 });
            const actions = driver.actions();
            let selectionOutput = await driver.findElement(
                By.id("selection_output")
            );
            let selectionOutputJson = JSON.parse(
                await selectionOutput.getAttribute("value")
            );
            assert.deepEqual(selectionOutputJson.slimeChunks, []);
            assert.deepEqual(selectionOutputJson.negative.slimeChunks, []);
            let centerButton = await driver.findElement(By.id("center_button"));
            let centerX = await driver.findElement(By.id("center_x"));
            let centerZ = await driver.findElement(By.id("center_z"));
            let loadJsonButton = await driver.findElement(By.id("load_json"));
            let slimeMap = await driver.findElement(By.id("demo"));
            // TODO: this should be element.setValue but that does not exist
            await driver.executeScript(
                'document.getElementById("selection_output").value = ' +
                    JSON.stringify(seedInfo)
            );
            await loadJsonButton.click();

            let numCandidates = await driver.findElement(
                By.id("num_candidates")
            );
            let numCandidatesText = await numCandidates.getAttribute("value");
            assert.equal(numCandidatesText, "");
            let estimateRuntime = await driver.findElement(
                By.id("estimate_runtime")
            );
            await estimateRuntime.click();
            numCandidatesText = await numCandidates.getAttribute("value");
            assert.equal(numCandidatesText, "1 * 2^30 candidates");
        });

        it("Load JSON and RUN!", async function() {
            this.timeout(60000);
            let seedInfoFile = await fs.readFile(
                "../seedinfo_tests/slime_1234.json"
            );
            let seedInfo = seedInfoFile.toString();
            await driver.get(APP_URL);
            await driver.manage().setTimeouts({ implicit: 500 });
            const actions = driver.actions();
            let selectionOutput = await driver.findElement(
                By.id("selection_output")
            );
            let selectionOutputJson = JSON.parse(
                await selectionOutput.getAttribute("value")
            );
            assert.deepEqual(selectionOutputJson.slimeChunks, []);
            assert.deepEqual(selectionOutputJson.negative.slimeChunks, []);
            let centerButton = await driver.findElement(By.id("center_button"));
            let centerX = await driver.findElement(By.id("center_x"));
            let centerZ = await driver.findElement(By.id("center_z"));
            let loadJsonButton = await driver.findElement(By.id("load_json"));
            let slimeMap = await driver.findElement(By.id("demo"));
            // TODO: this should be element.setValue but that does not exist
            await driver.executeScript(
                'document.getElementById("selection_output").value = ' +
                    JSON.stringify(seedInfo)
            );
            await loadJsonButton.click();

            let outputTextarea = await driver.findElement(
                By.id("output_textarea")
            );
            let outputTextareaText = await outputTextarea.getAttribute("value");
            assert.equal(
                outputTextareaText,
                "Found 48-bit seeds will appear here."
            );
            let runButton = await driver.findElement(By.id("ssf_start"));
            await runButton.click();
            outputTextareaText = await outputTextarea.getAttribute("value");
            assert.equal(outputTextareaText, "Calculating...");
            while (outputTextareaText[0] !== "F") {
                outputTextareaText = await outputTextarea.getAttribute("value");
            }
            assert.equal(
                outputTextareaText.startsWith("Found 34 seeds!"),
                true
            );
            assert.equal(outputTextareaText.includes("1234"), true);
        });

        it("Extend48 seed 1234", async function() {
            let seedInfoFile = await fs.readFile(
                "../seedinfo_tests/slime_1234.json"
            );
            let seedInfo = seedInfoFile.toString();
            await driver.get(APP_URL);
            await driver.manage().setTimeouts({ implicit: 500 });
            let outputExtend48 = await driver.findElement(
                By.id("output_extend48")
            );
            let outputExtend48Text = await outputExtend48.getAttribute("value");
            assert.equal(
                outputExtend48Text,
                "Found 64-bit seeds will appear here."
            );
            let seed48 = await driver.findElement(By.id("seed48"));
            await seed48.sendKeys("1234");

            outputExtend48Text = await outputExtend48.getAttribute("value");
            while (!outputExtend48Text.startsWith("Found 2 seeds!")) {
                outputExtend48Text = await outputExtend48.getAttribute("value");
            }
            assert.equal(outputExtend48Text.startsWith("Found 2 seeds!"), true);
            assert.equal(
                outputExtend48Text.includes("-3658893222261816110,"),
                true
            );
            assert.equal(
                outputExtend48Text.includes("8847884417922761938,"),
                true
            );
        });
    });
});
