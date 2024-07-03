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
