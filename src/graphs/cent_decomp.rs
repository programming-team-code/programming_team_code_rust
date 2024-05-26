//! # Centroid Decomposition

fn calc_sz(adj: &[Vec<usize>], u: usize, p: usize, sub_sz: &mut [usize]) {
    sub_sz[u] = 1;
    for &v in adj[u].iter() {
        if v != p {
            calc_sz(adj, v, u, sub_sz);
            sub_sz[u] += sub_sz[v];
        }
    }
}

fn dfs(
    adj: &mut [Vec<usize>],
    mut u: usize,
    sub_sz: &mut [usize],
    call_dfs: &mut dyn CentDecompDfs,
) {
    calc_sz(adj, u, u, sub_sz);
    let sz_root = sub_sz[u];
    let mut p = u;
    loop {
        let big_ch = adj[u]
            .iter()
            .filter(|&&v| v != p)
            .find(|&&v| sub_sz[v] * 2 > sz_root);
        if let Some(&v) = big_ch {
            p = u;
            u = v;
        } else {
            break;
        }
    }
    call_dfs.dfs(adj, u);
    for v in adj[u].clone() {
        adj[v].retain(|&x| x != u);
        dfs(adj, v, sub_sz, call_dfs);
    }
}

/// A trait containing the DFS method which is called on each centroid of the tree with the
/// back-edges outside of this centroid removed from `adj`
pub trait CentDecompDfs {
    /// The DFS method
    fn dfs(&mut self, adj: &[Vec<usize>], cent: usize);
}

/// # Example
/// - see count_paths_per_length.rs
///
/// # Params
/// - `adj`: adjacency list representing an unrooted undirected tree
/// - `call_dfs`: an object implementing `CentDecompDfs` trait
///
/// # Complexity
/// - Time: O(n log n)
/// - Space: O(n)
pub fn cent_decomp(mut adj: Vec<Vec<usize>>, call_dfs: &mut dyn CentDecompDfs) {
    let n = adj.len();
    let mut sub_sz = vec![0; n];
    for s in 0..n {
        if sub_sz[s] == 0 {
            dfs(&mut adj, s, &mut sub_sz, call_dfs);
        }
    }
}
