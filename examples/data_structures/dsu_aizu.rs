// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_1_A

use proconio::input;
use programming_team_code_rust::data_structures::dsu::DSU;

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
            _ => {
                assert_eq!(kind, 1);
                println!("{}", u8::from(dsu.same(u, v)));
            }
        }
    }
}
