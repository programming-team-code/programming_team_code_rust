// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_C

use proconio::input;
use programming_team_code_rust::lca::LCA;

fn main() {
    input! {
        n: usize,
    }

    let adj = (0..n)
        .map(|_| {
            input! {
                k: usize,
                c: [usize; k],
            }
            c
        })
        .collect::<Vec<Vec<usize>>>();

    let lca = LCA::new(&adj);

    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            u: usize,
            v: usize,
        }
        println!("{}", lca.lca(u, v));
    }
}
