//! Skew Heap Lazy

use std::mem::swap;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Heap<T: Clone> {
    pub cost: i64,
    pub value: T,
    pub lazy: Option<i64>,
    pub left: Option<Box<Heap<T>>>,
    pub right: Option<Box<Heap<T>>>,
}

impl<T: Clone> Heap<T> {
    pub fn new(cost: i64, value: T) -> Option<Box<Heap<T>>> {
        Some(Box::new(Heap {
            cost,
            value,
            lazy: None,
            left: None,
            right: None,
        }))
    }
}
#[derive(Default, Clone)]
pub struct SkewHeap<T: Clone> {
    node: Option<Box<Heap<T>>>,
}
impl<T: Clone> SkewHeap<T> {
    pub fn new() -> Self {
        Self { node: None }
    }

    #[inline]
    pub fn push(&mut self, cost: i64, value: T) {
        SkewHeap::merge(&mut self.node, Heap::new(cost, value));
    }
    #[inline]
    pub fn top(&self) -> Option<(i64, T)> {
        Some((self.node.as_ref()?.cost, self.node.as_ref()?.value.clone()))
    }
    #[inline]
    pub fn pop(&mut self) -> Option<(i64, T)> {
        Self::propagate(&mut self.node);
        let value = self.top()?;

        let (mut left, right) = {
            let mut tmp = self.node.take().unwrap();
            (tmp.left.take(), tmp.right.take())
        };
        SkewHeap::merge(&mut left, right);
        swap(&mut self.node, &mut left);

        Some(value)
    }

    #[inline]
    pub fn merge(a: &mut Option<Box<Heap<T>>>, mut b: Option<Box<Heap<T>>>) {
        if a.is_none() {
            swap(a, &mut b);
            return;
        }
        if b.is_none() {
            return;
        }
        Self::propagate(a);
        Self::propagate(&mut b);

        if a.as_ref().unwrap().cost > b.as_ref().unwrap().cost {
            swap(a, &mut b);
        }
        SkewHeap::merge(&mut a.as_mut().unwrap().right, b);

        let tmp = a.as_mut().unwrap();
        swap(&mut tmp.left, &mut tmp.right);
    }

    #[inline]
    pub fn add(&mut self, value: i64) {
        self.node.as_mut().unwrap().lazy = Some(value);
        Self::propagate(&mut self.node);
    }
    #[inline]
    fn propagate(node: &mut Option<Box<Heap<T>>>) {
        if let Some(n) = node.as_mut() {
            if n.lazy.is_none() {
                return;
            }
            if let Some(l) = n.left.as_mut() {
                l.lazy = n.lazy;
            }
            if let Some(r) = n.right.as_mut() {
                r.lazy = n.lazy;
            }

            n.cost += n.lazy.unwrap();
            n.lazy = None;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_heap() {
        let mut a = vec![SkewHeap::new(); 5];

        for i in 0..30 {
            a[i % 5].push(i as i64, 0);
        }

        for (i, e) in a.iter().enumerate() {
            assert_eq!(e.top().unwrap().0, i as i64);
        }

        for i in 1..5 {
            let buff = a[i].node.take();
            SkewHeap::merge(&mut a[0].node, buff);
        }

        for i in 0..15 {
            assert_eq!(a[0].pop().unwrap().0, i);
        }
        a[0].add(5);
        for i in 15..30 {
            assert_eq!(a[0].pop().unwrap().0, i + 5);
        }
    }
}
