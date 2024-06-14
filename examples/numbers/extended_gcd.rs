// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_1_B

use proconio::input;
use programming_team_code_rust::numbers::extended_gcd::ext_gcd;

fn do_asserts(a: i64, b: i64) {
    let (gcd, x, y) = ext_gcd(a, b);
    assert_eq!(a as i128 * x as i128 + b as i128 * y as i128, gcd as i128);
    if gcd != 0 {
        assert_eq!(a % gcd, 0);
        assert_eq!(b % gcd, 0);
    }
    if b != 0 {
        assert!(-(b / gcd).abs() < x);
        assert!(x < (b / gcd).abs());
    }
}

fn main() {
    input! {
        a: i64,
        b: i64,
    }

    let vals = [
        a,
        b,
        -a,
        -b,
        0_i64,
        1_000_000_000_000_000_000_i64,
        i64::MAX / 2,
        i64::MIN / 2,
    ];
    for &val_one in &vals {
        for &val_two in &vals {
            do_asserts(val_one, val_two);
        }
    }

    println!("{}", ext_gcd(a, b).0);
}
