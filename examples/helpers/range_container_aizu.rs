// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/DSL_2_D

use proconio::input;
use programming_team_code_rust::helpers::range_container::RangeContainer;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut rc = RangeContainer::default();
    let mut to_value = vec![i32::MAX; 2 * n + 2];
    rc.insert_range(0..(2 * n + 1) as i32);

    for _ in 0..q {
        input! {
            kind: usize,
        }
        match kind {
            0 => {
                input! {
                    le: i32,
                    ri: i32,
                    x: i32,
                }
                let le = 2 * le;
                let ri = 2 * ri;
                let save_range = rc.get_range(ri + 2).unwrap();
                let save_value = to_value[save_range.start as usize];
                rc.remove_range(le - 1..ri + 2);
                rc.insert_range(le..ri + 1);
                let save_range = rc.get_range(ri + 2).unwrap();
                to_value[save_range.start as usize] = save_value;
                to_value[le as usize] = x;
            }
            _ => {
                input! {
                    index: i32,
                }
                let range_containing = rc.get_range(2 * index).unwrap();
                println!("{}", to_value[range_containing.start as usize]);
            }
        }
    }
}
