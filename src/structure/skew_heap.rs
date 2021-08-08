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

#[inline]
pub fn merge<T: Ord + Clone>(a: &mut Option<Box<Heap<T>>>, mut b: Option<Box<Heap<T>>>) {
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
    merge(&mut a.as_mut().unwrap().right, b);

    let tmp = a.as_mut().unwrap();
    swap(&mut tmp.left, &mut tmp.right);
}

#[inline]
pub fn push<T: Ord + Clone>(heap: &mut Option<Box<Heap<T>>>, value: T) {
    merge(heap, Heap::new(value));
}

#[inline]
pub fn top<T: Ord + Clone>(heap: &Option<Box<Heap<T>>>) -> Option<T> {
    Some(heap.as_ref()?.value.clone())
}

#[inline]
pub fn pop<T: Ord + Clone>(heap: &mut Option<Box<Heap<T>>>) -> Option<T> {
    let value = top(heap)?;

    let (mut left, right) = {
        let mut tmp = heap.take().unwrap();
        (tmp.left.take(), tmp.right.take())
    };
    merge(&mut left, right);
    swap(heap, &mut left);

    Some(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_heap() {
        let mut a = vec![None; 5];

        for i in 0..30 {
            push(&mut a[i % 5], i);
        }

        for (i, e) in a.iter().enumerate() {
            assert_eq!(top(e).unwrap(), i);
        }

        for i in 1..5 {
            let buff = a[i].take();
            merge(&mut a[0], buff);
        }

        for i in 0..30 {
            assert_eq!(pop(&mut a[0]).unwrap(), i);
        }
    }
}
