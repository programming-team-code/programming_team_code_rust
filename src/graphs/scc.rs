//! # Strongly Connected Components

use crate::graphs::dfs_order::get_dfs_postorder;

fn dfs(u: usize, adj: &[Vec<usize>], seen: &mut [bool], scc_id: &mut [usize], num_sccs: usize) {
    scc_id[u] = num_sccs;
    seen[u] = true;
    for &v in &adj[u] {
        if !seen[v] {
            dfs(v, adj, seen, scc_id, num_sccs);
        }
    }
}

/// # Guarantees
/// - 0..num_sccs is a topological order of the SCCs
/// - 0 <= scc_id[u] < num_sccs
/// - for each edge u -> v: scc_id[u] <= scc_id[v]
///
/// # Example
/// ```
/// use programming_team_code_rust::graphs::scc::get_sccs;
///
/// let adj = vec![
///    vec![1],
///    vec![2],
///    vec![0],
///    vec![3],
///    vec![5],
///    vec![4],
///    vec![5],
/// ];
///
/// let (num_sccs, scc_id) = get_sccs(&adj);
/// assert_eq!(num_sccs, 4);
/// assert_eq!(scc_id, vec![3, 3, 3, 2, 1, 1, 0]);
/// ```
///
/// # Complexity
/// - Time: O(V + E)
/// - Space: O(V)
pub fn get_sccs(adj: &[Vec<usize>]) -> (usize, Vec<usize>) {
    let n = adj.len();
    let rv_adj = {
        let mut rv_adj = vec![vec![]; n];
        for (u, vs) in adj.iter().enumerate() {
            for &v in vs {
                rv_adj[v].push(u);
            }
        }
        rv_adj
    };
    let order = get_dfs_postorder(&adj);
    let mut num_sccs = 0;
    let mut scc_id = vec![0; n];
    let mut seen = vec![false; n];
    for &s in order.iter().rev() {
        if !seen[s] {
            dfs(s, &rv_adj, &mut seen, &mut scc_id, num_sccs);
            num_sccs += 1;
        }
    }
    (num_sccs, scc_id)
}
