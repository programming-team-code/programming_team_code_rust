use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijk(adj: &[Vec<(usize, u64)>], s: usize) -> Vec<u64> {
    let n = adj.len();
    let mut dist = vec![u64::MAX; n];
    let mut q = BinaryHeap::new();
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
