//! # Heavy Light Decomposition

use std::ops::Range;
use crate::graphs::dfs_order::get_dfs_preorder;

pub struct HLD {
    p: Vec<usize>,
    siz: Vec<usize>,
    tin: Vec<usize>,
    head: Vec<usize>,
    vals_edges: bool,
}

impl HLD {
    pub fn new(adj: &mut [Vec<usize>], vals_edges: bool) -> Self {
        let n = adj.len();
        let mut p = vec![0; n];
        let order = get_dfs_preorder(adj);
        for &u in &order {
            adj[u].retain(|&v| v != p[u]);
            for &v in &adj[u] {
                p[v] = u;
            }
        }
        let mut siz = vec![1; n];
        for &u in order.iter().rev() {
            for i in 0..adj[u].len() {
                let v = adj[u][i];
                siz[u] += siz[v];
                if siz[v] > siz[adj[u][0]] {
                    adj[u].swap(0, i);
                }
            }
        }
        let mut tin = vec![0; n];
        let mut head = vec![0; n];
        for (i, &u) in get_dfs_preorder(adj).iter().enumerate() {
            tin[u] = i;
            for &v in &adj[u] {
                head[v] = if v == adj[u][0] { head[u] } else { v };
            }
        }
        HLD { p, siz, tin, head, vals_edges }
    }

    pub fn lca(&self, mut u: usize, mut v: usize) -> usize {
        loop {
            if self.tin[u] > self.tin[v] {
                std::mem::swap(&mut u, &mut v);
            }
            if self.head[u] == self.head[v] {
                return u;
            }
            v = self.p[self.head[v]];
        }
    }

    pub fn sub_tree(&self, u: usize) -> Range<usize> {
        self.tin[u] + self.vals_edges as usize..self.tin[u] + self.siz[u]
    }

    pub fn in_sub(&self, u: usize, v: usize) -> bool {
        u == v || self.sub_tree(u).contains(&v)
    }
}
