//! # Binary Trie which can be used as a multiset of integers

use std::mem::size_of;
use std::ops::{BitAnd, BitOrAssign, Shl, Shr};

#[derive(Default)]
struct Node {
    next: [Option<usize>; 2],
    sub_sz: isize,
}

/// # Example
/// ```
/// use programming_team_code_rust::data_structures::binary_trie::BinaryTrie;
///
/// let mut trie = BinaryTrie::<usize>::default();
/// trie.update(1, 1);
/// trie.update(2, 1);
/// trie.update(3, 1);
/// trie.update(2, -1);
/// assert_eq!(trie.count(2), 0);
/// assert_eq!(trie.count(3), 1);
/// assert_eq!(trie.count(4), 0);
/// assert_eq!(trie.min_xor(0), 1);
/// assert_eq!(trie.min_xor(1), 0);
/// assert_eq!(trie.min_xor(2), 1);
/// assert_eq!(trie.min_xor(3), 0);
/// assert_eq!(trie.min_xor(4), 5);
/// ```
pub struct BinaryTrie<T> {
    t: Vec<Node>,
    mx_bit: usize,
    _marker: std::marker::PhantomData<T>,
}

impl<T> Default for BinaryTrie<T> {
    fn default() -> Self {
        BinaryTrie {
            t: vec![Node::default()],
            mx_bit: size_of::<T>() * 8,
            _marker: std::marker::PhantomData,
        }
    }
}

impl<T> BinaryTrie<T>
where
    T: Shr<Output = T>
        + Shl<Output = T>
        + BitAnd<Output = T>
        + BitOrAssign
        + From<u8>
        + Into<usize>
        + Copy
        + Default,
{
    /// Change the number of occurrences of `num` by `delta`
    ///
    /// # Complexity
    /// - Time: O(log(max_num))
    /// - Space: O(log(max_num))
    pub fn update(&mut self, num: T, delta: isize) {
        let mut v = 0;
        for i in (0..self.mx_bit).rev() {
            let bit = ((num >> T::from(i as u8)) & T::from(1)).into();
            if self.t[v].next[bit].is_none() {
                self.t[v].next[bit] = Some(self.t.len());
                self.t.push(Node::default());
            }
            self.t[v].sub_sz += delta;
            v = self.t[v].next[bit].unwrap();
        }
        self.t[v].sub_sz += delta;
    }

    /// Count the number of occurrences of `num`
    ///
    /// # Complexity
    /// - Time: O(log(max_num))
    /// - Space: O(1)
    pub fn count(&self, num: T) -> isize {
        let mut v = 0;
        for i in (0..self.mx_bit).rev() {
            let bit = ((num >> T::from(i as u8)) & T::from(1)).into();
            if self.t[v].next[bit].is_none() {
                return 0;
            }
            v = self.t[v].next[bit].unwrap();
        }
        self.t[v].sub_sz
    }

    /// Find the minimum xor of `num` and any number in the trie
    ///
    /// # Panics
    /// - if the trie is empty
    ///
    /// # Complexity
    /// - Time: O(log(max_num))
    /// - Space: O(1)
    pub fn min_xor(&self, num: T) -> T {
        assert!(self.t.len() > 1);
        let mut v = 0;
        let mut ans = T::default();
        for i in (0..self.mx_bit).rev() {
            let bit = ((num >> T::from(i as u8)) & T::from(1)).into();
            if self.t[v].next[bit].is_some() && self.t[self.t[v].next[bit].unwrap()].sub_sz > 0 {
                v = self.t[v].next[bit].unwrap();
            } else {
                ans |= T::from(1) << T::from(i as u8);
                v = self.t[v].next[bit ^ 1].unwrap();
            }
        }
        ans
    }
}
