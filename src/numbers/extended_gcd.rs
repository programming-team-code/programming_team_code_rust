//! # Extended GCD

pub fn extended_gcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    let (mut i, mut j, mut x, mut y) = (1, 0, 0, 1);
    while b != 0 {
        let quot = a / b;
        (i, x) = (x, i - quot * x);
        (j, y) = (y, j - quot * y);
        (a, b) = (b, a - quot * b);
    }
    (a, i, j)
}
