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
pub struct HopcroftKarp {
    pub matching_siz: usize,
    pub l_to_r: Vec<Option<usize>>,
    pub r_to_l: Vec<Option<usize>>,
    pub mvc_l: Vec<bool>,
    pub mvc_r: Vec<bool>,
}

impl HopcroftKarp {
    pub fn new(adj: &[Vec<usize>], rsz: usize) -> Self {
        let lsz = adj.len();
        let mut e = HopcroftKarp {
            matching_siz: 0,
            l_to_r: vec![None; lsz],
            r_to_l: vec![None; rsz],
            mvc_l: vec![false; lsz],
            mvc_r: vec![false; rsz],
        };
        loop {
            let mut dist = vec![usize::MAX; lsz];
            let mut q = VecDeque::new();
            for (i, _) in e
                .l_to_r
                .iter()
                .enumerate()
                .filter(|&(_, elem)| elem.is_none())
            {
                dist[i] = 0;
                q.push_back(i);
            }
            let mut found = false;
            for v in &mut e.mvc_l {
                *v = true;
            }
            for v in &mut e.mvc_r {
                *v = false;
            }
            while let Some(u) = q.pop_front() {
                e.mvc_l[u] = false;
                for &v in &adj[u] {
                    e.mvc_r[v] = true;
                    if let Some(w) = e.r_to_l[v] {
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
                return e;
            }
            fn dfs(u: usize, adj: &[Vec<usize>], dist: &mut [usize], e: &mut HopcroftKarp) -> bool {
                for &v in &adj[u] {
                    let w = e.r_to_l[v];
                    if w.is_none()
                        || dist[u] + 1 == dist[w.unwrap()] && dfs(w.unwrap(), adj, dist, e)
                    {
                        (e.l_to_r[u], e.r_to_l[v]) = (Some(v), Some(u));
                        return true;
                    }
                }
                dist[u] = usize::MAX;
                false
            }
            e.matching_siz += (0..lsz)
                .filter(|&i| e.l_to_r[i].is_none() && dfs(i, adj, &mut dist, &mut e))
                .count();
        }
    }
}
