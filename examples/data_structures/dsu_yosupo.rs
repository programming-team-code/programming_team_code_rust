// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

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
            _ => println!("{}", u8::from(dsu.same(u, v))),
        }
    }
}
