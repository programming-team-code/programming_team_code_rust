//! # Interval Container

use std::collections::BTreeMap;
use std::ops::Range;

pub type T = i32;

#[derive(Default)]
pub struct RangeContainer {
    mp: BTreeMap<T, T>,
}

impl RangeContainer {
    pub fn add_range(&mut self, range: Range<T>) {
        let mut mx_val = range.end;
        for (key, val) in self
            .mp
            .range(range.start..=range.end)
            .map(|(key, val)| (*key, *val))
            .collect::<Vec<_>>()
        {
            self.mp.remove(&key);
            mx_val = mx_val.max(val);
        }
        if let Some((key, val)) = self.mp.range_mut(..range.start).next_back() {
            if *val >= range.start {
                *val = std::cmp::max(*val, range.end);
                return;
            }
        }
        self.mp.insert(range.start, range.end);
    }

    pub fn remove_range(&mut self, range: Range<T>) {}
}
