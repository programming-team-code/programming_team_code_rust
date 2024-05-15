pub struct DSU {
    e: Vec<i32>,
}
impl DSU {
    pub fn new(n: usize) -> Self {
        DSU { e: vec![-1; n] }
    }
    pub fn leader(&mut self, x: usize) -> usize {
        if self.e[x] < 0 {
            x
        } else {
            let p = self.e[x] as usize;
            let r = self.leader(p);
            self.e[x] = r as i32;
            r
        }
    }
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.leader(x) == self.leader(y)
    }
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.leader(x);
        let mut y = self.leader(y);
        if x == y {
            return false;
        }
        if self.e[x] > self.e[y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.e[x] += self.e[y];
        self.e[y] = x as i32;
        true
    }
}
