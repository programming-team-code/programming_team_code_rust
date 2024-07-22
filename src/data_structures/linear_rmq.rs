//! # Linear Range Minimum Query

/// # Example
/// ```
/// use programming_team_code_rust::data_structures::linear_rmq::LinearRMQ;
///
/// let a = [1, 0, 2, 0, 3];
/// let rmq = LinearRMQ::new(&a, |&x, &y| x.lt(&y)); // lt -> right-most min
///                                                  // le -> left-most min
///                                                  // gt -> right-most max
///                                                  // ge -> left-most max
/// assert_eq!(rmq.query_idx(0..5), 3);
/// assert_eq!(*rmq.query(1..5), 0);
/// ```
pub struct LinearRMQ<T, F> {
    a: Vec<T>,
    cmp: F,
    head: Vec<usize>,
    t: Vec<(usize, usize)>,
}

impl<T: Clone, F: Fn(&T, &T) -> bool> LinearRMQ<T, F> {
    /// Create a new LinearRMQ instance
    ///
    /// # Complexity (n = a.len())
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn new(a: &[T], cmp: F) -> Self {
        let mut head = vec![0; a.len() + 1];
        let mut t = vec![(0, 0); a.len()];
        let mut st = vec![usize::MAX];
        for i in 0..=a.len() {
            let mut prev = usize::MAX;
            while *st.last().unwrap() != usize::MAX
                && (i == a.len() || !cmp(&a[*st.last().unwrap()], &a[i]))
            {
                if prev != usize::MAX {
                    head[prev] = *st.last().unwrap();
                }
                let pw2 = 1 << (st[st.len() - 2].wrapping_add(1) ^ i).ilog2();
                prev = i & 0_usize.wrapping_sub(pw2);
                t[*st.last().unwrap()].0 = prev;
                st.pop();
                t[(*st.last().unwrap()).wrapping_add(1)].1 |= pw2;
            }
            if prev != usize::MAX {
                head[prev] = i;
            }
            st.push(i);
        }
        for i in 1..a.len() {
            t[i].1 =
                (t[i].1 | t[i - 1].1) & 0_usize.wrapping_sub(t[i].0 & 0_usize.wrapping_sub(t[i].0));
        }
        Self {
            a: a.to_vec(),
            cmp,
            head,
            t,
        }
    }

    /// Gets the index of min/max of range
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn query_idx(&self, range: std::ops::Range<usize>) -> usize {
        assert!(!range.is_empty());
        let (mut le, mut ri) = (range.start, range.end - 1);
        let j = self.t[le].1
            & self.t[ri].1
            & 0_usize.wrapping_sub(1 << ((self.t[le].0 ^ self.t[ri].0) | 1).ilog2());
        {
            let mut k = self.t[le].1 ^ j;
            if k != 0 {
                k = 1 << k.ilog2();
                le = self.head[self.t[le].0 & 0_usize.wrapping_sub(k) | k];
            }
        }
        {
            let mut k = self.t[ri].1 ^ j;
            if k != 0 {
                k = 1 << k.ilog2();
                ri = self.head[self.t[ri].0 & 0_usize.wrapping_sub(k) | k];
            }
        }
        if (self.cmp)(&self.a[le], &self.a[ri]) {
            le
        } else {
            ri
        }
    }

    /// Gets the min/max of range
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn query(&self, range: std::ops::Range<usize>) -> &T {
        &self.a[self.query_idx(range)]
    }
}
