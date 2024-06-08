//! # Suffix Array

use std::cmp::Ordering;
use std::ops::Range;

use crate::data_structures::rmq::RMQ;
use ac_library::string::{lcp_array_arbitrary, suffix_array_manual};

/// # Example
/// ```
/// use programming_team_code_rust::helpers::compress::compress;
/// use programming_team_code_rust::strings::suffix_array::SufAry;
///
/// let s = "banana";
/// let suf_ary1 = SufAry::new(&s.chars().map(|c| c as usize).collect::<Vec<usize>>(), 255);
///
/// // 0 banana 3
/// // 1 anana  2
/// // 2 nana   5
/// // 3 ana    1
/// // 4 na     4
/// // 5 a      0
/// //
/// // 5 a      0
/// //   |
/// // 3 ana    1
/// //   |||
/// // 1 anana  2
/// //
/// // 0 banana 3
/// //
/// // 4 na     4
/// //   ||
/// // 2 nana   5
///
/// assert_eq!(suf_ary1.sa, [5, 3, 1, 0, 4, 2]);
/// assert_eq!(suf_ary1.sa_inv, [3, 2, 5, 1, 4, 0]);
/// assert_eq!(suf_ary1.lcp, [1, 3, 0, 0, 2]);
///
/// let a = [-4, 8, 1_000_000_000, 3];
/// let (a_comp, max_val) = compress(&a);
/// let suf_ary2 = SufAry::new(&a_comp, max_val);
///
/// assert_eq!(suf_ary2.sa, [0, 3, 1, 2]);
/// ```
pub struct SufAry {
    n: usize,
    s: Vec<usize>,
    /// suffix array
    pub sa: Vec<usize>,
    /// inverse suffix array
    pub sa_inv: Vec<usize>,
    /// longest common prefix array
    pub lcp: Vec<usize>,
    rmq: RMQ<usize>,
}

impl SufAry {
    /// Creates a new Suffix Array struct
    ///
    /// # Complexity
    /// - Time: O(n + max_val)
    /// - Space: O(n)
    pub fn new(s: &[usize], max_val: usize) -> Self {
        let sa = suffix_array_manual(
            &s.iter().map(|&x| x as i32).collect::<Vec<i32>>(),
            max_val as i32,
        );
        let lcp = lcp_array_arbitrary(s, &sa);
        let mut sa_inv = vec![0; s.len()];
        for (i, &elem) in sa.iter().enumerate() {
            sa_inv[elem] = i;
        }
        Self {
            n: sa.len(),
            s: s.to_vec(),
            sa_inv,
            rmq: RMQ::new(&lcp, std::cmp::min),
            lcp,
            sa,
        }
    }

    /// Gets max k such that s\[i1..i1 + k\] == s\[i2..i2 + k\]
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
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

    /// Gets s\[le1..\] compared with s\[le2..\]
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn cmp_sufs(&self, le1: usize, le2: usize) -> Ordering {
        if std::cmp::max(le1, le2) == self.n {
            le2.cmp(&le1)
        } else {
            self.sa_inv[le1].cmp(&self.sa_inv[le2])
        }
    }

    /// Gets s\[x\] compared with s\[y\]
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn cmp_substrs(&self, x: Range<usize>, y: Range<usize>) -> Ordering {
        if self.len_lcp(x.start, y.start) >= std::cmp::min(x.len(), y.len()) {
            x.len().cmp(&y.len())
        } else {
            self.sa_inv[x.start].cmp(&self.sa_inv[y.start])
        }
    }

    /// Gets range r such that:
    ///   - for all i in sa\[r\] s\[i..i + t.len()\] == t
    ///   - r.len() is the number of matches of t in s
    ///
    /// # Complexity
    /// - Time: O(|t| * log(|s|))
    /// - Space: O(1)
    pub fn find_str(&self, t: &[usize]) -> Range<usize> {
        let le = self.sa.partition_point(|&i| &self.s[i..] < t);
        let sz =
            self.sa[le..].partition_point(|&i| &self.s[i..std::cmp::min(i + t.len(), self.n)] == t);
        le..le + sz
    }

    /// Gets range r such that:
    ///   - for all i in sa\[r\] s\[i..i + substr.len()\] == s\[substr\]
    ///   - r.len() is the number of matches of s\[substr\] in s
    ///
    /// # Complexity
    /// - Time: O(log(|s|))
    /// - Space: O(1)
    pub fn find_substr(&self, substr: Range<usize>) -> Range<usize> {
        if substr.start == self.n {
            return 0..self.n;
        }
        let cmp = |i: usize, flip: bool| -> bool {
            flip ^ (self.len_lcp(i, substr.start) < substr.len())
        };
        let le = self.sa[..self.sa_inv[substr.start]].partition_point(|&i| cmp(i, false));
        let sz = self.sa[self.sa_inv[substr.start] + 1..].partition_point(|&i| cmp(i, true)) + 1;
        le..le + sz
    }
}
