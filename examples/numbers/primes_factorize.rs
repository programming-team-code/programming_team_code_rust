// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/library/6/NTL/all/NTL_1_C

use proconio::input;
use programming_team_code_rust::numbers::primes::Primes;

fn main() {
    input! {
        n: usize,
    }

    let primes = Primes::new(1001);
    let mut lcm_exps = vec![0; 1001];

    for _ in 0..n {
        input! {
            x: usize,
        }

        let mut exps = vec![0; 1001];
        let mut prev_factor = 0;

        for factor in primes.factorize(x) {
            assert!(prev_factor <= factor);
            exps[factor] += 1;
            lcm_exps[factor] = lcm_exps[factor].max(exps[factor]);
            prev_factor = factor;
        }
    }

    let mut lcm = 1;
    for i in 2i32..1001 {
        lcm *= i.pow(lcm_exps[i as usize]);
    }

    println!("{}", lcm);
}
