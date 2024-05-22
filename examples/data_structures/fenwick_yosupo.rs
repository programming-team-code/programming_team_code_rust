// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use proconio::input;
use programming_team_code_rust::data_structures::fenwick::Fenwick;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut fenwick = Fenwick::<u64>::new(n);
    for i in 0..n {
        input! {
            x: u64,
        }
        fenwick.add(i, x);
    }

    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            0 => {
                input! {
                    p: usize,
                    x: u64,
                }
                fenwick.add(p, x);
            }
            1 => {
                input! {
                    le: usize,
                    ri: usize,
                }
                println!("{}", fenwick.sum(le..ri));
            }
            _ => unreachable!(),
        }
    }
}
