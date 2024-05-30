// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use proconio::input;
use programming_team_code_rust::data_structures::fenwick::Fenwick;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n],
    }

    let mut fenwick = Fenwick::<u64>::build_on_array(&a);

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
            _ => {
                input! {
                    le: usize,
                    ri: usize,
                }
                println!("{}", fenwick.sum(le..ri));
            }
        }
    }
}
