// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/ITP1_4_A

use proconio::input;
use programming_team_code_rust::numbers::barrett::Barrett;

fn main() {
    input! {
        a: u32,
        b: u32,
    }
    let barr = Barrett::new(b);

    let (quot, rem) = barr.div(a as u64);

    println!("{} {} {:.8}", quot, rem, (a as f64) / (b as f64));
}
