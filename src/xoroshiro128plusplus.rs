pub struct Xoroshiro128PlusPlus {
    lo: u64,
    hi: u64,
}

impl Xoroshiro128PlusPlus {
    pub fn new(mut seed_lo: u64, mut seed_hi: u64) -> Self {
        if seed_lo | seed_hi == 0 {
            seed_lo = 11400714819323198485;
            seed_hi = 7640891576956012809;
        }

        Self {
            lo: seed_lo,
            hi: seed_hi,
        }
    }

    pub fn with_u64_seed(world_seed: u64) -> Self {
        let (lo, hi) = extend128(world_seed);

        Self::new(lo, hi)
    }

    pub fn next_long(&mut self) -> u64 {
        let l = self.lo;
        let h = self.hi;
        let n = l.wrapping_add(h).rotate_left(17).wrapping_add(l);

        let h = h ^ l;
        self.lo = l.rotate_left(49) ^ h ^ (h << 21);
        self.hi = h.rotate_left(28);

        n
    }

    pub fn next_int_n(&mut self, n: u32) -> i32 {
        let mut r = (self.next_long() & 0xFFFFFFFF).wrapping_mul(n as u64);

        if (r as u32) < n {
            while (r as u32) < (!n + 1) % n {
                r = (self.next_long() & 0xFFFFFFFF).wrapping_mul(n as u64);
            }
        }

        (r >> 32) as i32
    }

    pub fn next_double(&mut self) -> f64 {
        ((self.next_long() >> (64 - 53)) as f64) * 1.1102230246251565E-16
    }
}

pub fn mix_stafford_13(mut i: u64) -> u64 {
    i = (i ^ (i >> 30)).wrapping_mul(13787848793156543929);
    i = (i ^ (i >> 27)).wrapping_mul(10723151780598845931);

    i ^ (i >> 31)
}

pub fn extend128(x: u64) -> (u64, u64) {
    let lo = x ^ 7640891576956012809;
    let hi = lo.wrapping_add(11400714819323198485);

    (mix_stafford_13(lo), mix_stafford_13(hi))
}

#[cfg(test)]
mod tests {
    use super::*;

    mod xoroshiro {
        use super::*;

        #[test]
        fn seed_0() {
            let mut xs = Xoroshiro128PlusPlus::new(0, 0);

            assert_eq!(xs.next_long(), 6807859099481836695);
            assert_eq!(xs.next_long(), 5275285228792843439);
            assert_eq!(xs.next_long(), 16563609962399111895);
            assert_eq!(xs.next_long(), 10965461193141861783);
            assert_eq!(xs.next_long(), 10562481853947742313);
        }

        // values from cubiomes:
        // 1 0 131073
        // 1 0 598409205645317
        // 1 0 579350287391785545
        // 1 0 10165227389583065669
        // 1 0 543646121330869598
        // 0 1 131072
        // 0 1 35459252224001
        // 0 1 576535519371067973
        // 0 1 9910210976930800449
        // 0 1 4866527886176355657
        // 1 1 262145
        // 1 1 562949953421316
        // 1 1 2814768020717572
        // 1 1 327074010985177348
        // 1 1 4900490409448374293

        #[test]
        fn seed_1() {
            let mut xs = Xoroshiro128PlusPlus::new(1, 0);

            assert_eq!(xs.next_long(), 131073);
            assert_eq!(xs.next_long(), 598409205645317);
            assert_eq!(xs.next_long(), 579350287391785545);
            assert_eq!(xs.next_long(), 10165227389583065669);
            assert_eq!(xs.next_long(), 543646121330869598);
        }

        #[test]
        fn seed_2_64() {
            let mut xs = Xoroshiro128PlusPlus::new(0, 1);

            assert_eq!(xs.next_long(), 131072);
            assert_eq!(xs.next_long(), 35459252224001);
            assert_eq!(xs.next_long(), 576535519371067973);
            assert_eq!(xs.next_long(), 9910210976930800449);
            assert_eq!(xs.next_long(), 4866527886176355657);
        }

        #[test]
        fn seed_2_64_plus_1() {
            let mut xs = Xoroshiro128PlusPlus::new(1, 1);

            assert_eq!(xs.next_long(), 262145);
            assert_eq!(xs.next_long(), 562949953421316);
            assert_eq!(xs.next_long(), 2814768020717572);
            assert_eq!(xs.next_long(), 327074010985177348);
            assert_eq!(xs.next_long(), 4900490409448374293);
        }

        // values from cubiomes:
        // 1 0 7.1054273576010019e-15
        // 1 0 3.2439828039798613e-05
        // 1 0 0.03140664201101373
        // 1 0 0.55105808097975562
        // 1 0 0.029471115290512273
        // 0 1 7.1054273576010019e-15
        // 0 1 1.9222499147986127e-06
        // 0 1 0.031254053130858495
        // 0 1 0.53723361354890331
        // 0 1 0.26381500533268465
        // 1 1 1.4210854715202004e-14
        // 1 1 3.0517578125e-05
        // 1 1 0.00015258888015523553
        // 1 1 0.017730717663683837
        // 1 1 0.26565611740841533

        #[test]
        fn next_double_seed_1() {
            let mut xs = Xoroshiro128PlusPlus::new(1, 0);

            assert_eq!(xs.next_double(), 7.1054273576010019e-15);
            assert_eq!(xs.next_double(), 3.2439828039798613e-05);
            assert_eq!(xs.next_double(), 0.03140664201101373);
            assert_eq!(xs.next_double(), 0.55105808097975562);
            assert_eq!(xs.next_double(), 0.029471115290512273);
        }

        #[test]
        fn next_double_seed_2_64() {
            let mut xs = Xoroshiro128PlusPlus::new(0, 1);

            assert_eq!(xs.next_double(), 7.1054273576010019e-15);
            assert_eq!(xs.next_double(), 1.9222499147986127e-06);
            assert_eq!(xs.next_double(), 0.031254053130858495);
            assert_eq!(xs.next_double(), 0.53723361354890331);
            assert_eq!(xs.next_double(), 0.26381500533268465);
        }

        #[test]
        fn next_double_seed_2_64_plus_1() {
            let mut xs = Xoroshiro128PlusPlus::new(1, 1);

            assert_eq!(xs.next_double(), 1.4210854715202004e-14);
            assert_eq!(xs.next_double(), 3.0517578125e-05);
            assert_eq!(xs.next_double(), 0.00015258888015523553);
            assert_eq!(xs.next_double(), 0.017730717663683837);
            assert_eq!(xs.next_double(), 0.26565611740841533);
        }

        // values from cubiomes
        // 1 0 0
        // 1 0 0
        // 1 0 314
        // 1 0 4
        // 1 0 349
        // 0 1 0
        // 0 1 0
        // 0 1 64
        // 0 1 504
        // 0 1 97
        // 1 1 0
        // 1 1 0
        // 1 1 250
        // 1 1 500
        // 1 1 251

        #[test]
        fn next_int_seed_1() {
            let mut xs = Xoroshiro128PlusPlus::new(1, 0);

            assert_eq!(xs.next_int_n(1000), 0);
            assert_eq!(xs.next_int_n(1000), 0);
            assert_eq!(xs.next_int_n(1000), 314);
            assert_eq!(xs.next_int_n(1000), 4);
            assert_eq!(xs.next_int_n(1000), 349);
        }

        #[test]
        fn next_int_seed_2_64() {
            let mut xs = Xoroshiro128PlusPlus::new(0, 1);

            assert_eq!(xs.next_int_n(1000), 0);
            assert_eq!(xs.next_int_n(1000), 0);
            assert_eq!(xs.next_int_n(1000), 64);
            assert_eq!(xs.next_int_n(1000), 504);
            assert_eq!(xs.next_int_n(1000), 97);
        }

        #[test]
        fn next_int_seed_2_64_plus_1() {
            let mut xs = Xoroshiro128PlusPlus::new(1, 1);

            assert_eq!(xs.next_int_n(1000), 0);
            assert_eq!(xs.next_int_n(1000), 0);
            assert_eq!(xs.next_int_n(1000), 250);
            assert_eq!(xs.next_int_n(1000), 500);
            assert_eq!(xs.next_int_n(1000), 251);
        }
    }

    mod mix_stafford {
        use super::*;

        #[test]
        fn mix_0() {
            assert_eq!(mix_stafford_13(0), 0);
        }

        #[test]
        fn mix_1() {
            assert_eq!(mix_stafford_13(1), 6238072747940578789);
        }
    }

    mod extend {
        use super::*;

        // values from cubiomes
        // 0: 3847398142028685078, 7192185014346937746
        // 1: 5272463233947570727, 1927618558350093866
        // 9223372036854775808: 12064109425296606738, 5448932524140013571

        #[test]
        fn ext_0() {
            assert_eq!(extend128(0), (3847398142028685078, 7192185014346937746));
        }

        #[test]
        fn ext_1() {
            assert_eq!(extend128(1), (5272463233947570727, 1927618558350093866));
        }

        #[test]
        fn ext_2_63() {
            assert_eq!(
                extend128(1 << 63),
                (12064109425296606738, 5448932524140013571)
            );
        }
    }
}
