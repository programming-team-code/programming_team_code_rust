// verification-helper: PROBLEM https://judge.yosupo.jp/problem/two_edge_connected_components

use proconio::input;
use programming_team_code_rust::graphs::bridges::get_bridges;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    let mut adj = vec![vec![]; n];
    for (i, &(u, v)) in edges.iter().enumerate() {
        adj[u].push((v, i));
        adj[v].push((u, i));
    }

    let (num_2_edge_ccs, is_bridge, two_edge_ccid) = get_bridges(&adj, m);

    for (i, &(u, v)) in edges.iter().enumerate() {
        assert_eq!(is_bridge[i], two_edge_ccid[u] != two_edge_ccid[v]);
    }

    println!("{}", num_2_edge_ccs);

    let mut bridge_ccs = vec![vec![]; num_2_edge_ccs];
    for i in 0..n {
        bridge_ccs[two_edge_ccid[i]].push(i);
    }

    for cc in bridge_ccs.iter() {
        print!("{} ", cc.len());
        for u in cc {
            print!("{} ", u);
        }
        println!();
    }
}
