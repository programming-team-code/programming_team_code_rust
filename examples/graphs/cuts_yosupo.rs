// verification-helper: PROBLEM https://judge.yosupo.jp/problem/biconnected_components

use proconio::input;
use programming_team_code_rust::graphs::block_vertex_tree::get_bvt;
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

    let (num_bccs, is_cut, bcc_id) = get_cuts(&adj, m);
    let bvt = get_bvt(&adj, num_bccs, &bcc_id);

    for i in 0..n {
        assert_eq!(
            is_cut[i],
            adj[i]
                .iter()
                .any(|&(_, e_id)| bcc_id[e_id] != bcc_id[adj[i][0].1])
        );
    }

    let lone_nodes = adj
        .iter()
        .enumerate()
        .filter(|&(_, neighbors)| neighbors.is_empty())
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();

    println!("{}", num_bccs + lone_nodes.len());

    for u in lone_nodes {
        println!("1 {}", u);
    }

    for neighbors in bvt.iter().skip(n) {
        print!("{} ", neighbors.len());
        for u in neighbors {
            print!("{} ", u);
        }
        println!();
    }
}
