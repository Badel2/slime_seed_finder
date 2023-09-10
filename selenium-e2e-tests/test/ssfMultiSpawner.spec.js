const assert = require("assert");
const fs = require("fs").promises;
const { Builder, By, until } = require("selenium-webdriver");
const { suite } = require("selenium-webdriver/testing");
const firefox = require("selenium-webdriver/firefox");

const APP_ROOT_URL = process.env.SLIME_SEED_FINDER_DEMO_URL || 'https://badel2.github.io/slime_seed_finder';
const APP_URL = `${APP_ROOT_URL}/multi_spawner.html`;

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

suite(function(env) {
    describe("Slime Seed Finder multi_spawner.html", function() {
        let driver;

        before(async function() {
            let options = new firefox.Options();
            driver = await new Builder()
                .setFirefoxOptions(options)
                .forBrowser("firefox")
                .build();
        });

        after(() => driver.quit());

        // TODO: improve this tests by having a toml file with a list of generated worlds
        // So locally I can have hundreds of worlds but in github I only have
        // to push the small ones, to keep the repo small.
        it("Can find multi spawners in world zip - 1_18-rc3-1234-v4.zip", async function() {
            await driver.get(APP_URL);
            await driver.manage().setTimeouts({ implicit: 500 });
            let filepicker = await driver.findElement(
                By.id("filepicker")
            );
            filepicker.sendKeys(__dirname + "/../1_18-rc3-1234-v4.zip");
            let runButton = await driver.findElement(By.id("button_find_block"));
            await runButton.click();
            let howManyFound = await driver.findElement(
                By.id("how_many_found")
            );
            let howManyFoundText = await howManyFound.getAttribute("innerHTML");
            console.log("howManyFoundText: ", howManyFoundText);
            // TODO: this is howManyFoundText if the file does not exist
            // No world file specified, please select a world file and try again
            while (howManyFoundText === null || !howManyFoundText.startsWith("Found ")) {
                await sleep(100);
                howManyFound = await driver.findElement(
                    By.id("how_many_found")
                );
                howManyFoundText = await howManyFound.getAttribute("innerHTML");
                console.log("howManyFoundText: ", howManyFoundText);
            }
            assert.equal(howManyFoundText, "Found 9 multi-spawners");
        });

        it("Can find multi spawners in world zip - multi_spawner_test.zip", async function() {
            await driver.get(APP_URL);
            await driver.manage().setTimeouts({ implicit: 500 });
            let filepicker = await driver.findElement(
                By.id("filepicker")
            );
            filepicker.sendKeys(__dirname + "/../multi_spawner_test.zip");
            let runButton = await driver.findElement(By.id("button_find_block"));
            await runButton.click();
            let howManyFound = await driver.findElement(
                By.id("how_many_found")
            );
            let howManyFoundText = await howManyFound.getAttribute("innerHTML");
            console.log("howManyFoundText: ", howManyFoundText);
            while (howManyFoundText === null || !howManyFoundText.startsWith("Found ")) {
                await sleep(100);
                howManyFound = await driver.findElement(
                    By.id("how_many_found")
                );
                howManyFoundText = await howManyFound.getAttribute("innerHTML");
                console.log("howManyFoundText: ", howManyFoundText);
            }
            assert.equal(howManyFoundText, "Found 2 multi-spawners");
        });
    });
});
