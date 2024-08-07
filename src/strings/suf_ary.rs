//! # Suffix Array

use std::cmp::Ordering;
use std::ops::Range;

use crate::data_structures::rmq::RMQ;
use ac_library::string::{lcp_array_arbitrary, suffix_array_manual};

/// # Example
/// ```
/// use std::cmp::Ordering;
/// use programming_team_code_rust::helpers::compress::compress;
/// use programming_team_code_rust::strings::suf_ary::SufAry;
///
/// let s = "banana";
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
/// // build from Vec
/// let a = [-4, 8, 1_000_000_000, 3];
/// let (a_comp, max_val) = compress(&a);
/// let suf_ary = SufAry::new(&a_comp, max_val);
///
/// // build from String
/// let suf_ary = SufAry::new(&s.chars().map(|c| c as usize).collect::<Vec<usize>>(), 255);
/// let n = suf_ary.sa.len();
/// assert_eq!(suf_ary.sa, [5, 3, 1, 0, 4, 2]);
/// assert_eq!(suf_ary.sa_inv, [3, 2, 5, 1, 4, 0]);
/// assert_eq!(suf_ary.lcp, [1, 3, 0, 0, 2]);
///
/// assert_eq!(suf_ary.len_lcp(1, 3), 3);
/// assert!(std::panic::catch_unwind(|| suf_ary.len_lcp(1, n)).is_err());
///
/// assert_eq!(suf_ary.cmp_sufs(1, 3), Ordering::Greater);
/// assert!(std::panic::catch_unwind(|| suf_ary.cmp_sufs(n, 2)).is_err());
///
/// assert_eq!(suf_ary.cmp_substrs(1..4, 3..6), Ordering::Equal);
/// assert!(std::panic::catch_unwind(|| suf_ary.cmp_substrs(3..4, n..n)).is_err());
///
/// assert_eq!(suf_ary.find_str(&"ana".chars().map(|c| c as usize).collect::<Vec<usize>>()), 1..3);
/// assert_eq!(suf_ary.find_str(&[]), 0..n);
///
/// assert_eq!(suf_ary.find_substr(1..4), 1..3);
/// assert_eq!(suf_ary.find_substr(1..1), 0..n);
/// assert!(std::panic::catch_unwind(|| suf_ary.find_substr(n..n)).is_err());
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
    rmq: RMQ<usize, fn(&usize, &usize) -> usize>,
}

impl SufAry {
    /// Creates a new Suffix Array struct
    ///
    /// # Complexity
    /// - Time: O(n + max_val)
    /// - Space: O(n)
    pub fn new(s: &[usize], max_val: usize) -> Self {
        let sa = suffix_array_manual(
            &s.iter().map(|&x| x as i32).collect::<Vec<_>>(),
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
            rmq: RMQ::new(&lcp, |&x, &y| std::cmp::min(x, y)),
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
        if i1 == i2 {
            return self.n - i1;
        }
        let (mut le, mut ri) = (self.sa_inv[i1], self.sa_inv[i2]);
        if le > ri {
            (le, ri) = (ri, le);
        }
        self.rmq.query(le..ri)
    }

    /// Gets s\[le1..\] compared with s\[le2..\]
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn cmp_sufs(&self, le1: usize, le2: usize) -> Ordering {
        self.sa_inv[le1].cmp(&self.sa_inv[le2])
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
            self.cmp_sufs(x.start, y.start)
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
        let ri =
            self.sa[le..].partition_point(|&i| &self.s[i..(i + t.len()).min(self.n)] == t) + le;
        le..ri
    }

    /// Gets range r such that:
    ///   - for all i in sa\[r\] s\[i..i + substr.len()\] == s\[substr\]
    ///   - r.len() is the number of matches of s\[substr\] in s
    ///
    /// # Complexity
    /// - Time: O(log(|s|))
    /// - Space: O(1)
    pub fn find_substr(&self, substr: Range<usize>) -> Range<usize> {
        let cmp = |i: usize, flip: bool| -> bool {
            flip ^ (self.len_lcp(i, substr.start) < substr.len())
        };
        let idx = self.sa_inv[substr.start];
        let le = self.sa[..idx].partition_point(|&i| cmp(i, false));
        let ri = self.sa[idx + 1..].partition_point(|&i| cmp(i, true)) + idx + 1;
        le..ri
    }
}
