fn dfs_impl<F, G>(u: usize, adj: &[Vec<usize>], seen: &mut [bool], pre: &mut F, post: &mut G)
where
    F: FnMut(usize),
    G: FnMut(usize),
{
    pre(u);
    seen[u] = true;
    for &v in &adj[u] {
        if !seen[v] {
            dfs_impl(v, adj, seen, pre, post);
        }
    }
    post(u);
}

pub fn dfs(adj: &[Vec<usize>], mut pre: impl FnMut(usize), mut post: impl FnMut(usize)) {
    let n = adj.len();
    let mut seen = vec![false; n];
    for s in 0..n {
        if !seen[s] {
            dfs_impl(s, adj, &mut seen, &mut pre, &mut post);
        }
    }
}
