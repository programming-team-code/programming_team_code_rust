//! # Range Container

use std::collections::BTreeMap;
use std::ops::Range;

/// # Example
/// ```
/// use programming_team_code_rust::data_structures::range_container::RangeContainer;
///
/// let mut rc = RangeContainer::default();
/// rc.insert_range(-2..2);
/// rc.insert_range(2..3);
/// assert_eq!(rc.get_range(0), Some(-2..3));
/// assert_eq!(rc.get_range(3), None);
/// rc.remove_range(-1..1);
///
/// assert_eq!(rc.get_range(-2), Some(-2..-1));
/// assert_eq!(rc.get_range(2), Some(1..3));
/// ```
#[derive(Default)]
pub struct RangeContainer<T> {
    /// an entry le -> ri represents a range [le, ri)
    /// invariant: previous_ri < current_le
    pub mp: BTreeMap<T, T>,
}

impl<T: Copy + Ord> RangeContainer<T> {
    fn remove(&mut self, range: &Range<T>) -> Option<T> {
        let mut last_ri = None;
        for (le, ri) in self
            .mp
            .range(range.start..=range.end)
            .map(|(&le, &ri)| (le, ri))
            .collect::<Vec<_>>()
        {
            self.mp.remove(&le);
            last_ri = Some(ri);
        }
        last_ri
    }

    /// Inserts a range into the container
    ///
    /// # Complexity
    /// - Time: O(log n) ammortized
    /// - Space: O(1) ammortized
    pub fn insert_range(&mut self, mut range: Range<T>) {
        assert!(!range.is_empty());
        if let Some(last_ri) = self.remove(&range) {
            range.end = std::cmp::max(range.end, last_ri);
        }
        if let Some((_, ri)) = self.mp.range_mut(..range.start).next_back() {
            if *ri >= range.start {
                *ri = std::cmp::max(*ri, range.end);
                return;
            }
        }
        self.mp.insert(range.start, range.end);
    }

    /// Removes a range from the container
    ///
    /// # Complexity
    /// - Time: O(log n) ammortized
    /// - Space: O(1) ammortized
    pub fn remove_range(&mut self, range: Range<T>) {
        assert!(!range.is_empty());
        if let Some(last_ri) = self.remove(&range) {
            if range.end < last_ri {
                self.mp.insert(range.end, last_ri);
            }
        }
        if let Some((_, ri)) = self.mp.range_mut(..range.start).next_back() {
            let ri = std::mem::replace(ri, std::cmp::min(*ri, range.start));
            if range.end < ri {
                self.mp.insert(range.end, ri);
            }
        }
    }

    /// Gets range containing idx
    ///
    /// # Complexity
    /// - Time: O(log n)
    /// - Space: O(1)
    pub fn get_range(&self, idx: T) -> Option<Range<T>> {
        if let Some((&le, &ri)) = self.mp.range(..=idx).next_back() {
            if idx < ri {
                return Some(le..ri);
            }
        }
        None
    }
}
