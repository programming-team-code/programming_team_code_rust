/// # Example
/// ```
/// use programming_team_code_rust::numbers::primes::Primes;
///
/// let primes = Primes::new(100);
/// assert_eq!(primes.is_prime(2), true);
/// assert_eq!(primes.is_prime(4), false);
/// assert_eq!(primes.factorize(12), vec![2, 2, 3]);
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
}
