//! # Suffix Array

use ac_library::string::{lcp_array_arbitrary, suffix_array_arbitrary, suffix_array_manual};

pub struct SufAry {
    /// suffix array
    pub sa: Vec<usize>,
    /// inverse suffix array
    pub sa_inv: Vec<usize>,
    /// longest common prefix array
    pub lcp: Vec<usize>,
}

impl SufAry {
    pub fn new<T: Ord>(s: &[T]) -> Self {
        let sa = suffix_array_arbitrary(s);
        let mut sa_inv = vec![0; s.len()];
        for (i, &elem) in sa.iter().enumerate() {
            sa_inv[elem] = i;
        }
        Self {
            sa_inv,
            lcp: lcp_array_arbitrary(s, &sa),
            sa,
        }
    }
}
