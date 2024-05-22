// n choose k for a prime modulo
pub struct Binom {
    inv: Vec<u64>,
    fact: Vec<u64>,
    inv_fact: Vec<u64>,
    modulo: u64,
}
impl Binom {
    pub fn new(modulo: u64) -> Binom {
        Binom {
            inv: vec![1; 2],
            fact: vec![1; 2],
            inv_fact: vec![1; 2],
            modulo,
        }
    }
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
