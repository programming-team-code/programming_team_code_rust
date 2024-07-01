pub fn mono_st<T: Ord>(a: &[T]) -> Vec<Option<usize>> {
    let mut le = vec![None; a.len()];
    for (i, num) in a.iter().enumerate() {
        le[i] = Some(i - 1);
        while le[i].is_some() && a[le[i].unwrap()] < *num {
            le[i] = le[le[i].unwrap()];
        }
    }
    le
}
