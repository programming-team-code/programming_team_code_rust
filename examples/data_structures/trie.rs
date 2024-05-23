// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/all/ALDS1_4_C

use proconio::input;
use programming_team_code_rust::data_structures::trie::Trie;

fn main() {
    input! {
        n: usize,
    }

    let mut trie;
    for _ in 0..n {
        input! {
            command: String,
            key: String,
        }

        match command.as_str() {
            "insert" => {
                trie.insert(&key);
            }
            "find" => {
                if trie.find(&key) > 0 {
                    println!("yes");
                } else {
                    println!("no");
                }
            }
            _ => unreachable!(),
        }
    }
}
