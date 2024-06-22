//! # Longest Increasing Subsequence

/// # Example
/// ```
/// use programming_team_code_rust::helpers::lis::Lis;
///
/// let a = [3, 3, 2, 3, 1];
///
/// let mut lis = Lis::default();
/// let mut prev = vec![None; a.len()];
/// for (i, &num) in a.iter().enumerate() {
///    prev[i] = lis.push(num);
/// }
///
/// assert_eq!(lis.dp.len(), 2); // length of LIS
///
/// // Get indexes of LIS
/// let mut idx = lis.dp.last().unwrap().1;
/// let mut idxs = Vec::with_capacity(lis.dp.len());
/// idxs.push(idx);
/// while let Some(prev_idx) = prev[idx] {
///    idx = prev_idx;
///    idxs.push(idx);
/// }
/// idxs.reverse();
/// assert_eq!(idxs, [2, 3]);
/// ```
pub struct Lis<T> {
    next_idx: usize,
    pub dp: Vec<(T, usize)>,
}

impl<T> Default for Lis<T> {
    fn default() -> Self {
        Self {
            next_idx: 0,
            dp: Vec::new(),
        }
    }
}

impl<T: Copy + Ord> Lis<T> {
    pub fn push(&mut self, new_elem: T) -> Option<usize> {
        // change to `elem <= new_elem` for longest non-decreasing subsequence
        let idx = self.dp.partition_point(|&(elem, _)| elem < new_elem);
        if idx == self.dp.len() {
            self.dp.push((new_elem, self.next_idx));
        } else if self.dp[idx].0 > new_elem {
            self.dp[idx] = (new_elem, self.next_idx);
        }
        self.next_idx += 1;
        match idx {
            0 => None,
            _ => Some(self.dp[idx - 1].1),
        }
    }
}
