// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/DSL_2_D

use proconio::input;
use programming_team_code_rust::helpers::range_container::RangeContainer;

fn main() {
    input! {
        n: usize,
        q: usize,
    }
    let mut rc = RangeContainer::default();
    let mut to_value = vec![i32::MAX; 2 * n];
    rc.insert_range(0..(2 * n - 1) as i32);

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
                let key = rc.mp.range(..=ri + 2).next_back().unwrap().0;
                let save_value = to_value[*key as usize];
                //println!("hi, save val {}", save_value);
                rc.remove_range(le - 1..ri + 2);
                rc.insert_range(le..ri + 1);
                let key = rc.mp.range(..=ri + 2).next_back().unwrap().0;
                to_value[*key as usize] = save_value;
                to_value[le as usize] = x;

                /*
                println!("hi there");
                for (key, val) in &rc.mp {
                    println!("key, val: {} {}", key, val);
                }
                print!(" now the array: ");
                for &val in &to_value {
                    print!("{} ", val);
                }
                println!();
                */
            }
            _ => {
                input! {
                    index: i32,
                }
                let key = rc.mp.range(..=2 * index).next_back().unwrap().0;
                println!("{}", to_value[*key as usize]);
            }
        }
    }
}
