//! # Monotonic Stack

/// # Example
/// ```
/// use programming_team_code_rust::monotonic::mono_st::mono_st;
///
/// let a: Vec<u32> = vec![3, 1, 2, 2];
/// let n = a.len();
/// assert_eq!(mono_st(&a, |x, y| x.lt(y)), [usize::MAX, usize::MAX, 1, 1]);
/// assert_eq!(mono_st(&a, |x, y| x.le(y)), [usize::MAX, usize::MAX, 1, 2]);
/// assert_eq!(mono_st(&a, |x, y| x.gt(y)), [usize::MAX, 0, 0, 0]);
/// assert_eq!(mono_st(&a, |x, y| x.ge(y)), [usize::MAX, 0, 0, 2]);
///
/// let le = mono_st(&a, |x, y| x.lt(y));
/// let mut seen_index = vec![0; n];
/// for i in 0..n {
///    let mut j = i.wrapping_sub(1);
///    while j != le[i] {
///       seen_index[j] += 1;
///       j = le[j];
///    }
/// }
///
/// // clear the stack at the end
/// let mut j = n.wrapping_sub(1);
/// while j != usize::MAX {
///    seen_index[j] += 1;
///    j = le[j];
/// }
/// assert_eq!(seen_index, vec![1; n]);
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
