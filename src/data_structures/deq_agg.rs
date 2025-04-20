//! # Deque with Aggregate

/// - see <https://github.com/suisen-cp/cp-library-cpp/blob/main/library/datastructure/deque_aggregation.hpp>
///
/// simulate a deque with 2 stacks:
/// `le`, `ri` are stacks of { number, sum }
///     accumulate
///    <-----------  -------> fold numbers from inside
///   (     le     ][  ri    )
///
/// # Example
/// ```
/// use programming_team_code_rust::data_structures::deq_agg::DeqAgg;
///
/// let mut deq = DeqAgg::new(|&x, &y| std::cmp::max(x, y));
/// deq.push_front(5);
/// deq.push_front(3);
/// deq.push_back(1);
/// assert_eq!(deq[1], 5);
/// assert!(std::panic::catch_unwind(|| deq[3]).is_err());
/// assert_eq!(deq.query(), Some(5));
/// assert_eq!(deq.front(), Some(&3));
/// assert_eq!(deq.pop_front(), Some(3));
/// assert_eq!(deq.back(), Some(&1));
/// assert_eq!(deq.pop_back(), Some(1));
/// ```
pub struct DeqAgg<T, F> {
    le: Vec<(T, T)>,
    ri: Vec<(T, T)>,
    op: F,
}

impl<T: Clone, F: Fn(&T, &T) -> T> DeqAgg<T, F> {
    /// Creates new instance of DeqAgg
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn new(op: F) -> Self {
        Self {
            le: vec![],
            ri: vec![],
            op,
        }
    }

    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn len(&self) -> usize {
        self.le.len() + self.ri.len()
    }

    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn is_empty(&self) -> bool {
        self.le.is_empty() && self.ri.is_empty()
    }

    /// Gets deq\[0\] op deq\[1\] op ... op deq.back(); or None if empty
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn query(&self) -> Option<T> {
        if self.is_empty() {
            None
        } else if self.le.is_empty() {
            Some(self.ri.last().unwrap().1.clone())
        } else if self.ri.is_empty() {
            Some(self.le.last().unwrap().1.clone())
        } else {
            Some((self.op)(
                &self.le.last().unwrap().1,
                &self.ri.last().unwrap().1,
            ))
        }
    }

    /// Gets deq\[0\]; or None if empty
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn front(&self) -> Option<&T> {
        if let Some(last) = self.le.last() {
            Some(&last.0)
        } else if !self.ri.is_empty() {
            Some(&self.ri[0].0)
        } else {
            None
        }
    }

    /// Gets deq\[deq.len() - 1\]; or None if empty
    ///
    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn back(&self) -> Option<&T> {
        if let Some(last) = self.ri.last() {
            Some(&last.0)
        } else if !self.le.is_empty() {
            Some(&self.le[0].0)
        } else {
            None
        }
    }

    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn push_front(&mut self, elem: T) {
        self.le.push((
            elem.clone(),
            if let Some(last) = self.le.last() {
                (self.op)(&elem, &last.1)
            } else {
                elem
            },
        ));
    }

    /// # Complexity
    /// - Time: O(1)
    /// - Space: O(1)
    pub fn push_back(&mut self, elem: T) {
        self.ri.push((
            elem.clone(),
            if let Some(last) = self.ri.last() {
                (self.op)(&last.1, &elem)
            } else {
                elem
            },
        ));
    }

    /// Removes front, and returns it; or None if empty
    ///
    /// # Complexity
    /// - Time: O(1) ammortized
    /// - Space: O(1) ammortized
    pub fn pop_front(&mut self) -> Option<T> {
        if self.le.is_empty() {
            let mut ary = vec![];
            std::mem::swap(&mut ary, &mut self.ri);
            self.rebuild(ary.len().div_ceil(2), ary);
        }
        self.le.pop().map(|elem| elem.0)
    }

    /// Removes back, and returns it; or None if empty
    ///
    /// # Complexity
    /// - Time: O(1) ammortized
    /// - Space: O(1) ammortized
    pub fn pop_back(&mut self) -> Option<T> {
        if self.ri.is_empty() {
            let mut ary = vec![];
            std::mem::swap(&mut ary, &mut self.le);
            ary.reverse();
            self.rebuild(ary.len() / 2, ary);
        }
        self.ri.pop().map(|elem| elem.0)
    }

    fn rebuild(&mut self, le_len: usize, mut ary: Vec<(T, T)>) {
        self.ri = ary.split_off(le_len);
        self.le = ary;
        self.le.reverse();
        for i in 0..self.le.len() {
            self.le[i].1 = if i == 0 {
                self.le[0].0.clone()
            } else {
                (self.op)(&self.le[i].0, &self.le[i - 1].1)
            };
        }
        for i in 0..self.ri.len() {
            self.ri[i].1 = if i == 0 {
                self.ri[0].0.clone()
            } else {
                (self.op)(&self.ri[i - 1].1, &self.ri[i].0)
            };
        }
    }
}

/// # Complexity
/// - Time: O(1)
/// - Space: O(1)
impl<T, F> std::ops::Index<usize> for DeqAgg<T, F> {
    type Output = T;
    fn index(&self, i: usize) -> &Self::Output {
        if i < self.le.len() {
            &self.le[self.le.len() - i - 1].0
        } else {
            &self.ri[i - self.le.len()].0
        }
    }
}
