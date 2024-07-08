//! # Interval Container

use std::collections::BTreeMap;
use std::ops::Range;

pub type T = i32;

#[derive(Default)]
pub struct RangeContainer {
    mp: BTreeMap<T, T>,
}

impl RangeContainer {
    fn remove(&mut self, range: Range<T>) -> Option<T> {
        let mut last_end = None;
        for (key, num) in self
            .mp
            .range(range.start..=range.end)
            .map(|(&key, &num)| (key, num))
            .collect::<Vec<_>>()
        {
            self.mp.remove(&key);
            last_end = Some(num);
        }
        last_end
    }

    pub fn insert_range(&mut self, mut range: Range<T>) {
        if let Some(last_end) = self.remove(range.clone()) {
            range.end = std::cmp::max(range.end, last_end);
        }
        if let Some((_, num)) = self.mp.range_mut(..range.start).next_back() {
            if *num >= range.start {
                *num = std::cmp::max(*num, range.end);
                return;
            }
        }
        self.mp.insert(range.start, range.end);
    }

    pub fn remove_range(&mut self, range: Range<T>) {
        if let Some(last_end) = self.remove(range.clone()) {
            if range.end < last_end {
                self.mp.insert(range.end, last_end);
            }
        }
        if let Some((_, num)) = self.mp.range_mut(..range.start).next_back() {
            let num = std::mem::replace(num, std::cmp::min(*num, range.start));
            if range.end < num {
                self.mp.insert(range.end, num);
            }
        }
    }

    pub fn get_range(&self, idx: T) -> Option<Range<T>> {
        if let Some((key, num)) = self.mp.range(..=idx).next_back() {
            if idx < *num {
                return Some(*key..*num);
            }
        }
        None
    }
}
