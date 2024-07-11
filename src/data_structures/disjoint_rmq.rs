//! # Disjoint Range Minimum Query

pub struct DisjointRMQ<T, F> {
    t: Vec<Vec<T>>,
    op: F,
}

impl<T: Copy, F: Fn(T, T) -> T> DisjointRMQ<T, F> {
    /// Create a new Disjoint RMQ instance
    ///
    /// # Complexity (n = a.len())
    /// - Time: O(n log n)
    /// - Space: O(n log n)
    pub fn new(a: &[T], op: F) -> Self {}

    /// Query the range [range.start, range.end)
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn query(&self, range: std::ops::Range<usize>) -> T {}
}
