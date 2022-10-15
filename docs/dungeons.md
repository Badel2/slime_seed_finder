# Dungeon seed finder

You can use the `dungeon-seed` command to find the internal seed used to generate a specific
dungeon:

```
slime_seed_finder dungeon-seed --help
```

This seed is derived from the world seed, the `dungeon-seed-to-world-seed` command can be used to
find the world seed.

The current implementation requires 3 dungeon seeds to find the world seed, but that can probably be
improved.

Ideally there will be a pretty user interface to simplify this process, but for now you can use the
command line.

## Version support

This tool supports all the Minecraft versions since when dungeons were first introduced, up to 1.12
(1.13 is not supported yet). Supported versions:

* Alpha 1.0.4 (July 9, 2010) - 1.6.2 (July 8, 2013)
* 1.7 (October 25, 2013) - 1.12.2 (September 18, 2017)

Not supported yet:
* 1.13 - 1.14
* 1.15
* 1.16

If you need 1.13 support, try this other tool:

<https://github.com/hube12/DungeonCracker>

# Example

As an example, we will be trying to find the seed of a Minecraft 1.16 world. This version is not
supported so this will not work, but I wrote this guide before realizing that, and the process is
the same for older versions.

If you are playing on an older version, I recommend to create a copy of the world and open that copy
using a newer version of minecraft. I like to use 1.14. Some features such as spectator mode are
really useful.

All the images used here are available as an [imgur album](https://imgur.com/a/xMjLCSX).

First step: locate 3 dungeons.

## Dungeon 1

![dungeon1coords](https://i.imgur.com/EtEMhOU.png)

-244 17 -212

![dungeon1breakthewalls](https://i.imgur.com/yJVqKNs.png)

Break the walls. This is important, but if you cannot see the floor under the walls just set it to `?`.

![dungeon1facingwest](https://i.imgur.com/N8AZDoE.png)

Facing west, towards negative x

![dungeon1floor](https://i.imgur.com/xMNC3ma.png)

Map the floor. Draw a square using the following characters:

```
A = Air
C = Cobblestone
M = Mossy cobblestone
? = Either cobblestone or mossy cobblestone, not sure
```

![dungeon1floormapped](https://i.imgur.com/bk8MtDu.png)

```
CMCMMCMMC
MMCMMMCCM
MCMMMMCMM
MCMM?CCMC
CMMMCMCMM
CCMCCMMMM
MM???????
```

Now convert this into one line, separate rows with ; and surrond everything in quotes (""). The last
line must also end with ;

```
"CMCMMCMMC;MMCMMMCCM;MCMMMMCMM;MCMM?CCMC;CMMMCMCMM;CCMCCMMMM;MM???????;"
```

Run the command

```
slime_seed_finder dungeon-seed --spawner-x=-244 --spawner-y=17 --spawner-z=-212 --floor="CMCMMCMMC;MMCMMMCCM;MCMMMMCMM;MCMM?CCMC;CMMMCMCMM;CCMCCMMMM;MM???????;"
```

You should see something similar to:

```
Please double check that the entered data is correct:
The coordinates of the spawner are x: -244, y: 17, z: -212
When standing on the floor, the y coordinate of the player should be 17
This is the dungeon floor, and the coordinates of each corner are:
(-247, -208) ::: (-247, -216)

    CMCMMCMMC
    MMCMMMCCM
    MCMMMMCMM
    MCMM?CCMC
    CMMMCMCMM
    CCMCCMMMM
    MM???????

(-241, -208) ::: (-241, -216)

Started brutefroce using 4 threads. Estimated time: around 30 minutes
```

So let's verify that the floor is correct. `(-247, -208)` are the coordinates of the top-left corner, right. The characters also match the tiles.

![dungeon1corner](https://i.imgur.com/Z2pwrR5.png)

After around 30 minutes, the command should finish running and output the list of candidate seeds:

```
Found 1 dungeon seeds:
["-244,17,-212,149909570098943"]
```

Save this seed somewhere save.

## Dungeon 2

![dungeon2coords](https://i.imgur.com/NIZUedc.png)

454 19 -161

Let's try to find this one without breaking the walls

![dungeon2floor](https://i.imgur.com/cFPpt5T.png)

```
?????????
?CMCMMMC?
?MCMCCCM?
?MMMMMMM?
?CMMMMCM?
MCMCMMMM?
?????????
```

Run the command

```
slime_seed_finder dungeon-seed --spawner-x=454 --spawner-y=19 --spawner-z=-161 --floor="?????????;?CMCMMMC?;?MCMCCCM?;?MMMMMMM?;?CMMMMCM?;MCMCMMMM?;?????????;"
```

```
Please double check that the entered data is correct:
The coordinates of the spawner are x: 454, y: 19, z: -161
When standing on the floor, the y coordinate of the player should be 19
This is the dungeon floor, and the coordinates of each corner are:
(451, -157) ::: (451, -165)

    ?????????
    ?CMCMMMC?
    ?MCMCCCM?
    ?MMMMMMM?
    ?CMMMMCM?
    MCMCMMMM?
    ?????????

(457, -157) ::: (457, -165)

Started brutefroce using 4 threads. Estimated time: around 30 minutes
```

The corners are the coordinates inside the walls, so the top left corner of the dungeon is actually
`(452, -158)`

![dungeon2corner](https://i.imgur.com/nN3gpit.png)

```
Found 2 dungeon seeds:
["454,19,-161,167052204480180","454,19,-161,175684023910202"]
```

In this case there are 2 candidates because of too many `?` marks, but that's fine. We can assume
that the correct one is the first one and if that doesn't work try the next one.

## Dungeon 3

![dungeon3coords](https://i.imgur.com/2ymUh5j.png)

471 16 10

This dungeon has a hole in the ground. `?` means `C` or `M`, but a hole is different. Holes are
represented with `A` for Air. If there is a 2 tall hole, like here, you can be almost sure that this
is a real hole.

![dungeon3floor](https://i.imgur.com/YmGtVXi.png)

```
MMMMMMMCC
MCCCMMCMM
MMMMMMMCM
AACC?CCMM
CAMMMCMMC
MMMCMMMMM
MMMMMCMMM
```

But still, this holes can make the program fail so I recommend to find a different dungeon, if
possible.

```
slime_seed_finder dungeon-seed --spawner-x=471 --spawner-y=16 --spawner-z=10 --floor="MMMMMMMCC;MCCCMMCMM;MMMMMMMCM;AACC?CCMM;CAMMMCMMC;MMMCMMMMM;MMMMMCMMM;"
```

```
Please double check that the entered data is correct:
The coordinates of the spawner are x: 471, y: 16, z: 10
When standing on the floor, the y coordinate of the player should be 16
This is the dungeon floor, and the coordinates of each corner are:
(468, 14) ::: (468, 6)

    MMMMMMMCC
    MCCCMMCMM
    MMMMMMMCM
    AACC?CCMM
    CAMMMCMMC
    MMMCMMMMM
    MMMMMCMMM

(474, 14) ::: (474, 6)

Started brutefroce using 4 threads. Estimated time: around 30 minutes

```

Let's check the corner (468, 6) this time:


![dungeon3corner](https://i.imgur.com/S79EHsm.png)

```
Found 1 dungeon seeds:
["471,16,10,215961279147504"]
```

## dungeon-seed-to-world-seed

When you have 3 different dungeon seeds you can use this command to find the world seed.

```
["-244,17,-212,149909570098943"]
["454,19,-161,167052204480180","454,19,-161,175684023910202"]
["471,16,10,215961279147504"]
```

If one of the dungeons has more than possible seed, choose one of them.

```
slime_seed_finder dungeon-seed-to-world-seed -- "471,16,10,215961279147504" "454,19,-161,167052204480180" "-244,17,-212,149909570098943"
```

This command should find the world seed, but in this case we are on version 1.16, and that version
is not supported so this will not work. So let's pretend that this is the output:


```
Found 1 world seeds:
[1234]
```

Note that this are only the lower 48 bits, to get the full 64 bit seed you can use `extend48`:

```
echo [1234] > candidates.txt
slime_seed_finder extend48 --input-file candidates.txt
```

```
[
  -3658893222261816110,
  8847884417922761938
]
```

In this case there are 2 possible world seeds. Before Beta 1.8 the two seeds produce identical
worlds so just choose the one that looks better to you. But since Beta 1.8, biomes are different in
these two seeds, so you can use biomes to check which one is your seed. To do that, manually
generate the world and check if it matches.

If the `dungeon-seed-to-world-seed` command fails, you can increase the "limit of steps back" with
the `-l` flag. The runtime depends on `l^3`, so for example increasing it from the default 128 to
1016 will multiply the runtime by 500. For example, if it takes 200 seconds to run the command with
the default value, you can use this formula to calculate the time in seconds it will take to run
with `-l 1016`

```python
1016**3 / 128**3 * 200
```

The value of this limit depends on the population features present in the same chunk as the dungeon
but generated before the dungeon, so it is hard to set a maximum. The worst case would be a chunk
with 1 water lake, 1 lava lake, 8 dungeons, and this being the 8th dungeon.

```
slime_seed_finder dungeon-seed-to-world-seed -- "471,16,10,215961279147504" "454,19,-161,167052204480180" "-244,17,-212,149909570098943" -l 1100
```

You can use the `--resume-l` parameter to continue the bruteforce after having checked a lower value
of `-l`. For example, if the default value of `-l 128` did not find the world seed, use a command
like this to continue the bruteforce up to `-l 1100`:

```
slime_seed_finder dungeon-seed-to-world-seed -- "471,16,10,215961279147504" "454,19,-161,167052204480180" "-244,17,-212,149909570098943" -l 1100 --resume-l 128
```

# Dungeon floor templates

The only possible dungeon floor sizes are 7x7, 9x7, 7x9, 9x9.

Use this templates to draw the floor:

```
A = Air
C = Cobblestone
M = Mossy cobblestone
? = Either cobblestone or mossy cobblestone, not sure

MMMMMMM
MMMMMMM
MMMMMMM
MMMMMMM
MMMMMMM
MMMMMMM
MMMMMMM

MMMMMMMMM
MMMMMMMMM
MMMMMMMMM
MMMMMMMMM
MMMMMMMMM
MMMMMMMMM
MMMMMMMMM

MMMMMMM
MMMMMMM
MMMMMMM
MMMMMMM
MMMMMMM
MMMMMMM
MMMMMMM
MMMMMMM
MMMMMMM

MMMMMMMMM
MMMMMMMMM
MMMMMMMMM
MMMMMMMMM
MMMMMMMMM
MMMMMMMMM
MMMMMMMMM
MMMMMMMMM
MMMMMMMMM

"MMMMMMM;MMMMMMM;MMMMMMM;MMMMMMM;MMMMMMM;MMMMMMM;MMMMMMM;"
"MMMMMMMMM;MMMMMMMMM;MMMMMMMMM;MMMMMMMMM;MMMMMMMMM;MMMMMMMMM;MMMMMMMMM;"
"MMMMMMM;MMMMMMM;MMMMMMM;MMMMMMM;MMMMMMM;MMMMMMM;MMMMMMM;MMMMMMM;MMMMMMM;"
"MMMMMMMMM;MMMMMMMMM;MMMMMMMMM;MMMMMMMMM;MMMMMMMMM;MMMMMMMMM;MMMMMMMMM;MMMMMMMMMM;MMMMMMMMM;"
```


# Source

This video by Matthew Bolan explains a method to find the world seed using only one dungeon in a few minutes:

<https://www.youtube.com/watch?v=8CKh4x4iK38>

The algorithm implemented here is a simpler version of that, because we need three dungeons instead
of one, and a few hours instead of a few minutes.

This seems to be an implementation of that method:

<https://github.com/hube12/DungeonCracker>
