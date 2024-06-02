//! # Segment Tree

pub struct SegTree<T> {
    n: usize,
    unit: T,
    tree: Vec<T>,
    op: fn(T, T) -> T,
}

impl<T: Clone> SegTree<T> {
    /// Creates a new segment tree with n elements
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn new(n: usize, unit: T, op: fn(T, T) -> T) -> Self {
        Self {
            n,
            unit: unit.clone(),
            tree: vec![unit.clone(); 2 * n],
            op,
        }
    }

    pub fn update(&mut self, pos: usize, val: T) {
        let mut i = pos + self.n;
        self.tree[i] = val;
        while i >= 2 {
            i /= 2;
            self.tree[i] = (self.op)(self.tree[2 * i].clone(), self.tree[2 * i + 1].clone());
        }
    }

    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        let (mut ra, mut rb, mut le, mut ri) = (
            self.unit.clone(),
            self.unit.clone(),
            range.start + self.n,
            range.end + self.n,
        );
        while le < ri {
            if ri % 2 == 1 {
                ra = (self.op)(ra, self.tree[ri].clone());
                ri += 1;
            }
            if le % 2 == 1 {
                rb = (self.op)(self.tree[le - 1].clone(), rb);
            }
            le /= 2;
            ri /= 2;
        }
        (self.op)(ra, rb)
    }
}
