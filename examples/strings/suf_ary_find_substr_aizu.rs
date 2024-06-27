// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B

use proconio::input;
use programming_team_code_rust::strings::suf_ary::SufAry;

fn main() {
    input! {
        s: String,
        t: String
    }

    let n = s.chars().count();
    let m = t.chars().count();
    let mut both = t.chars().map(|x| x as usize).collect::<Vec<usize>>();
    both.extend(s.chars().map(|x| x as usize).collect::<Vec<usize>>());

    let suf_ary = SufAry::new(&both, 255);
    let range = suf_ary.find_substr(0..m);

    {
        let mut splits = (0..6)
            .map(|_| rand::random::<usize>() % (m + 1))
            .collect::<Vec<_>>();
        splits.push(0);
        splits.push(m);
        splits.sort();
        let subarrays = (1..splits.len())
            .map(|i| splits[i - 1]..splits[i])
            .collect::<Vec<_>>();
        let mut push_pop_range = 0..both.len();
        let mut push_pop_lcp_len = 0;
        let mut range_subarray = subarrays.len() / 2..subarrays.len() / 2;
        loop {
            let mut found = false;
            if range_subarray.start > 0 {
                found = true;
                range_subarray.start -= 1;
                let curr_substr = &subarrays[range_subarray.start];
                push_pop_range = suf_ary.push_front_substr(
                    curr_substr.clone(),
                    push_pop_range,
                    push_pop_lcp_len,
                );
                push_pop_lcp_len += curr_substr.len();
            }

            if range_subarray.end < subarrays.len() {
                found = true;
                let curr_substr = &subarrays[range_subarray.end];
                push_pop_range =
                    suf_ary.push_back_substr(curr_substr.clone(), push_pop_range, push_pop_lcp_len);
                push_pop_lcp_len += curr_substr.len();
                range_subarray.end += 1;
            }

            if !found {
                break;
            }
        }
        assert_eq!(range.is_empty(), push_pop_range.is_empty());
        if !range.is_empty() {
            assert_eq!(range, push_pop_range);
        }
    }

    let mut res = suf_ary.sa[range]
        .iter()
        .copied()
        .filter(|&i| i >= m)
        .map(|i| i - m)
        .collect::<Vec<usize>>();
    res.sort();

    for val in res {
        println!("{}", val);
    }
}
