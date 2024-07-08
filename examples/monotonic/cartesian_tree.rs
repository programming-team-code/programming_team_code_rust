// verification-helper: PROBLEM https://judge.yosupo.jp/problem/cartesian_tree

use proconio::input;
use programming_team_code_rust::monotonic::cartesian_tree::cart_tree;
use programming_team_code_rust::monotonic::mono_st::mono_st;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let le = mono_st(&a, |x, y| x.lt(y));
    let p = cart_tree(&le);

    println!(
        "{}",
        p.iter()
            .enumerate()
            .map(|(i, &x)| if x == usize::MAX {
                i.to_string()
            } else {
                x.to_string()
            })
            .collect::<Vec<_>>()
            .join(" ")
    );
}
