// verification-helper: PROBLEM https://judge.yosupo.jp/problem/two_sat

use proconio::input;
use programming_team_code_rust::graphs::two_sat::TwoSat;

fn main() {
    input! {
        _p: String,
        _cnf: String,
        n: usize,
        m: usize,
        clauses: [(isize, isize, usize); m],
    }

    let mut ts = TwoSat::new(n);

    for (x, y, _) in clauses {
        let f = x > 0;
        let g = y > 0;
        ts.add_clause(x.unsigned_abs() - 1, f, y.unsigned_abs() - 1, g);
    }

    let ans = ts.solve();

    if let Some(ans) = ans {
        println!("s SATISFIABLE");
        print!("v");
        for i in 1..=n {
            print!(
                " {}",
                if ans[i - 1] {
                    i as isize
                } else {
                    -(i as isize)
                }
            );
        }
        println!(" 0");
    } else {
        println!("s UNSATISFIABLE");
    }
}
