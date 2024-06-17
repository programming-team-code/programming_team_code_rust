//! # Extended GCD

/// See <https://nor-blog.codeberg.page/posts/2023-11-27-std-exchange-cpp/>
///
/// ext_gcd(a, b) returns (gcd, x, y) such that a * x + b * y == gcd
/// if a and b are coprime, then x is the inverse of a (mod b)
///
/// # Example
/// ```
/// use programming_team_code_rust::numbers::ext_gcd::ext_gcd;
///
/// let (a, b) = (4, -6);
/// let (gcd, x, y) = ext_gcd(a, b);
/// assert_eq!(gcd, -2);
/// assert_eq!(a * x + b * y, gcd);
/// assert_eq!(ext_gcd(0, 0), (0, 1, 0));
/// ```
///
/// - Time: O(log(min(a, b)))
/// - Space: O(1)
pub fn ext_gcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    let (mut x, mut y, mut i, mut j) = (1, 0, 0, 1);
    while b != 0 {
        let quot = a / b;
        (a, b) = (b, a - quot * b);
        (x, i) = (i, x - quot * i);
        (y, j) = (j, y - quot * j);
    }
    (a, x, y)
}
