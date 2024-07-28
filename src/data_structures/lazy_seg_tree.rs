//! # Lazy Segment Tree

use std::ops::Range;

/// see https://codeforces.com/blog/entry/112755
///
/// returns the split point of the range which
/// makes the segment tree a complete binary tree
fn split(tr: &Range<usize>) -> usize {
    let pw2 = 1 << tr.len().ilog2();
    (tr.start + pw2).min(tr.end - pw2 / 2)
}

fn op(vl: u64, vr: u64) -> u64 {
    vl + vr
}

/// # Example
/// ```
/// use programming_team_code_rust::data_structures::lazy_seg_tree::LazySegTree;
///
/// let mut seg_tree = LazySegTree::new(10);
/// seg_tree.update(0..5, 1);
/// seg_tree.update(5..10, 2);
/// assert_eq!(seg_tree.query(0..10), 15);
/// ```
pub struct LazySegTree {
    n: usize,
    tree: Vec<u64>,
    lazy: Vec<u64>,
}

impl LazySegTree {
    /// Creates a new segment tree with n elements
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn new(n: usize) -> Self {
        LazySegTree {
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
        let mut tree = vec![0; 2 * n];
        let pw2 = n.next_power_of_two();
        for i in 0..n {
            tree[(i + pw2) % n + n] = a[i];
        }
        for i in (1..n).rev() {
            tree[i] = op(tree[2 * i], tree[2 * i + 1]);
        }
        LazySegTree {
            n,
            tree,
            lazy: vec![0; n],
        }
    }

    fn apply(&mut self, delta: u64, tr: &Range<usize>, v: usize) {
        self.tree[v] += delta * tr.len() as u64;
        if v < self.n {
            self.lazy[v] += delta;
        }
    }

    fn push(&mut self, tr: &Range<usize>, v: usize) {
        if self.lazy[v] > 0 {
            let tm = split(tr);
            self.apply(self.lazy[v], &(tr.start..tm), 2 * v);
            self.apply(self.lazy[v], &(tm..tr.end), 2 * v + 1);
            self.lazy[v] = 0;
        }
    }

    /// Updates the range [qr.start, qr.end) by adding delta to each element
    /// in the range
    ///
    /// # Complexity
    /// - Time: O(log(n))
    /// - Space: O(log(n)) due to the program stack
    pub fn update(&mut self, qr: Range<usize>, delta: u64) {
        self.update_impl(&qr, delta, &(0..self.n), 1);
    }

    fn update_impl(&mut self, qr: &Range<usize>, delta: u64, tr: &Range<usize>, v: usize) {
        if qr.end <= tr.start || tr.end <= qr.start {
            return;
        }
        if qr.start <= tr.start && tr.end <= qr.end {
            return self.apply(delta, tr, v);
        }
        self.push(tr, v);
        let tm = split(tr);
        self.update_impl(qr, delta, &(tr.start..tm), 2 * v);
        self.update_impl(qr, delta, &(tm..tr.end), 2 * v + 1);
        self.tree[v] = op(self.tree[2 * v], self.tree[2 * v + 1]);
    }

    /// Queries the range [qr.start, qr.end)
    ///
    /// # Complexity
    /// - Time: O(log(n))
    /// - Space: O(log(n)) due to the program stack
    pub fn query(&mut self, qr: Range<usize>) -> u64 {
        self.query_impl(&qr, &(0..self.n), 1)
    }

    fn query_impl(&mut self, qr: &Range<usize>, tr: &Range<usize>, v: usize) -> u64 {
        if qr.end <= tr.start || tr.end <= qr.start {
            return 0;
        }
        if qr.start <= tr.start && tr.end <= qr.end {
            return self.tree[v];
        }
        self.push(tr, v);
        let tm = split(tr);
        op(
            self.query_impl(qr, &(tr.start..tm), 2 * v),
            self.query_impl(qr, &(tm..tr.end), 2 * v + 1),
        )
    }
}
