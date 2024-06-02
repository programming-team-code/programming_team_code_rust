// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite

use proconio::input;
use programming_team_code_rust::data_structures::seg_tree::SegTree;

fn main() {
    let md = 998244353;

    input! {
        n: usize,
        q: usize
    }

    let a = (0..n)
        .map(|_| {
            input! {
                u: usize,
                v: usize
            }
            (u, v)
        })
        .collect::<Vec<(usize, usize)>>();

    let mut seg_tree = SegTree::<(usize, usize)>::new(n, (1, 0), |x, y| x);

    for _ in 0..q {
        input! {
            t: usize
        }

        match t {
            0 => {
                input! {
                    idx: usize,
                    c: usize,
                    d: usize
                }
                seg_tree.update(idx, (c, d));
            }
            _ => {
                input! {
                    le: usize,
                    ri: usize,
                    x: usize
                }
                let (c, d) = seg_tree.query(le..ri);
                println!("{}", (c * x + d) % md);
            }
        }
    }
}
