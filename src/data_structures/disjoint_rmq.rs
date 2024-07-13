//! # Disjoint Range Minimum Query

/// # Example
/// ```
/// use programming_team_code_rust::data_structures::disjoint_rmq::DisjointRMQ;
///
/// let a = [1, 3, 1, 4, 5];
/// // (min, number of mins)
/// let rmq = DisjointRMQ::new(&a.iter().map(|&x| (x, 1)).collect::<Vec<_>>(), |&x, &y| {
///    if x.0 == y.0 {
///       (x.0, x.1 + y.1)
///    } else {
///       std::cmp::min(x, y)
///    }
/// });
/// assert_eq!(rmq.query(0..5), (1, 2));
/// assert_eq!(rmq.query(2..4), (1, 1));
/// assert!(std::panic::catch_unwind(|| rmq.query(0..0)).is_err());
/// ```
pub struct DisjointRMQ<T, F> {
    t: Vec<Vec<T>>,
    op: F,
}

impl<T: Clone, F: Fn(&T, &T) -> T> DisjointRMQ<T, F> {
    /// Create a new Disjoint RMQ instance
    ///
    /// # Complexity (n = a.len())
    /// - Time: O(n log n)
    /// - Space: O(n log n)
    pub fn new(a: &[T], op: F) -> Self {
        let mut t = vec![];
        let mut len = 1;
        while len <= a.len() {
            let mut le = 0;
            let mut row = a.to_vec();
            while le < a.len() {
                let mi = (le + len).min(a.len());
                let ri = (le + 2 * len).min(a.len());
                for i in (le..mi - 1).rev() {
                    row[i] = op(&row[i], &row[i + 1]);
                }
                for i in mi + 1..ri {
                    row[i] = op(&row[i - 1], &row[i]);
                }
                le += 2 * len;
            }
            t.push(row);
            len *= 2;
        }
        Self { t, op }
    }

    /// Query the range [range.start, range.end)
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        if range.len() == 1 {
            self.t[0][range.start].clone()
        } else {
            let lg = (range.start ^ (range.end - 1)).ilog2() as usize;
            (self.op)(&self.t[lg][range.start], &self.t[lg][range.end - 1])
        }
    }
}
