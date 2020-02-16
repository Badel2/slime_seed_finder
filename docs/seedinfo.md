# SeedInfo

In order to simplify the process of collecting data about a Minecraft world, I
decided to use a simple JSON file as a standard interface for the tools in this
repository.

```js
{
        // Version of the SeedInfo format (0.1)
    "seedInfo": "0.1",
        // Minecraft version of when the world was generated
    "version": "1.7",
        // Seed of the world, if known
    "worldSeed": "",
        // Hash of the seed of the world, if known
    "worldSeedHash": "",
        // Human readable description of the world
    "description": "This is the survival world",
    "options": {
        // Tool specific options: error margins and extend48
    },
    "biomes": {
        // Map of biome_id to list of block coordinates
        "7": [[31, 62], [31, 63], [31, 64]],
        "13": [[31, 65]]
    },

    // Height of the end pillars. The order is important.
    // Unimplemented.
    "endPillars": [94, 103, 100, 85, 91, 88, 76, 97, 79, 82],

    // Structures, with list of chunk coordinates
    "slimeChunks": [[3, 2], [1, 2]],
    // Unimplemented:
    "mineshafts": [],
    "netherForts": [],
    "strongholds": [],
    "desertTemples": [],
    "jungleTemples": [],
    "witchHuts": [],
    "villages": [],
    "oceanMonuments": [],
    "igloos": [],
    "woodlandMansions": [],
    "endCities": [],
    "oceanRuins": [],
    "shipwrecks": [],
    "buriedTreasures": [],
    "pillagerOutposts:": [],

    "negative": {
        // A copy of the structures above, to indicate that this world does
        // not have a structure at that coordinates.
        // This is useful to remove false positives with known non-slime chunks.
    },

    "and": [
        // A list of extra SeedInfo data from other Minecraft versions.
        // Useful when one part of the world was generated using a different
        // Minecraft version.
        // Unimplemented.
    ]
}
```

Empty fields can be omited.

The version field currently supports the strings "1.7", "1.8", ... up to "1.15",
and it assumes that this is the Java edition.

Coordinates can be specified as a tuple: `[31, 62]`, or as an object:
`{ "x": 31, "z": 62 }`.

