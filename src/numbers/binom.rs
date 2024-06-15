//! # Binomial Coefficient

const MOD: u64 = 1_000_000_007;

/// # Example
/// ```
/// use programming_team_code_rust::numbers::binom::Binom;
///
/// let mut binom = Binom::new();
/// assert_eq!(binom.comb(5, 2), 10);
/// assert_eq!(binom.comb(10, 3), 120);
/// ```
pub struct Binom {
    inv: Vec<u64>,
    fact: Vec<u64>,
    inv_fact: Vec<u64>,
}

impl Binom {
    /// Create a new instance of Binom.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn new() -> Binom {
        Binom {
            inv: vec![1; 2],
            fact: vec![1; 2],
            inv_fact: vec![1; 2],
        }
    }

    /// Calculate C(n, k)
    ///
    /// # Complexity
    /// - Time: O(1) amortized O(n)
    /// - Space: O(n)
    pub fn comb(&mut self, n: usize, k: usize) -> u64 {
        assert!((n as u64) < MOD);
        if n < k {
            return 0;
        }
        while self.inv.len() <= n {
            let i = self.inv.len();
            self.inv
                .push(MOD - (MOD / i as u64) * self.inv[MOD as usize % i] % MOD);
            self.fact.push(self.fact[i - 1] * i as u64 % MOD);
            self.inv_fact.push(self.inv_fact[i - 1] * self.inv[i] % MOD);
        }
        self.fact[n] * self.inv_fact[k] % MOD * self.inv_fact[n - k] % MOD
    }
}
