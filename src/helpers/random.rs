//! # Random Number Generator

struct Random {
    state: u64,
}

impl Random {
    fn new(seed: u64) -> Self {
        assert_ne!(seed, 0);
        Self { state: seed }
    }

    fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    fn next_in_range(&mut self, range: std::ops::Range<u64>) -> u64 {
        range.start + self.next() % range.len()
    }
}
