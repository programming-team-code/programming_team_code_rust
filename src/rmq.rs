pub struct RMQ<T> {
    t: Vec<Vec<T>>,
    op: fn(T, T) -> T,
}
impl<T: Copy> RMQ<T> {
    pub fn new(a: &[T], op: fn(T, T) -> T) -> Self {
        let mut t = vec![a.to_owned(); 1];
        let mut i = 0;
        while (2 << i) <= a.len() {
            t.push(
                (0..t[i].len() - (1 << i))
                    .map(|j| op(t[i][j], t[i][j + (1 << i)]))
                    .collect(),
            );
            i += 1;
        }
        Self { t, op }
    }

    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        let lg = range.len().ilog2() as usize;
        (self.op)(self.t[lg][range.start], self.t[lg][range.end - (1 << lg)])
    }

    pub fn query_unused_function(&self, range: std::ops::Range<usize>) -> T {
        let lg = range.len().ilog2() as usize;
        (self.op)(self.t[lg][range.start], self.t[lg][range.end - (1 << lg)])
    }
}
