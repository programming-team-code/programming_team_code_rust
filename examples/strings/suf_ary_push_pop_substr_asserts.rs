use programming_team_code_rust::strings::suf_ary::SufAry;
use std::ops::Range;
pub fn suf_ary_push_pop_substr_asserts(
    tot_len: usize,
    suf_ary: &SufAry,
    range: &Range<usize>,
    s_to_look_for_range: &Range<usize>,
) {
    let mut splits = (0..6)
        .map(|_| {
            s_to_look_for_range.start + (rand::random::<usize>() % (s_to_look_for_range.len() + 1))
        })
        .collect::<Vec<_>>();
    splits.push(s_to_look_for_range.start);
    splits.push(s_to_look_for_range.end);
    splits.sort();
    let subarrays = (1..splits.len())
        .map(|i| splits[i - 1]..splits[i])
        .collect::<Vec<_>>();
    let mut push_pop_range = 0..tot_len;
    let mut push_pop_lcp_len = 0;
    let mut range_subarray = subarrays.len() / 2..subarrays.len() / 2;
    loop {
        let mut found = false;
        if range_subarray.start > 0 {
            found = true;
            range_subarray.start -= 1;
            let curr_substr = &subarrays[range_subarray.start];
            push_pop_range =
                suf_ary.push_front_substr(curr_substr.clone(), push_pop_range, push_pop_lcp_len);
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
    assert_eq!(*range, push_pop_range);
}
