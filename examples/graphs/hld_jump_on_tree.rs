// verification-helper: PROBLEM https://judge.yosupo.jp/problem/jump_on_tree

use proconio::input;
use programming_team_code_rust::graphs::hld::HLD;
use programming_team_code_rust::graphs::dfs_order::get_dfs_preorder;

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

    let mut d: Vec<usize> = vec![0; n];
    for &u in get_dfs_preorder(&adj).iter() {
        if let Some(par) = hld_nodes.p[u] {
            d[u] = 1 + d[par];
        }
    }

    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
            k: usize,
        }

        assert_eq!(hld_nodes.dist(u, v), 1 + hld_edges.dist(u, v));
        let in_sub_naive = hld_nodes.kth_par(v, d[u].abs_diff(d[v])) == Some(u);
        assert_eq!(in_sub_naive, hld_nodes.in_sub(u, v));
        assert_eq!(in_sub_naive, hld_edges.in_sub(u, v));

        let res = hld_nodes.kth_on_path(u, v, k);
        assert_eq!(res, hld_edges.kth_on_path(u, v, k));

        match res {
            Some(w) => {
                assert!(k < hld_nodes.dist(u, v));
                assert!(hld_nodes.on_path(u, v, w));
                assert!(hld_edges.on_path(u, v, w));
                if w != u {
                    assert!(!hld_nodes.on_path(v, w, u));
                    assert!(!hld_edges.on_path(v, w, u));
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
