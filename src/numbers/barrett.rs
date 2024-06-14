//! # Barrett Mod

pub struct Barrett {
    denom: u32,
    im: u64,
}

impl Barrett {
    pub fn new(denom: u32) -> Self {
        assert_ne!(denom, 0);
        Self {
            denom,
            im: u64::MAX / (denom as u64),
        }
    }

    pub fn div(&self, numer: u64) -> (u64, u32) {
        let quot = ((self.im as u128 * numer as u128) >> 64) as u64;
        let rem = (numer - quot * self.denom as u64) as u32;
        if rem >= self.denom {
            (quot + 1, rem - self.denom)
        } else {
            (quot, rem)
        }
    }
}
