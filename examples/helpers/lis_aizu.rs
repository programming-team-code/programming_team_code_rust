// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/DPL_1_D

use proconio::input;
use programming_team_code_rust::helpers::lis::Lis;

fn main() {
    input! {
        n: usize,
        a: [u32; n]
    }

    let mut lis = Lis::default();

    assert_eq!(lis.get_lis(), vec![]);

    for &elem in &a {
        lis.push(elem);
    }

    let idxs = lis.get_lis();
    for i in 1..idxs.len() {
        assert!(a[idxs[i - 1]] < a[idxs[i]]);
    }

    println!("{}", idxs.len());
}
