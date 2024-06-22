// verification-helper: PROBLEM https://judge.yosupo.jp/problem/longest_increasing_subsequence

use proconio::input;
use programming_team_code_rust::helpers::lis::Lis;

fn main() {
    input! {
        n: usize,
        a: [i64; n]
    }

    let mut lis = Lis::default();

    for &elem in &a {
        lis.push(elem);
    }

    let idxs = lis.get_lis();

    println!("{}", idxs.len());
    println!(
        "{}",
        idxs.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
