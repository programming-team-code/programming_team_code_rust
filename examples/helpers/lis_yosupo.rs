// verification-helper: PROBLEM https://judge.yosupo.jp/problem/longest_increasing_subsequence

use proconio::input;
use programming_team_code_rust::helpers::lis::Lis;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut lis = Lis::default();

    let mut prev = vec![None; n];
    for (i, &elem) in a.iter().enumerate() {
        prev[i] = lis.push(elem);
    }

    println!("{}", lis.dp.len());
    let mut idx = lis.dp.last().unwrap().1;
    let mut res = Vec::with_capacity(lis.dp.len());
    res.push(idx);
    while let Some(prev_idx) = prev[idx] {
        idx = prev_idx;
        res.push(idx);
    }
    res.reverse();
    for idx in res {
        print!("{} ", idx);
    }
}
