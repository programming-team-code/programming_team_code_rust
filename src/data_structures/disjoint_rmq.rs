//! # Disjoint Range Minimum Query

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
                let mi = a.len().min(le + len);
                let ri = a.len().min(le + 2 * len);
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
