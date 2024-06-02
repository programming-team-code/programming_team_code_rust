//! # Range Minimum Query

/// # Example
/// ```
/// use programming_team_code_rust::data_structures::rmq::RMQ;
///
/// let ary = [0, 1, 2, 3, 4, 5, 6];
/// let a = [1, 3, 2, 4, 5];
/// let rmq = RMQ::new(&a, move |x, y| if ary[x] < ary[y] { x } else { y });
/// let rmq2 = RMQ::new(&a, std::cmp::min);
/// assert_eq!(rmq.query(0..5), 1);
/// assert_eq!(rmq.query(1..4), 2);
/// ```
pub struct RMQ<T> {
    t: Vec<Vec<T>>,
    op: Box<dyn Fn(T, T) -> T>,
}

impl<T: Copy> RMQ<T> {
    /// Create a new RMQ instance
    ///
    /// # Complexity (n = a.len())
    /// - Time: O(n log n)
    /// - Space: O(n log n)
    pub fn new(a: &[T], op: impl Fn(T, T) -> T + 'static) -> Self {
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
        Self {
            t,
            op: Box::new(op),
        }
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
