// verification-helper: PROBLEM https://judge.yosupo.jp/problem/enumerate_palindromes

use proconio::input;
use programming_team_code_rust::strings::suffix_array::SufAry;

fn main() {
    input! {
        s: String
    }

    let mut s_vec = s.chars().collect::<Vec<char>>();
    let n = s_vec.len();
    let s_rev = s_vec.clone().into_iter().rev();

    s_vec.push('$');
    s_vec.extend(s_rev);

    let suf_ary = SufAry::new(&s_vec);

    for i in 0..n {
        for j in i..std::cmp::min(i + 2, n) {
            print!(
                "{} ",
                suf_ary.len_lcp(j, (n - i - 1) + n + 1) * 2 - (i == j) as usize
            );
        }
    }

    println!();
}
