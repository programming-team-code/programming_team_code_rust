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

    let rmq = RMQ::new(&(0..n).collect::<Vec<_>>(), |i1, i2| {
        if a[i1] < a[i2] {
            i1
        } else {
            i2
        }
    });

    let le = mono_st(&a, |x, y| x.le(y));
    assert_eq!(le, mono_st(&a, |x, y| x.lt(y))); //TODO wait till lib checker PR merges and watch
                                                 //this fail
    let ri = mono_range(&le);

    {
        let mut iterations = 0;
        for (i, &num) in le.iter().enumerate().skip(1) {
            let mut j = i - 1;
            while j != num {
                iterations += 1;
                //TODO: change these to asserts
                //assert!(a[rmq.query(le[j - 1]..j)] >= a[j]);
                //if le[j - 1] > 0 {
                //assert!(a[le[j - 1] - 1] <= a[j]);
                //}
                // !cmp(a[k], a[j]) is true for all k in [le[j - 1], j)
                // cmp(a[le[j - 1] - 1], a[j]) is true
                j = le[j];
            }
        }
        let mut j = n - 1;
        while j != usize::MAX {
            iterations += 1;
            j = le[j];
        }
        assert_eq!(iterations, n);
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }

        let idx_min = rmq.query(l..r);

        assert_eq!(
            a[rmq.query(le[idx_min].wrapping_add(1)..ri[idx_min])],
            a[idx_min]
        );
        if le[idx_min] != usize::MAX {
            assert!(a[le[idx_min]] < a[idx_min]);
        }
        if ri[idx_min] < n {
            assert!(a[ri[idx_min]] < a[idx_min]);
        }

        println!("{}", a[idx_min]);
    }
}
