// verification-helper: PROBLEM https://judge.yosupo.jp/problem/queue_operate_all_composite

use proconio::input;
use programming_team_code_rust::data_structures::disjoint_rmq::DisjointRMQ;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        q: usize,
    }

    let mut que = Vec::new();
    let mut front = 0;
    let mut queries = Vec::new();

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
                que.push((a, b));
            }
            1 => {
                front += 1;
            }
            _ => {
                input! {
                    x: u64,
                }
                queries.push((front..que.len(), x));
            }
        }
    }

    let d_rmq = DisjointRMQ::new(&que, |&a, &b| (a.0 * b.0 % MOD, (b.0 * a.1 + b.1) % MOD));

    for (range, x) in queries {
        if range.is_empty() {
            println!("{}", x);
        } else {
            let (slope, y_int) = d_rmq.query(range);
            println!("{}", (slope * x + y_int) % MOD);
        }
    }
}
