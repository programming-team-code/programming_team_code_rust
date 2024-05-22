// verification-helper: PROBLEM https://judge.yosupo.jp/problem/binomial_coefficient_prime_mod

use proconio::input;
use programming_team_code_rust::numbers::binom::Binom;

fn main() {
    input! {
        t: usize,
        m: u64,
    }
    let mut binom = Binom::new(m);
    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
        }
        println!("{}", binom.comb(n, k));
    }
}
