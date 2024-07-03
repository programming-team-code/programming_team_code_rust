// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/7/DPL/all/DPL_3_B

use proconio::input;
use programming_team_code_rust::monotonic::count_rects::count_rects;

fn main() {
    input! {
        n: usize,
        m: usize,
        grid: [[u32; m]; n],
    }
    let grid = grid
        .iter()
        .map(|row| row.iter().map(|&num| num == 0).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let cnt = count_rects(&grid);

    {
        let mut pref_sum = vec![vec![0; m + 1]; n + 1];
        for i in 0..n {
            for j in 0..m {
                pref_sum[i + 1][j + 1] =
                    grid[i][j] as i64 + pref_sum[i + 1][j] + pref_sum[i][j + 1] - pref_sum[i][j];
            }
        }
        for len_i in 1..=5.min(n) {
            for len_j in 1..5.min(m) {
                let mut cnt_good_rects = 0;
                for i in 0..=n - len_i {
                    for j in 0..=m - len_j {
                        let cnt_rect = pref_sum[i + len_i][j + len_j]
                            - pref_sum[i + len_i][j]
                            - pref_sum[i][j + len_j]
                            + pref_sum[i][j];
                        if cnt_rect == len_i as i64 * len_j as i64 {
                            cnt_good_rects += 1;
                        }
                    }
                }
                assert_eq!(cnt_good_rects, cnt[len_i][len_j]);
            }
        }
    }

    let mut res = 0;
    for (i, row) in cnt.iter().enumerate().skip(1) {
        for (j, &num) in row.iter().enumerate().skip(1) {
            if num > 0 {
                res = res.max(i as u64 * j as u64);
            }
        }
    }
    println!("{}", res);
}
