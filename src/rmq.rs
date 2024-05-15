pub struct RMQ<T> {
    t: Vec<Vec<T>>,
}
impl<T: Ord + Copy> RMQ<T> {
    pub fn new(a: &Vec<T>) -> Self {
        let mut t = vec![a.clone(); 1];
        let mut i = 0;
        while (2 << i) <= a.len() {
            let mut new_row = vec![];
            for j in 0..t[i].len() - (1 << i) {
                new_row.push(t[i][j].min(t[i][j + (1 << i)]));
            }
            t.push(new_row);
            i += 1;
        }
        Self { t }
    }

    pub fn query(&self, range: std::ops::Range<usize>) -> T {
        let len = range.end - range.start;
        let lg = len.ilog2() as usize;
        self.t[lg][range.start].min(self.t[lg][range.end - (1 << lg)])
    }
}
