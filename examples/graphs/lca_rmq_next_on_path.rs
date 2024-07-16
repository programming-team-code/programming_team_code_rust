// verification-helper: PROBLEM https://judge.yosupo.jp/problem/tree_diameter

use proconio::input;
use programming_team_code_rust::graphs::lca::LCA;

fn main() {
    input! {
        n: usize,
    }

    let mut adj = vec![vec![]; n];
    let mut adj_w = vec![vec![]; n];
    for _ in 1..n {
        input! {
            u: usize,
            v: usize,
            w: i64,
        }
        adj[u].push(v);
        adj[v].push(u);
        adj_w[u].push((v, w));
        adj_w[v].push((u, w));
    }

    let lca = LCA::new(&adj);

    let mut dist_root = vec![0; n];
    fn dfs(u: usize, p: Option<usize>, adj_w: &[Vec<(usize, i64)>], dist_root: &mut Vec<i64>) {
        for &(v, w) in &adj_w[u] {
            if Some(v) == p {
                continue;
            }
            dist_root[v] = w + dist_root[u];
            dfs(v, Some(u), adj_w, dist_root);
        }
    }

    dfs(0, None, &adj_w, &mut dist_root);

    let dist_path =
        |u: usize, v: usize| -> i64 { dist_root[u] + dist_root[v] - 2 * dist_root[lca.lca(u, v)] };

    let mut u = 0;
    for i in 1..n {
        if dist_path(0, u) < dist_path(0, i) {
            u = i;
        }
    }

    let mut v = 0;
    for i in 1..n {
        if dist_path(u, v) < dist_path(u, i) {
            v = i;
        }
    }

    println!("{} {}", dist_path(u, v), lca.dist(u, v) + 1);

    print!("{} ", u);
    while u != v {
        u = lca.next_on_path(u, v);
        print!(" {}", u);
    }
    println!();
}
