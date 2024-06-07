//! # Value Compression

/// # Example
/// ```
/// use programming_team_code_rust::helpers::compress::compress;
///
/// let a_and_b = [-4, 3, 100, 3, 0, -1];
/// let (mut a, max_val) = compress(&a_and_b);
/// let b = a.split_off(3);
/// assert_eq!(a, [0, 3, 4]);
/// assert_eq!(b, [3, 2, 1]);
/// assert_eq!(max_val, 5);
/// ```
pub fn compress<T: Ord>(a: &[T]) -> (Vec<usize>, usize) {
    let n = a.len();
    let mut idx = (0..n).collect::<Vec<usize>>();
    idx.sort_by_key(|&i| &a[i]);
    let mut compressed = vec![0; n];
    let mut max_val = 0;
    for i in 0..n {
        compressed[idx[i]] = max_val;
        if i + 1 == n || a[idx[i]] != a[idx[i + 1]] {
            max_val += 1;
        }
    }
    (compressed, max_val)
}
