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
/// let dist = dijk(&adj, 0);
/// assert_eq!(dist, [0, 10, 110, u64::MAX]);
/// ```
///
/// # Complexity
/// - V: number of vertices
/// - E: number of edges
/// - Time: O(V + E log E)
/// - Space: O(V + E)
pub fn dijk(adj: &[Vec<(usize, u64)>], s: usize) -> Vec<u64> {
    use std::cmp::Reverse;
    let n = adj.len();
    let mut dist = vec![u64::MAX; n];
    let mut q = std::collections::BinaryHeap::new();
    q.push(Reverse((0, s)));
    while let Some(Reverse((d, v))) = q.pop() {
        if dist[v] <= d {
            continue;
        }
        dist[v] = d;
        for &(u, w) in &adj[v] {
            q.push(Reverse((dist[v] + w, u)));
        }
    }
    dist
}
