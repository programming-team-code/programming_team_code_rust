// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/GRL_7_A

use proconio::input;
use programming_team_code_rust::graphs::hopcroft_karp::hopcroft_karp;

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

    assert_eq!(
        matching_siz,
        l_to_r.iter().filter(|elem| elem.is_some()).count()
    );
    assert_eq!(
        matching_siz,
        r_to_l.iter().filter(|elem| elem.is_some()).count()
    );

    for (i, elem) in l_to_r.iter().enumerate().filter(|(_, elem)| elem.is_some()) {
        assert_eq!(Some(i), r_to_l[elem.unwrap()]);
    }
    for (i, elem) in r_to_l.iter().enumerate().filter(|(_, elem)| elem.is_some()) {
        assert_eq!(Some(i), l_to_r[elem.unwrap()]);
    }

    assert_eq!(
        matching_siz,
        mvc_l.iter().filter(|&elem| *elem).count() + mvc_r.iter().filter(|&elem| *elem).count()
    );
    for &(u, v) in edge_list.iter() {
        let either = mvc_l[u] || mvc_r[v];
        assert!(either);
    }

    println!("{}", matching_siz);
}
