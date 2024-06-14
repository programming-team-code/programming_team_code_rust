//! # Barrett Mod

pub struct Barrett {
    m: u32,
    im: u64,
}

impl Barrett {
    pub fn new(m: u32) -> Self {
        assert_ne!(m, 0);
        Self {
            m,
            im: u64::MAX / (m as u64),
        }
    }

    pub fn div(&self, a: u64) -> (u64, u32) {
        let quot = ((self.im as u128 * a as u128) >> 64) as u64;
        let rem = (a - quot * self.m as u64) as u32;
        if rem >= self.m {
            (quot + 1, rem - self.m)
        } else {
            (quot, rem)
        }
    }
}
