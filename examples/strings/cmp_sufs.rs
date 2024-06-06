// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/lesson/8/ITP2/all/ITP2_3_D

use proconio::input;
use programming_team_code_rust::strings::suffix_array::SufAry;

fn main() {
    input! {
        n: usize,
        mut a: [i32; n],
        m: usize,
        b: [i32; m]
    }

    a.extend(b);

    let suf_ary = SufAry::new_arbitrary::<i32>(&a);
    println!("{}", (suf_ary.cmp_substrs(0..n, n..n + m) == std::cmp::Ordering::Less) as usize);
}
