use crate::biome_layers::Map;
use ndarray::Array2;

struct VoronoiSeedFinder {
    map_voronoi: Map,
    map_prev: Array2<i32>,
    points: Array2<VorPoint>,
}

enum VorPoint {
    Unknown,
    Known {
        x: u16,
        z: u16,
    },
    Partial {
        x_lo: u16,
        x_hi: u16,
        z_lo: u16,
        z_hi: u16,
    },
    Shape {
        area: u32,
        inside: Box<dyn Fn(u16, u16) -> bool>,
    },
}

#[cfg(test)]
mod tests {
    //use super::*;
    use crate::mc_rng::McRng;

    #[test]
    fn voronoi_bits() {
        let mut i = 63;
        let mut world_seed = 0xA6ADADADCB2;
        let mut old_gen: Option<[i32; 8]> = None;
        loop {
            let base_seed = 10;
            world_seed ^= 1 << i;
            let mut r = McRng::new(base_seed, world_seed);

            let x = 1;
            let z = 2;
            let p_x = 3;
            let p_z = 4;

            r.set_chunk_seed((x + p_x) << 2, (z + p_z) << 2);
            let da1 = r.next_int_n(1024);
            let da2 = r.next_int_n(1024);

            r.set_chunk_seed((x + p_x + 1) << 2, (z + p_z) << 2);
            let db1 = r.next_int_n(1024);
            let db2 = r.next_int_n(1024);

            r.set_chunk_seed((x + p_x) << 2, (z + p_z + 1) << 2);
            let dc1 = r.next_int_n(1024);
            let dc2 = r.next_int_n(1024);

            r.set_chunk_seed((x + p_x + 1) << 2, (z + p_z + 1) << 2);
            let dd1 = r.next_int_n(1024);
            let dd2 = r.next_int_n(1024);

            let gen = [da1, da2, db1, db2, dc1, dc2, dd1, dd2];
            println!("------ Bit {} ------", i);
            println!("{:016X}", world_seed);
            println!("{:03X?}", gen);

            if let Some(old) = old_gen {
                if old != gen {
                    //println!("Bit {} differs:", i);
                    //println!("{:03X?}", gen.iter().zip(old.iter()).map(|(a, b)| a ^ b).collect::<Vec<_>>());
                    assert!(i < 34);
                }
            }

            old_gen = Some(gen);

            if i == 0 {
                break;
            }
            i -= 1;
        }
    }
}
