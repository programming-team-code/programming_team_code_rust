//! # Cartesian tree

/// Gets Vec p where p\[i\] = parent of i in cartesian tree, or usize::MAX for root
///
////// # Example
/// ```
/// use programming_team_code_rust::monotonic::mono_st::mono_st;
/// use programming_team_code_rust::monotonic::cartesian_tree::cart_tree;
///
/// // when cmp is lt:
/// let a = [2, 1, 3, 1, 1, 0, 2, 2, 1, 0, 2];
/// //      (---------------------------x---)
/// //      (---------------x---------)   (x)
/// //      (------------x) | (------x)    |
/// //      (---------x)    | (---x) |     |
/// //      (---x---)       | (x) |  |     |
/// //      (x) | (x)       |  |  |  |     |
/// //       |  |  |        |  |  |  |     |
/// //       0  1  2  3  4  5  6  7  8  9  10
///
/// let le = mono_st(&a, |x, y| x.lt(y)); // lt -> right-most min is root
/// let p = cart_tree(&le);               // le -> left-most min is root
///                                       // gt -> right-most max is root
///                                       // ge -> left-most max is root
///
/// assert_eq!(p, [1, 3, 1, 4, 5, 9, 7, 8, 5, usize::MAX, 9]);
/// ```
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(n)
pub fn cart_tree(le: &[usize]) -> Vec<usize> {
    let mut p = le.to_vec();
    for i in 0..p.len() {
        let mut j = i.wrapping_sub(1);
        while j != le[i] {
            if le[j] == le[i] {
                p[j] = i;
            }
            j = le[j];
        }
    }
    p
}
