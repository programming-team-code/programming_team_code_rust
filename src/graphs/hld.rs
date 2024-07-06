//! # Heavy Light Decomposition

use crate::graphs::dfs_order::{get_dfs_postorder, get_dfs_preorder};
use crate::monotonic::mono_st::mono_st;
use std::ops::Range;

/// # Example
/// ```
/// use programming_team_code_rust::graphs::hld::HLD;
/// use programming_team_code_rust::data_structures::fenwick::Fenwick;
///
/// let n = 4;
/// let mut adj = vec![vec![]; n];
/// for (u, v) in [(0,1), (0,2), (2,3)] {
///    adj[u].push(v);
///    adj[v].push(u);
/// }
///
/// let hld = HLD::new(&mut adj, false);
/// let mut fenwick = Fenwick::<usize>::new(n);
/// let init_vals = [10, 1, 100, 1000];
/// for i in 0..n {
///    fenwick.add(hld.tin[i], init_vals[i]);
/// }
///
/// assert_eq!(hld.lca(1, 3), 0);
/// assert_eq!(fenwick.sum(hld.sub_tree(2)), 1100);
/// let mut sum = 0;
/// hld.path(1, 2, |range, _| sum += fenwick.sum(range));
/// assert_eq!(sum, 111);
/// ```
pub struct HLD {
    /// parent
    pub p: Vec<Option<usize>>,
    /// time in
    pub tin: Vec<usize>,
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
        let mut tin = vec![0; n];
        let mut head = vec![0; n];
        let ord = get_dfs_preorder(adj);
        for (i, &u) in ord.iter().enumerate() {
            tin[u] = i;
            for &v in &adj[u] {
                head[v] = if v == adj[u][0] { head[u] } else { v };
            }
        }
        HLD {
            p,
            siz,
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

    /// If !vals_edges, then gets number of nodes on path from u to v
    /// If vals_edges, then gets number of edges on path from u to v
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn dist(&self, u: usize, v: usize) -> usize {
        let mut dst = 0;
        self.path(u, v, |range, _| dst += range.len());
        dst
    }

    /// Returns true iff v is in u's subtree
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn in_sub(&self, u: usize, v: usize) -> bool {
        u == v || self.sub_tree(u).contains(&self.tin[v])
    }

    /// Returns true iff w is on the path from u to v
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn on_path(&self, u: usize, v: usize, w: usize) -> bool {
        self.in_sub(self.lca(u, v), w) && (self.in_sub(w, u) || self.in_sub(w, v))
    }

    /// Returns a node k edges up from u, or None
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn kth_par(&self, mut u: usize, mut k: usize) -> Option<usize> {
        loop {
            let len_path = self.tin[u] - self.tin[self.head[u]];
            if k <= len_path {
                return Some(self.ord[self.tin[u] - k]);
            }
            match self.p[self.head[u]] {
                Some(v) => u = v,
                None => return None,
            }
            k -= len_path + 1;
        }
    }

    /// Returns the node \[u, p\[u\], .., lca(u, v), .., p\[v\], v\]\[k\], or None
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn kth_on_path(&self, u: usize, v: usize, k: usize) -> Option<usize> {
        let mut dst_side = [0; 2];
        self.path(u, v, |range, u_anc| dst_side[u_anc as usize] += range.len());
        if k < dst_side[1] {
            return self.kth_par(u, k);
        }
        let dst = dst_side[0] + dst_side[1] - !self.vals_edges as usize;
        if k <= dst {
            self.kth_par(v, dst - k)
        } else {
            None
        }
    }

    /// # Auxiliary Tree
    ///
    /// - see <https://github.com/kth-competitive-programming/kactl/blob/main/content/graph/CompressTree.h>
    ///
    /// # Example
    /// ```
    /// use programming_team_code_rust::graphs::hld::HLD;
    ///
    /// let n = 5;
    /// let mut adj = vec![vec![]; n];
    /// for (u, v) in [(0,1), (1,2), (2,3), (2,4)] {
    ///    adj[u].push(v);
    ///    adj[v].push(u);
    /// }
    ///
    /// let hld = HLD::new(&mut adj, false);
    ///
    /// let (par, to_node) = hld.aux_tree(vec![0, 3, 4]);
    /// // 0, 1, .., par.len()-1 is a topological/dfs order of aux tree
    /// assert_eq!(par, [usize::MAX, 0, 1, 1]);
    /// assert_eq!(to_node, [0, 2, 3, 4]);
    /// ```
    ///
    /// # Complexity
    /// - k = nodes.len()
    /// - Time: O((k log k) + (k log n))
    /// - Space: O(k)
    pub fn aux_tree(&self, mut nodes: Vec<usize>) -> (Vec<usize>, Vec<usize>) {
        nodes.sort_by(|&a, &b| self.tin[a].cmp(&self.tin[b]));
        let siz = nodes.len();
        for i in 1..siz {
            nodes.push(self.lca(nodes[i - 1], nodes[i]));
        }
        nodes.sort_by(|&a, &b| self.tin[a].cmp(&self.tin[b]));
        nodes.dedup();
        (mono_st(&nodes, |&x, &y| self.in_sub(x, y)), nodes)
    }
}
