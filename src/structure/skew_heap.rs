//! Skew Heap

use std::mem::swap;
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Heap<T: Ord + Clone> {
    pub value: T,
    pub left: Option<Box<Heap<T>>>,
    pub right: Option<Box<Heap<T>>>,
}

impl<T: Ord + Clone> Heap<T> {
    pub fn new(value: T) -> Option<Box<Heap<T>>> {
        Some(Box::new(Heap {
            value,
            left: None,
            right: None,
        }))
    }
}
#[derive(Default, Clone)]
pub struct SkewHeap<T: Ord + Clone> {
    node: Option<Box<Heap<T>>>,
}
impl<T: Ord + Clone> SkewHeap<T> {
    pub fn new() -> Self {
        Self { node: None }
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        SkewHeap::merge(&mut self.node, Heap::new(value));
    }
    #[inline]
    pub fn top(&self) -> Option<T> {
        Some(self.node.as_ref()?.value.clone())
    }
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
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
        if a > &mut b {
            swap(a, &mut b);
        }
        SkewHeap::merge(&mut a.as_mut().unwrap().right, b);

        let tmp = a.as_mut().unwrap();
        swap(&mut tmp.left, &mut tmp.right);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_heap() {
        let mut a = vec![SkewHeap::new(); 5];

        for i in 0..30 {
            a[i % 5].push(i);
        }

        for (i, e) in a.iter().enumerate() {
            assert_eq!(e.top().unwrap(), i);
        }

        for i in 1..5 {
            let buff = a[i].node.take();
            SkewHeap::merge(&mut a[0].node, buff);
        }

        for i in 0..30 {
            assert_eq!(a[0].pop().unwrap(), i);
        }
    }
}
