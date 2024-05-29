// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/3/DSL/all/DSL_2_G

use proconio::input;
use programming_team_code_rust::data_structures::seg_tree::SegTree;

fn main() {
    input! {
        n: usize,
        q: usize
    }

    let mut seg_tree = SegTree::new(n);
    for _ in 0..q {
        input! {
            t: usize
        }

        match t {
            0 => {
                input! {
                    le: usize,
                    ri: usize,
                    delta: u64
                }
                seg_tree.update(le - 1, ri, delta);
            }
            _ => {
                input! {
                    le: usize,
                    ri: usize
                }
                println!("{}", seg_tree.query(le - 1, ri));
            }
        }
    }
}
