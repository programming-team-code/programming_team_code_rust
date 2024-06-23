// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/DPL_1_D

use proconio::input;
use programming_team_code_rust::helpers::lis::Lis;

fn main() {
    input! {
        n: usize,
        a: [u32; n]
    }

    let mut lis = Lis::default();

    for elem in a {
        lis.push(elem);
    }

    println!("{}", lis.dp.len());
}
