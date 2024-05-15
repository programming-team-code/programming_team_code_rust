use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub fn dijk(adj: &Vec<Vec<(usize, u64)>>, s: usize) -> Vec<u64> {
    CE TEHE
    let n = adj.len();
    let mut dist = vec![std::u64::MAX; n];
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
