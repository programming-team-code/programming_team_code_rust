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
        //let mut order = Vec::with_capacity(n);
        let mut order = vec![0; n];
        fn dfs(
            u: usize,
            p: &mut Vec<Option<usize>>,
            adj: &Vec<Vec<usize>>,
            d: &mut Vec<usize>,
            tin: &mut Vec<usize>,
            order: &mut Vec<usize>,
            timer: &mut usize,
        ) {
            //eprintln!("{}", timer);
            tin[u] = *timer;
            order[*timer] = u;
            *timer += 1;
            //order.push(u);
            for &v in &adj[u] {
                if p[u] != Some(v) {
                    d[v] = d[u] + 1;
                    p[v] = Some(u);
                    dfs(v, p, adj, d, tin, order, timer);
                }
            }
        }
        let mut timer = 0;
        for s in 0..n {
            if p[s].is_none() {
                dfs(s, &mut p, adj, &mut d, &mut tin, &mut order, &mut timer);
            }
        }
        let mut d_with_order = Vec::with_capacity(order.len());
        for u in order {
            d_with_order.push((d[u], u));
        }
        //let d_with_order = order.into_iter().map(|&u| (d[u], u)).collect();
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
