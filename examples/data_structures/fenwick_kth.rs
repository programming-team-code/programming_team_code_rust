// verification-helper: PROBLEM https://judge.yosupo.jp/problem/predecessor_problem

use proconio::input;
use programming_team_code_rust::data_structures::fenwick::Fenwick;

fn main() {
    input! {
        n: usize,
        q: usize,
        s: String,
    }

    let mut fenwick =
        Fenwick::<i32>::build_on_array(&s.chars().map(|c| (c == '1') as i32).collect::<Vec<_>>());

    for _ in 0..q {
        input! {
            t: u8,
            k: usize,
        }
        match t {
            0 => {
                if fenwick.sum(k..k + 1) == 0 {
                    fenwick.add(k, 1);
                }
            }
            1 => {
                if fenwick.sum(k..k + 1) == 1 {
                    fenwick.add(k, -1);
                }
            }
            2 => {
                println!("{}", fenwick.sum(k..k + 1));
            }
            3 => {
                let cnt = fenwick.sum(0..k);
                let res = fenwick.kth(cnt + 1) - 1;
                if res == n {
                    println!("-1");
                } else {
                    println!("{}", res);
                }
            }
            _ => {
                let cnt_le = fenwick.sum(0..k + 1);
                println!("{}", fenwick.kth(cnt_le) as i32 - 1);
            }
        }
    }
}
