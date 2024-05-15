pub struct RMQ<T> {
    t: Vec<Vec<T>>,
    op: fn(T, T) -> T,
}
impl<T: Ord + Copy> RMQ<T> {
    pub fn new(a: &Vec<T>, op: fn(T, T) -> T) -> Self {
        let mut t = vec![a.clone(); 1];
        let mut i = 0;
        while (2 << i) <= a.len() {
            let mut new_row = vec![];
            new_row.reserve(t[i].len() - (1 << i));
            for j in 0..t[i].len() - (1 << i) {
                new_row.push(op(t[i][j], t[i][j + (1 << i)]));
            }
            t.push(new_row);
            i += 1;
        }
        Self { t, op }
    }

    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        let lg = range.len().ilog2() as usize;
        (self.op)(self.t[lg][range.start], self.t[lg][range.end - (1 << lg)])
    }
}
