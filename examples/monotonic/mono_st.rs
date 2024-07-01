// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use proconio::input;
use programming_team_code_rust::data_structures::rmq::RMQ;
use programming_team_code_rust::monotonic::mono_st::mono_st;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32],
    }

    let le = mono_st(&a);

    let rmq = RMQ::new(&(0..n).collect::<Vec<_>>(), |i1, i2| {
        if a[i1] < a[i2] {
            i1
        } else {
            i2
        }
    });

    for _ in 0..q {
        input! {
            le: usize,
            ri: usize,
        }
        let idx_min = rmq.query(le..ri);

        println!("{}", idx_min);
    }
}
