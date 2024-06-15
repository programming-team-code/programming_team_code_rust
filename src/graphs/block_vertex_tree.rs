//! # Block Vertex Tree

pub fn get_bvt(adj: &[Vec<(usize, usize)>], num_bccs: usize, bcc_id: &[usize]) -> Vec<Vec<usize>> {
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
        for &id in &bvt[i] {
            vis[id - n] = false;
        }
    }
    bvt
}
