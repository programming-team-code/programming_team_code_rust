//! # Suffix Array

use std::cmp::Ordering;
use std::ops::Range;

use crate::data_structures::rmq::RMQ;
use ac_library::string::{lcp_array_arbitrary, suffix_array, suffix_array_arbitrary};

pub struct SufAry {
    n: usize,
    //s: Vec<usize>,
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
    /*
    pub fn new(s: &str) -> Self {
        let sa = suffix_array(s);
        let lcp = lcp_array_arbitrary(s, &sa);
        Self {
            n: sa.len(),
            //s: s.iter().map(|&x| x as usize).collect::<Vec<usize>>(),
            sa_inv: get_inv(&sa),
            rmq: RMQ::new(&lcp, std::cmp::min),
            sa,
            lcp,
        }
    }
    */

    pub fn new_arbitrary<T: Ord>(s: &[T]) -> Self {
        let sa = suffix_array_arbitrary(s);
        let lcp = lcp_array_arbitrary(s, &sa);
        Self {
            n: sa.len(),
            //s: s.to_vec(),
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

    pub fn cmp_sufs(&self, le1: usize, le2: usize) -> Ordering {
        if std::cmp::max(le1, le2) == self.n {
            le2.cmp(&le1)
        } else {
            self.sa_inv[le1].cmp(&self.sa_inv[le2])
        }
    }

    pub fn cmp_substrs(&self, x: Range<usize>, y: Range<usize>) -> Ordering {
        if self.len_lcp(x.start, y.start) >= std::cmp::min(x.len(), y.len()) {
            x.len().cmp(&y.len())
        } else {
            self.sa_inv[x.start].cmp(&self.sa_inv[y.start])
        }
    }

    /*
    pub fn find_substr(&self, substr: Range<usize>) -> Range<usize> {
        if substr.start == self.n {
            return 0..self.n;
        }

        self.sa.partition_point(|&idx| );

        0..1
    }
    */
}
