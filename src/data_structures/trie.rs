//! # Trie

const ALPHABET_SIZE: usize = 26;
const FIRST_CHAR: char = 'A';

#[derive(Default)]
struct Node {
    next: [Option<usize>; ALPHABET_SIZE],
    cnt_words: usize,
}

/// # Example
/// ```
/// use programming_team_code_rust::data_structures::trie::Trie;
///
/// let mut trie = Trie::default();
/// let hello = vec!['H', 'E', 'L', 'L', 'O'];
/// let world = vec!['W', 'O', 'R', 'L', 'D'];
/// let hello_world = vec!['H', 'E', 'L', 'L', 'O', 'W', 'O', 'R', 'L', 'D'];
/// trie.insert(&hello);
/// trie.insert(&hello);
/// trie.insert(&world);
/// assert_eq!(trie.find(&hello), 2);
/// assert_eq!(trie.find(&world), 1);
/// assert_eq!(trie.find(&hello_world), 0);
/// ```
pub struct Trie {
    t: Vec<Node>,
}

impl Default for Trie {
    fn default() -> Self {
        Trie {
            t: vec![Node::default()],
        }
    }
}

impl Trie {
    /// Insert a string into the Trie
    ///
    /// # Complexity
    /// - Time: O(|s|)
    /// - Space: O(|s|)
    pub fn insert(&mut self, s: &[char]) {
        let mut v = 0;
        for &ch in s {
            let idx = ch as usize - FIRST_CHAR as usize;
            if self.t[v].next[idx].is_none() {
                self.t[v].next[idx] = Some(self.t.len());
                self.t.push(Node::default());
            }
            v = self.t[v].next[idx].unwrap();
        }
        self.t[v].cnt_words += 1;
    }

    /// Find the number of times a string appears in the Trie
    ///
    /// # Complexity
    /// - Time: O(|s|)
    /// - Space: O(1)
    pub fn find(&self, s: &[char]) -> usize {
        let mut v = 0;
        for &ch in s {
            let idx = ch as usize - FIRST_CHAR as usize;
            if let Some(u) = self.t[v].next[idx] {
                v = u;
            } else {
                return 0;
            }
        }
        self.t[v].cnt_words
    }
}
