//! # Bridge Edges

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
    struct Env {
        timer: usize,
        tin: Vec<usize>,
        num_2_edge_ccs: usize,
        is_bridge: Vec<bool>,
        two_edge_ccid: Vec<usize>,
        st: Vec<usize>,
    }
    let mut e = Env {
        timer: 1,
        tin: vec![0; n],
        num_2_edge_ccs: 0,
        is_bridge: vec![false; m],
        two_edge_ccid: vec![0; n],
        st: Vec::with_capacity(n),
    };
    fn dfs(e: &mut Env, u: usize, p_id: Option<usize>, adj: &[Vec<(usize, usize)>]) -> usize {
        e.tin[u] = e.timer;
        let (mut low, st_sz) = (e.timer, e.st.len());
        e.timer += 1;
        e.st.push(u);
        for &(v, e_id) in &adj[u] {
            if Some(e_id) == p_id {
                continue;
            }
            if e.tin[v] == 0 {
                low = low.min(dfs(e, v, Some(e_id), adj));
            }
            low = low.min(e.tin[v]);
        }
        if e.tin[u] == low {
            if let Some(p) = p_id {
                e.is_bridge[p] = true;
            }
            for &id in e.st.iter().skip(st_sz) {
                e.two_edge_ccid[id] = e.num_2_edge_ccs;
            }
            e.st.truncate(st_sz);
            e.num_2_edge_ccs += 1;
        }
        low
    }
    for i in 0..n {
        if e.tin[i] == 0 {
            dfs(&mut e, i, None, adj);
        }
    }
    (e.num_2_edge_ccs, e.is_bridge, e.two_edge_ccid)
}
