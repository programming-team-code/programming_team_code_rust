// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite

use proconio::input;
use programming_team_code_rust::data_structures::seg_tree::SegTree;
use programming_team_code_rust::graphs::hld::HLD;

fn main() {
    let md = 998244353;

    input! {
        n: usize,
        q: usize,
    }

    let a = (0..n)
        .map(|_| {
            input! {
                c: usize,
                d: usize
            }
            (c, d)
        })
        .collect::<Vec<(usize, usize)>>();

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
    for i in 0..n {
        input_a[hld.tin[i]] = a[i];
    }

    let mut st_forwards = SegTree::<(usize, usize)>::build_on_array(
        &input_a,
        move |x, y| (x.0 * y.0 % md, (y.0 * x.1 + y.1) % md),
        (1, 0),
    );
    let mut st_backwards = SegTree::<(usize, usize)>::build_on_array(
        &input_a,
        move |x, y| (x.0 * y.0 % md, (x.0 * y.1 + x.1) % md),
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
                    c: usize,
                    d: usize
                }
                st_forwards.set(hld.tin[u], (c, d));
                st_backwards.set(hld.tin[u], (c, d));
            }
            _ => {
                input! {
                    u: usize,
                    v: usize,
                    x: usize
                }
                let (mut u_anc_val, mut v_anc_val) = (st_forwards.unit, st_backwards.unit);
                hld.path(u, v, |range, u_anc| {
                    if u_anc {
                        u_anc_val = (st_forwards.op)(u_anc_val, st_backwards.query(range));
                    } else {
                        v_anc_val = (st_forwards.op)(st_forwards.query(range), v_anc_val);
                    }
                });
                let res = (st_forwards.op)(u_anc_val, v_anc_val);
                println!("{}", (res.0 * x + res.1) % md);
            }
        }
    }
}
