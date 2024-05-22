// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use proconio::input;
use programming_team_code_rust::data_structures::rmq::RMQ;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
        queries: [(usize, usize); q],
    }

    let rmq = RMQ::new(&a, std::cmp::min);
    for (le, ri) in queries {
        println!("{}", rmq.query(le..ri));
    }
}
