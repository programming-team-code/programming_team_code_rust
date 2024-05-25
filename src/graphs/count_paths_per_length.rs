//! # Count the number of paths of each length in a tree
use crate::graphs::cent_decomp::{cent_decomp, CentDecompDfs};

// TODO: update this to use FFT
fn conv(a: &[u64], b: &[u64]) -> Vec<u64> {
    let len_a = a.len();
    let len_b = b.len();
    let mut c = vec![0; len_a + len_b - 1];
    for i in 0..len_a {
        for j in 0..len_b {
            c[i + j] += a[i] * b[j];
        }
    }
    c
}

struct CountPathsPerLength {
    num_paths: Vec<u64>,
}

impl CountPathsPerLength {
    fn new(n: usize) -> Self {
        Self {
            num_paths: vec![0; n],
        }
    }
}

impl CentDecompDfs for CountPathsPerLength {
    fn dfs(&mut self, adj: &[Vec<usize>], cent: usize) {
        let mut child_depths = vec![vec![]];
        for &child in adj[cent].iter() {
            let mut my_child_depths = vec![0];

            use std::collections::VecDeque;

            let mut q = VecDeque::new();
            q.push_back((child, cent));

            while !q.is_empty() {
                my_child_depths.push(q.len() as u64);

                let mut new_q = VecDeque::new();
                while let Some((u, p)) = q.pop_front() {
                    for &v in adj[u].iter() {
                        if v != p {
                            new_q.push_back((v, u));
                        }
                    }
                }

                q = new_q;
            }

            child_depths.push(my_child_depths);
        }

        child_depths.sort_by_key(|v| v.len());

        let mut acc = vec![1];
        for depth_arr in child_depths {
            let res = conv(&acc, &depth_arr);
            for (d, &cnt) in res.iter().enumerate() {
                self.num_paths[d] += cnt;
            }

            acc.resize(acc.len().max(depth_arr.len()), 0);
            for (d, &cnt) in depth_arr.iter().enumerate() {
                acc[d] += cnt;
            }
        }
    }
}

/// # Example
/// ```
/// use programming_team_code_rust::graphs::count_paths_per_length::count_paths_per_length;
/// let adj = vec![
///    vec![1, 2],
///    vec![0, 3, 4],
///    vec![0, 5, 6],
///    vec![1],
///    vec![1],
///    vec![2],
///    vec![2],
/// ];
/// let res = count_paths_per_length(&adj);
/// assert_eq!(res, vec![0, 6, 7, 4, 4, 0, 0]);
/// ```
///
/// # Complexity
/// - Time: O(n log^2 n) with FFT for convolution
/// - Space: O(n)
pub fn count_paths_per_length(adj: &[Vec<usize>]) -> Vec<u64> {
    let n = adj.len();
    let mut obj = CountPathsPerLength::new(n);
    cent_decomp(adj.to_vec(), &mut obj);
    obj.num_paths
}
