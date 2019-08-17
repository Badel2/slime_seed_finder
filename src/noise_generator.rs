use crate::java_rng::JavaRng;

/// Perlin noise generator
///
/// Uses the JavaRng as seed, so it only has 48 bits of entropy.
pub struct NoiseGeneratorPerlin {
    a: f64,
    b: f64,
    c: f64,
    d: [i32; 256],
}

impl NoiseGeneratorPerlin {
    pub fn new(seed: i64) -> Self {
        let mut r = JavaRng::with_seed(seed as u64);
        let a = r.next_double() * 256.0;
        let b = r.next_double() * 256.0;
        let c = r.next_double() * 256.0;
        let mut d = [0; 256];

        for i in 0..256 {
            d[i] = i as i32;
        }

        // Shuffle array
        for i in 0..256 {
            // j: random number in range i..256
            let j = r.next_int_n(256 - i as i32) + i as i32;
            let j = (j & 0xFF) as usize;
            d.swap(i, j);
        }

        Self { a, b, c, d }
    }

    pub fn get_ocean_temp(&self, mut d1: f64, mut d2: f64, mut d3: f64) -> f64 {
        d1 += self.a;
        d2 += self.b;
        d3 += self.c;
        let i1 = split_int(&mut d1) & 0xFF;
        let i2 = split_int(&mut d2) & 0xFF;
        let i3 = split_int(&mut d3) & 0xFF;
        let t1 = smootherstep(d1);
        let t2 = smootherstep(d2);
        let t3 = smootherstep(d3);

        let a1 = self.d[i1 as usize & 0xFF] + i2;
        let a2 = self.d[a1 as usize & 0xFF] + i3;
        let a3 = self.d[(a1 + 1) as usize & 0xFF] + i3;
        let b1 = self.d[(i1 + 1) as usize & 0xFF] + i2;
        let b2 = self.d[b1 as usize & 0xFF] + i3;
        let b3 = self.d[(b1 + 1) as usize & 0xFF] + i3;

        let mut l1 = indexed_lerp(self.d[(a2    ) as usize & 0xFF], d1      , d2      , d3);
        let     l2 = indexed_lerp(self.d[(b2    ) as usize & 0xFF], d1 - 1.0, d2      , d3);
        let mut l3 = indexed_lerp(self.d[(a3    ) as usize & 0xFF], d1      , d2 - 1.0, d3);
        let     l4 = indexed_lerp(self.d[(b3    ) as usize & 0xFF], d1 - 1.0, d2 - 1.0, d3);
        let mut l5 = indexed_lerp(self.d[(a2 + 1) as usize & 0xFF], d1      , d2      , d3 - 1.0);
        let     l6 = indexed_lerp(self.d[(b2 + 1) as usize & 0xFF], d1 - 1.0, d2      , d3 - 1.0);
        let mut l7 = indexed_lerp(self.d[(a3 + 1) as usize & 0xFF], d1      , d2 - 1.0, d3 - 1.0);
        let     l8 = indexed_lerp(self.d[(b3 + 1) as usize & 0xFF], d1 - 1.0, d2 - 1.0, d3 - 1.0);

        l1 = lerp(t1, l1, l2);
        l3 = lerp(t1, l3, l4);
        l5 = lerp(t1, l5, l6);
        l7 = lerp(t1, l7, l8);

        l1 = lerp(t2, l1, l3);
        l5 = lerp(t2, l5, l7);

        lerp(t3, l1, l5)
    }
}

// Split d1 between fractional part and integer part.
// Return integer part, and mutate argument to always be between 0 and 1
fn split_int(d1: &mut f64) -> i32 {
    let i1 = *d1 as i32 - if *d1 < 0.0 { 1 } else { 0 };
    *d1 -= i1 as f64;
    i1
}

// Smootherstep by Perlin.
// Sigmoid function which operates in the [0, 1] domain.
// This will "smooth out" the noise, so values close to 0 go closer to 0,
// values closer to 1 go even closer to 1, and values close to 0.5 stay the
// same.
fn smootherstep(d1: f64) -> f64 {
    d1*d1*d1 * (d1 * (d1*6.0-15.0) + 10.0)
}

// Linear interpolation between from and to.
// When part=0, return from and when part=1, return to.
fn lerp(part: f64, from: f64, to: f64) -> f64 {
    from + part * (to - from)
}

fn indexed_lerp(idx: i32, d1: f64, d2: f64, d3: f64) -> f64 {
    /* Table of vectors to cube edge centres (12 + 4 extra), used for ocean PRNG */
    let c_edge_x = [1.0,-1.0, 1.0,-1.0, 1.0,-1.0, 1.0,-1.0, 0.0, 0.0, 0.0, 0.0,  1.0, 0.0,-1.0, 0.0];
    let c_edge_y = [1.0, 1.0,-1.0,-1.0, 0.0, 0.0, 0.0, 0.0, 1.0,-1.0, 1.0,-1.0,  1.0,-1.0, 1.0,-1.0];
    let c_edge_z = [0.0, 0.0, 0.0, 0.0, 1.0, 1.0,-1.0,-1.0, 1.0, 1.0,-1.0,-1.0,  0.0, 1.0, 0.0,-1.0];

    let idx = (idx & 0xF) as usize;

    c_edge_x[idx] * d1 + c_edge_y[idx] * d2 + c_edge_z[idx] * d3
}

