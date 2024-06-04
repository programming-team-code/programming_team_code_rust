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

    let hld = HLD::new(&mut adj, false);

    fn dfs(u: usize, adj: &[Vec<usize>], d: &mut Vec<usize>) {
        for &v in &adj[u] {
            d[v] = 1 + d[u];
            dfs(v, adj, d);
        }
    }

    let mut d = vec![0; n];
    dfs(0, &adj, &mut d);

    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
            k: usize,
        }

        let in_sub_naive = hld.kth_par(v, d[u].abs_diff(d[v])) == Some(u);
        assert_eq!(in_sub_naive, hld.in_sub(u, v));

        match hld.kth_on_path(u, v, k) {
            Some(w) => {
                assert!(k < hld.dist(u, v));
                assert!(hld.on_path(u, v, w));
                if w != u {
                    assert!(!hld.on_path(v, w, u));
                }
                println!("{}", w);
            }
            None => {
                assert!(k >= hld.dist(u, v));
                println!("-1");
            }
        }
    }
}
