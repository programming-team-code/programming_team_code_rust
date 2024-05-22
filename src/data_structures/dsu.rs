//! # Disjoint Set Union

/// # Example
/// ```
/// use programming_team_code_rust::data_structures::dsu::DSU;
///
/// let mut dsu = DSU::new(10);
/// assert_eq!(dsu.same(1, 2), false);
/// dsu.unite(1, 3);
/// dsu.unite(2, 4);
/// assert_eq!(dsu.same(1, 4), false);
/// dsu.unite(2, 3);
/// assert_eq!(dsu.same(1, 4), true);
/// assert_eq!(dsu.leader(3), 2);
/// assert_eq!(dsu.leader(5), 5);
/// ```
pub struct DSU {
    e: Vec<i32>,
}

impl DSU {
    /// Create a new DSU with n elements
    ///
    /// # Complexity
    /// - Time: O(n)
    /// - Space: O(n)
    pub fn new(n: usize) -> Self {
        DSU { e: vec![-1; n] }
    }

    /// Get the leader of the group that x belongs to
    ///
    /// # Complexity
    /// - Time: O(α(n))
    /// - Space: O(1)
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

    /// Check if x and y belong to the same group
    ///
    /// # Complexity
    /// - Time: O(α(n))
    /// - Space: O(1)
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.leader(x) == self.leader(y)
    }

    /// Unite the groups that x and y belong to
    /// Returns true if x and y were in different groups
    /// Returns false if x and y were in the same group
    ///
    /// # Complexity
    /// - Time: O(α(n))
    /// - Space: O(1)
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
