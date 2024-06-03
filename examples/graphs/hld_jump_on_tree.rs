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

    let hld_nodes = HLD::new(&mut adj, false);
    let hld_edges = HLD::new(&mut adj, true);

    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
            k: usize,
        }
        assert_eq!(hld_nodes.dist(u, v), 1 + hld_edges.dist(u, v));
        let in_sub_naive = hld_nodes.kth_par(v, hld_nodes.d[u].abs_diff(hld_nodes.d[v])) == Some(u);
        assert_eq!(in_sub_naive, hld_nodes.in_sub(u, v));
        assert_eq!(in_sub_naive, hld_edges.in_sub(u, v));
        match hld_nodes.kth_on_path(u, v, k) {
            Some(w) => {
                assert!(k < hld_nodes.dist(u, v));
                assert!(hld_nodes.on_path(u, v, w));
                assert!(hld_edges.on_path(u, v, w));
                if w != u {
                    assert!(!hld_nodes.on_path(v, w, u));
                    assert!(!hld_edges.on_path(v, w, u));
                }
                if w != v {
                    assert!(!hld_nodes.on_path(u, w, v));
                    assert!(!hld_edges.on_path(u, w, v));
                }
                println!("{}", w);
            }
            None => {
                assert!(k >= hld_nodes.dist(u, v));
                println!("-1");
            }
        }
    }
}
