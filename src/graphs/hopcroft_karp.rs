//! # Hopcroft Karp algorithm for Bipartite Matching

use std::collections::VecDeque;

/// # Example
/// ```
/// use programming_team_code_rust::graphs::hopcroft_karp::hopcroft_karp;
///
/// let mut adj = vec![vec![]; 2];
/// for (u, v) in [(0, 0), (0, 2), (1, 2)] {
///    adj[u].push(v);
/// }
///
/// let (matching_siz, l_to_r, r_to_l, mvc_l, mvc_r) = hopcroft_karp(&adj, 3);
///
/// assert_eq!(matching_siz, 2);
/// assert_eq!(l_to_r, [Some(0), Some(2)]);
/// assert_eq!(r_to_l, [Some(0), None, Some(1)]);
/// assert_eq!(mvc_l, [true, true]);
/// assert_eq!(mvc_r, [false, false, false]);
/// ```
///
/// # Complexity
/// - V: number of vertices; V = lsz + rsz
/// - E: number of edges
/// - Time: O(V + E * sqrt(v))
/// - Space: O(V)
#[allow(clippy::type_complexity)]
pub fn hopcroft_karp(
    adj: &[Vec<usize>],
    rsz: usize,
) -> (
    usize,
    Vec<Option<usize>>,
    Vec<Option<usize>>,
    Vec<bool>,
    Vec<bool>,
) {
    fn dfs(
        u: usize,
        adj: &[Vec<usize>],
        dist: &mut [usize],
        l_to_r: &mut [Option<usize>],
        r_to_l: &mut [Option<usize>],
    ) -> bool {
        for &v in &adj[u] {
            let w = r_to_l[v];
            if w.is_none()
                || dist[u] + 1 == dist[w.unwrap()] && dfs(w.unwrap(), adj, dist, l_to_r, r_to_l)
            {
                (l_to_r[u], r_to_l[v]) = (Some(v), Some(u));
                return true;
            }
        }
        dist[u] = usize::MAX;
        false
    }
    let (lsz, mut matching_siz) = (adj.len(), 0);
    let (mut l_to_r, mut r_to_l) = (vec![None::<usize>; lsz], vec![None::<usize>; rsz]);
    loop {
        let (mut dist, mut q) = (vec![usize::MAX; lsz], VecDeque::new());
        for (i, _) in l_to_r
            .iter()
            .enumerate()
            .filter(|&(_, elem)| elem.is_none())
        {
            dist[i] = 0;
            q.push_back(i);
        }
        let (mut found, mut mvc_l, mut mvc_r) = (false, vec![true; lsz], vec![false; rsz]);
        while let Some(u) = q.pop_front() {
            mvc_l[u] = false;
            for &v in &adj[u] {
                mvc_r[v] = true;
                if let Some(w) = r_to_l[v] {
                    if dist[w] > 1 + dist[u] {
                        dist[w] = 1 + dist[u];
                        q.push_back(w);
                    }
                } else {
                    found = true;
                }
            }
        }
        if !found {
            return (matching_siz, l_to_r, r_to_l, mvc_l, mvc_r);
        }
        for i in 0..lsz {
            matching_siz +=
                (l_to_r[i].is_none() && dfs(i, adj, &mut dist, &mut l_to_r, &mut r_to_l)) as usize;
        }
    }
}
