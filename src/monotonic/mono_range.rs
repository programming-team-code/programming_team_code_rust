pub fn mono_range(le: &[usize]) -> Vec<usize> {
    let n = le.len();
    let mut ri = vec![n; n];
    for i in 1..n {
        let mut j = i - 1;
        while j != le[i] {
            ri[j] = i;
            j = le[j];
        }
    }
    ri
}
