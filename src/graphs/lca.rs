//! # Lowest Common Ancestor

use crate::data_structures::rmq::RMQ;
use crate::graphs::dfs_order::get_dfs_preorder;

type OpType = fn((usize, usize), (usize, usize)) -> (usize, usize);

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
/// ```
pub struct LCA {
    tin: Vec<usize>,
    p: Vec<Option<usize>>,
    rmq: RMQ<(usize, usize), OpType>,
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
        for (i, &u) in order.iter().enumerate() {
            tin[u] = i;
            for &v in &adj[u] {
                if Some(v) != p[u] {
                    (p[v], d[v]) = (Some(u), d[u] + 1);
                }
            }
        }
        LCA {
            tin,
            p,
            rmq: RMQ::new(
                &order
                    .iter()
                    .map(|&u| (d[u], u))
                    .collect::<Vec<(usize, usize)>>(),
                std::cmp::min,
            ),
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
            std::mem::swap(&mut le, &mut ri);
        }
        self.p[self.rmq.query(le + 1..ri + 1).1].unwrap()
    }
}
