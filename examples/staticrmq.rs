// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use proconio::input;
use programming_team_code_rust::rmq::RMQ;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
        queries: [(usize, usize); q],
    }

    let rmq = RMQ::new(&a);
    for (le, ri) in queries {
        println!("{}", rmq.query(le..ri));
    }
}
