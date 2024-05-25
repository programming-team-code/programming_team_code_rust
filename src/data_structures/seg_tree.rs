//! # Segment Tree

/// see https://codeforces.com/blog/entry/112755
///
/// returns the split point of the range which
/// makes the segment tree a complete binary tree
fn split(tl: usize, tr: usize) -> usize {
    let pw2 = 1 << (tr - tl).ilog2();
    (tl + pw2).min(tr - pw2 / 2)
}

fn op(vl: u64, vr: u64) -> u64 {
    vl + vr
}

/// # Example
/// ```
/// use programming_team_code_rust::data_structures::seg_tree::SegTree;
///
/// let mut seg_tree = SegTree::new(10);
/// seg_tree.update(0, 5, 1);
/// seg_tree.update(5, 10, 2);
/// assert_eq!(seg_tree.query(0, 10), 15);
/// ```
pub struct SegTree {
    n: usize,
    tree: Vec<u64>,
    lazy: Vec<u64>,
}

impl SegTree {
    /// Creates a new segment tree with n elements
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn new(n: usize) -> Self {
        SegTree {
            n,
            tree: vec![0; 2 * n],
            lazy: vec![0; n],
        }
    }

    /// Constructs a segment tree on a given array
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn build_on_array(a: &[u64]) -> Self {
        let n = a.len();
        let mut pw2 = 1;
        while pw2 < n {
            pw2 *= 2;
        }
        let mut tree = vec![0; 2 * n];
        for i in 0..n {
            tree[(i + pw2) % n + n] = a[i];
        }
        for i in (1..n).rev() {
            tree[i] = op(tree[2 * i], tree[2 * i + 1]);
        }
        SegTree {
            n,
            tree,
            lazy: vec![0; n],
        }
    }

    fn apply(&mut self, delta: u64, tl: usize, tr: usize, v: usize) {
        self.tree[v] += delta * (tr - tl) as u64;
        if v < self.n {
            self.lazy[v] += delta;
        }
    }

    fn push(&mut self, tl: usize, tm: usize, tr: usize, v: usize) {
        if self.lazy[v] > 0 {
            self.apply(self.lazy[v], tl, tm, 2 * v);
            self.apply(self.lazy[v], tm, tr, 2 * v + 1);
            self.lazy[v] = 0;
        }
    }

    /// Updates the range [le, ri) by adding delta to each element
    /// in the range
    ///
    /// # Complexity
    /// - Time: O(log(n))
    /// - Space: O(log(n)) due to the program stack
    pub fn update(&mut self, le: usize, ri: usize, delta: u64) {
        self.update_impl(le, ri, delta, 0, self.n, 1);
    }

    fn update_impl(&mut self, le: usize, ri: usize, delta: u64, tl: usize, tr: usize, v: usize) {
        if ri <= tl || tr <= le {
            return;
        }
        if le <= tl && tr <= ri {
            return self.apply(delta, tl, tr, v);
        }
        let tm = split(tl, tr);
        self.push(tl, tm, tr, v);
        self.update_impl(le, ri, delta, tl, tm, 2 * v);
        self.update_impl(le, ri, delta, tm, tr, 2 * v + 1);
        self.tree[v] = op(self.tree[2 * v], self.tree[2 * v + 1]);
    }

    /// Queries the range [le, ri)
    ///
    /// # Complexity
    /// - Time: O(log(n))
    /// - Space: O(log(n)) due to the program stack
    pub fn query(&mut self, le: usize, ri: usize) -> u64 {
        self.query_impl(le, ri, 0, self.n, 1)
    }

    fn query_impl(&mut self, le: usize, ri: usize, tl: usize, tr: usize, v: usize) -> u64 {
        if ri <= tl || tr <= le {
            return 0;
        }
        if le <= tl && tr <= ri {
            return self.tree[v];
        }
        let tm = split(tl, tr);
        self.push(tl, tm, tr, v);
        op(
            self.query_impl(le, ri, tl, tm, 2 * v),
            self.query_impl(le, ri, tm, tr, 2 * v + 1),
        )
    }
}
