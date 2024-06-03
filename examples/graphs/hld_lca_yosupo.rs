// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca

use proconio::input;
use programming_team_code_rust::graphs::hld::HLD;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut adj = vec![vec![]; n];
    for c in 1..n {
        input! {
            p: usize,
        }
        adj[c].push(p);
        adj[p].push(c);
    }

    let hld = HLD::new(&mut adj, false);
    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
        }
        println!("{}", hld.lca(u, v));
    }
}
