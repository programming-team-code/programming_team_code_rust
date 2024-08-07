//! # Count Rectangles

use crate::monotonic::mono_range::mono_range;
use crate::monotonic::mono_st::mono_st;

/// grid is a n-by-m boolean array
///
/// Gets an (n+1)-by-(m+1) vec cnt where cnt\[i\]\[j\] = the number of times an i-by-j subrectangle
/// appears in the grid such that all i*j cells in the subrectangle are true.
///
/// cnt\[i\]\[0\] and cnt\[0\]\[j\] will contain garbage numbers.
///
/// # Example
/// ```
/// use programming_team_code_rust::monotonic::count_rects::count_rects;
///
/// let mut cnt = count_rects(&[vec![true, true, false],
///                             vec![true, true, true]]);
///
/// // remove garbage values
/// cnt.drain(0..1);
/// for i in 0..cnt.len() {
///    cnt[i].drain(0..1);
/// }
///
/// assert_eq!(cnt, [[5, 3, 1],
///                  [2, 1, 0]]);
///
/// assert!(std::panic::catch_unwind(|| count_rects(&[])).is_err());
/// ```
///
/// # Complexity
/// - Time: O(nm)
/// - Space: O(nm)
pub fn count_rects(grid: &[Vec<bool>]) -> Vec<Vec<i32>> {
    let (n, m) = (grid.len(), grid[0].len());
    let mut cnt = vec![vec![0; m + 1]; n + 1];
    let mut h = vec![0; m];
    for row in grid {
        for j in 0..m {
            h[j] = row[j] as usize * (h[j] + 1);
        }
        let le = mono_st(&h, |x, y| x.lt(y));
        let ri = mono_range(&le);
        for j in 0..m {
            let (cnt_l, cnt_r) = (j.wrapping_sub(le[j]), ri[j] - j);
            cnt[h[j]][cnt_l + cnt_r - 1] += 1;
            cnt[h[j]][cnt_l - 1] -= 1;
            cnt[h[j]][cnt_r - 1] -= 1;
        }
    }
    for row in cnt.iter_mut().skip(1) {
        for _ in 0..2 {
            for j in (1..m).rev() {
                row[j] += row[j + 1];
            }
        }
    }
    for i in (1..n).rev() {
        for j in 1..=m {
            cnt[i][j] += cnt[i + 1][j];
        }
    }
    cnt
}
