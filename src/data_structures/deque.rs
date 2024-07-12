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
}
