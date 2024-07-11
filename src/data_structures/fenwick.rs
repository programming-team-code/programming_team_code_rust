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
/// ```
pub struct Fenwick<T> {
    ary: Vec<T>,
}

impl<T: Clone + Default + std::ops::AddAssign<T> + std::cmp::PartialOrd> Fenwick<T> {
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

    /// Gets minimum pos such that self.sum(0..pos) >= sum; or n+1
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn kth(&self, mut sum: T) -> usize
    where
        T: std::ops::SubAssign<T>,
    {
        if sum <= T::default() {
            return 0;
        }
        let mut pos = 0;
        for pw in (0..=self.ary.len().ilog2()).map(|i| 1 << i).rev() {
            if pos + pw <= self.ary.len() && self.ary[pos + pw - 1] < sum {
                pos += pw;
                sum -= self.ary[pos - 1].clone();
            }
        }
        pos + 1
    }
}
