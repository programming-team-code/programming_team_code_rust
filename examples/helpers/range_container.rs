// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/DSL_2_D

use proconio::input;
use programming_team_code_rust::helpers::range_container::RangeContainer;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut rc = RangeContainer::default();
    let mut to_value = BTreeMap::<i32, u32>::new();
    rc.insert_range(0..n as i32);
    to_value.insert(0, 0);

    for _ in 0..q {
        input! {
            kind: usize,
        }
        match kind {
            0 => {
                input! {
                    le: i32,
                    ri: i32,
                    x: u32,
                }
                rc.remove_range(le - 1..ri + 1);
                rc.insert_range(le..ri);
                to_value.insert(le, x);
            }
            _ => {
                input! {
                    index: i32,
                }
                let key = rc.mp.range(..=index).next_back().unwrap().0;
                println!("{}", to_value.get(key).unwrap());
            }
        }
    }
}
