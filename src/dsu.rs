/// Disjoint Set Union
///
/// # Examples
/// ```
/// use programming_team_code_rust::dsu::Dsu;
/// let mut dsu = Dsu::new(5);
/// assert_eq!(dsu.leader(0), 0);
/// assert_eq!(dsu.leader(1), 1);
/// assert_eq!(dsu.leader(2), 2);
/// assert_eq!(dsu.leader(3), 3);
/// assert_eq!(dsu.leader(4), 4);
/// assert_eq!(dsu.unite(0, 1), true);
/// assert_eq!(dsu.leader(0), 0);
/// assert_eq!(dsu.leader(1), 0);
/// assert_eq!(dsu.leader(2), 2);
/// assert_eq!(dsu.leader(3), 3);
/// assert_eq!(dsu.leader(4), 4);
/// assert_eq!(dsu.unite(1, 2), true);
/// assert_eq!(dsu.leader(0), 0);
/// assert_eq!(dsu.leader(1), 0);
/// assert_eq!(dsu.leader(2), 0);
/// assert_eq!(dsu.leader(3), 3);
/// assert_eq!(dsu.leader(4), 4);
/// assert_eq!(dsu.unite(3, 4), true);
/// assert_eq!(dsu.leader(0), 0);
/// assert_eq!(dsu.leader(1), 0);
/// assert_eq!(dsu.leader(2), 0);
/// assert_eq!(dsu.leader(3), 3);
/// assert_eq!(dsu.leader(4), 3);
/// assert_eq!(dsu.unite(0, 3), true);
/// assert_eq!(dsu.leader(0), 0);
/// assert_eq!(dsu.leader(1), 0);
/// assert_eq!(dsu.leader(2), 0);
/// assert_eq!(dsu.leader(3), 0);
/// assert_eq!(dsu.leader(4), 0);
/// ```
pub struct Dsu {
    e: Vec<i32>,
}
impl Dsu {
    /// Create a new Disjoint Set Union with n elements.
    pub fn new(n: usize) -> Self {
        Dsu { e: vec![-1; n] }
    }

    /// Find the leader of x.
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

    /// Determine if x and y are in the same set.
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.leader(x) == self.leader(y)
    }

    /// Unite the sets of x and y.
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
