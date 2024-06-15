// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/GRL_3_A

use proconio::input;
use programming_team_code_rust::graphs::cuts::get_cuts;

fn main() {
    input! {
        n: usize,
        m: usize,
        edges: [(usize, usize); m],
    }

    let mut adj = vec![vec![]; n];
    for (i, &(u, v)) in edges.iter().enumerate() {
        adj[u].push((v, i));
        adj[v].push((u, i));
    }

    let (_, is_cut, bcc_id) = get_cuts(&adj, m);

    /*
    for i in 0..n {
        let mut all_same = true;
        for &(_, e_id) in &adj[i] {
            if bcc_id[e_id] != bcc_id[adj[i][0].1] {
                all_same = false;
            }
        }
        assert_eq!(all_same, !is_cut[i]);
    }
    */

    let all_cut_nodes = is_cut
        .iter()
        .enumerate()
        .filter(|(_, &value)| value)
        .map(|(index, _)| index)
        .collect::<Vec<usize>>();

    for u in all_cut_nodes {
        println!("{}", u);
    }
}
