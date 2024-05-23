// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca

use proconio::input;
use programming_team_code_rust::graphs::lca::LCA;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    // undirected tree and a rooted directed tree
    let mut undir = vec![vec![]; n + n];
    let mut dir = vec![vec![]; n];
    for c in 1..n {
        input! {
            p: usize,
        }

        // undirected tree
        undir[p].push(c);
        undir[c].push(p);

        // directed tree
        dir[p].push(c);
    }

    let undir_lca = LCA::new(&undir);
    let dir_lca = LCA::new(&dir);
    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
        }

        let res_undir = undir_lca.lca(u, v);
        let res_dir = dir_lca.lca(u, v);

        assert_eq!(res_undir, res_dir);

        println!("{}", res_undir);
    }
}
