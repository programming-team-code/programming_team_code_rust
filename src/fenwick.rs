pub struct Fenwick<T> {
    n: usize,
    ary: Vec<T>,
}
impl<T: Clone + Default + std::ops::AddAssign<T>> Fenwick<T> {
    pub fn new(n: usize) -> Self {
        Fenwick {
            n,
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
    pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
    where
        T: std::ops::AddAssign<U>,
    {
        let n = self.n;
        while idx < n {
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
