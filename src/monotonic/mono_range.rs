pub fn mono_range(le: &[Option<usize>]) -> Vec<usize> {
    let mut ri = vec![le.len(); le.len()];
    for (i, &num) in le.iter().enumerate().skip(1) {
        let mut j = Some(i - 1);
        while j != num {
            ri[j.unwrap()] = i;
            j = le[j.unwrap()];
        }
    }
    ri
}
