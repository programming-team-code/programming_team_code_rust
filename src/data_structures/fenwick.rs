//! # Fenwick Tree (Binary Indexed Tree)

/// # Example
/// ```
/// use programming_team_code_rust::data_structures::fenwick::Fenwick;
///
/// let mut fenwick = Fenwick::new(5);
/// fenwick.add(1, 3);
/// fenwick.add(2, 2);
/// fenwick.add(3, 1);
/// assert_eq!(fenwick.sum(1..3), 5);
/// assert_eq!(fenwick.kth(5), 2);
/// assert!(std::panic::catch_unwind(|| fenwick.kth(0)).is_err());
/// ```
pub struct Fenwick<T> {
    ary: Vec<T>,
}

impl<T: Clone + Default + std::ops::AddAssign<T>> Fenwick<T> {
    /// Creates a new Fenwick Tree with size n
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn new(n: usize) -> Self {
        Fenwick {
            ary: vec![T::default(); n],
        }
    }

    /// Creates a Fenwick Tree on a given array
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn build_on_array(a: &[T]) -> Self {
        let mut ary = a.to_vec();
        for i in 0..a.len() {
            let j = i | (i + 1);
            if j < a.len() {
                let tmp = ary[i].clone();
                ary[j] += tmp;
            }
        }
        Fenwick { ary }
    }

    fn accum(&self, mut idx: usize) -> T {
        let mut sum = T::default();
        while idx > 0 {
            sum += self.ary[idx - 1].clone();
            idx &= idx - 1;
        }
        sum
    }

    /// Increments the value at `idx` by `val`
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn add(&mut self, mut idx: usize, val: T) {
        while idx < self.ary.len() {
            self.ary[idx] += val.clone();
            idx |= idx + 1;
        }
    }

    /// Query the sum of the range [range.start, range.end)
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn sum(&self, range: std::ops::Range<usize>) -> T
    where
        T: std::ops::Sub<Output = T>,
    {
        self.accum(range.end) - self.accum(range.start)
    }

    /// Gets maximum pos such that sum of [0, pos) < sum
    ///
    /// Requires fenwick.sum(i..i + 1) >= 0 and sum > 0
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn kth(&self, mut sum: T) -> usize
    where
        T: std::ops::SubAssign<T> + std::cmp::PartialOrd,
    {
        assert!(sum > T::default());
        let mut pos = 0;
        let mut pw = self.ary.len().next_power_of_two();
        while pw > 0 {
            if pos + pw <= self.ary.len() && self.ary[pos + pw - 1] < sum {
                pos += pw;
                sum -= self.ary[pos - 1].clone();
            }
            pw /= 2;
        }
        pos
    }
}
