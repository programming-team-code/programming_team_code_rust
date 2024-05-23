//! # Strongly Connected Components

/// # Example
/// ```
/// use programming_team_code_rust::graphs::scc::get_sccs;
/// ```
///
/// # Complexity
pub struct Sccs {
    num_sccs: usize,
    scc_id: Vec<usize>,
}

pub fn get_sccs(adj: &[Vec<usize>]) -> Sccs {
    let n = adj.len();
    let timer = 1;
    let mut seen = vec![false; n];
    let mut tin = vec![0; n];
    let mut st = Vec::with_capacity(n);
    let mut scc_id = vec![0; n];
    let mut num_sccs = 0;
    fn dfs(
        u: usize,
        adj: &[Vec<usize>],
        seen: &mut [bool],
        timer: &mut usize,
        tin: &mut [usize],
        st: &mut Vec<usize>,
        scc_id: &mut [usize],
        num_sccs: &mut usize,
    ) -> usize {
        let low = *timer;
        *timer += 1;
        tin[u] = low;
        let siz = st.len();
        st.push(u);
        for &v in &adj[u] {
            if !seen[v] {
                low = low.min(if tin[v] > 0 {
                    tin[v]
                } else {
                    dfs(v, adj, seen, timer, tin, st, scc_id, num_sccs)
                });
            }
        }
        tin[u]
    }
    for s in 0..n {
        if tin[s] == 0 {
            dfs(
                s,
                adj,
                &mut seen,
                &mut timer,
                &mut tin,
                &mut st,
                &mut scc_id,
                &mut num_sccs,
            );
        }
    }
}
