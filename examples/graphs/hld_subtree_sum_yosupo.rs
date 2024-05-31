// verification-helper: PROBLEM https://judge.yosupo.jp/problem/vertex_add_subtree_sum

use proconio::input;
use programming_team_code_rust::graphs::hld::HLD;
use programming_team_code_rust::data_structures::fenwick::Fenwick;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut adj = vec![vec![]; n];
    for c in 1..n {
        input! {
            p: usize,
        }
        adj[c].push(p);
        adj[p].push(c);
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
                }
                println!("{}", fenwick.sum(hld.sub_tree(u)));
            }
        }
    }
}
