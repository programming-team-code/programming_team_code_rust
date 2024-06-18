// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B

use proconio::input;
use programming_team_code_rust::strings::suf_ary::SufAry;

fn main() {
    input! {
        s: String,
        t: String
    }

    let n = s.chars().count();
    let m = t.chars().count();
    let mut both = s.chars().map(|x| x as usize).collect::<Vec<usize>>();
    both.extend(t.chars().map(|x| x as usize).collect::<Vec<usize>>());

    let suf_ary = SufAry::new(&both, 255);
    assert_eq!(suf_ary.find_substr(n + m..n + m), 0..n + m);
    let range = suf_ary.find_substr(n..n + m);
    let mut res = suf_ary.sa[range]
        .iter()
        .copied()
        .filter(|&i| i + m <= n)
        .collect::<Vec<usize>>();
    res.sort();

    for val in res {
        println!("{}", val);
    }
}
