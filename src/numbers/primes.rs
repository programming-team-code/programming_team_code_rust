pub struct Primes {
    min_fact: Vec<usize>,
}
impl Primes {
    pub fn new(n: usize) -> Primes {
        let mut min_fact = (0..n).collect::<Vec<_>>();
        let mut i = 2;
        while i * i < n {
            if min_fact[i] == i {
                let mut j = i * i;
                while j < n {
                    min_fact[j] = min_fact[j].min(i);
                    j += i;
                }
            }
            i += 1;
        }
        Primes { min_fact }
    }
    pub fn is_prime(&self, x: usize) -> bool {
        x >= 2 && self.min_fact[x] == x
    }
    // returns the prime factors of `x` in sorted order
    pub fn factorize(&self, mut x: usize) -> Vec<usize> {
        let mut facts = vec![];
        while x > 1 {
            let p = self.min_fact[x];
            facts.push(p);
            x /= p;
        }
        facts
    }
}
