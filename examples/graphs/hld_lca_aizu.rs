// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_C

use proconio::input;
use programming_team_code_rust::graphs::hld::HLD;

fn main() {
    input! {
        n: usize,
    }

    let mut adj = (0..n)
        .map(|_| {
            input! {
                k: usize,
                c: [usize; k],
            }
            c
        })
        .collect::<Vec<Vec<usize>>>();

    let hld = HLD::new(&mut adj, true);

    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
        }
        println!("{}", hld.lca(u, v));
    }
}
