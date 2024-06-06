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

    a.push(1001); //TODO figure out why this is WA, but pushing -1 is AC
    a.extend(b);

    let suf_ary = SufAry::new_arbitrary::<i32>(&a);
    let res = suf_ary.cmp_substrs(0..n, n + 1..n + 1 + m);

    if res == std::cmp::Ordering::Equal {
        assert_eq!(std::cmp::Ordering::Greater, suf_ary.cmp_sufs(0, n + 1));
    } else {
        assert_eq!(res, suf_ary.cmp_sufs(0, n + 1));
    }

    println!("{}", (res == std::cmp::Ordering::Less) as usize);
}
