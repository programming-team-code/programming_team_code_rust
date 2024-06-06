//! # Suffix Array

use crate::data_structures::rmq::RMQ;
use ac_library::string::{lcp_array_arbitrary, suffix_array_arbitrary, suffix_array_manual};

pub struct SufAry {
    n: usize,
    /// suffix array
    pub sa: Vec<usize>,
    /// inverse suffix array
    pub sa_inv: Vec<usize>,
    /// longest common prefix array
    pub lcp: Vec<usize>,
    rmq: RMQ<usize>,
}

fn get_inv(a: &[usize]) -> Vec<usize> {
    let mut inv = vec![0; a.len()];
    for (i, &elem) in a.iter().enumerate() {
        inv[elem] = i;
    }
    inv
}

impl SufAry {
    pub fn new(s: &[char]) -> Self {
        let sa = suffix_array_manual(&s.iter().map(|&x| x as i32).collect::<Vec<i32>>(), 255);
        let lcp = lcp_array_arbitrary(s, &sa);
        Self {
            n: sa.len(),
            sa_inv: get_inv(&sa),
            rmq: RMQ::new(&lcp, std::cmp::min),
            lcp,
            sa,
        }
    }

    pub fn new_arbitrary<T: Ord>(s: &[T]) -> Self {
        let sa = suffix_array_arbitrary(s);
        let lcp = lcp_array_arbitrary(s, &sa);
        Self {
            n: sa.len(),
            sa_inv: get_inv(&sa),
            rmq: RMQ::new(&lcp, std::cmp::min),
            lcp,
            sa,
        }
    }

    pub fn len_lcp(&self, i1: usize, i2: usize) -> usize {
        let mx = std::cmp::max(i1, i2);
        if i1 == i2 || mx == self.n {
            return self.n - mx;
        }
        let (mut le, mut ri) = (self.sa_inv[i1], self.sa_inv[i2]);
        if le > ri {
            std::mem::swap(&mut le, &mut ri);
        }
        self.rmq.query(le..ri)
    }
}
