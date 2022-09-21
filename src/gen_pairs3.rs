// When generators can be used in stable Rust, rewrite this file.

#[derive(Debug)]
enum ResumeState<T, L> {
    // Initial state, yield (0, 0, 0)
    Default,
    // Inside nested for loops, yield (i, j, m)
    S1 {
        i: L,
        j: L,
        i_a: T,
        i_b: T,
        i_c: T,
        j_a: T,
        j_b: T,
        j_c: T,
        yield_123: u8,
    },
    // Inside single for loop, yield (i, m, m)
    S2 {
        i: L,
        i_a: T,
        i_b: T,
        i_c: T,
        yield_123: u8,
    },
    // Outside both loops, yield (m, m, m)
    S3,
}

/// Iterator over "pairs" of 3 elements `(T, T, T)`, in an order that allows checking all the
/// combinations with values `< n` before checking the first value `== n`.
///
/// So if this was using pairs of 2 elements, the order would be
///
/// ```text
/// (0,0)
/// (0,1)
/// (1,0)
/// (1,1)
/// (0,2)
/// (1,2)
/// (2,0)
/// (2,1)
/// (2,2)
/// ```
///
/// This allows to implement a resumable iterator. For example, we can first check all the combinations from 0
/// to 100, and then if that did not find the value that we were looking for, extend the search
/// from 100 to 200.
pub struct GenPairs3L<T, F, L> {
    seed: (T, T, T),
    increment: F,
    limit: L,
    limits: (L, L, L),
    m: L,
    m_a: T,
    m_b: T,
    m_c: T,
    resume_state: ResumeState<T, L>,
}

impl<T, F, L> GenPairs3L<T, F, L>
where
    T: Clone + std::fmt::Debug,
    F: FnMut(T) -> T,
    L: Counter + Clone + Ord + std::fmt::Display + std::fmt::Debug,
{
    pub fn new(seed: (T, T, T), increment: F, limits: (L, L, L)) -> Self {
        let (m_a, m_b, m_c) = (seed.0.clone(), seed.1.clone(), seed.2.clone());
        let limit = max3(limits.0.clone(), limits.1.clone(), limits.2.clone());

        Self {
            seed,
            increment,
            limit,
            limits,
            m: L::zero(),
            m_a,
            m_b,
            m_c,
            resume_state: ResumeState::Default,
        }
    }

    /// Create an iterator that only runs one step of the inner state machine. Here step refers to
    /// the max value that will be used: with step=2, all the yielded elements will have a 2 at
    /// some position, and the iterator will stop before yielding an element with a 3.
    pub fn new_one_step(seed: (T, T, T), mut increment: F, mut limits: (L, L, L), step: L) -> Self {
        let (mut m_a, mut m_b, mut m_c) = (seed.0.clone(), seed.1.clone(), seed.2.clone());
        let mut new_limit = step.clone();
        new_limit.increment();
        let limit = std::cmp::min(
            new_limit,
            max3(limits.0.clone(), limits.1.clone(), limits.2.clone()),
        );
        limits.0 = std::cmp::min(limits.0.clone(), limit.clone());
        limits.1 = std::cmp::min(limits.1.clone(), limit.clone());
        limits.2 = std::cmp::min(limits.2.clone(), limit.clone());
        let resume_state = if step.done(&limit) || step == L::zero() {
            ResumeState::Default
        } else {
            ResumeState::S1 {
                i: L::zero(),
                j: L::zero(),
                i_a: seed.0.clone(),
                i_b: seed.1.clone(),
                i_c: seed.2.clone(),
                j_a: seed.0.clone(),
                j_b: seed.1.clone(),
                j_c: seed.2.clone(),
                yield_123: 0,
            }
        };
        let mut m = L::zero();
        while !m.done(&step) {
            m.increment();
            m_a = (increment)(m_a.clone());
            m_b = (increment)(m_b.clone());
            m_c = (increment)(m_c.clone());
        }

        Self {
            seed,
            increment,
            limit,
            limits,
            m: step,
            m_a,
            m_b,
            m_c,
            resume_state,
        }
    }

    // Used to implement `next`, but this function can return `None` before the iterator has
    // finished. In that case, the caller is expected to call this function again.
    // Returns the yielded element and a boolean flag indicating if the iterator has finished.
    fn maybe_next(&mut self) -> (Option<(T, T, T)>, bool) {
        match &mut self.resume_state {
            ResumeState::Default => {
                // Edge case 0 >= 0, and also exhausted case after S3 returns last item
                if self.m.done(&self.limit) {
                    return (None, true);
                }

                // Yield (0, 0, 0) and go to S1 state
                let mut res = Some((self.m_a.clone(), self.m_b.clone(), self.m_c.clone()));

                if L::zero().done(&self.limits.0)
                    || L::zero().done(&self.limits.1)
                    || L::zero().done(&self.limits.2)
                {
                    res = None;
                }

                self.m.increment();
                self.m_a = (self.increment)(self.m_a.clone());
                self.m_b = (self.increment)(self.m_b.clone());
                self.m_c = (self.increment)(self.m_c.clone());

                self.resume_state = ResumeState::S1 {
                    i: L::zero(),
                    j: L::zero(),
                    i_a: self.seed.0.clone(),
                    i_b: self.seed.1.clone(),
                    i_c: self.seed.2.clone(),
                    j_a: self.seed.0.clone(),
                    j_b: self.seed.1.clone(),
                    j_c: self.seed.2.clone(),
                    yield_123: 0,
                };

                (res, false)
            }
            ResumeState::S1 {
                i,
                j,
                i_a,
                i_b,
                i_c,
                j_a,
                j_b,
                j_c,
                yield_123,
            } => {
                // 0: m_c, 1: m_b, 2: m_a
                match yield_123 {
                    0 => {
                        let mut res = Some((i_a.clone(), j_b.clone(), self.m_c.clone()));
                        if i.done(&self.limits.0)
                            || j.done(&self.limits.1)
                            || self.m.done(&self.limits.2)
                        {
                            res = None;
                        }

                        *yield_123 += 1;
                        (res, false)
                    }
                    1 => {
                        let mut res = Some((i_a.clone(), self.m_b.clone(), j_c.clone()));
                        if i.done(&self.limits.0)
                            || self.m.done(&self.limits.1)
                            || j.done(&self.limits.2)
                        {
                            res = None;
                        }

                        *yield_123 += 1;
                        (res, false)
                    }
                    _ => {
                        let mut res = Some((self.m_a.clone(), i_b.clone(), j_c.clone()));
                        if self.m.done(&self.limits.0)
                            || i.done(&self.limits.1)
                            || j.done(&self.limits.2)
                        {
                            res = None;
                        }

                        *yield_123 = 0;
                        // Check loop end condition
                        j.increment();
                        if !j.done(&self.m) {
                            // Continue next iteration
                            *j_a = (self.increment)(j_a.clone());
                            *j_b = (self.increment)(j_b.clone());
                            *j_c = (self.increment)(j_c.clone());
                        } else {
                            // Check loop end condition
                            i.increment();
                            if !i.done(&self.m) {
                                // Continue next iteration
                                *i_a = (self.increment)(i_a.clone());
                                *i_b = (self.increment)(i_b.clone());
                                *i_c = (self.increment)(i_c.clone());
                                *j = L::zero();
                                *j_a = self.seed.0.clone();
                                *j_b = self.seed.1.clone();
                                *j_c = self.seed.2.clone();
                            } else {
                                // Exit loop
                                self.resume_state = ResumeState::S2 {
                                    i: L::zero(),
                                    i_a: self.seed.0.clone(),
                                    i_b: self.seed.1.clone(),
                                    i_c: self.seed.2.clone(),
                                    yield_123: 0,
                                };
                            }
                        }

                        (res, false)
                    }
                }
            }
            ResumeState::S2 {
                i,
                i_a,
                i_b,
                i_c,
                yield_123,
            } => {
                // 0: not m_a, 1: not m_b, 2: not m_c
                match yield_123 {
                    0 => {
                        let mut res = Some((i_a.clone(), self.m_b.clone(), self.m_c.clone()));
                        if i.done(&self.limits.0)
                            || self.m.done(&self.limits.1)
                            || self.m.done(&self.limits.2)
                        {
                            res = None;
                        }

                        *yield_123 += 1;
                        (res, false)
                    }
                    1 => {
                        let mut res = Some((self.m_a.clone(), i_b.clone(), self.m_c.clone()));
                        if self.m.done(&self.limits.0)
                            || i.done(&self.limits.1)
                            || self.m.done(&self.limits.2)
                        {
                            res = None;
                        }
                        *yield_123 += 1;
                        (res, false)
                    }
                    _ => {
                        let mut res = Some((self.m_a.clone(), self.m_b.clone(), i_c.clone()));
                        if self.m.done(&self.limits.0)
                            || self.m.done(&self.limits.1)
                            || i.done(&self.limits.2)
                        {
                            res = None;
                        }
                        *yield_123 = 0;
                        // Check loop end condition
                        i.increment();
                        if !i.done(&self.m) {
                            // Continue next iteration
                            *i_a = (self.increment)(i_a.clone());
                            *i_b = (self.increment)(i_b.clone());
                            *i_c = (self.increment)(i_c.clone());
                        } else {
                            // Exit loop
                            self.resume_state = ResumeState::S3;
                        }

                        (res, false)
                    }
                }
            }
            ResumeState::S3 => {
                // Yield (m, m, m) and go to S1 state
                let mut res = Some((self.m_a.clone(), self.m_b.clone(), self.m_c.clone()));
                if self.m.done(&self.limits.0)
                    || self.m.done(&self.limits.1)
                    || self.m.done(&self.limits.2)
                {
                    res = None;
                }

                self.m.increment();

                if self.m.done(&self.limit) {
                    self.resume_state = ResumeState::Default;
                } else {
                    // This log is helpful to see progress, but it should be removed, the caller
                    // needs to be able to decide whether this function should log or not, and show
                    // a better message.
                    log::debug!("GenPairs3L: m={}", self.m);
                    self.m_a = (self.increment)(self.m_a.clone());
                    self.m_b = (self.increment)(self.m_b.clone());
                    self.m_c = (self.increment)(self.m_c.clone());
                    self.resume_state = ResumeState::S1 {
                        i: L::zero(),
                        j: L::zero(),
                        i_a: self.seed.0.clone(),
                        i_b: self.seed.1.clone(),
                        i_c: self.seed.2.clone(),
                        j_a: self.seed.0.clone(),
                        j_b: self.seed.1.clone(),
                        j_c: self.seed.2.clone(),
                        yield_123: 0,
                    };
                }

                (res, false)
            }
        }
    }
}

impl<T, F, L> Iterator for GenPairs3L<T, F, L>
where
    T: Clone + std::fmt::Debug,
    F: FnMut(T) -> T,
    L: Counter + Clone + Ord + std::fmt::Display + std::fmt::Debug,
{
    type Item = (T, T, T);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let (res, done) = self.maybe_next();

            if let Some(res) = res {
                return Some(res);
            }

            if done {
                return None;
            }
        }
    }
}

// Helper trait to make GenPairs3L generic over the counter type (u32 or u64)
pub trait Counter {
    fn zero() -> Self;
    fn increment(&mut self);
    fn done(&self, limit: &Self) -> bool;
}

impl Counter for u32 {
    fn zero() -> Self {
        0
    }

    fn increment(&mut self) {
        *self += 1;
    }

    fn done(&self, limit: &Self) -> bool {
        self >= limit
    }
}

impl Counter for u64 {
    fn zero() -> Self {
        0
    }

    fn increment(&mut self) {
        *self += 1;
    }

    fn done(&self, limit: &Self) -> bool {
        self >= limit
    }
}

fn max3<T: Ord>(a: T, b: T, c: T) -> T {
    std::cmp::max(std::cmp::max(a, b), c)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn naive_all_pairs3(seed: (u64, u64, u64), limits: (u64, u64, u64)) -> Vec<(u64, u64, u64)> {
        let mut res = vec![];

        for i in 0..limits.0 {
            for j in 0..limits.1 {
                for k in 0..limits.2 {
                    res.push((seed.0 + i, seed.1 + j, seed.2 + k));
                }
            }
        }

        res
    }

    #[test]
    fn integer_counter_same_as_naive_if_sorted() {
        let mut res = vec![];
        let seed = (0, 100, 10000);
        let increment = |x| x + 1;
        let limits = (3u64, 5, 2);
        let gen = GenPairs3L::new(seed, increment, limits);

        for x in gen {
            res.push(x);
        }

        res.sort();
        let res_naive = naive_all_pairs3(seed, limits);

        assert_eq!(res, res_naive);
    }

    #[test]
    fn integer_counter_step_0() {
        let mut res = vec![];
        let seed = (0, 100, 10000);
        let increment = |x| x + 1;
        let limits = (3u32, 5, 2);
        let gen = GenPairs3L::new_one_step(seed, increment, limits, 0);

        for x in gen {
            res.push(x);
        }

        assert_eq!(res, vec![(0, 100, 10000)]);
    }

    #[test]
    fn integer_counter_step_1() {
        let mut res = vec![];
        let seed = (0, 100, 10000);
        let increment = |x| x + 1;
        let limits = (3u32, 5, 2);
        let gen = GenPairs3L::new_one_step(seed, increment, limits, 1);

        for x in gen {
            res.push(x);
        }

        assert_eq!(
            res,
            vec![
                (0, 100, 10001),
                (0, 101, 10000),
                (1, 100, 10000),
                (0, 101, 10001),
                (1, 100, 10001),
                (1, 101, 10000),
                (1, 101, 10001)
            ]
        );
    }

    #[test]
    fn integer_counter_step_2() {
        let mut res = vec![];
        let seed = (0, 100, 10000);
        let increment = |x| x + 1;
        let limits = (3u32, 5, 2);
        let gen = GenPairs3L::new_one_step(seed, increment, limits, 2);

        for x in gen {
            res.push(x);
        }

        assert_eq!(
            res,
            vec![
                (0, 102, 10000),
                (2, 100, 10000),
                (0, 102, 10001),
                (2, 100, 10001),
                (1, 102, 10000),
                (2, 101, 10000),
                (1, 102, 10001),
                (2, 101, 10001),
                (2, 102, 10000),
                (2, 102, 10001)
            ]
        );
    }

    #[test]
    fn integer_counter_step_3() {
        let mut res = vec![];
        let seed = (0, 100, 10000);
        let increment = |x| x + 1;
        let limits = (3u32, 5, 2);
        let gen = GenPairs3L::new_one_step(seed, increment, limits, 3);

        for x in gen {
            res.push(x);
        }

        assert_eq!(
            res,
            vec![
                (0, 103, 10000),
                (0, 103, 10001),
                (1, 103, 10000),
                (1, 103, 10001),
                (2, 103, 10000),
                (2, 103, 10001)
            ]
        );
    }

    #[test]
    fn integer_counter_step_4() {
        let mut res = vec![];
        let seed = (0, 100, 10000);
        let increment = |x| x + 1;
        let limits = (3u32, 5, 2);
        let gen = GenPairs3L::new_one_step(seed, increment, limits, 4);

        for x in gen {
            res.push(x);
        }

        assert_eq!(
            res,
            vec![
                (0, 104, 10000),
                (0, 104, 10001),
                (1, 104, 10000),
                (1, 104, 10001),
                (2, 104, 10000),
                (2, 104, 10001)
            ]
        );
    }

    #[test]
    fn integer_counter_step_5() {
        let mut res = vec![];
        let seed = (0, 100, 10000);
        let increment = |x| x + 1;
        let limits = (3u32, 5, 2);
        let gen = GenPairs3L::new_one_step(seed, increment, limits, 5);

        for x in gen {
            res.push(x);
        }

        assert_eq!(res, vec![]);
    }
}
