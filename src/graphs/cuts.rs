//! # Cut nodes

/// # Guarantees
/// - 0 <= bcc_id\[u\] < num_bccs
///
/// # Example
/// ```
/// use programming_team_code_rust::graphs::cuts::get_cuts;
///
/// let mut adj = vec![vec![]; 3];
/// for (i, &(u, v)) in [(0,1), (0,1), (1,2), (0,1)].iter().enumerate() {
///    adj[u].push((v, i));
///    adj[v].push((u, i));
/// }
///
/// let (num_bccs, is_cut, bcc_id) = get_cuts(&adj, 4);
///
/// assert_eq!(num_bccs, 2);
/// assert_eq!(is_cut, vec![false, true, false]);
/// assert_eq!(bcc_id, vec![1, 1, 0, 1]);
///
/// // self edges not allowed
/// adj = vec![vec![]; 1];
/// for (i, &(u, v)) in [(0,0)].iter().enumerate() {
///    adj[u].push((v, i));
///    adj[v].push((u, i));
/// }
/// assert!(std::panic::catch_unwind(|| get_cuts(&adj, 1)).is_err());
/// ```
///
/// # Complexity
/// - Time: O(V + E)
/// - Space: O(V)
pub fn get_cuts(adj: &[Vec<(usize, usize)>], m: usize) -> (usize, Vec<bool>, Vec<usize>) {
    let n = adj.len();
    struct Env {
        timer: usize,
        tin: Vec<usize>,
        num_bccs: usize,
        is_cut: Vec<bool>,
        bcc_id: Vec<usize>,
        st: Vec<usize>,
    }
    let mut e = Env {
        timer: 1,
        tin: vec![0; n],
        num_bccs: 0,
        is_cut: vec![false; n],
        bcc_id: vec![0; m],
        st: Vec::with_capacity(m),
    };
    fn dfs(e: &mut Env, v: usize, p_id: Option<usize>, adj: &[Vec<(usize, usize)>]) -> usize {
        e.tin[v] = e.timer;
        let (mut low, mut deg) = (e.timer, 0);
        e.timer += 1;
        for &(u, e_id) in &adj[v] {
            assert_ne!(v, u);
            if Some(e_id) == p_id {
                continue;
            }
            if e.tin[u] == 0 {
                let st_sz = e.st.len();
                e.st.push(e_id);
                let low_ch = dfs(e, u, Some(e_id), adj);
                if low_ch >= e.tin[v] {
                    e.is_cut[v] = true;
                    for &id in e.st.iter().skip(st_sz) {
                        e.bcc_id[id] = e.num_bccs;
                    }
                    e.st.truncate(st_sz);
                    e.num_bccs += 1;
                }
                low = low.min(low_ch);
                deg += 1;
            } else if e.tin[u] < e.tin[v] {
                e.st.push(e_id);
                low = low.min(e.tin[u]);
            }
        }
        if p_id.is_none() {
            e.is_cut[v] = deg > 1;
        }
        low
    }
    for i in 0..n {
        if e.tin[i] == 0 {
            dfs(&mut e, i, None, adj);
        }
    }
    (e.num_bccs, e.is_cut, e.bcc_id)
}
