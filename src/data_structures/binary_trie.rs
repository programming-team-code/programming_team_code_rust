//! # Binary Trie which can be used as a multiset of integers

#[derive(Default)]
struct Node {
    next: [Option<usize>; 2],
    sub_sz: isize,
}

/// # Example
/// ```
/// use programming_team_code_rust::data_structures::binary_trie::BinaryTrie;
///
/// let mut trie = BinaryTrie::default();
/// assert!(std::panic::catch_unwind(|| trie.min_xor(4)).is_err());
/// trie.update(1, 1);
/// assert!(std::panic::catch_unwind(|| trie.min_xor(4)).is_ok());
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
pub struct BinaryTrie {
    t: Vec<Node>,
}

impl Default for BinaryTrie {
    fn default() -> Self {
        BinaryTrie {
            t: vec![Node::default()],
        }
    }
}

impl BinaryTrie {
    /// Change the number of occurrences of `num` by `delta`
    ///
    /// # Complexity
    /// - Time: O(log(max_num))
    /// - Space: O(log(max_num))
    pub fn update(&mut self, num: usize, delta: isize) {
        let mut v = 0;
        for i in (0..usize::BITS).rev() {
            let bit = (num >> i) & 1;
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
    pub fn count(&self, num: usize) -> isize {
        let mut v = 0;
        for i in (0..usize::BITS).rev() {
            let bit = (num >> i) & 1;
            if self.t[v].next[bit].is_none() {
                return 0;
            }
            v = self.t[v].next[bit].unwrap();
        }
        self.t[v].sub_sz
    }

    /// Find the minimum xor of `num` and any number in the trie
    ///
    /// # Complexity
    /// - Time: O(log(max_num))
    /// - Space: O(1)
    pub fn min_xor(&self, num: usize) -> usize {
        assert!(self.t.len() > 1);
        let mut v = 0;
        let mut ans = 0;
        for i in (0..usize::BITS).rev() {
            let bit = (num >> i) & 1;
            if self.t[v].next[bit].is_some() && self.t[self.t[v].next[bit].unwrap()].sub_sz > 0 {
                v = self.t[v].next[bit].unwrap();
            } else {
                ans |= 1 << i;
                v = self.t[v].next[bit ^ 1].unwrap();
            }
        }
        ans
    }
}
