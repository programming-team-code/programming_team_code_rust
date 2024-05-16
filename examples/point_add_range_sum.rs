// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use proconio::input;
use programming_team_code_rust::fenwick::Fenwick;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
        queries: [(usize, usize, usize); q],
    }

    let mut fenwick = Fenwick::<u64>::new(n);
    for i in 0..n {
        fenwick.add(i, a[i] as u64);
    }

    for que in queries {
        if que.0 == 0 {
            fenwick.add(que.1, que.2 as u64);
        } else {
            println!("{}", fenwick.sum(que.1..que.2));
        }
    }
}
