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
/// ```
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(n)
pub fn mono_st<T: Ord, F: Fn(&T, &T) -> bool>(a: &[T], cmp: F) -> Vec<usize> {
    let mut le = (0..a.len()).collect::<Vec<_>>();
    for (i, num) in a.iter().enumerate() {
        while le[i] > 0 && !cmp(&a[le[i] - 1], num) {
            le[i] = le[le[i] - 1];
        }
    }
    le
}
