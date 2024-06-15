// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb

use proconio::input;
use programming_team_code_rust::numbers::mod_int::{Mint, MOD};

fn main() {
    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            a: i64,
            b: i64,
        }

        let a_mod = Mint::new(a);
        let b_mod = Mint::new(b);

        assert_eq!((a_mod + b_mod).val, ((a + b) % MOD as i64) as u32);

        println!("{}", a + b);
    }
}
