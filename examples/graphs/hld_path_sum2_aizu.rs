// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_E

use proconio::input;
use programming_team_code_rust::data_structures::lazy_seg_tree::LazySegTree;
use programming_team_code_rust::graphs::hld::HLD;

fn main() {
    input! {
        n: usize,
    }

    let mut adj = (0..n)
        .map(|_| {
            input! {
                k: usize,
                childs: [usize; k],
            }
            childs
        })
        .collect::<Vec<Vec<usize>>>();

    let hld = HLD::new(&mut adj, true);
    let mut st = LazySegTree::new(n);

    input! {
        q: usize,
    }

    for _ in 0..q {
        input! {
            t: usize
        }

        match t {
            0 => {
                input! {
                    u: usize,
                    delta: u64,
                }
                hld.path(u, 0, |range, _| st.update(range, delta));
            }
            _ => {
                input! {
                    u: usize,
                }
                let mut sum = 0;
                hld.path(u, 0, |range, _| sum += st.query(range));
                println!("{}", sum);
            }
        }
    }
}
