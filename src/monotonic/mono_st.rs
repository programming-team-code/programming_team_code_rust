pub fn mono_st<T: Ord>(a: &[T]) -> Vec<usize> {
    let mut le = (0..a.len()).collect::<Vec<_>>();
    for (i, num) in a.iter().enumerate() {
        while le[i] > 0 && a[le[i] - 1] < *num {
            le[i] = le[le[i] - 1];
        }
    }
    le
}
