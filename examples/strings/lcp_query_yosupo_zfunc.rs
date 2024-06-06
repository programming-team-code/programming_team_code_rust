// verification-helper: PROBLEM https://judge.yosupo.jp/problem/zalgorithm

use proconio::input;
use programming_team_code_rust::strings::suffix_array::SufAry;

fn main() {
    input! {
        s: String
    }

    let s_vec = s.chars().collect::<Vec<char>>();

    let suf_ary = SufAry::new(&s_vec);

    for i in 0..s_vec.len() {
        print!("{} ", suf_ary.len_lcp(0, i));
    }

    println!();
}
