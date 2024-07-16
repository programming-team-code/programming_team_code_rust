// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite

use proconio::input;
use programming_team_code_rust::data_structures::seg_tree::SegTree;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        q: usize
    }

    let a = (0..n)
        .map(|_| {
            input! {
                c: u64,
                d: u64
            }
            (c, d)
        })
        .collect::<Vec<(u64, u64)>>();

    let mut seg_tree = SegTree::build_on_array(
        &a,
        |x, y| (x.0 * y.0 % MOD, (y.0 * x.1 + y.1) % MOD),
        (1, 0),
    );

    for _ in 0..q {
        input! {
            t: usize
        }

        match t {
            0 => {
                input! {
                    idx: usize,
                    c: u64,
                    d: u64
                }
                seg_tree.set(idx, (c, d));
            }
            _ => {
                input! {
                    le: usize,
                    ri: usize,
                    x: u64
                }
                let (c, d) = seg_tree.query(le..ri);
                println!("{}", (c * x + d) % MOD);
            }
        }
    }
}
