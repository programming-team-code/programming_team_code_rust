// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/lesson/8/ITP2/all/ITP2_3_D

use proconio::input;
use programming_team_code_rust::helpers::compress::compress;
use programming_team_code_rust::strings::suf_ary::SufAry;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        m: usize,
        b: [i32; m]
    }

    let are_equal = a == b;

    a.push(-1);
    a.extend(b);

    let (cmps, max_val) = compress(&a);
    let suf_ary = SufAry::new(&cmps, max_val);
    let res = suf_ary.cmp_substrs(0..n, n + 1..n + 1 + m);

    if are_equal {
        assert_eq!(res, std::cmp::Ordering::Equal);
        assert_eq!(suf_ary.cmp_sufs(0, n + 1), std::cmp::Ordering::Greater);
        println!("0");
    } else {
        assert_eq!(res, suf_ary.cmp_sufs(0, n + 1));
        println!("{}", (res == std::cmp::Ordering::Less) as usize);
    }
}
