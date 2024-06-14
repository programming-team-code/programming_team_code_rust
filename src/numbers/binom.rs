//! # Binomial Coefficient

use crate::numbers::barrett::Barrett;

/// # Example
/// ```
/// use programming_team_code_rust::numbers::barrett::Barrett;
/// use programming_team_code_rust::numbers::binom::Binom;
///
/// let barr = Barrett::new(1_000_000_007);
/// let mut binom = Binom::new(barr);
/// assert_eq!(binom.comb(5, 2), 10);
/// assert_eq!(binom.comb(10, 3), 120);
/// ```
pub struct Binom {
    inv: Vec<u32>,
    fact: Vec<u32>,
    inv_fact: Vec<u32>,
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
    pub fn comb(&mut self, n: usize, k: usize) -> u32 {
        assert!((n as u32) < self.barr.denom);
        if n < k {
            return 0;
        }
        while self.inv.len() <= n {
            let i = self.inv.len();
            self.inv.push(
                self.barr.denom
                    - self.barr.mult(
                        self.barr.denom / i as u32,
                        self.inv[self.barr.denom as usize % i],
                    ),
            );
            self.fact.push(self.barr.mult(self.fact[i - 1], i as u32));
            self.inv_fact
                .push(self.barr.mult(self.inv_fact[i - 1], self.inv[i]));
        }
        self.barr.mult(
            self.barr.mult(self.fact[n], self.inv_fact[k]),
            self.inv_fact[n - k],
        )
    }
}
