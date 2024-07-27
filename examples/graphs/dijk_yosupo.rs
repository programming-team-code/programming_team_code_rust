// verification-helper: PROBLEM https://judge.yosupo.jp/problem/shortest_path

use proconio::input;
use programming_team_code_rust::graphs::dijk::dijk;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        edges: [(usize, usize, u64); m],
    }

    let mut adj = vec![vec![]; n];
    for (u, v, w) in edges {
        adj[u].push((v, w));
    }

    let dist = dijk(&adj, s);

    if dist[t] == u64::MAX {
        println!("{}", -1);
        return;
    }

    let mut par = vec![None; n];
    {
        let mut seen = vec![false; n];
        seen[s] = true;
        let mut q = std::collections::VecDeque::new();
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            for &(v, w) in &adj[u] {
                if seen[v] || dist[u] + w != dist[v] {
                    continue;
                }
                par[v] = Some(u);
                seen[v] = true;
                q.push_back(v);
            }
        }
    }
    let mut path = vec![t];
    let mut u = t;
    while let Some(v) = par[u] {
        u = v;
        path.push(u);
    }

    path.reverse();

    println!("{} {}", dist[t], path.len() - 1);
    for it in 0..path.len() - 1 {
        println!("{} {}", path[it], path[it + 1]);
    }
}
