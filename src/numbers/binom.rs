/// # Example
/// ```
/// use programming_team_code_rust::numbers::binom::Binom;
///
/// let mut binom = Binom::new(1_000_000_007);
/// assert_eq!(binom.comb(5, 2), 10);
/// assert_eq!(binom.comb(10, 3), 120);
/// ```
pub struct Binom {
    inv: Vec<u64>,
    fact: Vec<u64>,
    inv_fact: Vec<u64>,
    modulo: u64,
}

impl Binom {
    /// Create a new instance of Binom.
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn new(modulo: u64) -> Binom {
        Binom {
            inv: vec![1; 2],
            fact: vec![1; 2],
            inv_fact: vec![1; 2],
            modulo,
        }
    }

    /// Calculate C(n, k)
    ///
    /// # Complexity
    /// - Time: O(1) amortized O(n)
    /// - Space: O(n)
    pub fn comb(&mut self, n: usize, k: usize) -> u64 {
        assert!((n as u64) < self.modulo);
        if n < k {
            return 0;
        }
        while self.inv.len() <= n {
            let i = self.inv.len();
            self.inv.push(
                self.modulo
                    - (self.modulo / i as u64) * self.inv[self.modulo as usize % i] % self.modulo,
            );
            self.fact.push(self.fact[i - 1] * i as u64 % self.modulo);
            self.inv_fact
                .push(self.inv_fact[i - 1] * self.inv[i] % self.modulo);
        }
        self.fact[n] * self.inv_fact[k] % self.modulo * self.inv_fact[n - k] % self.modulo
    }
}
