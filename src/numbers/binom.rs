//! # Binomial Coefficient

use crate::numbers::barrett::Barrett;

/// # Example
/// ```
/// use programming_team_code_rust::numbers::binom::Binom;
///
/// let barr = Barrett::new(1_000_000_007);
/// let mut binom = Binom::new(barr);
/// assert_eq!(binom.comb(5, 2), 10);
/// assert_eq!(binom.comb(10, 3), 120);
/// ```
pub struct Binom {
    inv: Vec<u64>,
    fact: Vec<u64>,
    inv_fact: Vec<u64>,
    barr: Barrett,
}

impl Binom {
    /// Create a new instance of Binom.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn new(barr: Barrett) -> Binom {
        Binom {
            inv: vec![1; 2],
            fact: vec![1; 2],
            inv_fact: vec![1; 2],
            barr,
        }
    }

    /// Calculate C(n, k)
    ///
    /// # Complexity
    /// - Time: O(1) amortized O(n)
    /// - Space: O(n)
    pub fn comb(&mut self, n: usize, k: usize) -> u64 {
        assert!((n as u64) < self.barr.denom);
        if n < k {
            return 0;
        }
        while self.inv.len() <= n {
            let i = self.inv.len();
            self.inv.push(
                self.barr.denom
                    - self
                        .barr
                        .div((self.barr.denom / i as u64) * self.inv[self.barr.denom as usize % i])
                        .1,
            );
            self.fact.push(self.barr.div(self.fact[i - 1] * i as u64).1);
            self.inv_fact
                .push(self.barr.div(self.inv_fact[i - 1] * self.inv[i]).1);
        }
        self.barr
            .div(self.barr.div(self.fact[n] * self.inv_fact[k]).1 * self.inv_fact[n - k])
            .1
    }
}
