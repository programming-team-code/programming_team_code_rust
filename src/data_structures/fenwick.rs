/// Fenwick Tree
///
/// Solves point add and range sum query
///
/// # Example
/// ```
/// use crate::programming_team_code_rust::data_structures::fenwick::Fenwick;
///
/// let mut fenwick = Fenwick::new(10);
/// fenwick.add(0, 1);
/// fenwick.add(1, 2);
/// fenwick.add(2, 3);
/// assert_eq!(fenwick.sum(0..3), 6);
/// assert_eq!(fenwick.sum(0..2), 3);
/// assert_eq!(fenwick.sum(1..3), 5);
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
}
