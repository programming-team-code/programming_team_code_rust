//! # Segment Tree

/// # Example
/// ```
/// use programming_team_code_rust::data_structures::seg_tree::SegTree;
///
/// let mut st = SegTree::<usize>::new(3, |&x, &y| x + y, 0);
/// st.set(1, 2);
/// st.set(2, 3);
/// assert_eq!(st.query(0..3), 5);
/// ```
pub struct SegTree<T> {
    n: usize,
    /// associative operation
    pub op: fn(&T, &T) -> T,
    /// identity element
    pub unit: T,
    tree: Vec<T>,
}

impl<T: Clone> SegTree<T> {
    /// Creates a new segment tree with n elements
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn new(n: usize, op: fn(&T, &T) -> T, unit: T) -> Self {
        Self {
            n,
            op,
            tree: vec![unit.clone(); 2 * n],
            unit,
        }
    }

    /// Creates a segment tree on a given array
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn build_on_array(a: &[T], op: fn(&T, &T) -> T, unit: T) -> Self {
        let n = a.len();
        let mut tree = vec![unit.clone(); n];
        tree.extend(a.to_vec());
        for i in (1..n).rev() {
            tree[i] = op(&tree[2 * i], &tree[2 * i + 1]);
        }
        Self { n, op, unit, tree }
    }

    /// Sets the value at `idx` to `val`
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn set(&mut self, mut i: usize, val: T) {
        i += self.n;
        self.tree[i] = val;
        while i >= 2 {
            i /= 2;
            self.tree[i] = (self.op)(&self.tree[2 * i], &self.tree[2 * i + 1]);
        }
    }

    /// Query the op of the range [range.start, range.end)
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        let (mut vl, mut vr, mut le, mut ri) = (
            self.unit.clone(),
            self.unit.clone(),
            range.start + self.n,
            range.end + self.n,
        );
        while le < ri {
            if le % 2 == 1 {
                vl = (self.op)(&vl, &self.tree[le]);
                le += 1;
            }
            if ri % 2 == 1 {
                vr = (self.op)(&self.tree[ri - 1], &vr);
            }
            le /= 2;
            ri /= 2;
        }
        (self.op)(&vl, &vr)
    }
}
