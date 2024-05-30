// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_1_C

use proconio::input;
use programming_team_code_rust::numbers::primes::Primes;

fn main() {
    let primes = Primes::new(100_000_001);

    for i in -4..=1 {
        assert_eq!(primes.is_prime(i), False);
    }

    input! {
        n: usize,
    }

    let mut cnt = 0;
    for _ in 0..n {
        input! {
            x: usize,
        }

        if primes.is_prime(x) {
            cnt += 1;
        }
    }

    println!("{}", cnt);
}
