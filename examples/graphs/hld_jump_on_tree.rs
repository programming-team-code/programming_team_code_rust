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
        match hld_nodes.kth_on_path(u, v, k) {
            Some(w) => {
                assert!(k <= hld_nodes.dist(u, v));
                assert!(hld_nodes.on_path(u, v, w));
                let u_sub_tree = hld_nodes.in_sub(w, u);
                let v_sub_tree = hld_nodes.in_sub(w, v);
                assert_eq!(u_sub_tree, hld_edges.in_sub(w, u));
                assert_eq!(v_sub_tree, hld_edges.in_sub(w, v));
                assert!(u_sub_tree || v_sub_tree);
                assert_eq!(u_sub_tree && v_sub_tree, w == hld_nodes.lca(u, v));
                println!("{}", w);
            }
            None => {
                assert!(k > hld_nodes.dist(u, v));
                println!("-1");
            }
        }
    }
}
