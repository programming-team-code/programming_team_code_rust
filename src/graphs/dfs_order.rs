fn dfs_preorder(v: usize, adj: &[Vec<usize>], seen: &mut [bool], order: &mut Vec<usize>) {
    order.push(v);
    seen[v] = true;
    for &u in &adj[v] {
        if !seen[u] {
            dfs_preorder(u, adj, seen, order);
        }
    }
}

fn dfs_postorder(v: usize, adj: &[Vec<usize>], seen: &mut [bool], order: &mut Vec<usize>) {
    seen[v] = true;
    for &u in &adj[v] {
        if !seen[u] {
            dfs_postorder(u, adj, seen, order);
        }
    }
    order.push(v);
}

fn get_dfs_order<F>(adj: &[Vec<usize>], dfs_order: F) -> Vec<usize>
where
    F: Fn(usize, &[Vec<usize>], &mut [bool], &mut Vec<usize>),
{
    let n = adj.len();
    let mut seen = vec![false; n];
    let mut order = Vec::with_capacity(n);
    for s in 0..n {
        if !seen[s] {
            dfs_order(s, adj, &mut seen, &mut order);
        }
    }
    order
}

pub fn get_dfs_preorder(adj: &[Vec<usize>]) -> Vec<usize> {
    get_dfs_order(adj, dfs_preorder)
}

pub fn get_dfs_postorder(adj: &[Vec<usize>]) -> Vec<usize> {
    get_dfs_order(adj, dfs_postorder)
}
