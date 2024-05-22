// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B

use proconio::input;
use programming_team_code_rust::data_structures::fenwick::Fenwick;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut fenwick = Fenwick::<usize>::new(n);
    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            0 => {
                input! {
                i: usize,
                x: usize,
                }
                fenwick.add(i - 1, x);
            }
            1 => {
                input! {
                le: usize,
                ri: usize,
                }
                println!("{}", fenwick.sum(le - 1..ri));
            }
            _ => unreachable!(),
        }
    }
}
