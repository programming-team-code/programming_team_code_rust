// verification-helper: PROBLEM https://judge.yosupo.jp/problem/point_set_range_composite

use proconio::input;
use programming_team_code_rust::data_structures::seg_tree::SegTree;

fn main() {
    input! {
        n: usize,
        q: usize
    }

    let a = (0..n)
        .map(|_| {
            input! {
                c: usize,
                d: usize
            }
            (c, d)
        })
        .collect::<Vec<(usize, usize)>>();

    let mut seg_tree = SegTree::<(usize, usize)>::build_on_array(
        &a,
        move |x, y| (x.0 * y.0 % 998244353, (y.0 * x.1 + y.1) % 998244353),
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
                    c: usize,
                    d: usize
                }
                seg_tree.set(idx, (c, d));
            }
            _ => {
                input! {
                    le: usize,
                    ri: usize,
                    x: usize
                }
                let (c, d) = seg_tree.query(le..ri);
                println!("{}", (c * x + d) % 998244353);
            }
        }
    }
}
