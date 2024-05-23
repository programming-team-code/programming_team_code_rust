// verification-helper: PROBLEM https://judge.yosupo.jp/problem/set_xor_min

use proconio::input;
use programming_team_code_rust::data_structures::binary_trie::BinaryTrie;

fn main() {
    // test the compilation of the various useful types of binary tries
    {
        let _ = BinaryTrie::<u8>::default();
    }
    {
        let _ = BinaryTrie::<u16>::default();
    }
    {
        let _ = BinaryTrie::<u32>::default();
    }
    {
        let _ = BinaryTrie::<u64>::default();
    }
    {
        let _ = BinaryTrie::<u128>::default();
    }

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
                if trie.count(x) == 0 {
                    trie.update(x, 1);
                }
            }
            1 => {
                if trie.count(x) == 1 {
                    trie.update(x, -1);
                }
            }
            2 => {
                println!("{}", trie.min_xor(x));
            }
            _ => unreachable!(),
        }
    }
}
