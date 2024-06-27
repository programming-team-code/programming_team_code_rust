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
/// let suf_ary1 = SufAry::new(&s.chars().map(|c| c as usize).collect::<Vec<_>>(), 255);
/// let n = suf_ary1.sa.len();
/// assert_eq!(suf_ary1.sa, [5, 3, 1, 0, 4, 2]);
/// assert_eq!(suf_ary1.sa_inv, [3, 2, 5, 1, 4, 0]);
/// assert_eq!(suf_ary1.lcp, [1, 3, 0, 0, 2]);
///
/// let a = [-4, 8, 1_000_000_000, 3];
/// let (a_comp, max_val) = compress(&a);
/// let suf_ary2 = SufAry::new(&a_comp, max_val);
///
/// assert_eq!(suf_ary1.len_lcp(1, 3), 3);
///
/// assert_eq!(suf_ary1.cmp_sufs(1, 3), Ordering::Greater);
/// assert!(std::panic::catch_unwind(|| suf_ary1.cmp_sufs(n, 2)).is_err());
///
/// assert_eq!(suf_ary1.cmp_substrs(1..4, 3..6), Ordering::Equal);
///
/// assert_eq!(suf_ary1.find_str(&"ana".chars().map(|c| c as usize).collect::<Vec<usize>>()), 1..3);
/// assert_eq!(suf_ary1.find_str(&[]), 0..n);
///
/// assert_eq!(suf_ary1.find_substr(1..4), 1..3);
/// assert_eq!(suf_ary1.find_substr(2..2), 0..n);
/// assert!(std::panic::catch_unwind(|| suf_ary1.find_substr(n..n)).is_err());
///
/// assert_eq!(suf_ary1.push_back_char('n' as usize, 0..3, 1), 1..3);
/// assert_eq!(suf_ary1.push_back_char('n' as usize, n..n, 0), n..n);
///
/// assert_eq!(suf_ary1.push_front_char('a' as usize, 4..6, 2), 1..3);
/// assert_eq!(suf_ary1.push_front_char('a' as usize, n..n, 0), n..n);
///
/// assert_eq!(suf_ary1.push_back_substr(4..6, 0..3, 1), 1..3);
/// assert_eq!(suf_ary1.push_back_substr(4..6, n..n, 0), n..n);
///
/// assert_eq!(suf_ary1.push_front_substr(3..5, 0..3, 1), 1..3);
/// assert_eq!(suf_ary1.push_front_substr(3..5, n..n, 0), n..n);
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
    rmq: RMQ<usize, fn(usize, usize) -> usize>,
    cnt: Vec<usize>,
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
        let mut cnt = vec![0; max_val + 1];
        for &c in s {
            cnt[c + 1] += 1;
        }
        for i in 1..cnt.len() {
            cnt[i] += cnt[i - 1];
        }
        Self {
            n: sa.len(),
            s: s.to_vec(),
            sa_inv,
            rmq: RMQ::new(&lcp, std::cmp::min),
            lcp,
            sa,
            cnt,
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
    ///   - for all i in sa\[r\] s\[i..i + s_substr.len()\] == s\[s_substr\]
    ///   - r.len() is the number of matches of s\[s_substr\] in s
    ///
    /// # Complexity
    /// - Time: O(log(|s|))
    /// - Space: O(1)
    pub fn find_substr(&self, s_substr: Range<usize>) -> Range<usize> {
        let cmp = |i: usize, flip: bool| -> bool {
            flip ^ (self.len_lcp(i, s_substr.start) < s_substr.len())
        };
        let idx = self.sa_inv[s_substr.start];
        let le = self.sa[..idx].partition_point(|&i| cmp(i, false));
        let ri = self.sa[idx + 1..].partition_point(|&i| cmp(i, true)) + idx + 1;
        le..ri
    }

    /// let t = s[sa[sa_range.start]..sa[sa_range.start] + lcp_len] + c
    ///
    /// Gets range r such that:
    ///   - for all i in sa\[r\] s\[i..i + t.len()\] == t
    ///   - r.len() is the number of matches of t in s
    ///
    /// # Complexity
    /// - Time: O(log(|s|))
    /// - Space: O(1)
    pub fn push_back_char(&self, c: usize, sa_range: Range<usize>, lcp_len: usize) -> Range<usize> {
        if !sa_range.is_empty() {
            assert!(lcp_len <= self.len_lcp(self.sa[sa_range.start], self.sa[sa_range.end - 1]));
        }
        let le = self.sa[sa_range.clone()]
            .partition_point(|&i| i + lcp_len == self.n || self.s[i + lcp_len] < c)
            + sa_range.start;
        let ri = self.sa[le..sa_range.end].partition_point(|&i| self.s[i + lcp_len] == c) + le;
        le..ri
    }

    /// let t = c + s[sa[sa_range.start]..sa[sa_range.start] + lcp_len]
    ///
    /// Gets range r such that:
    ///   - for all i in sa\[r\] s\[i..i + t.len()\] == t
    ///   - r.len() is the number of matches of t in s
    ///
    /// # Complexity
    /// - Time: O(log(|s|))
    /// - Space: O(1)
    pub fn push_front_char(
        &self,
        c: usize,
        sa_range: Range<usize>,
        lcp_len: usize,
    ) -> Range<usize> {
        if !sa_range.is_empty() {
            assert!(lcp_len <= self.len_lcp(self.sa[sa_range.start], self.sa[sa_range.end - 1]));
        }
        if sa_range.is_empty() {
            sa_range
        } else {
            let i = self.sa[sa_range.start];
            self.push_back_substr(i..i + lcp_len, self.cnt[c]..self.cnt[c + 1], 1)
        }
    }

    /// let t = s[sa[sa_range.start]..sa[sa_range.start] + lcp_len] + s[s_substr]
    ///
    /// Gets range r such that:
    ///   - for all i in sa\[r\] s\[i..i + t.len()\] == t
    ///   - r.len() is the number of matches of t in s
    ///
    /// # Complexity
    /// - Time: O(log(|s|))
    /// - Space: O(1)
    pub fn push_back_substr(
        &self,
        s_substr: Range<usize>,
        sa_range: Range<usize>,
        lcp_len: usize,
    ) -> Range<usize> {
        if !sa_range.is_empty() {
            assert!(lcp_len <= self.len_lcp(self.sa[sa_range.start], self.sa[sa_range.end - 1]));
        }
        let cmp = |mut i: usize| -> Ordering {
            i += lcp_len;
            self.cmp_substrs(i..(i + s_substr.len()).min(self.n), s_substr.clone())
        };
        let le = self.sa[sa_range.clone()].partition_point(|&i| cmp(i) == Ordering::Less)
            + sa_range.start;
        let ri = self.sa[le..sa_range.end].partition_point(|&i| cmp(i) == Ordering::Equal) + le;
        le..ri
    }

    /// let t = s[s_substr] + s[sa[sa_range.start]..sa[sa_range.start] + lcp_len]
    ///
    /// Gets range r such that:
    ///   - for all i in sa\[r\] s\[i..i + t.len()\] == t
    ///   - r.len() is the number of matches of t in s
    ///
    /// # Complexity
    /// - Time: O(log(|s|))
    /// - Space: O(1)
    pub fn push_front_substr(
        &self,
        s_substr: Range<usize>,
        sa_range: Range<usize>,
        lcp_len: usize,
    ) -> Range<usize> {
        if !sa_range.is_empty() {
            assert!(lcp_len <= self.len_lcp(self.sa[sa_range.start], self.sa[sa_range.end - 1]));
        }
        if sa_range.is_empty() {
            sa_range
        } else {
            let i = self.sa[sa_range.start];
            self.push_back_substr(
                i..i + lcp_len,
                self.find_substr(s_substr.clone()),
                s_substr.len(),
            )
        }
    }
}
