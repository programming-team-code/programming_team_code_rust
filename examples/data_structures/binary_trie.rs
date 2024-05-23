// verification-helper: PROBLEM https://judge.yosupo.jp/problem/set_xor_min

use proconio::input;
use programming_team_code_rust::data_structures::binary_trie::BinaryTrie;

fn main() {
    input! {
        q: usize,
    }

    let mut trie = BinaryTrie::<usize>::default();
    for _ in 0..q {
        input! {
            t: u8,
            x: usize,
        }

        match t {
            0 => {
                trie.update(x, 1);
            }
            1 => {
                trie.update(x, -1);
            }
            2 => {
                println!("{}", trie.min_xor(x));
            }
            _ => unreachable!(),
        }
    }
}
