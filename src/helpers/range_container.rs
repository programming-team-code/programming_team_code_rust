//! # Interval Container

use std::collections::BTreeMap;
use std::ops::Range;

pub type T = i32;

#[derive(Default)]
pub struct RangeContainer {
    mp: BTreeMap<T, T>,
}

impl RangeContainer {
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

    pub fn insert_range(&mut self, mut range: Range<T>) {
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

    pub fn remove_range(&mut self, range: Range<T>) {
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

    pub fn get_range(&self, idx: T) -> Option<Range<T>> {
        if let Some((&le, &ri)) = self.mp.range(..=idx).next_back() {
            if idx < ri {
                return Some(le..ri);
            }
        }
        None
    }
}
