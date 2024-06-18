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
    struct Env {
        timer: usize,
        tin: Vec<usize>,
        num_bccs: usize,
        is_cut: Vec<bool>,
        bcc_id: Vec<usize>,
        st: Vec<usize>,
    }
    fn dfs(u: usize, p_id: Option<usize>, adj: &[Vec<(usize, usize)>], e: &mut Env) -> usize {
        e.tin[u] = e.timer;
        let (mut low, mut deg) = (e.timer, 0);
        e.timer += 1;
        for &(v, e_id) in &adj[u] {
            assert_ne!(u, v);
            if Some(e_id) == p_id {
                continue;
            }
            if e.tin[v] == 0 {
                let st_sz = e.st.len();
                e.st.push(e_id);
                let low_ch = dfs(v, Some(e_id), adj, e);
                if low_ch >= e.tin[u] {
                    e.is_cut[u] = true;
                    for &id in e.st.iter().skip(st_sz) {
                        e.bcc_id[id] = e.num_bccs;
                    }
                    e.st.truncate(st_sz);
                    e.num_bccs += 1;
                }
                low = low.min(low_ch);
                deg += 1;
            } else if e.tin[v] < e.tin[u] {
                e.st.push(e_id);
                low = low.min(e.tin[v]);
            }
        }
        if p_id.is_none() {
            e.is_cut[u] = deg > 1;
        }
        low
    }
    let n = adj.len();
    let mut e = Env {
        timer: 1,
        tin: vec![0; n],
        num_bccs: 0,
        is_cut: vec![false; n],
        bcc_id: vec![0; m],
        st: Vec::with_capacity(m),
    };
    for i in 0..n {
        if e.tin[i] == 0 {
            dfs(i, None, adj, &mut e);
        }
    }
    (e.num_bccs, e.is_cut, e.bcc_id)
}
