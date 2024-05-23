pub fn dfs_order(adj: &[Vec<usize>]) -> Vec<usize> {
    let n = adj.len();
    let mut order = Vec::with_capacity(n);
    fn dfs(u: usize, p: usize, adj: &[Vec<usize>], order: &mut Vec<usize>) {
        order.push(u);
        for &v in &adj[u] {
            if v != p {
                dfs(v, u, adj, order);
            }
        }
    }
    dfs(0, 0, adj, &mut order);
    order
}
