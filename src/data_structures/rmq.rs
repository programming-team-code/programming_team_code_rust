//! # Range Minimum Query

/// # Example
/// ```
/// use programming_team_code_rust::data_structures::rmq::RMQ;
///
/// let a = [1, 3, 2, 4, 5];
/// let rmq1 = RMQ::new(&a, std::cmp::min);
/// assert_eq!(rmq1.query(0..5), 1);
/// assert_eq!(rmq1.query(1..4), 2);
///
/// let outside_var = 5;
/// let rmq2 = RMQ::new(&a, |x, y| if x + outside_var < y + outside_var { x } else { y });
/// assert_eq!(rmq2.query(0..5), 1);
/// assert_eq!(rmq2.query(1..4), 2);
/// ```
pub struct RMQ<T, F> {
    t: Vec<Vec<T>>,
    op: F,
}

impl<T: Clone, F: Fn(&T, &T) -> T> RMQ<T, F> {
    /// Create a new RMQ instance
    ///
    /// # Complexity (n = a.len())
    /// - Time: O(n log n)
    /// - Space: O(n log n)
    pub fn new(a: &[T], op: F) -> Self {
        let mut t = vec![a.to_vec(); 1];
        let mut i = 0;
        while (2 << i) <= a.len() {
            t.push(
                (0..t[i].len() - (1 << i))
                    .map(|j| op(&t[i][j], &t[i][j + (1 << i)]))
                    .collect(),
            );
            i += 1;
        }
        Self { t, op }
    }

    /// Query the range [range.start, range.end)
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        let lg = range.len().ilog2() as usize;
        (self.op)(&self.t[lg][range.start], &self.t[lg][range.end - (1 << lg)])
    }
}
