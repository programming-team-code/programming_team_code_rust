pub struct Fenwick<T> {
    ary: Vec<T>,
}
impl<T: Clone + Default + std::ops::AddAssign<T>> Fenwick<T> {
    pub fn new(n: usize) -> Self {
        Fenwick {
            ary: vec![T::default(); n],
        }
    }
    pub fn accum(&self, mut idx: usize) -> T {
        let mut sum = T::default();
        while idx > 0 {
            sum += self.ary[idx - 1].clone();
            idx &= idx - 1;
        }
        sum
    }
    /// performs data[idx] += val;
    pub fn add(&mut self, mut idx: usize, val: T) {
        while idx < self.ary.len() {
            self.ary[idx] += val.clone();
            idx |= idx + 1;
        }
    }
    /// Returns data[l] + ... + data[r - 1].
    pub fn sum(&self, range: std::ops::Range<usize>) -> T
    where
        T: std::ops::Sub<Output = T>,
    {
        self.accum(range.end) - self.accum(range.start)
    }
}