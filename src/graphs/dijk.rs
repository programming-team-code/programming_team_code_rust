//! # Dijkstra's algorithm

/// # Example
/// ```
/// use programming_team_code_rust::graphs::dijk::dijk;
///
/// let mut adj = vec![vec![]; 4];
/// for (u, v, w) in [(0, 1, 10), (1, 2, 100), (0, 2, 111), (2, 2, 0)] {
///    adj[u].push((v, w));
/// }
///
/// let (dist, par) = dijk(&adj, 0);
/// assert_eq!(dist, [0, 10, 110, u64::MAX]);
/// assert_eq!(par, [None, Some(0), Some(1), None]);
/// ```
///
/// # Complexity
/// - V: number of vertices
/// - E: number of edges
/// - Time: O(V + E log E)
/// - Space: O(V + E)
pub fn dijk(adj: &[Vec<(usize, u64)>], s: usize) -> (Vec<u64>, Vec<Option<usize>>) {
    use std::cmp::Reverse;
    let n = adj.len();
    let mut dist = vec![u64::MAX; n];
    let mut par = vec![None; n];
    let mut q = std::collections::BinaryHeap::new();
    q.push(Reverse((0, s, None)));
    while let Some(Reverse((d, u, p))) = q.pop() {
        if dist[u] <= d {
            continue;
        }
        (dist[u], par[u]) = (d, p);
        for &(v, w) in &adj[u] {
            q.push(Reverse((dist[u] + w, v, Some(u))));
        }
    }
    (dist, par)
}
