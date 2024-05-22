// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca

use proconio::input;
use programming_team_code_rust::graphs::lca::LCA;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    // forest with an undirected tree and a rooted directed tree
    let mut adj = vec![vec![]; n + n];
    for v in 1..n {
        input! {
            p: usize,
        }

        // undirected tree
        adj[p].push(i);
        adj[i].push(p);

        // directed tree
        adj[n + p].push(n + i);
    }

    let lca = LCA::new(&adj);
    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
        }
        let res_undir = lca.lca(u, v);
        let res_dir = lca.lca(n + u, n + v) - n;
        assert!(res_undir == res_dir);
        println!("{}", res_undir);
    }
}
