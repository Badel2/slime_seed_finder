use crate::anvil::WorldSearchInterface;
use std::collections::HashMap;

#[derive(Debug)]
pub struct BlockPattern {
    pub palette: HashMap<char, BlockPatternItem>,
    // map[y][z][x]
    pub map: Vec<Vec<Vec<char>>>,
    // List of possible rotation indexes
    pub rotations: Vec<u8>,
}

impl BlockPattern {
    pub fn compile(self) -> CompiledBlockPattern {
        let mut maps = crate::patterns::symmetry::get_rotated_maps(&self.map, &self.rotations);
        // Remove duplicates (can happen if the map is somewhat symmetric)
        maps.sort_unstable();
        maps.dedup();

        CompiledBlockPattern {
            palette: self.palette,
            maps,
        }
    }
}

#[derive(Debug)]
pub struct CompiledBlockPattern {
    palette: HashMap<char, BlockPatternItem>,
    // maps[rot][y][z][x]
    maps: Vec<Vec<Vec<Vec<char>>>>,
}

impl CompiledBlockPattern {
    fn map_dims(map: &Vec<Vec<Vec<char>>>) -> (u32, u32, u32) {
        let ys = map.len();
        let zs = map[0].len();
        let xs = map[0][0].len();

        (xs as u32, ys as u32, zs as u32)
    }

    pub fn max_y_size(&self) -> u32 {
        let mut max_y = 0;

        for map in &self.maps {
            let (_xs, ys, _zs) = Self::map_dims(map);
            max_y = std::cmp::max(max_y, ys);
        }

        max_y
    }

    // TODO: ideally, instead of a check_position, a CompiledBlockPattern would have a simple
    // search(world_bounds, world_interface) function. This will allow changing the iteration order
    // depending on the pattern.
    pub fn check_position<W: WorldSearchInterface>(
        &self,
        x: i64,
        y: i64,
        z: i64,
        y_range: std::ops::Range<isize>,
        world: &mut W,
    ) -> bool {
        for map in &self.maps {
            let (xs, ys, zs) = Self::map_dims(map);

            for py in 0..ys {
                for pz in 0..zs {
                    'next_pattern_block: for px in 0..xs {
                        let expected_c = map[py as usize][pz as usize][px as usize];
                        let expected = &self.palette[&expected_c];
                        let bx = x + px as i64;
                        let by = y + py as i64;
                        let bz = z + pz as i64;
                        if !expected.check(bx, by, bz, y_range.clone(), world) {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}

#[derive(Debug)]
pub enum BlockPatternItem {
    BlockName(String),
    BlockProperty { key: String, value: String },
    BlockPropertyExists { key: String },
    Not(Box<BlockPatternItem>),
    Or(Vec<BlockPatternItem>),
    And(Vec<BlockPatternItem>),
    Any,
}

impl BlockPatternItem {
    fn check<W: WorldSearchInterface>(
        &self,
        x: i64,
        y: i64,
        z: i64,
        y_range: std::ops::Range<isize>,
        world: &mut W,
    ) -> bool {
        match self {
            BlockPatternItem::BlockName(block_name) => {
                let world_block_name = match world.get_block_name(x, y, z) {
                    Some(x) => x,
                    None => {
                        // Out of bounds: could be above world height limit, or in
                        // a chunk that has not been generated yet.
                        if !y_range.contains(&(y as isize)) {
                            // y value outside of chunk y range, means we are above
                            // or below world height limit, so treat missing block
                            // as air
                            "minecraft:air"
                        } else {
                            // Not generated chunk, treat as not matched
                            return false;
                        }
                    }
                };

                block_name == world_block_name
            }
            BlockPatternItem::BlockProperty { key, value } => {
                let world_prop_value = match world.get_block_property(x, y, z, key) {
                    Some(x) => x,
                    None => {
                        // Out of bounds: could be above world height limit, or in
                        // a chunk that has not been generated yet.
                        if !y_range.contains(&(y as isize)) {
                            // y value outside of chunk y range, means we are above
                            // or below world height limit, so treat missing block
                            // as air (having no properties)
                            None
                        } else {
                            // Not generated chunk, treat as not matched
                            return false;
                        }
                    }
                };

                world_prop_value == Some(value)
            }
            BlockPatternItem::BlockPropertyExists { key } => {
                let world_prop_value = match world.get_block_property(x, y, z, key) {
                    Some(x) => x,
                    None => {
                        // Out of bounds: could be above world height limit, or in
                        // a chunk that has not been generated yet.
                        if !y_range.contains(&(y as isize)) {
                            // y value outside of chunk y range, means we are above
                            // or below world height limit, so treat missing block
                            // as air (having no properties)
                            None
                        } else {
                            // Not generated chunk, treat as not matched
                            return false;
                        }
                    }
                };

                world_prop_value.is_some()
            }
            BlockPatternItem::Not(pi) => !pi.check(x, y, z, y_range, world),
            BlockPatternItem::Or(vp) => vp.iter().any(|p| p.check(x, y, z, y_range.clone(), world)),
            BlockPatternItem::And(vp) => {
                vp.iter().all(|p| p.check(x, y, z, y_range.clone(), world))
            }
            BlockPatternItem::Any => true,
        }
    }
}

pub fn parse_block_pattern_map(x: &str) -> Result<Vec<Vec<Vec<char>>>, String> {
    let mut line_size = None;
    let mut block_size = None;
    let mut blocks = vec![];
    for block in x.split_terminator(|c| c == '\n' || c == ';') {
        log::debug!("Parse block: {:?}", block);
        let mut tiles = vec![];
        let mut num_lines = 0;
        for line in block.split_terminator(|c| c == ',') {
            log::debug!("Parse line: {:?}", line);
            if let Some(line_size) = line_size {
                if line.len() != line_size {
                    return Err(format!("All lines should have the same length"));
                }
            } else {
                line_size = Some(line.len());
            }
            let mut ts = vec![];
            for c in line.chars() {
                ts.push(c);
            }
            tiles.push(ts);
            num_lines += 1;
        }

        if let Some(block_size) = block_size {
            if num_lines != block_size {
                return Err(format!("All blocks should have the same length"));
            }
        } else {
            block_size = Some(num_lines);
        }

        blocks.push(tiles);
    }

    blocks.reverse();

    Ok(blocks)
}
