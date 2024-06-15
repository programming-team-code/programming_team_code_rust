//! # Cut nodes

pub fn get_cuts(adj: &[Vec<(usize, usize)>], m: usize) -> (usize, Vec<bool>, Vec<usize>) {
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
                    for i in st_sz..st.len() {
                        bcc_id[st[i]] = *num_bccs;
                    }
                    st.truncate(st_sz);
                    *num_bccs += 1;
                }
                low = low.min(tin[v]);
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
    let (n, mut timer, mut num_bccs, mut bcc_id) = (adj.len(), 1, 0, vec![0; m]);
    let (mut tin, mut is_cut, mut node_stack) = (vec![0; n], vec![false; n], Vec::with_capacity(n));
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
                &mut node_stack,
            );
        }
    }
    (num_bccs, is_cut, bcc_id)
}
