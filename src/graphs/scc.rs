//! # Strongly Connected Components

use crate::graphs::dfs_order::get_dfs_postorder;

/// # Guarantees
/// - 0..num_sccs is a topological order of the SCCs
/// - 0 <= scc_id\[v\] < num_sccs
/// - for each edge u -> v: scc_id\[u\] <= scc_id\[v\]
///
/// # Example
/// ```
/// use programming_team_code_rust::graphs::scc::get_sccs;
///
/// let n = 4;
/// let mut adj = vec![vec![]; n];
/// for (u, v) in [(0,1), (1,2), (2,0), (1,3), (3,3)] {
///    adj[u].push(v);
/// }
///
/// let (num_sccs, scc_id) = get_sccs(&adj);
/// assert_eq!(num_sccs, 2);
/// assert_eq!(scc_id, [0, 0, 0, 1]);
/// ```
///
/// # Complexity
/// - Time: O(V + E)
/// - Space: O(V)
pub fn get_sccs(adj: &[Vec<usize>]) -> (usize, Vec<usize>) {
    let n = adj.len();
    let rv_adj = {
        let mut rv_adj = vec![vec![]; n];
        for (v, vs) in adj.iter().enumerate() {
            for &u in vs {
                rv_adj[u].push(v);
            }
        }
        rv_adj
    };
    fn dfs(v: usize, adj: &[Vec<usize>], seen: &mut [bool], scc_id: &mut [usize], num_sccs: usize) {
        scc_id[v] = num_sccs;
        seen[v] = true;
        for &u in &adj[v] {
            if !seen[u] {
                dfs(u, adj, seen, scc_id, num_sccs);
            }
        }
    }
    let order = get_dfs_postorder(adj);
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
