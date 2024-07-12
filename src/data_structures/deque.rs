//! # Deque with op

pub struct Deque<T, F> {
    le: Vec<(T, T)>,
    ri: Vec<(T, T)>,
    op: F,
}

impl<T: Clone, F: Fn(&T, &T) -> T> Deque<T, F> {
    pub fn new(op: F) -> Self {
        Self {
            le: Vec::new(),
            ri: Vec::new(),
            op,
        }
    }

    pub fn len(&self) -> usize {
        self.le.len() + self.ri.len()
    }

    pub fn query(&self) -> Option<T> {
        if self.len() == 0 {
            None
        } else if self.le.is_empty() {
            Some(self.ri.last().unwrap().1.clone())
        } else if self.ri.is_empty() {
            Some(self.le.last().unwrap().1.clone())
        } else {
            Some((self.op)(
                &self.le.last().unwrap().1,
                &self.ri.last().unwrap().1,
            ))
        }
    }

    pub fn front(&self) -> Option<&T> {
        if let Some(last) = self.le.last() {
            Some(&last.0)
        } else if !self.ri.is_empty() {
            Some(&self.ri[0].0)
        } else {
            None
        }
    }

    pub fn back(&self) -> Option<&T> {
        if let Some(last) = self.ri.last() {
            Some(&last.0)
        } else if !self.le.is_empty() {
            Some(&self.le[0].0)
        } else {
            None
        }
    }

    pub fn push_front(&mut self, elem: T) {
        self.le.push((
            elem.clone(),
            if let Some(last) = self.le.last() {
                (self.op)(&elem, &last.1)
            } else {
                elem
            },
        ));
    }

    pub fn push_back(&mut self, elem: T) {
        self.ri.push((
            elem.clone(),
            if let Some(last) = self.ri.last() {
                (self.op)(&last.1, &elem)
            } else {
                elem
            },
        ));
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.le.is_empty() {
            let mut ary = vec![];
            std::mem::swap(&mut ary, &mut self.ri);
            self.rebuild((ary.len() + 1) / 2, ary);
        }
        self.le.pop().map(|elem| elem.0)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.ri.is_empty() {
            let mut ary = vec![];
            std::mem::swap(&mut ary, &mut self.le);
            ary.reverse();
            self.rebuild(ary.len() / 2, ary);
        }
        self.ri.pop().map(|elem| elem.0)
    }

    fn rebuild(&mut self, le_len: usize, mut ary: Vec<(T, T)>) {
        self.ri = ary.split_off(le_len);
        self.le = ary;
        self.le.reverse();
        if !self.le.is_empty() {
            self.le[0].1 = self.le[0].0.clone();
            for i in 1..self.le.len() {
                self.le[i].1 = (self.op)(&self.le[i].0, &self.le[i - 1].1);
            }
        }
        if !self.ri.is_empty() {
            self.ri[0].1 = self.ri[0].0.clone();
            for i in 1..self.ri.len() {
                self.ri[i].1 = (self.op)(&self.ri[i - 1].1, &self.ri[i].0);
            }
        }
    }
}
