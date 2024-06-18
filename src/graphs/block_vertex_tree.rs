//! # Block Vertex Tree

/// # Guarantees
/// - 0 <= bcc_id\[u\] < num_bccs
///
/// # Example
/// ```
/// use programming_team_code_rust::graphs::cuts::get_cuts;
/// use programming_team_code_rust::graphs::block_vertex_tree::get_block_vertex_tree;
///
/// let edge_list = [(0,1), (0,1), (1,2), (0,1)];
/// let (n, m) = (3, 4);
/// let mut adj = vec![vec![]; n];
/// for (i, &(u, v)) in edge_list.iter().enumerate() {
///    adj[u].push((v, i));
///    adj[v].push((u, i));
/// }
///
/// let (num_bccs, _, bcc_id) = get_cuts(&adj, m);
/// let bvt = get_block_vertex_tree(&adj, num_bccs, &bcc_id);
///
/// assert_eq!(bvt, [vec![4], vec![4, 3], vec![3], vec![1, 2], vec![0, 1]]);
///
/// // indexes 0..n are nodes
/// // indexes n..n + num_bccs are bccs
///
/// for u in 0..n {
///    // loop over each unique bcc containing a node u
///    for bccid in bvt[u].iter().map(|v| v - n) {
///       assert!(0 <= bccid && bccid < num_bccs);
///    }
/// }
/// for bccid in 0..num_bccs {
///     // loop over each unique node inside a bcc
///    for &u in bvt[bccid + n].iter() {
///       assert!(0 <= u && u < n);
///    }
/// }
/// ```
///
/// # Complexity
/// - Time: O(V + E)
/// - Space: O(V)
pub fn get_block_vertex_tree(
    adj: &[Vec<(usize, usize)>],
    num_bccs: usize,
    bcc_id: &[usize],
) -> Vec<Vec<usize>> {
    let (n, mut vis) = (adj.len(), vec![false; num_bccs]);
    let mut bvt = vec![vec![]; n + num_bccs];
    for i in 0..n {
        for id in adj[i].iter().map(|&(_, e_id)| bcc_id[e_id]) {
            if !vis[id] {
                vis[id] = true;
                bvt[i].push(id + n);
                bvt[id + n].push(i);
            }
        }
        for id in bvt[i].iter().map(|v| v - n) {
            vis[id] = false;
        }
    }
    bvt
}
