const assert = require("assert");
const fs = require("fs").promises;
const os = require("os");
const path = require("path");
const { Builder, By, until } = require("selenium-webdriver");
const { suite } = require("selenium-webdriver/testing");
const firefox = require("selenium-webdriver/firefox");

const APP_ROOT_URL = process.env.SLIME_SEED_FINDER_DEMO_URL || 'https://badel2.github.io/slime_seed_finder';
const APP_URL = `${APP_ROOT_URL}/biomes.html`;

function sleep(ms) {
    return new Promise(resolve => setTimeout(resolve, ms));
}

async function checkFileExists(filePath) {
  try {
    await fs.access(filePath);
    return true;
  } catch (err) {
    return false;
  }
}

async function waitForFileToExist(filePath, timeout) {
  const start = Date.now();

  while (Date.now() - start < timeout) {
    if (await checkFileExists(filePath)) {
      return true;
    }
    // Sleep for a short duration before checking again
    await new Promise(resolve => setTimeout(resolve, 100));
  }

  return false;
}

async function getFileSize(filePath) {
  const stats = await fs.stat(filePath);
  return stats.size;
}

async function biomeMapFinishedGenerating(driver) {
    // Use some implementation details to check if the biome map finished rendering
    let x = await driver.executeScript("return numGeneratingFragments()");
    return x === 0;
}

async function waitForBiomeMapToFinishGenerating(driver, timeout) {
  const start = Date.now();

  while (Date.now() - start < timeout) {
    if (await biomeMapFinishedGenerating(driver)) {
      return true;
    }
    // Sleep for a short duration before checking again
    await new Promise(resolve => setTimeout(resolve, 100));
  }

  return false;
}

async function getPNGResolution(filePath) {
  try {
    const buffer = await fs.readFile(filePath);

    // Check for PNG header signature
    if (
      buffer.readUInt32BE(0) !== 0x89504e47 || // PNG signature
      buffer.readUInt32BE(12) !== 0x49484452 // IHDR chunk signature
    ) {
      throw new Error('Not a valid PNG file');
    }

    // Extract width and height from IHDR chunk
    const width = buffer.readUInt32BE(16);
    const height = buffer.readUInt32BE(20);

    return { width, height };
  } catch (error) {
    console.error('Error when reading PNG file resolution: ', error);
    throw error;
  }
}

suite(function(env) {
    describe("Slime Seed Finder biomes.html", function() {
        let driver;
        let tmpDir;

        before(async function() {
            let options = new firefox.Options();
            // Change download location to a tmp dir
            let appPrefix = "ssf-selenium-e2e-tests";
            tmpDir = await fs.mkdtemp(path.join(os.tmpdir(), appPrefix + "-"));
            options.setPreference("browser.download.folderList", 2);
            options.setPreference("browser.download.manager.showWhenStarting", false);
            options.setPreference("browser.download.dir", tmpDir);
            options.setPreference("browser.helperApps.neverAsk.saveToDisk", "application/x-gzip");
            driver = await new Builder()
                .setFirefoxOptions(options)
                .forBrowser("firefox")
                .build();
        });

        // TODO: we could remove tmpDir
        after(() => driver.quit());

        it("Biome map generates", async function() {
            this.timeout(60000);
            await driver.get(APP_URL);
            await driver.manage().setTimeouts({ implicit: 500 });
            await sleep(1000);
            let mapSeedInput = await driver.findElement(
                By.id("worldSeed")
            );
            let mapSeed = await mapSeedInput.getAttribute("value");
            let minecraftVersionSelect = await driver.findElement(By.id("minecraftVersion"));
            let minecraftVersion = await minecraftVersionSelect.getAttribute("value");
            console.log(`Generating map for seed ${mapSeed} and version ${minecraftVersion}`);
            // Wait for biome map to generate
            assert.equal(await waitForBiomeMapToFinishGenerating(driver, 20000), true, "Biome map did not generate in time");
        });

        it("Can download the biome map as a PNG", async function() {
            // Biome map was generated in previous test
            assert.equal(await biomeMapFinishedGenerating(driver), true, "Biome map did not generate in time");
            let downloadButton = await driver.findElement(By.id("download_button"));
            await downloadButton.click();
            // Wait for downloaded file to exist
            // Filename depends on worldSeed
            let mapSeedInput = await driver.findElement(
                By.id("worldSeed")
            );
            let mapSeed = await mapSeedInput.getAttribute("value");
            let downloadedMapPath = path.join(tmpDir, `seed_${mapSeed}.png`);
            assert.equal(await waitForFileToExist(downloadedMapPath, 5000), true, `File ${downloadedMapPath} does not exist`);
            // File size should not be 0 bytes
            assert.equal(await getFileSize(downloadedMapPath) > 0, true, `File size is zero`);
            let { width, height } = await getPNGResolution(downloadedMapPath);
            assert.equal(width > 100 && height > 100, true, `PNG resolution seems too small`);
        });
    });
});
