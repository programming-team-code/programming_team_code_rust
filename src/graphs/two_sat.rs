//! # Two Satisfiability

use crate::graphs::scc::get_sccs;

/// # Example
/// ```
/// use programming_team_code_rust::graphs::two_sat::TwoSat;
///
/// let mut ts = TwoSat::new(2);
/// ts.add_var();
/// ts.add_clause(0, false, 1, true);
/// ts.add_clause(1, false, 2, true);
/// ts.add_clause(2, false, 0, true);
/// assert_eq!(ts.solve(), Some(vec![false, false, false]));
/// ```
pub struct TwoSat {
    adj: Vec<Vec<usize>>,
}

impl TwoSat {
    /// Create a new instance of TwoSat
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn new(n: usize) -> Self {
        Self {
            adj: vec![vec![]; 2 * n],
        }
    }

    /// Add a clause to the 2-SAT instance
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn add_clause(&mut self, i: usize, f: bool, j: usize, g: bool) {
        self.adj[2 * i + !f as usize].push(2 * j + g as usize);
        self.adj[2 * j + !g as usize].push(2 * i + f as usize);
    }

    /// Solve the 2-SAT instance
    /// Returns None if there is no solution, otherwise returns a valid assignment
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn solve(&self) -> Option<Vec<bool>> {
        let (_, scc_id) = get_sccs(&self.adj);
        let mut res = vec![false; self.adj.len() / 2];
        for i in 0..self.adj.len() / 2 {
            if scc_id[2 * i] == scc_id[2 * i + 1] {
                return None;
            }
            res[i] = scc_id[2 * i] < scc_id[2 * i + 1];
        }
        Some(res)
    }

    // TODO: implement this https://github.com/kth-competitive-programming/kactl/blob/main/content/graph/2sat.h#L43-L54
}
