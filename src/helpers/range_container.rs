//! # Interval Container

use std::collections::BTreeMap;
use std::ops::Range;

pub type T = i32;

#[derive(Default)]
pub struct RangeContainer {
    pub mp: BTreeMap<T, T>,
}

impl RangeContainer {
    fn remove(&mut self, range: Range<T>) -> Option<T> {
        let mut last_end = None;
        for (key, val) in self
            .mp
            .range(range.start..=range.end)
            .map(|(key, val)| (*key, *val))
            .collect::<Vec<_>>()
        {
            self.mp.remove(&key);
            last_end = Some(val);
        }
        last_end
    }

    pub fn insert_range(&mut self, mut range: Range<T>) {
        if let Some(last_end) = self.remove(range.clone()) {
            range.end = std::cmp::max(range.end, last_end);
        }
        if let Some((_, val)) = self.mp.range_mut(..range.start).next_back() {
            if *val >= range.start {
                *val = std::cmp::max(*val, range.end);
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
        if let Some((_, val)) = self.mp.range_mut(..range.start).next_back() {
            let tmp = *val;
            *val = std::cmp::min(*val, range.start);
            let val = tmp;
            if range.end < val {
                self.mp.insert(range.end, val);
            }
        }
    }
}
