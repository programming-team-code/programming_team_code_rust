//! # Primes

/// # Example
/// ```
/// use programming_team_code_rust::numbers::primes::Primes;
///
/// let primes = Primes::new(100);
/// assert_eq!(primes.is_prime(2), true);
/// assert_eq!(primes.is_prime(4), false);
/// assert_eq!(primes.factorize(12), [2, 2, 3]);
/// assert_eq!(primes.divisorize(12), [1, 2, 4, 3, 6, 12]);
/// assert!(std::panic::catch_unwind(|| primes.divisorize(0)).is_err());
/// ```
pub struct Primes {
    min_fact: Vec<usize>,
}

impl Primes {
    /// Constructs a new Primes instance with the given size
    ///
    /// # Complexity
    /// - Time: O(n log log n)
    /// - Space: O(n)
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

    /// Returns a boolean indicating whether the given number is prime
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn is_prime(&self, x: usize) -> bool {
        x >= 2 && self.min_fact[x] == x
    }

    /// Returns a vector of prime factors of the given number
    /// The factors are sorted in ascending order
    ///
    /// # Complexity
    /// - Time: O(log x)
    /// - Space: O(log x)
    pub fn factorize(&self, mut x: usize) -> Vec<usize> {
        let mut facts = vec![];
        while x > 1 {
            let p = self.min_fact[x];
            facts.push(p);
            x /= p;
        }
        facts
    }

    /// Returns a vector of all divisors of the given number
    ///
    /// # Complexity
    /// - Time: O(x ^ (1/3))
    /// - Space: O(x ^ (1/3))
    pub fn divisorize(&self, mut x: usize) -> Vec<usize> {
        assert_ne!(x, 0);
        let mut divs = vec![1];
        while x > 1 {
            let mut new_divs = divs.clone();
            let prime = self.min_fact[x];
            while self.min_fact[x] == prime {
                for i in new_divs.len() - divs.len()..new_divs.len() {
                    new_divs.push(new_divs[i] * prime);
                }
                x /= prime;
            }
            divs = new_divs;
        }
        divs
    }
}
