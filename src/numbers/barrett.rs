//! # Barrett Division and Remainder

/// See <https://github.com/kth-competitive-programming/kactl/blob/main/content/various/FastMod.h>
///
/// # Example
/// ```
/// use programming_team_code_rust::numbers::barrett::Barrett;
/// let barr = Barrett::new(6_u32);
/// assert_eq!(barr.div(20_u64), (3, 2));
/// ```
///
/// # Panics
/// ```panic
/// use programming_team_code_rust::numbers::barrett::Barrett;
/// let barr = Barrett::new(0_u32);
/// ```
pub struct Barrett {
    /// denominator
    pub denom: u32,
    im: u64,
}

impl Barrett {
    /// Creates a new Barrett struct
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn new(denom: u32) -> Self {
        assert_ne!(denom, 0);
        Self {
            denom,
            im: u64::MAX / (denom as u64),
        }
    }

    /// Gets quotient and remainder of numer / denom
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn div(&self, numer: u64) -> (u64, u32) {
        let quot = ((self.im as u128 * numer as u128) >> 64) as u64;
        let rem = (numer - quot * self.denom as u64) as u32;
        if rem >= self.denom {
            (quot + 1, rem - self.denom)
        } else {
            (quot, rem)
        }
    }

    /// Gets x * y % denom
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn mult(&self, x: u32, y: u32) -> u32 {
        self.div(x as u64 * y as u64).1
    }
}
