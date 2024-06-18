//! # Count the number of paths of each length in a tree
use crate::graphs::cent_decomp::{cent_decomp, CentDecompDfs};
use crate::numbers::fft::fft_multiply;

fn conv(a: &[u64], b: &[u64]) -> Vec<u64> {
    let a = a.iter().map(|&x| x as f64).collect::<Vec<_>>();
    let b = b.iter().map(|&x| x as f64).collect::<Vec<_>>();
    let res_len = a.len() + b.len() - 1;
    let mut res = fft_multiply(a, b);
    res.resize(res_len, 0.0);
    res.iter().map(|&x| x.round() as u64).collect()
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
///
/// let mut adj = vec![vec![]; 4];
/// for (u, v) in [(0,1), (0,2), (0,3)] {
///    adj[u].push(v);
///    adj[v].push(u);
/// }
///
/// let res = count_paths_per_length(&adj);
/// assert_eq!(res, [0, 3, 3, 0]);
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
