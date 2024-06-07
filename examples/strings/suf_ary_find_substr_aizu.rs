// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B

use proconio::input;
use programming_team_code_rust::strings::suffix_array::SufAry;

fn main() {
    input! {
        s: String,
        t: String
    }

    let s_vec = s.chars().map(|x| x as usize).collect::<Vec<usize>>();
    let t_vec = t.chars().map(|x| x as usize).collect::<Vec<usize>>();

    let suf_ary = SufAry::new(&s_vec, 255);

    /*
    for &val in &suf_ary.sa {
        println!("{:?}", &s_vec[val..]);
    }
    */

    let range = suf_ary.find_str(&t_vec);

    //println!("range is {} {}", range.start, range.end);

    let mut res: Vec<usize> = Vec::new();
    res.extend_from_slice(&suf_ary.sa[range]);

    //println!("{:?}", &res);

    res.sort();

    for val in res {
        println!("{}", val);
    }
}
