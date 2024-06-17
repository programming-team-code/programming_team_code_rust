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

    let (dist, par) = dijk(&adj, s);

    assert_eq!(par[s], None);

    if dist[t] == u64::MAX {
        println!("{}", -1);
        return;
    }

    let mut path = vec![];
    let mut u = t;
    path.push(u);
    while let Some(prev_u) = par[u] {
        path.push(prev_u);
        u = prev_u;
    }

    path.reverse();

    println!("{} {}", dist[t], path.len() - 1);
    for it in 0..path.len() - 1 {
        println!("{} {}", path[it], path[it + 1]);
    }
}
