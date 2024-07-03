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
        let mut count_index = vec![0; n];
        let mut do_asserts = |j: usize| {
            count_index[j] += 1;
            let range = le[j].wrapping_add(1)..j;
            if !range.is_empty() {
                assert!(a[rmq.query(range)] >= a[j]);
            }
            if le[j] != usize::MAX {
                assert!(a[le[j]] < a[j]);
            }
        };
        for i in 0..n {
            let mut j = i.wrapping_sub(1);
            while j != le[i] {
                do_asserts(j);
                j = le[j];
            }
        }
        let mut j = n.wrapping_sub(1);
        while j != usize::MAX {
            do_asserts(j);
            j = le[j];
        }
        assert_eq!(count_index, vec![1; n]);
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
