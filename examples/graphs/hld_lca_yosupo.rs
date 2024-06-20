// verification-helper: PROBLEM https://judge.yosupo.jp/problem/lca

use proconio::input;
use programming_team_code_rust::graphs::hld::HLD;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    // undirected forest and a rooted directed tree
    let mut undir_for = vec![vec![]; n + n];
    let mut dir = vec![vec![]; n];
    for c in 1..n {
        input! {
            p: usize,
        }

        // undirected forest
        for &(u, v) in [(p, c), (p + n, c + n)].iter() {
            undir_for[u].push(v);
            undir_for[v].push(u);
        }

        // directed tree
        dir[p].push(c);
    }

    let undir_hld = HLD::new(&mut undir_for, false);
    let dir_hld = HLD::new(&mut dir, true);

    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
        }
        let res_undir1 = undir_hld.lca(u, v);
        let res_undir2 = undir_hld.lca(u + n, v + n) - n;
        let res_dir = dir_hld.lca(u, v);

        assert_eq!(res_undir1, res_undir2);
        assert_eq!(res_undir1, res_dir);

        println!("{}", res_dir);
    }
}
