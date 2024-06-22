//! # Random Number Generator

pub struct Random {
    state: u64,
}

impl Random {
    pub fn new(seed: u64) -> Self {
        assert_ne!(seed, 0);
        Self { state: seed }
    }

    pub fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    pub fn next_in_range(&mut self, range: std::ops::Range<i64>) -> i64 {
        range.start + (self.next() % (range.end - range.start) as u64) as i64
    }
}
