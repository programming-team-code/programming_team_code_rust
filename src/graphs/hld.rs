//! # Heavy Light Decomposition

use crate::graphs::dfs_order::{get_dfs_postorder, get_dfs_preorder};
use std::ops::Range;

pub struct HLD {
    pub p: Vec<usize>,
    pub tin: Vec<usize>,
    siz: Vec<usize>,
    head: Vec<usize>,
    vals_edges: bool,
}

impl HLD {
    pub fn new(adj: &mut [Vec<usize>], vals_edges: bool) -> Self {
        let n = adj.len();
        let mut p = vec![0; n];
        let mut siz = vec![1; n];
        for &u in get_dfs_postorder(adj).iter() {
            if let Some(i) = adj[u].iter().position(|&v| p[v] != u) {
                p[u] = adj[u][i];
                adj[u].swap_remove(i);
            }
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
        HLD {
            p,
            siz,
            tin,
            head,
            vals_edges,
        }
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

    pub fn path(&self, mut u: usize, mut v: usize, mut f: impl FnMut(Range<usize>, bool)) {
        let mut u_anc = false;
        loop {
            if self.tin[u] > self.tin[v] {
                std::mem::swap(&mut u, &mut v);
                u_anc = !u_anc;
            }
            if self.head[u] == self.head[v] {
                break;
            }
            f(self.tin[self.head[v]]..self.tin[v] + 1, u_anc);
            v = self.p[self.head[v]];
        }
        f(
            self.tin[u] + self.vals_edges as usize..self.tin[v] + 1,
            u_anc,
        );
    }

    pub fn sub_tree(&self, u: usize) -> Range<usize> {
        self.tin[u] + self.vals_edges as usize..self.tin[u] + self.siz[u]
    }

    pub fn in_sub(&self, u: usize, v: usize) -> bool {
        u == v || self.sub_tree(u).contains(&v)
    }

    pub fn dist_edges(&self, u: usize, v: usize) -> usize {
        0
    }

    pub fn kth_par(&self, u: usize, k: usize) -> usize {
    }

    pub fn kth_on_path(&self, u: usize, v: usize, k: usize) -> usize {
    }
}
