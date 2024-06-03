//! # Heavy Light Decomposition

use crate::graphs::dfs_order::{get_dfs_postorder, get_dfs_preorder};
use std::ops::Range;

/// # Example
/// ```
/// use programming_team_code_rust::graphs::hld::HLD;
/// use programming_team_code_rust::data_structures::fenwick::Fenwick;
///
/// let mut adj = vec![
///    vec![1, 2],
///    vec![0, 3, 4],
///    vec![0, 5],
///    vec![1],
///    vec![1],
///    vec![2],
/// ];
/// let fenwick = Fenwick::<usize>::build_on_array(&[3, 2, 4, 5, 1, 2]);
///
/// let hld = HLD::new(&mut adj, false);
/// assert_eq!(hld.lca(3, 4), 1);
/// assert_eq!(fenwick.sum(hld.sub_tree(0)), 17);
/// let mut sum = 0;
/// hld.path(3, 4, |range, _| sum += fenwick.sum(range));
/// assert_eq!(sum, 11);
/// ```
pub struct HLD {
    /// parent
    pub p: Vec<Option<usize>>,
    /// time in
    pub tin: Vec<usize>,
    /// depth
    pub d: Vec<usize>,
    ord: Vec<usize>,
    siz: Vec<usize>,
    head: Vec<usize>,
    vals_edges: bool,
}

impl HLD {
    /// Create a new HLD struct
    ///
    /// `adj` can be undirected tree or a directed tree (rooted at node 0)
    /// `adj` can also be an undirected forest
    ///
    /// # Complexity (n = adj.len())
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn new(adj: &mut [Vec<usize>], vals_edges: bool) -> Self {
        let n = adj.len();
        let mut p = vec![None; n];
        let mut siz = vec![0; n];
        for &u in get_dfs_postorder(adj).iter() {
            adj[u].retain(|&v| siz[v] > 0);
            siz[u] = 1;
            for i in 0..adj[u].len() {
                let v = adj[u][i];
                p[v] = Some(u);
                siz[u] += siz[v];
                if siz[v] > siz[adj[u][0]] {
                    adj[u].swap(0, i);
                }
            }
        }
        let mut d = vec![0; n];
        let mut tin = vec![0; n];
        let mut head = vec![0; n];
        let ord = get_dfs_preorder(adj);
        for (i, &u) in ord.iter().enumerate() {
            tin[u] = i;
            for &v in &adj[u] {
                d[v] = 1 + d[u];
                head[v] = if v == adj[u][0] { head[u] } else { v };
            }
        }
        HLD {
            p,
            siz,
            d,
            ord,
            tin,
            head,
            vals_edges,
        }
    }

    /// Calls callback `f` on ranges representing the path from u to v
    ///
    /// # Complexity
    /// - Time: O(log n) calls to `f`
    /// - Space: O(1)
    pub fn path(&self, mut u: usize, mut v: usize, mut f: impl FnMut(Range<usize>, bool)) {
        let mut u_anc = false;
        loop {
            if self.tin[u] > self.tin[v] {
                std::mem::swap(&mut u, &mut v);
                u_anc = !u_anc;
            }
            if self.head[u] == self.head[v] {
                break;
            }
            f(self.tin[self.head[v]]..self.tin[v] + 1, u_anc);
            v = self.p[self.head[v]].unwrap();
        }
        f(
            self.tin[u] + self.vals_edges as usize..self.tin[v] + 1,
            u_anc,
        );
    }

    /// Gets range representing the subtree of u
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn sub_tree(&self, u: usize) -> Range<usize> {
        self.tin[u] + self.vals_edges as usize..self.tin[u] + self.siz[u]
    }

    /// Gets the lowest common ancestor of u and v
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        loop {
            if self.tin[u] > self.tin[v] {
                std::mem::swap(&mut u, &mut v);
            }
            if self.head[u] == self.head[v] {
                return u;
            }
            v = self.p[self.head[v]].unwrap();
        }
    }

    pub fn dist(&self, u: usize, v: usize) -> usize {
        self.d[u] + self.d[v] - 2 * self.d[self.lca(u, v)]
    }

    pub fn in_sub(&self, u: usize, v: usize) -> bool {
        u == v || self.sub_tree(u).contains(&self.tin[v])
    }

    pub fn on_path(&self, u: usize, v: usize, w: usize) -> bool {
        self.dist(u, w) + self.dist(w, v) == self.dist(u, v)
    }

    pub fn kth_par(&self, mut u: usize, mut k: usize) -> Option<usize> {
        loop {
            let len_path = self.tin[u] - self.tin[self.head[u]];
            if k <= len_path {
                return Some(self.ord[self.tin[u] - k]);
            }
            k -= len_path + 1;
            match self.p[self.head[u]] {
                Some(v) => u = v,
                None => return None,
            }
        }
    }

    pub fn kth_on_path(&self, u: usize, v: usize, k: usize) -> Option<usize> {
        let d_lca = self.d[self.lca(u, v)];
        let (u_dist, v_dist) = (self.d[u] - d_lca, self.d[v] - d_lca);
        if k <= u_dist {
            return self.kth_par(u, k);
        }
        if k <= u_dist + v_dist {
            return self.kth_par(v, u_dist + v_dist - k);
        }
        None
    }
}
