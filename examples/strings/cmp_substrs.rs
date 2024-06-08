// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/lesson/8/ITP2/all/ITP2_3_D

use proconio::input;
use programming_team_code_rust::strings::suffix_array::SufAry;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        m: usize,
        b: [usize; m]
    }

    let are_equal = a == b;

    a.extend(b);

    let suf_ary = SufAry::new(&a, 1000);
    let res = suf_ary.cmp_substrs(0..n, n..n + m);

    assert_eq!(
        suf_ary.cmp_substrs(n + m..n + m, 0..n),
        std::cmp::Ordering::Less
    );

    if are_equal {
        assert_eq!(res, std::cmp::Ordering::Equal);
        println!("0");
    } else {
        println!(
            "{}",
            (suf_ary.cmp_substrs(0..n, n..n + m) == std::cmp::Ordering::Less) as usize
        );
    }
}
