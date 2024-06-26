// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B

use proconio::input;
use programming_team_code_rust::strings::suf_ary::SufAry;

fn main() {
    input! {
        s: String,
        t: String
    }

    let s = s.chars().map(|x| x as usize).collect::<Vec<usize>>();
    let t = t.chars().map(|x| x as usize).collect::<Vec<usize>>();

    let suf_ary = SufAry::new(&s, 255);

    let mut range = 0..s.len();
    for (i, &c) in t.iter().enumerate() {
        range = suf_ary.push_back_char(c, range, i);
        if range.is_empty() {
            break;
        }
    }
    let mut res: Vec<usize> = Vec::new();
    res.extend_from_slice(&suf_ary.sa[range]);
    res.sort();

    for val in res {
        println!("{}", val);
    }
}
