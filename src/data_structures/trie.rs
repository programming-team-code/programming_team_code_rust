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
/// trie.insert("HELLO");
/// trie.insert("HELLO");
/// trie.insert("WORLD");
/// assert_eq!(trie.find("HELLO"), 2);
/// assert_eq!(trie.find("WORLD"), 1);
/// assert_eq!(trie.find("WORLDS"), 0);
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
    pub fn insert(&mut self, s: &str) {
        let mut v = 0;
        for ch in s.chars() {
            let idx = (ch as u8 - FIRST_CHAR as u8) as usize;
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
    pub fn find(&self, s: &str) -> usize {
        let mut v = 0;
        for ch in s.chars() {
            let idx = (ch as u8 - FIRST_CHAR as u8) as usize;
            if self.t[v].next[idx].is_none() {
                return 0;
            }
            v = self.t[v].next[idx].unwrap();
        }
        self.t[v].cnt_words
    }
}
