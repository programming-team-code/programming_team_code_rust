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
/// ```
pub struct Lis<T> {
    /// dp\[i\].0 = smallest number such that there exists a LIS of length i+1 ending in this number
    /// dp\[i\].1 = index in original array of dp\[i\].0
    pub dp: Vec<(T, usize)>,
    #[allow(clippy::type_complexity)]
    st: Vec<(Option<usize>, Option<(usize, (T, usize))>)>,
}

impl<T> Default for Lis<T> {
    fn default() -> Self {
        Self {
            dp: Vec::new(),
            st: Vec::new(),
        }
    }
}

impl<T: Copy + Ord> Lis<T> {
    pub fn push(&mut self, new_elem: T) {
        // change to `elem <= new_elem` for longest non-decreasing subsequence
        let idx = self.dp.partition_point(|&(elem, _)| elem < new_elem);
        let mut prev = None;
        if idx == self.dp.len() {
            self.dp.push((new_elem, self.st.len()));
        } else if self.dp[idx].0 > new_elem {
            // TODO fix bug only after test fails
            prev = Some((idx, self.dp[idx]));
            self.dp[idx] = (new_elem, self.st.len());
        }
        self.st.push((
            match idx {
                0 => None,
                _ => Some(self.dp[idx - 1].1),
            },
            prev,
        ));
    }

    pub fn pop(&mut self) {
        let (_, prev) = self.st.pop().unwrap();
        if let Some((idx, prev)) = prev {
            self.dp[idx] = prev;
        } else {
            self.dp.pop();
        }
    }

    pub fn get_lis(&self) -> Vec<usize> {
        let mut idxs = Vec::with_capacity(self.dp.len());
        let mut idx = self.dp.last().unwrap().1;
        idxs.push(idx);
        while let Some(prev) = self.st[idx].0 {
            idx = prev;
            idxs.push(idx);
        }
        idxs.reverse();
        idxs
    }
}
