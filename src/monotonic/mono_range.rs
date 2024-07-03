//! # Monotonic Range

/// Gets vec ri where ri[i] = smallest index such that i < ri[i] && !a[i].cmp(&a[ri[i]]), or n
///
/// # Example
/// ```
/// use programming_team_code_rust::monotonic::mono_st::mono_st;
/// use programming_team_code_rust::monotonic::mono_range::mono_range;
///
/// let a: Vec<u32> = vec![3, 1, 2, 2];
///
/// let le = mono_st(&a, |x, y| x.lt(y));
/// let ri = mono_range(&le);
/// assert_eq!(ri, [1, 4, 3, 4]);
/// ```
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(n)
pub fn mono_range(le: &[usize]) -> Vec<usize> {
    let mut ri = vec![le.len(); le.len()];
    for i in 0..le.len() {
        let mut j = i.wrapping_sub(1);
        while j != le[i] {
            ri[j] = i;
            j = le[j];
        }
    }
    ri
}
