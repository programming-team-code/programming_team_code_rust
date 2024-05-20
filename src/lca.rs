use crate::rmq::RMQ;

pub struct LCA {
    tin: Vec<usize>,
    p: Vec<Option<usize>>,
    rmq: RMQ<(usize, usize)>,
}
impl LCA {
    /// adj can be undirected tree, directed tree (rooted at node 0), or a forest of undirected
    /// trees
    pub fn new(adj: &Vec<Vec<usize>>) -> Self {
        let n = adj.len();
        let mut d = vec![0; n];
        let mut tin = vec![0; n];
        let mut p = vec![None; n];
        let mut order = Vec::with_capacity(n);
        fn dfs(
            u: usize,
            p: &mut Vec<Option<usize>>,
            adj: &Vec<Vec<usize>>,
            d: &mut Vec<usize>,
            tin: &mut Vec<usize>,
            order: &mut Vec<usize>,
        ) {
            tin[u] = order.len();
            order.push(u);
            for &v in &adj[u] {
                if p[u] != Some(v) {
                    d[v] = d[u] + 1;
                    p[v] = Some(u);
                    dfs(v, p, adj, d, tin, order);
                }
            }
        }
        for s in 0..n {
            if p[s].is_none() {
                dfs(s, &mut p, adj, &mut d, &mut tin, &mut order);
            }
        }
        let d_with_order: Vec<(usize, usize)> = order.iter().map(|&u| (d[u], u)).collect();
        let rmq = RMQ::new(&d_with_order, std::cmp::min);
        LCA { tin, p, rmq }
    }
    /// gets the lowest common ancestor of u and v
    pub fn lca(&self, u: usize, v: usize) -> usize {
        if u == v {
            return u;
        }
        let (mut le, mut ri) = (self.tin[u], self.tin[v]);
        if le > ri {
            std::mem::swap(&mut le, &mut ri);
        }
        self.p[self.rmq.query(le + 1..ri + 1).1].unwrap()
    }
}