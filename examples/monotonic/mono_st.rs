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

    let rmq = RMQ::new(
        &(0..n).map(|i| (i, i)).collect::<Vec<_>>(),
        |(min_i1, max_i1), (min_i2, max_i2)| {
            let min_idx = if a[min_i1] < a[min_i2] {
                min_i1
            } else {
                min_i2
            };
            let max_idx = if a[max_i1] > a[max_i2] {
                max_i1
            } else {
                max_i2
            };
            (min_idx, max_idx)
        },
    );

    let compares = vec![
        |x: u32, y: u32| -> bool { x.le(&y) },
        |x: u32, y: u32| -> bool { x.lt(&y) },
        |x: u32, y: u32| -> bool { x.ge(&y) },
        |x: u32, y: u32| -> bool { x.gt(&y) },
    ];

    let mut le = Vec::new();
    let mut ri = Vec::new();
    for &cmp in &compares {
        le.push(mono_st(&a, |&x, &y| cmp(x, y)));
        ri.push(mono_range(le.last().unwrap()));
    }
    let le = le;
    let ri = ri;

    for curr_le in &le {
        let mut count_index = vec![0; n];
        for i in 0..n {
            let mut j = i.wrapping_sub(1);
            while j != curr_le[i] {
                count_index[j] += 1;
                j = curr_le[j];
            }
        }
        let mut j = n.wrapping_sub(1);
        while j != usize::MAX {
            count_index[j] += 1;
            j = curr_le[j];
        }
        assert_eq!(count_index, vec![1; n]);
    }

    for _ in 0..q {
        input! {
            l: usize,
            r: usize,
        }

        let (min_idx, max_idx) = rmq.query(l..r);

        for i in 0..4 {
            let idx = if i < 2 { min_idx } else { max_idx };
            let rmq_res = rmq.query(le[i][idx].wrapping_add(1)..ri[i][idx]);
            assert_eq!(a[if i < 2 { rmq_res.0 } else { rmq_res.1 }], a[idx]);
            if le[i][idx] != usize::MAX {
                assert!(compares[i](a[le[i][idx]], a[idx]));
            }
            if ri[i][idx] < n {
                assert!(!compares[i](a[idx], a[ri[i][idx]]));
            }
        }

        println!("{}", a[min_idx]);
    }
}
