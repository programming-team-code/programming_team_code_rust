// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=GRL_5_D

use proconio::input;
use programming_team_code_rust::data_structures::fenwick::Fenwick;
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
    let mut fenwick = Fenwick::<usize>::new(n);

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
                    delta: usize,
                }
                hld.path(u, hld.p[u], |range, _| {
                    assert!(range.len() <= 1);
                    if range.len() == 1 {
                        fenwick.add(range.start, delta)
                    }
                });
            }
            _ => {
                input! {
                    u: usize,
                }
                let mut sum = 0;
                hld.path(u, 0, |range, _| sum += fenwick.sum(range));
                println!("{}", sum);
            }
        }
    }
}
