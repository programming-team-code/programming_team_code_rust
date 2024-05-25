//! # Lowest Common Ancestor

use crate::data_structures::rmq::RMQ;

pub fn dfs_order(adj: &[Vec<usize>]) -> Vec<usize> {
    let n = adj.len();
    fn dfs(u: usize, adj: &[Vec<usize>], seen: &mut [bool], order: &mut Vec<usize>) {
        order.push(u);
        seen[u] = true;
        for &v in &adj[u] {
            if !seen[v] {
                dfs(v, adj, seen, order);
            }
        }
    }
    let mut seen = vec![false; n];
    let mut order = Vec::with_capacity(n);
    for s in 0..n {
        if !seen[s] {
            dfs(s, adj, &mut seen, &mut order);
        }
    }
    order
}

/// # Example
/// ```
/// use programming_team_code_rust::graphs::lca::LCA;
///
/// let adj = vec![
///    vec![1, 2],
///    vec![0, 3, 4],
///    vec![0, 5],
///    vec![1],
///    vec![1],
///    vec![2],
/// ];
///
/// let lca = LCA::new(&adj);
/// assert_eq!(lca.lca(0, 1), 0);
/// assert_eq!(lca.lca(0, 2), 0);
/// assert_eq!(lca.lca(0, 5), 0);
/// assert_eq!(lca.lca(3, 4), 1);
/// ```
pub struct LCA {
    tin: Vec<usize>,
    p: Vec<usize>,
    rmq: RMQ<(usize, usize)>,
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
        let mut p = vec![0; n];
        let mut d = vec![0; n];
        let order = dfs_order(adj);
        for (i, &u) in order.iter().enumerate() {
            tin[u] = i;
            for &v in &adj[u] {
                if v != p[u] {
                    p[v] = u;
                    d[v] = d[u] + 1;
                }
            }
        }
        let d_with_order: Vec<(usize, usize)> = order.iter().map(|&u| (d[u], u)).collect();
        let rmq = RMQ::new(&d_with_order, std::cmp::min);
        LCA { tin, p, rmq }
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
        self.p[self.rmq.query(le + 1..ri + 1).1]
    }
}
