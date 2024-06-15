//! # Bridge Edges

/// # Guarantees
/// - 0 <= two_edge_ccid\[u\] < num_2_edge_ccs
///
/// # Example
/// ```
/// use programming_team_code_rust::graphs::bridges::get_bridges;
///
/// let edge_list = [(0,1), (0,1), (1,2), (2,2)];
/// let mut adj = vec![vec![]; 3];
/// for (i, &(u, v)) in edge_list.iter().enumerate() {
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
    #[allow(clippy::too_many_arguments)]
    fn dfs(
        u: usize,
        p_id: Option<usize>,
        adj: &[Vec<(usize, usize)>],
        timer: &mut usize,
        tin: &mut [usize],
        num_2_edge_ccs: &mut usize,
        is_bridge: &mut [bool],
        two_edge_ccid: &mut [usize],
        st: &mut Vec<usize>,
    ) -> usize {
        tin[u] = *timer;
        let (mut low, st_sz) = (*timer, st.len());
        *timer += 1;
        st.push(u);
        for &(v, e_id) in &adj[u] {
            if Some(e_id) == p_id {
                continue;
            }
            if tin[v] == 0 {
                low = low.min(dfs(
                    v,
                    Some(e_id),
                    adj,
                    timer,
                    tin,
                    num_2_edge_ccs,
                    is_bridge,
                    two_edge_ccid,
                    st,
                ));
            }
            low = low.min(tin[v]);
        }
        if tin[u] == low {
            if let Some(p) = p_id {
                is_bridge[p] = true;
            }
            for i in st_sz..st.len() {
                two_edge_ccid[st[i]] = *num_2_edge_ccs;
            }
            st.truncate(st_sz);
            *num_2_edge_ccs += 1;
        }
        low
    }
    let (n, mut timer, mut num_2_edge_ccs, mut is_bridge) = (adj.len(), 1, 0, vec![false; m]);
    let (mut tin, mut two_edge_ccid, mut st) = (vec![0; n], vec![0; n], Vec::with_capacity(n));
    for i in 0..n {
        if tin[i] == 0 {
            dfs(
                i,
                None,
                adj,
                &mut timer,
                &mut tin,
                &mut num_2_edge_ccs,
                &mut is_bridge,
                &mut two_edge_ccid,
                &mut st,
            );
        }
    }
    (num_2_edge_ccs, is_bridge, two_edge_ccid)
}
