// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use proconio::input;
use programming_team_code_rust::data_structures::fenwick::Fenwick;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        queries: [(usize, usize, usize); q],
    }

    let mut fenwick = Fenwick::<u64>::new(n);
    for (i, &elem) in a.iter().enumerate() {
        fenwick.add(i, elem as u64);
    }

    for que in queries {
        if que.0 == 0 {
            fenwick.add(que.1, que.2 as u64);
        } else {
            println!("{}", fenwick.sum(que.1..que.2));
        }
    }
}
