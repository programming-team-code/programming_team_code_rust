// verification-helper: PROBLEM https://judge.yosupo.jp/problem/frequency_table_of_tree_distance

use proconio::input;
use programming_team_code_rust::graphs::count_paths_per_length::count_paths_per_length;

fn main() {
    input! {
        n: usize
    }

    let mut adj = vec![vec![]; n];
    for _ in 0..n - 1 {
        input! {
            u: usize,
            v: usize
        }
        adj[u].push(v);
        adj[v].push(u);
    }

    let paths_per_length = count_paths_per_length(&adj);

    println!(
        "{}",
        paths_per_length
            .iter()
            .skip(1)
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
