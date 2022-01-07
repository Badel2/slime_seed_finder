use crate::java_rng::JavaRng;
use crate::xoroshiro128plusplus::Xoroshiro128PlusPlus;

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

        for (i, di) in d.iter_mut().enumerate() {
            *di = i as i32;
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

        let mut l1 = indexed_lerp(self.d[(a2) as usize & 0xFF], d1, d2, d3);
        let l2 = indexed_lerp(self.d[(b2) as usize & 0xFF], d1 - 1.0, d2, d3);
        let mut l3 = indexed_lerp(self.d[(a3) as usize & 0xFF], d1, d2 - 1.0, d3);
        let l4 = indexed_lerp(self.d[(b3) as usize & 0xFF], d1 - 1.0, d2 - 1.0, d3);
        let mut l5 = indexed_lerp(self.d[(a2 + 1) as usize & 0xFF], d1, d2, d3 - 1.0);
        let l6 = indexed_lerp(self.d[(b2 + 1) as usize & 0xFF], d1 - 1.0, d2, d3 - 1.0);
        let mut l7 = indexed_lerp(self.d[(a3 + 1) as usize & 0xFF], d1, d2 - 1.0, d3 - 1.0);
        let l8 = indexed_lerp(
            self.d[(b3 + 1) as usize & 0xFF],
            d1 - 1.0,
            d2 - 1.0,
            d3 - 1.0,
        );

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
    d1 * d1 * d1 * (d1 * (d1 * 6.0 - 15.0) + 10.0)
}

// Linear interpolation between from and to.
// When part=0, return from and when part=1, return to.
fn lerp(part: f64, from: f64, to: f64) -> f64 {
    from + part * (to - from)
}

fn indexed_lerp(idx: i32, d1: f64, d2: f64, d3: f64) -> f64 {
    /* Table of vectors to cube edge centres (12 + 4 extra), used for ocean PRNG */
    let c_edge_x = [
        1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, -1.0, 0.0,
    ];
    let c_edge_y = [
        1.0, 1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0, 1.0, -1.0,
    ];
    let c_edge_z = [
        0.0, 0.0, 0.0, 0.0, 1.0, 1.0, -1.0, -1.0, 1.0, 1.0, -1.0, -1.0, 0.0, 1.0, 0.0, -1.0,
    ];

    let idx = (idx & 0xF) as usize;

    c_edge_x[idx] * d1 + c_edge_y[idx] * d2 + c_edge_z[idx] * d3
}

/// Perlin noise generator
///
/// Uses the Xoroshiro128PlusPlus generator.
pub struct NoiseGeneratorPerlin128 {
    a: f64,
    b: f64,
    c: f64,
    d: [i32; 256],
    amplitude: f64,
    lacunarity: f64,
}

impl NoiseGeneratorPerlin128 {
    pub fn new(r: &mut Xoroshiro128PlusPlus, amplitude: f64, lacunarity: f64) -> Self {
        let a = r.next_double() * 256.0;
        let b = r.next_double() * 256.0;
        let c = r.next_double() * 256.0;
        let mut d = [0; 256];

        for (i, di) in d.iter_mut().enumerate() {
            *di = i as i32;
        }

        // Shuffle array
        for i in 0..256 {
            // j: random number in range i..256
            let j = (r.next_int_n(256 - i as u32) as i32) + i as i32;
            let j = (j & 0xFF) as usize;
            d.swap(i, j);
        }

        Self {
            a,
            b,
            c,
            d,
            amplitude,
            lacunarity,
        }
    }

    pub fn sample(&self, mut d1: f64, mut d2: f64, mut d3: f64, yamp: f64, ymin: f64) -> f64 {
        d1 += self.a;
        d2 += self.b;
        d3 += self.c;
        let i1 = split_int(&mut d1) & 0xFF;
        let i2 = split_int(&mut d2) & 0xFF;
        let i3 = split_int(&mut d3) & 0xFF;
        let t1 = smootherstep(d1);
        let t2 = smootherstep(d2);
        let t3 = smootherstep(d3);

        if yamp != 0.0 {
            let yclamp = if ymin < d2 { ymin } else { d2 };
            d2 -= (yclamp / yamp).floor() * yamp;
        }

        let a1 = self.d[i1 as usize & 0xFF] + i2;
        let a2 = self.d[a1 as usize & 0xFF] + i3;
        let a3 = self.d[(a1 + 1) as usize & 0xFF] + i3;
        let b1 = self.d[(i1 + 1) as usize & 0xFF] + i2;
        let b2 = self.d[b1 as usize & 0xFF] + i3;
        let b3 = self.d[(b1 + 1) as usize & 0xFF] + i3;

        let mut l1 = indexed_lerp(self.d[(a2) as usize & 0xFF], d1, d2, d3);
        let l2 = indexed_lerp(self.d[(b2) as usize & 0xFF], d1 - 1.0, d2, d3);
        let mut l3 = indexed_lerp(self.d[(a3) as usize & 0xFF], d1, d2 - 1.0, d3);
        let l4 = indexed_lerp(self.d[(b3) as usize & 0xFF], d1 - 1.0, d2 - 1.0, d3);
        let mut l5 = indexed_lerp(self.d[(a2 + 1) as usize & 0xFF], d1, d2, d3 - 1.0);
        let l6 = indexed_lerp(self.d[(b2 + 1) as usize & 0xFF], d1 - 1.0, d2, d3 - 1.0);
        let mut l7 = indexed_lerp(self.d[(a3 + 1) as usize & 0xFF], d1, d2 - 1.0, d3 - 1.0);
        let l8 = indexed_lerp(
            self.d[(b3 + 1) as usize & 0xFF],
            d1 - 1.0,
            d2 - 1.0,
            d3 - 1.0,
        );

        l1 = lerp(t1, l1, l2);
        l3 = lerp(t1, l3, l4);
        l5 = lerp(t1, l5, l6);
        l7 = lerp(t1, l7, l8);

        l1 = lerp(t2, l1, l3);
        l5 = lerp(t2, l5, l7);

        lerp(t3, l1, l5)
    }
}

pub struct NoiseGeneratorOctave128 {
    octaves: Vec<NoiseGeneratorPerlin128>,
}

impl NoiseGeneratorOctave128 {
    pub fn new(xr: &mut Xoroshiro128PlusPlus, amplitudes: &[f64], omin: i32) -> Self {
        let md5_octave_n: [(u64, u64); 13] = [
            (0xb198de63a8012672, 0x7b84cad43ef7b5a8), // md5 "octave_-12"
            (0x0fd787bfbc403ec3, 0x74a4a31ca21b48b8), // md5 "octave_-11"
            (0x36d326eed40efeb2, 0x5be9ce18223c636a), // md5 "octave_-10"
            (0x082fe255f8be6631, 0x4e96119e22dedc81), // md5 "octave_-9"
            (0x0ef68ec68504005e, 0x48b6bf93a2789640), // md5 "octave_-8"
            (0xf11268128982754f, 0x257a1d670430b0aa), // md5 "octave_-7"
            (0xe51c98ce7d1de664, 0x5f9478a733040c45), // md5 "octave_-6"
            (0x6d7b49e7e429850a, 0x2e3063c622a24777), // md5 "octave_-5"
            (0xbd90d5377ba1b762, 0xc07317d419a7548d), // md5 "octave_-4"
            (0x53d39c6752dac858, 0xbcd1c5a80ab65b3e), // md5 "octave_-3"
            (0xb4a24d7a84e7677b, 0x023ff9668e89b5c4), // md5 "octave_-2"
            (0xdffa22b534c5f608, 0xb9b67517d3665ca9), // md5 "octave_-1"
            (0xd50708086cef4d7c, 0x6e1651ecc7f43309), // md5 "octave_0"
        ];

        let len = amplitudes.len();
        let mut lacuna = 2.0_f64.powf(omin as f64);
        let mut persist = 2.0_f64.powf((len - 1) as f64) / (((1u64 << len) as f64) - 1.0);

        let xlo = xr.next_long();
        let xhi = xr.next_long();

        let mut octaves = Vec::with_capacity(len);

        let mut i = 0;
        while i < len {
            if amplitudes[i] != 0.0 {
                let (pxrlo, pxrhi) = md5_octave_n[(12 + omin) as usize + i];
                let mut pxr = Xoroshiro128PlusPlus::new(xlo ^ pxrlo, xhi ^ pxrhi);
                octaves.push(NoiseGeneratorPerlin128::new(
                    &mut pxr,
                    amplitudes[i] * persist,
                    lacuna,
                ));
            }

            i += 1;
            lacuna *= 2.0;
            persist *= 0.5;
        }

        Self { octaves }
    }

    pub fn sample(&self, x: f64, y: f64, z: f64) -> f64 {
        let mut v = 0.0;

        for p in &self.octaves {
            let lf = p.lacunarity;
            let ax = x * lf;
            let ay = y * lf;
            let az = z * lf;

            let pv = p.sample(ax, ay, az, 0.0, 0.0);
            v += p.amplitude * pv;
        }

        v
    }
}

pub struct NoiseGeneratorDoublePerlin128 {
    amplitude: f64,
    octave_a: NoiseGeneratorOctave128,
    octave_b: NoiseGeneratorOctave128,
}

impl NoiseGeneratorDoublePerlin128 {
    pub fn new(xr: &mut Xoroshiro128PlusPlus, amplitudes: &[f64], omin: i32) -> Self {
        let octave_a = NoiseGeneratorOctave128::new(xr, amplitudes, omin);
        let octave_b = NoiseGeneratorOctave128::new(xr, amplitudes, omin);

        // trim amplitudes of zero
        let len = len_without_start_zeros_and_end_zeros(amplitudes);
        let amplitude = (10.0 / 6.0) * (len as f64) / ((len + 1) as f64);

        Self {
            amplitude,
            octave_a,
            octave_b,
        }
    }

    pub fn sample(&self, x: f64, y: f64, z: f64) -> f64 {
        let f: f64 = 337.0 / 331.0;
        let mut v = 0.0;

        v += self.octave_a.sample(x, y, z);
        v += self.octave_b.sample(x * f, y * f, z * f);

        v * self.amplitude
    }
}

// Returns length of x, but ignoring all the consecutive 0.0 elements at the start, and at the end.
fn len_without_start_zeros_and_end_zeros(x: &[f64]) -> usize {
    if x.is_empty() {
        return 0;
    }

    let mut start = 0;
    let mut end = x.len() - 1;

    while x[end] == 0.0 {
        if end == 0 {
            // All zeros
            return 0;
        } else {
            end -= 1;
        }
    }

    // Will not index out of bounds because here x cannot be all zeros
    while x[start] == 0.0 {
        start += 1;
    }

    end - start + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bug_continue_not_updating_lacuna_and_persist() {
        // The seed doesn't affect this test
        let mut xr = Xoroshiro128PlusPlus::new(1, 0);
        // Use a list of amplitudes with one element equal to "0.0"
        let amplitudes = [1.5, 0.0, 1.0];
        let n = NoiseGeneratorOctave128::new(&mut xr, &amplitudes, -10);

        // The amplitude equal to "0.0" is skipped
        assert_eq!(n.octaves.len(), 2);
        // octaves[0] is initialized using amplitudes[0]
        assert_eq!(n.octaves[0].amplitude, 0.8571428571428571);
        assert_eq!(n.octaves[0].lacunarity, 0.0009765625);
        // But octaves[1] is initialized using amplitudes[2]
        assert_eq!(n.octaves[1].amplitude, 0.14285714285714285);
        // And the lacunarity takes into account the missing octave
        assert_eq!(n.octaves[1].lacunarity, 0.0009765625 * 4.0);
    }

    #[test]
    fn len_trim_zeros() {
        assert_eq!(len_without_start_zeros_and_end_zeros(&[]), 0);
        assert_eq!(len_without_start_zeros_and_end_zeros(&[0.0]), 0);
        assert_eq!(len_without_start_zeros_and_end_zeros(&[1.0]), 1);
        assert_eq!(len_without_start_zeros_and_end_zeros(&[0.0, 0.0]), 0);
        assert_eq!(len_without_start_zeros_and_end_zeros(&[1.0, 0.0]), 1);
        assert_eq!(len_without_start_zeros_and_end_zeros(&[0.0, 1.0]), 1);
        assert_eq!(len_without_start_zeros_and_end_zeros(&[1.0, 1.0]), 2);
        assert_eq!(len_without_start_zeros_and_end_zeros(&[0.0, 1.0, 1.0]), 2);
        assert_eq!(len_without_start_zeros_and_end_zeros(&[1.0, 0.0, 1.0]), 3);
        assert_eq!(len_without_start_zeros_and_end_zeros(&[1.0, 1.0, 0.0]), 2);
        assert_eq!(len_without_start_zeros_and_end_zeros(&[0.0, 1.0, 0.0]), 1);
    }
}
