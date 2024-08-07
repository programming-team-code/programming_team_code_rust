//! # Lowest Common Ancestor

use crate::data_structures::rmq::RMQ;
use crate::graphs::dfs_order::get_dfs_preorder;

type Pair = (usize, usize);

/// # Example
/// ```
/// use programming_team_code_rust::graphs::lca::LCA;
///
/// let n = 4;
/// let mut adj = vec![vec![]; n];
/// for (u, v) in [(0,1), (0,2), (2,3)] {
///    adj[u].push(v);
///    adj[v].push(u);
/// }
///
/// let lca = LCA::new(&adj);
/// assert_eq!(lca.lca(1, 3), 0);
/// assert_eq!(lca.lca(2, 3), 2);
/// assert_eq!(lca.dist(1, 3), 3);
/// assert_eq!(lca.in_sub(2, 3), true);
/// assert_eq!(lca.next_on_path(0, 3), 2);
/// assert!(std::panic::catch_unwind(|| lca.next_on_path(1, 1)).is_err());
/// ```
pub struct LCA {
    tin: Vec<usize>,
    p: Vec<Option<usize>>,
    d: Vec<usize>,
    siz: Vec<usize>,
    rmq: RMQ<Pair, fn(&Pair, &Pair) -> Pair>,
}

impl LCA {
    /// Create a new LCA struct
    ///
    /// `adj` can be undirected tree or a directed tree (rooted at node 0)
    /// `adj` can also be an undirected forest
    ///
    /// # Complexity (n = adj.len())
    /// - Time: O(n log n)
    /// - Space: O(n log n)
    pub fn new(adj: &[Vec<usize>]) -> Self {
        let n = adj.len();
        let mut tin = vec![0; n];
        let mut p = vec![None; n];
        let mut d = vec![0; n];
        let order = get_dfs_preorder(adj);
        for (i, &v) in order.iter().enumerate() {
            tin[v] = i;
            for &u in &adj[v] {
                if Some(u) != p[v] {
                    (p[u], d[u]) = (Some(v), d[v] + 1);
                }
            }
        }
        let mut siz = vec![1; n];
        for &v in order.iter().rev() {
            if let Some(par) = p[v] {
                siz[par] += siz[v];
            }
        }
        LCA {
            p,
            rmq: RMQ::new(
                &order.iter().map(|&v| (d[v], v)).collect::<Vec<_>>(),
                |&x, &y| if x.0 < y.0 { x } else { y },
            ),
            d,
            tin,
            siz,
        }
    }

    /// Gets the lowest common ancestor of u and v
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn lca(&self, u: usize, v: usize) -> usize {
        if u == v {
            return u;
        }
        let (mut le, mut ri) = (self.tin[u], self.tin[v]);
        if le > ri {
            (le, ri) = (ri, le);
        }
        self.p[self.rmq.query(le + 1..ri + 1).1].unwrap()
    }

    /// Gets number of edges on path from u to v
    ///
    /// # Complexity
    /// - Time: O(1)
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

    /// Gets \[u, p\[u\], .., lca(u,v), .., p\[v\], v\]\[1\]
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn next_on_path(&self, u: usize, v: usize) -> usize {
        assert!(u != v);
        if self.in_sub(u, v) {
            self.rmq.query(self.tin[u] + 1..self.tin[v] + 1).1
        } else {
            self.p[u].unwrap()
        }
    }
}
