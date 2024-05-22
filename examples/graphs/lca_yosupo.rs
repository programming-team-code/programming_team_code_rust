// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca

use proconio::input;
use programming_team_code_rust::graphs::lca::LCA;

fn main() {
    input! {
        n: usize,
        q: usize,
        p: [usize; n - 1],
        queries: [(usize, usize); q],
    }

    let mut adj = vec![vec![]; n + n];
    for (i, &parent) in p.iter().enumerate() {
        adj[parent].push(i + 1);
        adj[i + 1].push(parent);
        adj[n + parent].push(n + i + 1);
    }

    let lca = LCA::new(&adj);
    for (u, v) in queries {
        let res = lca.lca(u, v);
        let res_other = lca.lca(n + u, n + v) - n;
        assert!(res == res_other);
        println!("{}", res);
    }
}
