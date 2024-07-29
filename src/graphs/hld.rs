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
        for &v in get_dfs_postorder(adj).iter() {
            adj[v].retain(|&u| siz[u] > 0);
            siz[v] = 1;
            for i in 0..adj[v].len() {
                let u = adj[v][i];
                p[u] = Some(v);
                siz[v] += siz[u];
                if siz[u] > siz[adj[v][0]] {
                    adj[v].swap(0, i);
                }
            }
        }
        let mut tin = vec![0; n];
        let mut head = vec![0; n];
        let mut d = vec![0; n];
        let ord = get_dfs_preorder(adj);
        for (i, &v) in ord.iter().enumerate() {
            tin[v] = i;
            for &u in &adj[v] {
                d[u] = 1 + d[v];
                head[u] = if u == adj[v][0] { head[v] } else { u };
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
                (u, v) = (v, u);
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

    /// Gets range representing the subtree of v
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn sub_tree(&self, v: usize) -> Range<usize> {
        self.tin[v] + self.vals_edges as usize..self.tin[v] + self.siz[v]
    }

    /// Gets the lowest common ancestor of u and v
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        loop {
            if self.tin[u] > self.tin[v] {
                (u, v) = (v, u);
            }
            if self.head[u] == self.head[v] {
                return u;
            }
            v = self.p[self.head[v]].unwrap();
        }
    }

    /// Gets number of edges on path from u to v
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn dist(&self, u: usize, v: usize) -> usize {
        self.d[u] + self.d[v] - 2 * self.d[self.lca(u, v)]
    }

    /// Returns true iff v is in u's subtree
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn in_sub(&self, u: usize, v: usize) -> bool {
        (self.tin[u]..self.tin[u] + self.siz[u]).contains(&self.tin[v])
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
    pub fn kth_par(&self, mut v: usize, mut k: usize) -> Option<usize> {
        loop {
            let len_path = self.tin[v] - self.tin[self.head[v]];
            if k <= len_path {
                return Some(self.ord[self.tin[v] - k]);
            }
            match self.p[self.head[v]] {
                Some(u) => v = u,
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
        let lca_d = self.d[self.lca(u, v)];
        let u_lca = self.d[u] - lca_d;
        let v_lca = self.d[v] - lca_d;
        if k <= u_lca {
            self.kth_par(u, k)
        } else if k <= u_lca + v_lca {
            self.kth_par(v, u_lca + v_lca - k)
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
        nodes.sort_by(|&u, &v| self.tin[u].cmp(&self.tin[v]));
        let siz = nodes.len();
        for i in 1..siz {
            nodes.push(self.lca(nodes[i - 1], nodes[i]));
        }
        nodes.sort_by(|&u, &v| self.tin[u].cmp(&self.tin[v]));
        nodes.dedup();
        (mono_st(&nodes, |&u, &v| self.in_sub(u, v)), nodes)
    }
}
