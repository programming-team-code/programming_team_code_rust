//! # Segment Tree

pub struct SegTree<T> {
    n: usize,
    op: fn(T, T) -> T,
    unit: T,
    tree: Vec<T>,
}

impl<T: Clone> SegTree<T> {
    /// Creates a new segment tree with n elements
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn new(n: usize, op: fn(T, T) -> T, unit: T) -> Self {
        Self {
            n,
            op,
            unit: unit.clone(),
            tree: vec![unit.clone(); 2 * n],
        }
    }

    /// Creates a segment tree on a given array
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn build_on_array(a: &[T], op: fn(T, T) -> T, unit: T) -> Self {
        let n = a.len();
        let mut tree = vec![unit.clone(); n];
        tree.extend(a.to_vec());
        for i in (1..n).rev() {
            tree[i] = op(tree[2 * i].clone(), tree[2 * i + 1].clone());
        }
        Self {
            n,
            op,
            unit: unit.clone(),
            tree,
        }
    }

    pub fn set(&mut self, pos: usize, val: T) {
        let mut i = pos + self.n;
        self.tree[i] = val;
        while i >= 2 {
            i /= 2;
            self.tree[i] = (self.op)(self.tree[2 * i].clone(), self.tree[2 * i + 1].clone());
        }
    }

    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        let (mut vl, mut vr, mut le, mut ri) = (
            self.unit.clone(),
            self.unit.clone(),
            range.start + self.n,
            range.end + self.n,
        );
        while le < ri {
            if le % 2 == 1 {
                vl = (self.op)(vl, self.tree[le].clone());
                le += 1;
            }
            if ri % 2 == 1 {
                ri -= 1;
                vr = (self.op)(self.tree[ri].clone(), vr);
            }
            le /= 2;
            ri /= 2;
        }
        (self.op)(vl, vr)
    }
}
