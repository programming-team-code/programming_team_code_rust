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
/// ```
///
/// # Panics
/// ```panic
/// use programming_team_code_rust::graphs::cuts::get_cuts;
/// let mut adj = vec![vec![]; 1];
/// for (i, &(u, v)) in [(0,0)].iter().enumerate() {
///    adj[u].push((v, i));
///    adj[v].push((u, i));
/// }
/// let (_, _, _) = get_cuts(&adj, 1);
/// ```
///
/// # Complexity
/// - Time: O(V + E)
/// - Space: O(V)
pub fn get_cuts(adj: &[Vec<(usize, usize)>], m: usize) -> (usize, Vec<bool>, Vec<usize>) {
    #[allow(clippy::too_many_arguments)]
    fn dfs(
        u: usize,
        p_id: Option<usize>,
        adj: &[Vec<(usize, usize)>],
        timer: &mut usize,
        tin: &mut [usize],
        num_bccs: &mut usize,
        is_cut: &mut [bool],
        bcc_id: &mut [usize],
        st: &mut Vec<usize>,
    ) -> usize {
        tin[u] = *timer;
        let (mut low, mut deg) = (*timer, 0);
        *timer += 1;
        for &(v, e_id) in &adj[u] {
            assert_ne!(u, v);
            if Some(e_id) == p_id {
                continue;
            }
            if tin[v] == 0 {
                let st_sz = st.len();
                st.push(e_id);
                let low_ch = dfs(v, Some(e_id), adj, timer, tin, num_bccs, is_cut, bcc_id, st);
                if low_ch >= tin[u] {
                    is_cut[u] = true;
                    for &id in st.iter().skip(st_sz) {
                        bcc_id[id] = *num_bccs;
                    }
                    st.truncate(st_sz);
                    *num_bccs += 1;
                }
                low = low.min(low_ch);
                deg += 1;
            } else if tin[v] < tin[u] {
                st.push(e_id);
                low = low.min(tin[v]);
            }
        }
        if p_id.is_none() {
            is_cut[u] = deg > 1;
        }
        low
    }
    let (n, mut timer, mut num_bccs, mut bcc_id, mut st) =
        (adj.len(), 1, 0, vec![0; m], Vec::with_capacity(m));
    let (mut tin, mut is_cut) = (vec![0; n], vec![false; n]);
    for i in 0..n {
        if tin[i] == 0 {
            dfs(
                i,
                None,
                adj,
                &mut timer,
                &mut tin,
                &mut num_bccs,
                &mut is_cut,
                &mut bcc_id,
                &mut st,
            );
        }
    }
    (num_bccs, is_cut, bcc_id)
}
