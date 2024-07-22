// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use proconio::input;
use programming_team_code_rust::data_structures::linear_rmq::LinearRMQ;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let rmq = LinearRMQ::new(&a, |&x, &y| x.lt(&y));
    for _ in 0..q {
        input! {
            le: usize,
            ri: usize,
        }
        let idx_min = rmq.query_idx(le..ri);
        assert!((le..ri).contains(&idx_min));
        let res = rmq.query(le..ri);
        assert_eq!(a[idx_min], *res);
        println!("{}", res);
    }
}
