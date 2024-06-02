// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=DSL_2_B

use proconio::input;
use programming_team_code_rust::data_structures::seg_tree::SegTree;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut st = SegTree::<usize>::new(n, |x, y| x + y, 0);
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
                st.update(i - 1, st.query(i - 1..i) + x);
            }
            _ => {
                input! {
                    le: usize,
                    ri: usize,
                }
                println!("{}", st.query(le - 1..ri));
            }
        }
    }
}
