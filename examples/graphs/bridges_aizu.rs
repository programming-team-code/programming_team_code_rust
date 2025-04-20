// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/GRL_3_B

use proconio::input;
use programming_team_code_rust::graphs::bridges::get_bridges;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut edges: [(usize, usize); m],
    }

    let mut adj = vec![vec![]; n];
    for (i, &(u, v)) in edges.iter().enumerate() {
        adj[u].push((v, i));
        adj[v].push((u, i));
    }

    let (_, is_bridge, two_edge_ccid) = get_bridges(&adj, m);

    for (i, &(u, v)) in edges.iter().enumerate() {
        assert_eq!(is_bridge[i], two_edge_ccid[u] != two_edge_ccid[v]);
    }

    let mut all_bridges: Vec<(usize, usize)> = vec![];

    for (i, &mut (mut u, mut v)) in edges.iter_mut().enumerate() {
        if is_bridge[i] {
            if u > v {
                std::mem::swap(&mut u, &mut v);
            }
            all_bridges.push((u, v));
        }
    }
    all_bridges.sort();
    for (u, v) in all_bridges {
        println!("{} {}", u, v);
    }
}
