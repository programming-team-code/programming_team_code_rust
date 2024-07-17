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
                (le, ri) = (ri, le);
            }
            ri += 1;
            match rng.gen_range(0..2) {
                0 => {
                    rc.insert_range(le as i32..ri as i32);
                    for elem in vis.iter_mut().take(ri).skip(le) {
                        *elem = true;
                    }
                }
                _ => {
                    rc.remove_range(le as i32..ri as i32);
                    for elem in vis.iter_mut().take(ri).skip(le) {
                        *elem = false;
                    }
                }
            }
            let mut to_end = vec![None; max_n + 1];
            for i in (0..max_n).rev() {
                if vis[i] && !vis[i + 1] {
                    to_end[i] = Some(i + 1);
                } else if vis[i] {
                    to_end[i] = to_end[i + 1];
                }
            }
            let mut naive_mp = BTreeMap::<i32, i32>::new();
            let mut start = None;
            for i in 0..max_n + 1 {
                if vis[i] {
                    if start.is_none() {
                        start = Some(i);
                    }
                    assert_eq!(
                        rc.get_range(i as i32).unwrap(),
                        start.unwrap() as i32..to_end[i].unwrap() as i32
                    );
                } else {
                    assert_eq!(rc.get_range(i as i32), None);
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
