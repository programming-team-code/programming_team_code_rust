pub fn mono_range(le: &[usize]) -> Vec<usize> {
    let mut ri = vec![le.len(); le.len()];
    for i in 0..le.len() {
        let mut j = i.wrapping_sub(1);
        while j != le[i] {
            ri[j] = i;
            j = le[j];
        }
    }
    ri
}
