// verification-helper: PROBLEM https://judge.yosupo.jp/problem/bipartitematching

use proconio::input;
use programming_team_code_rust::graphs::hopcroft_karp::hopcroft_karp;

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

    let (matching_siz, l_to_r, r_to_l, mvc_l, mvc_r) = hopcroft_karp(&adj, rsz);

    hopcroft_karp_asserts(matching_siz, &l_to_r, &r_to_l, &mvc_l, &mvc_r, &edge_list);

    println!("{}", matching_siz);
    for (i, elem) in l_to_r.iter().enumerate().filter(|(_, elem)| elem.is_some()) {
        println!("{} {}", i, elem.unwrap());
    }
}
