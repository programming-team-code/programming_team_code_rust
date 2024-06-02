// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_set_path_composite

use proconio::input;
use programming_team_code_rust::data_structures::seg_tree::SegTree;
use programming_team_code_rust::graphs::hld::HLD;

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let a = (0..n).map(|_| {
        input! {
            a: usize,
            b: usize
        }
        (a,b)
    }).collect::<(usize, usize)>();

    input! {
        edges: [(usize, usize); n - 1],
    }

    let mut adj = vec![vec![]; n];
    for &(u, v) in edges.iter() {
        adj[u].push(v);
        adj[v].push(u);
    }

    let hld = HLD::new(&mut adj, false);
    let mut fenwick = Fenwick::<usize>::new(n);

    for i in 0..n {
        fenwick.add(hld.tin[i], a[i]);
    }

    for _ in 0..q {
        input! {
            t: usize
        }

        match t {
            0 => {
                input! {
                    u: usize,
                    delta: usize,
                }
                fenwick.add(hld.tin[u], delta);
            }
            _ => {
                input! {
                    u: usize,
                    v: usize,
                }
                let mut sum = 0;
                hld.path(u, v, |range, _| sum += fenwick.sum(range));
                println!("{}", sum);
            }
        }
    }
}
