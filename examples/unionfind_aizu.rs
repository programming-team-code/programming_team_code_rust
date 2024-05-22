// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A

use proconio::input;
use programming_team_code_rust::dsu::DSU;

fn main() {
    input! {
        n: usize,
        queries: [(u8, usize, usize)],
    }

    let mut dsu = DSU::new(n);
    for (kind, u, v) in queries {
        match kind {
            0 => {
                dsu.unite(u, v);
            }
            1 => println!("{}", u8::from(dsu.same(u, v))),
            _ => unreachable!(),
        }
    }
}
