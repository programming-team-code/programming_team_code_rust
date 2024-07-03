//! # Count Rectangles

use crate::monotonic::mono_range::mono_range;
use crate::monotonic::mono_st::mono_st;

pub fn count_rects(grid: &[Vec<bool>]) -> Vec<Vec<i64>> {
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
            let (cnt_l, cnt_r) = (j - le[j].wrapping_add(1), ri[j] - j - 1);
            cnt[h[j]][cnt_l + cnt_r + 1] += 1;
            cnt[h[j]][cnt_l] -= 1;
            cnt[h[j]][cnt_r] -= 1;
        }
    }
    for row in cnt.iter_mut().skip(1) {
        for _ in 0..2 {
            for j in (2..=m).rev() {
                row[j - 1] += row[j];
            }
        }
    }
    for i in (2..=n).rev() {
        for j in 1..=m {
            cnt[i - 1][j] += cnt[i][j];
        }
    }
    cnt
}
