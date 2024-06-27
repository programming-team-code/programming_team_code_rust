// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_D

use proconio::input;
use programming_team_code_rust::strings::suf_ary::SufAry;

mod suf_ary_push_pop_char_asserts;
use suf_ary_push_pop_char_asserts::suf_ary_push_pop_char_asserts;

fn main() {
    input! {
        s: String,
        q: usize,
    }

    let s_vec = s.chars().map(|x| x as usize).collect::<Vec<usize>>();
    let suf_ary = SufAry::new(&s_vec, 255);

    for _ in 0..q {
        input! {
            t: String
        }
        let t = t.chars().map(|x| x as usize).collect::<Vec<usize>>();
        let range = suf_ary.find_str(&t);
        suf_ary_push_pop_char_asserts(s.len(), &suf_ary, &range, &t);
        println!("{}", !range.is_empty() as usize);
    }
}
