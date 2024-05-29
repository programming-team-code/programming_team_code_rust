// verification-helper: PROBLEM https://judge.yosupo.jp/problem/scc

use proconio::input;
use programming_team_code_rust::graphs::scc::get_sccs;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    let mut adj = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        adj[u].push(v);
    }

    let (num_sccs, scc_id) = get_sccs(&adj);

    for (u, v) in edges {
        assert!(scc_id[u] <= scc_id[v]);
    }

    println!("{}", num_sccs);

    let mut sccs = vec![vec![]; num_sccs];
    for i in 0..n {
        sccs[scc_id[i]].push(i);
    }

    for scc in sccs {
        print!("{}", scc.len());
        for v in scc {
            print!(" {}", v);
        }
        println!();
    }
}
