//! # Centroid Decomposition

fn calc_sz(adj: &[Vec<usize>], v: usize, p: usize, sub_sz: &mut [usize]) {
    sub_sz[v] = 1;
    for &u in adj[v].iter() {
        if u != p {
            calc_sz(adj, u, v, sub_sz);
            sub_sz[v] += sub_sz[u];
        }
    }
}

fn dfs(
    adj: &mut [Vec<usize>],
    mut v: usize,
    sub_sz: &mut [usize],
    call_dfs: &mut dyn CentDecompDfs,
) {
    calc_sz(adj, v, v, sub_sz);
    let sz_root = sub_sz[v];
    let mut p = v;
    loop {
        let big_ch = adj[v]
            .iter()
            .filter(|&&u| u != p)
            .find(|&&u| sub_sz[u] * 2 > sz_root);
        if let Some(&u) = big_ch {
            p = v;
            v = u;
        } else {
            break;
        }
    }
    call_dfs.dfs(adj, v);
    for u in adj[v].clone() {
        adj[u].retain(|&x| x != v);
        dfs(adj, u, sub_sz, call_dfs);
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
