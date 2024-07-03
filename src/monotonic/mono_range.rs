pub fn mono_range(le: &[usize]) -> Vec<usize> {
    let mut ri = vec![le.len(); le.len()];
    for (i, &num) in le.iter().enumerate().skip(1) {
        let mut j = i - 1;
        while j != num {
            ri[j] = i;
            j = le[j];
        }
    }
    ri
}
