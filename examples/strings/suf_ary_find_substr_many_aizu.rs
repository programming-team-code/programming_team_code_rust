// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/problems/ALDS1_14_D

use proconio::input;
use programming_team_code_rust::data_structures::rmq::RMQ;
use programming_team_code_rust::strings::suf_ary::SufAry;

mod suf_ary_push_pop_substr_asserts;
use suf_ary_push_pop_substr_asserts::suf_ary_push_pop_substr_asserts;

fn main() {
    input! {
        s: String,
        q: usize
    }

    let num_queries_find_substr = q.min(100);

    let mut s = s.chars().map(|x| x as usize).collect::<Vec<usize>>();

    let mut length = vec![s.len()];

    for _ in 0..num_queries_find_substr {
        input! {
            t: String,
        }
        let t = t.chars().map(|x| x as usize).collect::<Vec<usize>>();
        s.extend(t);
        length.push(s.len());
    }

    let suf_ary = SufAry::new(&s, 255);
    let rmq = RMQ::new(&suf_ary.sa, std::cmp::min);

    for i in 0..num_queries_find_substr {
        let s_to_look_for_range = length[i]..length[i + 1];
        let range = suf_ary.find_substr(s_to_look_for_range.clone());
        suf_ary_push_pop_substr_asserts(s.len(), &suf_ary, &range, &s_to_look_for_range);

        let idx = rmq.query(range);
        println!(
            "{}",
            (idx + length[i + 1] - length[i] <= length[0]) as usize
        );
    }

    for _ in num_queries_find_substr..q {
        input! {
            t: String,
        }
        let t = t.chars().map(|x| x as usize).collect::<Vec<usize>>();
        let match_range = suf_ary.find_str(&t);
        let idx = if match_range.is_empty() {
            *length.last().unwrap()
        } else {
            rmq.query(match_range)
        };
        println!("{}", (idx + t.len() <= length[0]) as usize);
    }
}
