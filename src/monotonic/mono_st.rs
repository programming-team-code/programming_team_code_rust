pub fn mono_st<T: Ord, F: Fn(&T, &T) -> bool>(a: &[T], cmp: F) -> Vec<usize> {
    let mut le = (0..a.len()).collect::<Vec<_>>();
    for (i, num) in a.iter().enumerate() {
        while le[i] > 0 && !cmp(&a[le[i] - 1], num) {
            le[i] = le[le[i] - 1];
        }
    }
    le
}
