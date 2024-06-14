// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_1_B

use proconio::input;
use programming_team_code_rust::numbers::extended_gcd::extended_gcd;

fn main() {
    input! {
        a: i64,
        b: i64,
    }
    let (gcd, x, y) = extended_gcd(a, b);
    assert_eq!(x * a + y * b, gcd);
    println!("{}", gcd);
}
