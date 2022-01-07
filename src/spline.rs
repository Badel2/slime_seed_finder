#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub enum SplineType {
    Continentalness,
    Erosion,
    Ridges,
    Weirdness,
}

// Two kinds of splines:
// FixSpline { val: f32 }
// Spline { typ: SplineType, ch: Vec<(loc, der, FixSpline | Spline)> }
#[derive(Debug, Clone)]
pub enum Spline {
    Fix {
        val: f32,
    },
    Sp {
        typ: SplineType,
        ch: Vec<SplineChild>,
    },
}

#[derive(Debug, Clone)]
pub struct SplineChild {
    loc: f32,
    der: f32,
    val: Spline,
}

impl Spline {
    pub fn new(typ: SplineType) -> Self {
        Self::Sp { typ, ch: vec![] }
    }

    pub fn new_fix(val: f32) -> Self {
        Self::Fix { val }
    }

    pub fn new_38219(f: f32, bl: bool) -> Self {
        let mut sp = Self::new(SplineType::Ridges);
        let i = get_offset_value(-1.0, f);
        let k = get_offset_value(1.0, f);
        let l = 1.0 - (1.0 - f) * 0.5;
        let u = 0.5 * (1.0 - f);
        let l = u / (0.46082947 * l) - 1.17;

        if -0.65 < l && l < 1.0 {
            let u = get_offset_value(-0.65, f);
            let p = get_offset_value(-0.75, f);
            let q = (p - i) * 4.0;
            let r = get_offset_value(l, f);
            let s = (k - r) / (1.0 - l);

            sp.add(-1.0, Spline::new_fix(i), q);
            sp.add(-0.75, Spline::new_fix(p), 0.0);
            sp.add(-0.65, Spline::new_fix(u), 0.0);
            sp.add(l - 0.01, Spline::new_fix(r), 0.0);
            sp.add(l, Spline::new_fix(r), s);
            sp.add(1.0, Spline::new_fix(k), s);
        } else {
            let u = (k - i) * 0.5;
            if bl {
                sp.add(-1.0, Spline::new_fix(if i > 0.2 { i } else { 0.2 }), 0.0);
                sp.add(0.0, Spline::new_fix(lerp(0.5, i, k)), u);
            } else {
                sp.add(-1.0, Spline::new_fix(i), u);
            }
            sp.add(1.0, Spline::new_fix(k), u);
        }

        sp
    }

    pub fn new_flat_offset(f: f32, g: f32, h: f32, i: f32, j: f32, k: f32) -> Self {
        let mut sp = Self::new(SplineType::Ridges);

        let mut l = 0.5 * (g - f);
        if l < k {
            l = k;
        }
        let m = 5.0 * (h - g);

        sp.add(-1.0, Spline::new_fix(f), l);
        sp.add(-0.4, Spline::new_fix(g), if l < m { l } else { m });
        sp.add(0.0, Spline::new_fix(h), m);
        sp.add(0.4, Spline::new_fix(i), 2.0 * (i - h));
        sp.add(1.0, Spline::new_fix(j), 0.7 * (j - i));

        sp
    }

    pub fn new_land(f: f32, g: f32, h: f32, i: f32, j: f32, k: f32, bl: bool) -> Self {
        let sp1 = Spline::new_38219(lerp(i, 0.6, 1.5), bl);
        let sp2 = Spline::new_38219(lerp(i, 0.6, 1.0), bl);
        let sp3 = Spline::new_38219(i, bl);

        let ih = 0.5 * i;

        let sp4 = Spline::new_flat_offset(f - 0.15, ih, ih, ih, i * 0.6, 0.5);
        let sp5 = Spline::new_flat_offset(f, j * i, g * i, ih, i * 0.6, 0.5);
        let sp6 = Spline::new_flat_offset(f, j, j, g, h, 0.5);
        let sp7 = Spline::new_flat_offset(f, j, j, g, h, 0.5);

        let mut sp8 = Spline::new(SplineType::Ridges);
        sp8.add(-1.0, Spline::new_fix(f), 0.0);
        sp8.add(-0.4, sp6.clone(), 0.0);
        sp8.add(0.0, Spline::new_fix(h + 0.07), 0.0);

        let sp9 = Spline::new_flat_offset(-0.02, k, k, g, h, 0.0);
        let mut sp = Spline::new(SplineType::Erosion);

        sp.add(-0.85, sp1, 0.0);
        sp.add(-0.7, sp2, 0.0);
        sp.add(-0.4, sp3, 0.0);
        sp.add(-0.35, sp4, 0.0);
        sp.add(-0.1, sp5, 0.0);
        sp.add(0.2, sp6, 0.0);

        if bl {
            sp.add(0.4, sp7.clone(), 0.0);
            sp.add(0.45, sp8.clone(), 0.0);
            sp.add(0.55, sp8, 0.0);
            sp.add(0.58, sp7, 0.0);
        }

        sp.add(0.7, sp9, 0.0);

        sp
    }

    pub fn new_continental() -> Self {
        let mut sp = Spline::new(SplineType::Continentalness);

        let sp1 = Spline::new_land(-0.15, 0.0, 0.0, 0.1, 0.0, -0.03, false);
        let sp2 = Spline::new_land(-0.10, 0.03, 0.1, 0.1, 0.01, -0.03, false);
        let sp3 = Spline::new_land(-0.10, 0.03, 0.1, 0.7, 0.01, -0.03, true);
        let sp4 = Spline::new_land(-0.05, 0.03, 0.1, 1.0, 0.01, 0.01, true);

        sp.add(-1.1, Spline::new_fix(0.044), 0.0);
        sp.add(-1.02, Spline::new_fix(-0.2222), 0.0);
        sp.add(-0.51, Spline::new_fix(-0.2222), 0.0);
        sp.add(-0.44, Spline::new_fix(-0.12), 0.0);
        sp.add(-0.18, Spline::new_fix(-0.12), 0.0);
        sp.add(-0.16, sp1.clone(), 0.0);
        sp.add(-0.15, sp1, 0.0);
        sp.add(-0.10, sp2, 0.0);
        sp.add(0.25, sp3, 0.0);
        sp.add(1.00, sp4, 0.0);

        sp
    }

    pub fn add(&mut self, loc: f32, val: Spline, der: f32) {
        match self {
            Self::Sp { ch, .. } => {
                ch.push(SplineChild { loc, val, der });
            }
            Self::Fix { .. } => {
                panic!("Attempted to add child to fix spline");
            }
        }
    }

    pub fn get_spline(&self, vals: &[f32]) -> f32 {
        match self {
            Self::Fix { val } => *val,
            Self::Sp { typ, ch } => {
                let f = vals[*typ as usize];
                let mut i = 0;

                while i < ch.len() {
                    if ch[i].loc >= f {
                        break;
                    }

                    i += 1;
                }

                if i == 0 || i == ch.len() {
                    if i != 0 {
                        i -= 1;
                    }

                    let v = ch[i].val.get_spline(vals);
                    return v + ch[i].der * (f - ch[i].loc);
                }

                let sp1 = &ch[i - 1].val;
                let sp2 = &ch[i].val;
                let g = ch[i - 1].loc;
                let h = ch[i].loc;
                let k = (f - g) / (h - g);
                let l = ch[i - 1].der;
                let m = ch[i].der;
                let n = sp1.get_spline(vals);
                let o = sp2.get_spline(vals);
                let p = l * (h - g) - (o - n);
                let q = -m * (h - g) + (o - n);
                let r = lerp(k, n, o) + k * (1.0 - k) * lerp(k, p, q);

                r
            }
        }
    }
}

// Linear interpolation between from and to.
// When part=0, return from and when part=1, return to.
fn lerp(part: f32, from: f32, to: f32) -> f32 {
    from + part * (to - from)
}

fn get_offset_value(weirdness: f32, continentalness: f32) -> f32 {
    let f0 = 1.0 - (1.0 - continentalness) * 0.5;
    let f1 = 0.5 * (1.0 - continentalness);
    let f2 = (weirdness + 1.17) * 0.46082947;
    let off = f2 * f0 - f1;

    if weirdness < -0.7 {
        if off > -0.2222 {
            off
        } else {
            -0.2222
        }
    } else if off > 0.0 {
        off
    } else {
        0.0
    }
}
