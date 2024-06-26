// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/GRL_7_A

use proconio::input;
use programming_team_code_rust::graphs::hopcroft_karp::HopcroftKarp;

mod hopcroft_karp_asserts;
use hopcroft_karp_asserts::hopcroft_karp_asserts;

fn main() {
    input! {
        lsz: usize,
        rsz: usize,
        m: usize,
        edge_list: [(usize, usize); m],
    }

    let mut adj = vec![vec![]; lsz];
    for &(u, v) in edge_list.iter() {
        adj[u].push(v);
    }

    let HopcroftKarp {
        matching_siz,
        l_to_r,
        r_to_l,
        mvc_l,
        mvc_r,
    } = HopcroftKarp::new(&adj, rsz);

    hopcroft_karp_asserts(matching_siz, &l_to_r, &r_to_l, &mvc_l, &mvc_r, &edge_list);

    println!("{}", matching_siz);
}
