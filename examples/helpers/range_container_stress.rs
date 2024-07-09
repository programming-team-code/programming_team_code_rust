// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/all/ITP1_1_A

use programming_team_code_rust::helpers::range_container::RangeContainer;
use rand::{thread_rng, Rng};
use std::collections::BTreeMap;

fn main() {
    let mut rng = thread_rng();
    for _ in 0..100 {
        let max_n = rng.gen_range(1..100);
        let mut vis = vec![false; max_n + 1];
        let mut rc = RangeContainer::default();
        for _ in 0..100 {
            let mut le = rng.gen_range(0..max_n);
            let mut ri = rng.gen_range(0..max_n);
            if le > ri {
                std::mem::swap(&mut le, &mut ri);
            }
            ri += 1;
            match rng.gen_range(0..2) {
                0 => {
                    rc.insert_range(le as i32..ri as i32);
                    for i in le..ri {
                        vis[i] = true;
                    }
                }
                _ => {
                    rc.remove_range(le as i32..ri as i32);
                    for i in le..ri {
                        vis[i] = false;
                    }
                }
            }
            let mut naive_mp = BTreeMap::<i32, i32>::new();
            let mut start = None;
            for i in 0..max_n + 1 {
                if vis[i] {
                    if start.is_none() {
                        start = Some(i);
                    }
                } else {
                    if let Some(curr_start) = start {
                        naive_mp.insert(curr_start as i32, i as i32);
                    }
                    start = None;
                }
            }
            assert_eq!(rc.mp, naive_mp);
        }
    }
    println!("Hello World");
}
