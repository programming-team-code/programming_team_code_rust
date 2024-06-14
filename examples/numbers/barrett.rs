// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/ITP1_4_A

use proconio::input;
use programming_team_code_rust::numbers::barrett::Barrett;

fn do_asserts(numerator: u64, denominator: u32) {
    let barr = Barrett::new(denominator);
    let (quot, rem) = barr.div(numerator);
    assert_eq!(quot * denominator as u64 + rem as u64, numerator);
    assert!(rem < denominator);
}

fn main() {
    input! {
        numerator: u32,
        denominator: u32,
    }

    let test_numerators = [
        numerator as u64,
        denominator as u64,
        0_u64,
        1_u64,
        u64::MAX - 35128,
        u64::MAX - 1,
        u64::MAX,
        numerator as u64 * denominator as u64,
        numerator as u64 * numerator as u64,
        denominator as u64 * denominator as u64,
        numerator as u64 * 2_000_000_000_u64,
        denominator as u64 * 2_000_000_000_u64,
    ];

    for test_num in test_numerators {
        do_asserts(test_num, numerator);
        do_asserts(test_num, denominator);
        do_asserts(test_num, u32::MAX / 2);
        do_asserts(test_num, u32::MAX / 2 - 1);
        do_asserts(test_num, u32::MAX / 2 - 2);
    }

    let barr = Barrett::new(denominator);

    let (quot, rem) = barr.div(numerator as u64);
    println!(
        "{} {} {:.8}",
        quot,
        rem,
        (numerator as f64) / (denominator as f64)
    );
}
