//! # Manacher's algorithm

/// See <https://codeforces.com/blog/entry/12143#comment-324162>
///
/// subarray [le, ri] has "center" i = le + ri
///
/// center 0 2 4 6 8
/// string b a a b a
/// center  1 3 5 7
///
/// man = {0, 1, 1, 0, 2, 3, 2, 4, 4}
///
/// # Example
/// ```
/// use programming_team_code_rust::strings::manacher::manacher;
///
/// assert_eq!(manacher(&"baaba".chars().collect::<Vec<_>>()), vec![0, 1, 1, 0, 2, 3, 2, 4, 4]);
/// ```
///
/// # Panics
/// ```panic
/// use programming_team_code_rust::strings::manacher::manacher;
/// ma nacher(&"".chars().collect::<Vec<_>>());
/// ```
///
/// # Complexity (n = s.len())
/// - Time: O(n)
/// - Space: O(n)
pub fn manacher<T: std::cmp::PartialEq>(s: &[T]) -> Vec<usize> {
    let n = s.len();
    let mut p = 0;
    let mut man = vec![0; 2 * n - 1];
    for i in 0..2 * n - 1 {
        let mut ri = if i <= 2 * (p - man[p]) {
            p - man[p].max(man[2 * p - i])
        } else {
            i / 2
        };
        man[i] = i - ri;
        while man[i] > 0 && ri + 1 < n && s[man[i] - 1] == s[ri + 1] {
            man[i] -= 1;
            ri += 1;
            p = i;
        }
    }
    man
}
