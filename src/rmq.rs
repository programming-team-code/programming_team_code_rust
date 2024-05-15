pub struct RMQ<T> {
    t: Vec<Vec<T>>,
    op: fn(T, T) -> T,
}
impl<T: Ord + Copy> RMQ<T> {
    pub fn new(a: &Vec<T>, op: fn(T, T) -> T) -> Self {
        let mut t = vec![a.clone(); 1];
        let mut i = 0;
        while (2 << i) <= a.len() {
            t.push(Vec::with_capacity(t[i].len() - (1 << i)));
            for j in 0..t[i].len() - (1 << i) {
                let x = op(t[i][j], t[i][j + (1 << i)]);
                t[i + 1].push(x);
            }
            i += 1;
        }
        Self { t, op }
    }

    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        let lg = range.len().ilog2() as usize;
        (self.op)(self.t[lg][range.start], self.t[lg][range.end - (1 << lg)])
    }
}
