//! # Bridge Edges

use crate::helpers::recursive_fnmut::recursive_fnmut;

/// # Guarantees
/// - 0 <= two_edge_ccid\[u\] < num_2_edge_ccs
///
/// # Example
/// ```
/// use programming_team_code_rust::graphs::bridges::get_bridges;
///
/// let mut adj = vec![vec![]; 3];
/// for (i, &(u, v)) in [(0,1), (0,1), (1,2), (2,2)].iter().enumerate() {
///    adj[u].push((v, i));
///    adj[v].push((u, i));
/// }
///
/// let (num_2_edge_ccs, is_bridge, two_edge_ccid) = get_bridges(&adj, 4);
///
/// assert_eq!(num_2_edge_ccs, 2);
/// assert_eq!(is_bridge, vec![false, false, true, false]);
/// assert_eq!(two_edge_ccid, vec![1, 1, 0]);
/// ```
///
/// # Complexity
/// - Time: O(V + E)
/// - Space: O(V)
pub fn get_bridges(adj: &[Vec<(usize, usize)>], m: usize) -> (usize, Vec<bool>, Vec<usize>) {
    let n = adj.len();
    let mut timer = 1;
    let mut tin = vec![0; n];
    let mut st = Vec::with_capacity(n);

    let mut num_2_edge_ccs = 0;
    let mut is_bridge = vec![false; m];
    let mut two_edge_ccid = vec![0; n];

    for i in 0..n {
        if tin[i] == 0 {
            let mut dfs = recursive_fnmut!(|dfs, u: usize, p_id: Option<usize>| -> usize {
                tin[u] = timer;
                let (mut low, st_sz) = (timer, st.len());
                timer += 1;
                st.push(u);
                for &(v, e_id) in &adj[u] {
                    if Some(e_id) == p_id {
                        continue;
                    }
                    if tin[v] == 0 {
                        low = low.min(dfs(v, Some(e_id)));
                    }
                    low = low.min(tin[v]);
                }
                if tin[u] == low {
                    if let Some(p) = p_id {
                        is_bridge[p] = true;
                    }
                    for &id in st.iter().skip(st_sz) {
                        two_edge_ccid[id] = num_2_edge_ccs;
                    }
                    st.truncate(st_sz);
                    num_2_edge_ccs += 1;
                }
                low
            });
            dfs(i, None);
        }
    }
    (num_2_edge_ccs, is_bridge, two_edge_ccid)
}
