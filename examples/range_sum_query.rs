// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B

use proconio::input;
use programming_team_code_rust::data_structures::fenwick::Fenwick;

fn main() {
    input! {
        n: usize,
        q: usize,
        queries: [(usize, usize, usize); q],
    }

    let mut fenwick = Fenwick::<usize>::new(n);
    for que in queries {
        if que.0 == 0 {
            fenwick.add(que.1 - 1, que.2);
        } else {
            println!("{}", fenwick.sum(que.1 - 1..que.2));
        }
    }
}
