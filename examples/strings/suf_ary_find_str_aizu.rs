// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B

use proconio::input;
use programming_team_code_rust::strings::suf_ary::SufAry;

mod suf_ary_push_pop_char_asserts;
use suf_ary_push_pop_char_asserts::suf_ary_push_pop_char_asserts;

fn main() {
    input! {
        s: String,
        t: String
    }

    let s = s.chars().map(|x| x as usize).collect::<Vec<usize>>();
    let t = t.chars().map(|x| x as usize).collect::<Vec<usize>>();

    let suf_ary = SufAry::new(&s, 255);
    let range = suf_ary.find_str(&t);

    suf_ary_push_pop_char_asserts(s.len(), &suf_ary, &range, &t);

    let mut res: Vec<usize> = Vec::new();
    res.extend_from_slice(&suf_ary.sa[range]);
    res.sort();

    for val in res {
        println!("{}", val);
    }
}
