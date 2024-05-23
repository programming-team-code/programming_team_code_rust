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
