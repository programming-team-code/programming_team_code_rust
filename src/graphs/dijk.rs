//! # Dijkstra's algorithm

/// # Example
/// ```
/// use programming_team_code_rust::graphs::dijk::dijk;
///
/// let adj = vec![
///    vec![(1, 1), (2, 4)],
///    vec![(2, 2), (3, 5)],
///    vec![(3, 1)],
///    vec![],
///    vec![(0, 7)],
///    vec![(1, 3), (3, 2)],
///    vec![(0, 2), (2, 3)],
///    vec![(1, 2), (4, 2)],
/// ];
///
/// let dist = dijk(&adj, 0);
/// assert_eq!(dist, vec![0, 1, 3, 4, u64::MAX, u64::MAX, u64::MAX, u64::MAX]);
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
    while let Some(Reverse((d, u))) = q.pop() {
        if dist[u] <= d {
            continue;
        }
        dist[u] = d;
        for &(v, w) in &adj[u] {
            q.push(Reverse((dist[u] + w, v)));
        }
    }
    dist
}
