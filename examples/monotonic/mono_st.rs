// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use proconio::input;
use programming_team_code_rust::monotonic::mono_st::mono_st;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32],
    }
    let le = mono_st(&a);
}
