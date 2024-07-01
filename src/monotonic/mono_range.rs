pub fn mono_range(le: &[usize]) -> Vec<usize> {
    let mut ri = vec![le.len(); le.len()];
    for (i, &num) in le.iter().enumerate() {
        let mut j = i;
        while j != num {
            ri[j - 1] = i;
            j = le[j - 1];
        }
    }
    ri
}
