// verification-helper: PROBLEM https://judge.yosupo.jp/problem/suffixarray

use proconio::input;
use programming_team_code_rust::strings::suffix_array::SufAry;

fn main() {
    input! {
        s: String
    }

    let s_vec = s.chars().collect::<Vec<char>>();

    let suf_ary = SufAry::new(&s_vec);

    for val in suf_ary.sa {
        print!("{} ", val);
    }
    println!();
}
