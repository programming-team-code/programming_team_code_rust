// verification-helper: PROBLEM https://judge.yosupo.jp/problem/deque_operate_all_composite

use proconio::input;
use programming_team_code_rust::data_structures::deque::Deque;
use std::collections::VecDeque;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        q: usize,
    }

    let mut std_deq = VecDeque::new();
    let mut deq =
        Deque::new(|&i: &(u64, u64), &j: &(u64, u64)| (i.0 * j.0 % MOD, (j.0 * i.1 + j.1) % MOD));

    for _ in 0..q {
        input! {
            t: u8,
        }
        match t {
            0 => {
                input! {
                    a: u64,
                    b: u64,
                }
                deq.push_front((a, b));
                std_deq.push_front((a, b));
            }
            1 => {
                input! {
                    a: u64,
                    b: u64,
                }
                deq.push_back((a, b));
                std_deq.push_back((a, b));
            }
            2 => {
                deq.pop_front();
                std_deq.pop_front();
            }
            3 => {
                deq.pop_back();
                std_deq.pop_back();
            }
            _ => {
                input! {
                    x: u64,
                }
                if let Some((a, b)) = deq.query() {
                    println!("{}", (a * x + b) % MOD);
                } else {
                    println!("{}", x);
                }
            }
        }
    }
}
