use programming_team_code_rust::strings::suf_ary::SufAry;
use std::ops::Range;
pub fn suf_ary_push_pop_char_asserts(
    n: usize,
    suf_ary: &SufAry,
    range: &Range<usize>,
    t: &[usize],
) {
    let mut push_pop_sa_range = 0..n;
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
        assert_eq!(*range, push_pop_sa_range);
    }
}
