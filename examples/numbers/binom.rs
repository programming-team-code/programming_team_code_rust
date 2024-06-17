// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/DPL_5_E

use proconio::input;
use programming_team_code_rust::numbers::binom::Binom;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut binom = Binom::default();
    println!("{}", binom.comb(k, n));
}
