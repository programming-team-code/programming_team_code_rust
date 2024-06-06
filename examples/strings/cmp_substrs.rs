// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/lesson/8/ITP2/all/ITP2_5_A

use proconio::input;
use programming_team_code_rust::strings::suffix_array::SufAry;

fn main() {
    input! {
        n: usize,
        a: [i32; 2 * n]
    }

    let suf_ary = SufAry::new_arbitrary::<i32>(&a);

    let mut indexes = (0..n).collect::<Vec<usize>>();

    indexes.sort_by(|&i1, &i2| suf_ary.cmp_substrs(2 * i1..2 * i1 + 2, 2 * i2..2 * i2 + 2));

    for i in indexes {
        println!("{} {}", a[2 * i], a[2 * i + 1]);
    }
}
