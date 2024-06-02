// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_E

use proconio::input;
use programming_team_code_rust::data_structures::seg_tree::SegTree;
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
    let mut st = SegTree::new(n);

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
                hld.path(0, u, |range, _| st.update(range, delta));
            }
            _ => {
                input! {
                    u: usize,
                }
                let mut sum = 0;
                hld.path(0, u, |range, _| sum += st.query(range));
                println!("{}", sum);
            }
        }
    }
}
