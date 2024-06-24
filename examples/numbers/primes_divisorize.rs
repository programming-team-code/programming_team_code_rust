// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/all/ITP1_1_A

use programming_team_code_rust::numbers::primes::Primes;

fn main() {
    let n = 100_000;
    let mut divs = vec![vec![]; n + 1];
    for i in 1..=n {
        for j in 1..=n / i {
            divs[i * j].push(i);
        }
    }

    let primes = Primes::new(n + 1);

    #[allow(clippy::needless_range_loop)]
    for i in 1..=n {
        let mut all_divisors = primes.divisorize(i);
        all_divisors.sort();
        assert_eq!(all_divisors, divs[i]);
    }

    println!("Hello World");
}
