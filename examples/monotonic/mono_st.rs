// verification-helper: PROBLEM https://judge.yosupo.jp/problem/staticrmq

use proconio::input;
use programming_team_code_rust::data_structures::rmq::RMQ;
use programming_team_code_rust::monotonic::mono_range::mono_range;
use programming_team_code_rust::monotonic::mono_st::mono_st;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
    }

    let le = mono_st(&a, |x, y| x.le(y));
    let le1 = mono_st(&a, |x, y| x.lt(y));
    assert_eq!(le, le1);
    let ri = mono_range(&le);

    let rmq = RMQ::new(&(0..n).collect::<Vec<_>>(), |i1, i2| {
        if a[i1] < a[i2] {
            i1
        } else {
            i2
        }
    });

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }

        let idx_min = rmq.query(l..r);

        assert_eq!(a[rmq.query(le[idx_min]..ri[idx_min])], a[idx_min]);
        if le[idx_min] > 0 {
            assert!(a[le[idx_min] - 1] < a[idx_min]);
        }
        if ri[idx_min] < n {
            assert!(a[ri[idx_min]] < a[idx_min]);
        }

        println!("{}", a[idx_min]);
    }
}
