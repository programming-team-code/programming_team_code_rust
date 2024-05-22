// verification-helper: PROBLEM https://judge.yosupo.jp/problem/shortest_path

use proconio::input;
use programming_team_code_rust::graphs::dijk::dijk;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: usize,
        t: usize,
        edges: [(usize, usize, u64); m],
    }

    let mut adj = vec![vec![]; n];
    for (u, v, w) in edges {
        adj[u].push((v, w));
    }

    fn dfs(
        adj: &Vec<Vec<(usize, u64)>>,
        dist: &Vec<u64>,
        u: usize,
        path: &mut Vec<usize>,
        seen: &mut Vec<bool>,
        t: usize,
    ) {
        if seen[u] {
            return;
        }
        seen[u] = true;
        path.push(u);
        if u == t {
            println!("{} {}", dist[u], path.len() - 1);
            for i in 0..path.len() - 1 {
                println!("{} {}", path[i], path[i + 1]);
            }
            std::process::exit(0);
        }
        for &(v, w) in &adj[u] {
            if dist[u] + w == dist[v] {
                dfs(adj, dist, v, path, seen, t);
            }
        }
        path.pop();
    }

    let dist = dijk(&adj, s);
    let mut path = vec![];
    let mut seen = vec![false; n];
    dfs(&adj, &dist, s, &mut path, &mut seen, t);

    println!("{}", -1);
}
