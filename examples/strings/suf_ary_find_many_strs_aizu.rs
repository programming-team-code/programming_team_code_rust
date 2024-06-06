// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_D

use proconio::input;
use programming_team_code_rust::strings::suffix_array::SufAry;

fn main() {
    input! {
        s: String,
        q: usize
    }

    let s_vec = s.chars().map(|x| x as usize).collect::<Vec<usize>>();
    let suf_ary = SufAry::new(&s_vec, 255);

    for _ in 0..q {
        input! {
            t: String
        }
        let t_vec = t.chars().map(|x| x as usize).collect::<Vec<usize>>();
        let range = suf_ary.find_str(&t_vec);
        println!("{}", !range.is_empty() as usize);
    }
}
