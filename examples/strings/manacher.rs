// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_palindromes

use proconio::input;
use programming_team_code_rust::strings::manacher::manacher;

fn main() {
    input! {
        s: String
    }

    let man = manacher(&s.chars().collect::<Vec<_>>())
        .iter()
        .enumerate()
        .map(|(i, &x)| i - 2 * x + 1)
        .collect::<Vec<_>>();

    println!(
        "{}",
        man.iter()
            .map(|&x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
