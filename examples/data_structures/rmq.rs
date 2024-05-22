// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use proconio::input;
use programming_team_code_rust::data_structures::rmq::RMQ;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let rmq = RMQ::new(&a, std::cmp::min);
    for _ in 0..q {
        input! {
            le: usize,
            ri: usize,
        }
        println!("{}", rmq.query(le..ri));
    }
}
