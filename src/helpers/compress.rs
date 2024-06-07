//! # Value Compression

// mention https://doc.rust-lang.org/nightly/std/vec/struct.Vec.html#method.split_off in example
// for compressing multiple arrays
pub fn compress<T: Ord>(a: &[T]) -> (Vec<usize>, usize) {
    let n = a.len();
    let mut idx = (0..n).collect::<Vec<usize>>();
    idx.sort_by_key(|&i| &a[i]);
    let mut compressed = vec![0; n];
    let mut max_val = 0;
    for i in 0..n {
        compressed[idx[i]] = max_val;
        if i + 1 == n || a[idx[i]] != a[idx[i + 1]] {
            max_val += 1;
        }
    }
    (compressed, max_val)
}
