// verification-helper: PROBLEM https://judge.yosupo.jp/problem/jump_on_tree

use proconio::input;
use programming_team_code_rust::graphs::hld::HLD;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut adj = vec![vec![]; n];
    for _ in 1..n {
        input! {
            u: usize,
            v: usize
        }
        adj[u].push(v);
        adj[v].push(u);
    }

    let hld = HLD::new(&mut adj, true);

    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
            k: usize,
        }

        assert_eq!(hld.kth_par(u, hld.d[u]), Some(0));
        assert_eq!(hld.kth_par(u, hld.d[u] + 1), None);

        match hld.kth_on_path(u, v, k) {
            Some(w) => {
                assert!(k <= hld.dist(u, v));
                assert!(hld.on_path(u, v, w));
                if w != u {
                    assert!(!hld.on_path(v, w, u));
                }
                println!("{}", w);
            }
            None => {
                assert!(k > hld.dist(u, v));
                println!("-1");
            }
        }
    }
}
