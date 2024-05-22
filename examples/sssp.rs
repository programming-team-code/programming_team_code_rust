// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_1_A

use proconio::input;
use programming_team_code_rust::dijk::dijk;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        edges: [(usize, usize, u64); m],
    }

    let mut adj = vec![vec![]; n];
    for (u, v, w) in edges {
        adj[u].push((v, w));
    }

    let dist = dijk(&adj, s);
    for d in dist {
        if d == std::u64::MAX {
            println!("INF");
        } else {
            println!("{}", d);
        }
    }
}
