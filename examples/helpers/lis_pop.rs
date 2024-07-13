// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/all/ITP1_1_A

use programming_team_code_rust::helpers::lis::Lis;

fn lis_quadratic(a: &[i32]) -> usize {
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
    for _ in 0..100 {
        let mut lis = Lis::default();
        let mut a = Vec::new();
        assert_eq!(lis.get_lis(), vec![]);
        for _ in 0..1000 {
            match rand::random::<u8>() % 3 {
                0 => {
                    let new_num = rand::random::<i32>();
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
