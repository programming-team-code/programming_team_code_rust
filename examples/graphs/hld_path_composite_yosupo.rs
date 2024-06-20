// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite

use proconio::input;
use programming_team_code_rust::data_structures::seg_tree::SegTree;
use programming_team_code_rust::graphs::hld::HLD;

const MOD: u64 = 998_244_353;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let a = (0..n)
        .map(|_| {
            input! {
                c: u64,
                d: u64
            }
            (c, d)
        })
        .collect::<Vec<(u64, u64)>>();

    input! {
        edges: [(usize, usize); n - 1],
    }

    let mut adj = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        adj[u].push(v);
        adj[v].push(u);
    }

    let hld = HLD::new(&mut adj, false);

    let mut input_a = vec![(0, 0); n];
    for (i, &elem) in a.iter().enumerate() {
        input_a[hld.tin[i]] = elem;
    }

    let mut st_forwards = SegTree::<(u64, u64)>::build_on_array(
        &input_a,
        |x, y| (x.0 * y.0 % MOD, (y.0 * x.1 + y.1) % MOD),
        (1, 0),
    );
    let mut st_backwards = SegTree::<(u64, u64)>::build_on_array(
        &input_a,
        |x, y| (x.0 * y.0 % MOD, (x.0 * y.1 + x.1) % MOD),
        (1, 0),
    );

    for _ in 0..q {
        input! {
            t: usize
        }

        match t {
            0 => {
                input! {
                    u: usize,
                    c: u64,
                    d: u64
                }
                st_forwards.set(hld.tin[u], (c, d));
                st_backwards.set(hld.tin[u], (c, d));
            }
            _ => {
                input! {
                    u: usize,
                    v: usize,
                    x: u64
                }
                let (mut u_anc_val, mut v_anc_val) = (st_forwards.unit, st_backwards.unit);
                hld.path(u, v, |range, u_anc| {
                    if u_anc {
                        u_anc_val = (st_forwards.op)(&u_anc_val, &st_backwards.query(range));
                    } else {
                        v_anc_val = (st_forwards.op)(&st_forwards.query(range), &v_anc_val);
                    }
                });
                let res = (st_forwards.op)(&u_anc_val, &v_anc_val);
                println!("{}", (res.0 * x + res.1) % MOD);
            }
        }
    }
}
