// verification-helper: PROBLEM https://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ALDS1_14_B

use proconio::input;
use programming_team_code_rust::strings::suf_ary::SufAry;

fn main() {
    input! {
        s: String,
        t: String
    }

    let s = s.chars().map(|x| x as usize).collect::<Vec<usize>>();
    let t = t.chars().map(|x| x as usize).collect::<Vec<usize>>();

    let suf_ary = SufAry::new(&s, 255);
    let range = suf_ary.find_str(&t);

    let mut push_pop_sa_range = 0..s.len();
    let mut push_pop_t_range = t.len() / 2..t.len() / 2;
    loop {
        let mut found = false;

        if push_pop_t_range.start > 0 {
            found = true;
            let lcp_len = push_pop_t_range.len();
            push_pop_t_range.start -= 1;
            push_pop_sa_range =
                suf_ary.push_front_char(t[push_pop_t_range.start], push_pop_sa_range, lcp_len);
        }

        if push_pop_t_range.end < t.len() {
            found = true;
            push_pop_sa_range = suf_ary.push_back_char(
                t[push_pop_t_range.end],
                push_pop_sa_range,
                push_pop_t_range.len(),
            );
            push_pop_t_range.end += 1;
        }

        if !found {
            break;
        }
    }

    assert_eq!(range.is_empty(), push_pop_sa_range.is_empty());
    if !range.is_empty() {
        assert_eq!(range, push_pop_sa_range);
    }

    let mut res: Vec<usize> = Vec::new();
    res.extend_from_slice(&suf_ary.sa[range]);
    res.sort();

    for val in res {
        println!("{}", val);
    }
}
