// verification-helper: PROBLEM https://judge.yosupo.jp/problem/unionfind

use proconio::input;
use programming_team_code_rust::dsu::Dsu;

fn main() {
    input! {
        n: usize,
        queries: [(u8, usize, usize)],
    }

    let mut dsu = Dsu::new(n);
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
