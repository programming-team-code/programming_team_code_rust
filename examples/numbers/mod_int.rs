// verification-helper: PROBLEM https://judge.yosupo.jp/problem/many_aplusb

use proconio::input;
use programming_team_code_rust::numbers::mod_int::{Mint, MOD};

fn main() {
    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            a: u64,
            b: u64,
        }

        let a_mod = Mint::new(a);
        let b_mod = Mint::new(b);

        assert_eq!((a_mod + b_mod).val, (a + b) % MOD);

        println!("{}", a + b);
    }
}
