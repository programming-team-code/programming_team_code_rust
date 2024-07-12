//! # Deque with op

pub struct RMQ<T, F> {
    t: Vec<Vec<T>>,
    op: F,
}

impl<T: Copy, F: Fn(T, T) -> T> RMQ<T, F> {
    /// Create a new RMQ instance
    ///
    /// # Complexity (n = a.len())
    /// - Time: O(n log n)
    /// - Space: O(n log n)
    pub fn new(a: &[T], op: F) -> Self {
        let mut t = vec![a.to_owned(); 1];
        let mut i = 0;
        while (2 << i) <= a.len() {
            t.push(
                (0..t[i].len() - (1 << i))
                    .map(|j| op(t[i][j], t[i][j + (1 << i)]))
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
        (self.op)(self.t[lg][range.start], self.t[lg][range.end - (1 << lg)])
    }
}
