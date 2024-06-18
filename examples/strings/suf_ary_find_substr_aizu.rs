// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B

use proconio::input;
use programming_team_code_rust::strings::suf_ary::SufAry;

fn main() {
    input! {
        s: String,
        t: String
    }

    let m = t.chars().count();
    let mut both = t.chars().map(|x| x as usize).collect::<Vec<usize>>();
    both.extend(s.chars().map(|x| x as usize).collect::<Vec<usize>>());

    let suf_ary = SufAry::new(&both, 255);
    let mut res = suf_ary.sa[suf_ary.find_substr(0..m)]
        .iter()
        .copied()
        .filter(|&i| i >= m)
        .map(|i| i - m)
        .collect::<Vec<usize>>();
    res.sort();

    for val in res {
        println!("{}", val);
    }
}
