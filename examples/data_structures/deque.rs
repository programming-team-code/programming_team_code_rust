// verification-helper: PROBLEM https://judge.yosupo.jp/problem/deque_operate_all_composite

use proconio::input;
use programming_team_code_rust::data_structures::deque::Deque;
use rand::{thread_rng, Rng};
use std::collections::VecDeque;

const MOD: u64 = 998_244_353;

fn main() {
    let mut rng = thread_rng();
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
        assert_eq!(deq.len(), std_deq.len());
        assert_eq!(deq.front(), std_deq.front());
        assert_eq!(deq.back(), std_deq.back());
        if !std_deq.is_empty() {
            for _ in 0..3 {
                let i = rng.gen_range(0..deq.len());
                assert_eq!(deq[i], std_deq[i]);
            }
        }
    }
}
