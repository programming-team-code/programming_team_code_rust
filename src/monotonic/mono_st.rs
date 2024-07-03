//! # Monotonic Stack

/// # Example
/// ```
/// use programming_team_code_rust::monotonic::mono_st::mono_st;
///
/// let a: Vec<u32> = vec![3, 1, 2, 2];
/// assert_eq!(mono_st(&a, |x, y| x.lt(y)), [0, 0, 2, 2]);
/// assert_eq!(mono_st(&a, |x, y| x.le(y)), [0, 0, 2, 3]);
/// assert_eq!(mono_st(&a, |x, y| x.gt(y)), [0, 1, 1, 1]);
/// assert_eq!(mono_st(&a, |x, y| x.ge(y)), [0, 1, 1, 3]);
///
/// let le = mono_st(&a, |x, y| x.lt(y));
/// let mut iterations = 0;
/// for i in 0..a.len() {
///    let mut j = i;
///    while j != le[i] {
///       iterations += 1;
///       // !cmp(a[k], a[j]) is true for all k in [le[j - 1], j)
///       // cmp(a[le[j - 1] - 1], a[j]) is true
///       j = le[j - 1];
///    }
/// }
/// let mut j = n;
/// while le[i] != le[j] {
///    iterations += 1;
///    j = le[j - 1];
/// }
/// assert_eq!(iterations, a.len());
/// ```
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(n)
pub fn mono_st<T: Ord, F: Fn(&T, &T) -> bool>(a: &[T], cmp: F) -> Vec<usize> {
    let mut le = vec![0; a.len()];
    for i in 0..a.len() {
        le[i] = i.wrapping_sub(1);
        while le[i] != usize::MAX && !cmp(&a[le[i]], &a[i]) {
            le[i] = le[le[i]];
        }
    }
    le
}
