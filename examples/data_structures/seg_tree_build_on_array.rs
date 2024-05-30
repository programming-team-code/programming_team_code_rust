// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_add_range_sum

use proconio::input;
use programming_team_code_rust::data_structures::seg_tree::SegTree;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u64; n]
    }

    let mut seg_tree = SegTree::build_on_array(&a);
    for _ in 0..q {
        input! {
            t: usize
        }

        match t {
            0 => {
                input! {
                    p: usize,
                    x: u64
                }
                seg_tree.update(p..p + 1, x);
            }
            _ => {
                input! {
                    le: usize,
                    ri: usize
                }
                println!("{}", seg_tree.query(le..ri));
            }
        }
    }
}
