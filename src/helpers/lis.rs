//! # Longest Increasing Subsequence

pub struct Lis<T> {
    next_idx: usize,
    pub a: Vec<(T, usize)>,
}

impl<T> Default for Lis<T> {
    fn default() -> Self {
        Self {
            next_idx: 0,
            a: Vec::new(),
        }
    }
}

impl<T: Copy + Ord> Lis<T> {
    pub fn push(&mut self, new_elem: T) -> Option<usize> {
        let idx = self.a.partition_point(|&(elem, _)| elem < new_elem);
        if idx == self.a.len() {
            self.a.push((new_elem, self.next_idx));
        } else if self.a[idx].0 > new_elem {
            self.a[idx] = (new_elem, self.next_idx);
        }
        self.next_idx += 1;
        match idx {
            0 => None,
            _ => Some(self.a[idx - 1].1),
        }
    }
}
