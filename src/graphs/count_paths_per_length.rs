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

fn dfs_order(adj: &[Vec<usize>], u: usize, p: usize) -> Vec<usize> {
    fn dfs(u: usize, p: usize, adj: &[Vec<usize>], order: &mut Vec<usize>) {
        order.push(u);
        for &v in &adj[u] {
            if v != p {
                dfs(v, u, adj, order);
            }
        }
    }
    let mut order = Vec::new();
    dfs(u, p, adj, &mut order);
    order
}

struct TreeFreqDist {
    seen: Vec<bool>,
    depth: Vec<usize>,
    with_dist: Vec<u64>,
}

impl TreeFreqDist {
    fn new(n: usize) -> Self {
        Self {
            seen: vec![false; n],
            depth: vec![0; n],
            with_dist: vec![0; n],
        }
    }
}

impl CentDecompDfs for TreeFreqDist {
    fn dfs(&mut self, adj: &[Vec<usize>], cent: usize) {
        let mut ch_and_order = adj[cent]
            .iter()
            .map(|&ch| (ch, dfs_order(adj, ch, cent)))
            .collect::<Vec<_>>();

        ch_and_order.sort_by_key(|(_, order)| order.len());

        let mut acc = vec![1];

        self.seen[cent] = true;
        for (ch, order) in ch_and_order {
            self.depth[ch] = 1;
            for &u in order.iter() {
                self.seen[u] = true;
                for &v in adj[u].iter() {
                    if !self.seen[v] {
                        self.depth[v] = self.depth[u] + 1;
                    }
                }
            }

            let mut cur = vec![0; order.len() + 1];
            for &u in order.iter() {
                cur[self.depth[u]] += 1;
            }

            let span = conv(&acc, &cur);
            for (d, cnt) in span.iter().enumerate() {
                self.with_dist[d] += cnt;
            }

            acc.resize(acc.len().max(cur.len()), 0);
            for d in 0..cur.len() {
                acc[d] += cur[d];
            }
        }

        for u in dfs_order(adj, cent, cent) {
            self.seen[u] = false;
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
    let mut tree_freq_dist = TreeFreqDist::new(n);
    cent_decomp(adj.to_vec(), &mut tree_freq_dist);
    tree_freq_dist.with_dist
}
