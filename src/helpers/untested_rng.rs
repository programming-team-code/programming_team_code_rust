//! # Random Number Generator

/// see <https://en.wikipedia.org/wiki/Xorshift>
///
/// # Example
/// ```
/// use std::time::{SystemTime, UNIX_EPOCH};
/// use programming_team_code_rust::helpers::untested_rng::UntestedRNG;
///
/// // for submitting
/// let mut rng = UntestedRNG::new(
///     SystemTime::now()
///         .duration_since(UNIX_EPOCH)
///         .unwrap()
///         .subsec_nanos() as u64,
/// );
/// // fixed seed for debugging
/// let mut rng = UntestedRNG::new(12345);
/// assert_eq!(rng.get(), 13289605635609);
/// assert_eq!(rng.get_in_range(-3..4), 0);
/// ```
struct UntestedRNG {
    state: u64,
}

impl UntestedRNG {
    /// Creates a new RNG
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    fn new(seed: u64) -> Self {
        assert_ne!(seed, 0);
        Self { state: seed }
    }

    /// Gets a new random number
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    fn get(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Gets a new random number in range
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    fn get_in_range(&mut self, range: std::ops::Range<i64>) -> i64 {
        range.start + (self.get() % (range.end - range.start) as u64) as i64
    }
}
