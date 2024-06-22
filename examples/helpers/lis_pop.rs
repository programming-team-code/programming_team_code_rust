// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/all/ITP1_1_A

use programming_team_code_rust::helpers::lis::Lis;
use programming_team_code_rust::helpers::random::Random;

fn lis_quadratic(a: &[i64]) -> usize {
    let n = a.len();
    if n == 0 {
        return 0;
    }
    let mut dp = vec![1; n];
    for i in 0..n {
        for j in 0..i {
            if a[j] < a[i] {
                dp[i] = dp[i].max(1 + dp[j]);
            }
        }
    }
    *dp.iter().max().unwrap()
}

fn main() {
    let mut rng = Random::new(12345);
    for _ in 0..10 {
        let mut lis = Lis::default();
        let mut a = Vec::new();
        for _ in 0..100 {
            match rng.next_in_range(0..3) {
                0 => {
                    let new_num = rng.next_in_range(-10..10);
                    lis.push(new_num);
                    a.push(new_num);
                }
                1 => {
                    if !a.is_empty() {
                        lis.pop();
                        a.pop();
                    }
                }
                _ => {
                    let idxs = lis.get_lis();

                    assert_eq!(idxs.len(), lis_quadratic(&a));
                    for i in 1..idxs.len() {
                        assert!(a[idxs[i - 1]] < a[idxs[i]]);
                    }
                }
            }
        }
    }

    println!("Hello World");
}
