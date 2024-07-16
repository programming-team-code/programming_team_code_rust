// verification-helper: PROBLEM https://judge.yosupo.jp/problem/set_xor_min

use proconio::input;
use programming_team_code_rust::data_structures::binary_trie::BinaryTrie;

fn main() {
    input! {
        q: usize,
    }

    let mut trie = BinaryTrie::default();
    for _ in 0..q {
        input! {
            t: u8,
            x: u32,
        }

        match t {
            0 => {
                if trie.count(x) == 0 {
                    trie.update(x, 1);
                }
            }
            1 => {
                if trie.count(x) == 1 {
                    trie.update(x, -1);
                }
            }
            _ => println!("{}", trie.min_xor(x)),
        }
    }
}
