// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/GRL_5_B

use proconio::input;
use programming_team_code_rust::graphs::hld::HLD;
use std::collections::VecDeque;

fn main() {
    input! {
        n: usize,
        edges: [(usize, usize, u32); n - 1],
    }

    let mut adj_weighted = vec![vec![]; n];
    let mut adj = vec![vec![]; n];
    for &(u, v, w) in &edges {
        adj[u].push(v);
        adj[v].push(u);
        adj_weighted[u].push((v, w));
        adj_weighted[v].push((u, w));
    }
    let adj_weighted = adj_weighted;

    let mut dist = vec![0; n];
    {
        fn dfs(u: usize, p: Option<usize>, adj_weighted: &[Vec<(usize, u32)>], dist: &mut [u32]) {
            for &(v, w) in &adj_weighted[u] {
                if Some(v) == p {
                    continue;
                }
                dist[v] = w + dist[u];
                dfs(v, Some(u), adj_weighted, dist);
            }
        }
        dfs(0, None, &adj_weighted, &mut dist);
    }
    let dist = dist;

    let hld = HLD::new(&mut adj, true);

    let weighted_dist = |u: usize, v: usize| -> u32 {
        let lc = hld.lca(u, v);
        dist[u] + dist[v] - 2 * dist[lc]
    };

    let mut diam_u = 0;
    for i in 1..n {
        if weighted_dist(0, i) > weighted_dist(0, diam_u) {
            diam_u = i;
        }
    }
    let mut diam_v = 0;
    for i in 1..n {
        if weighted_dist(diam_u, i) > weighted_dist(diam_u, diam_v) {
            diam_v = i;
        }
    }

    for u in 0..n {
        let (par, to_node) = hld.aux_tree(vec![diam_u, diam_v, u]);
        let mut aux_adj = vec![vec![]; par.len()];
        for i in 1..par.len() {
            let edge_w = dist[to_node[i]] - dist[to_node[par[i]]];
            aux_adj[i].push((par[i], edge_w));
            aux_adj[par[i]].push((i, edge_w));
        }
        let mut q = VecDeque::new();
        q.push_back((to_node.iter().position(|&x| x == u).unwrap(), None, 0));
        let mut res = 0;
        while let Some((node, parent, curr_dist)) = q.pop_front() {
            res = res.max(curr_dist);
            for &(v, w) in &aux_adj[node] {
                if Some(v) == parent {
                    continue;
                }
                q.push_back((v, Some(node), curr_dist + w));
            }
        }
        println!("{}", res);
    }
}
