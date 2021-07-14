//! BIT
use std::clone::Clone;
use std::convert::From;
use std::ops::{Add, AddAssign, Sub};

///binaryIndexTree
#[derive(Clone, Debug)]
pub struct FenwickTree<T> {
    array: Vec<T>,
}

impl<T> FenwickTree<T>
where
    T: Add + Sub + Clone + Copy + From<u8> + AddAssign,
{
    pub fn new(size: usize) -> FenwickTree<T> {
        let v: Vec<T> = vec![T::from(0u8); size + 1];
        Self { array: v }
    }
    pub fn add(&mut self, mut i: usize, x: T) {
        while i < self.array.len() {
            self.array[i] += x;
            i += i & i.wrapping_neg();
        }
    }
}

pub trait Sum<T, U> {
    fn sum(&self, i: T) -> U;
}

impl<T> Sum<usize, T> for FenwickTree<T>
where
    T: Add + Sub + Clone + Copy + From<u8> + AddAssign,
{
    fn sum(&self, mut i: usize) -> T {
        if i == 0 {
            return T::from(0u8);
        }
        let mut s = T::from(0u8);

        while i > 0 {
            s += self.array[i];
            i -= i & i.wrapping_neg();
        }
        s
    }
}

impl<T> Sum<(usize, usize), T> for FenwickTree<T>
where
    T: Add + Sub + Clone + Copy + From<u8> + AddAssign,
    T: std::ops::Sub<Output = T>,
{
    fn sum(&self, i: (usize, usize)) -> T {
        let sum_l = <FenwickTree<T> as Sum<usize, T>>::sum(self, i.0 - 1);
        let sum_r = <FenwickTree<T> as Sum<usize, T>>::sum(self, i.1);
        sum_r - sum_l
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum() {
        let mut a = FenwickTree::new(100);

        for i in 1..101 {
            a.add(i, i);
        }

        assert_eq!((0..101).sum::<usize>(), a.sum(100));
        assert_eq!((2..101).sum::<usize>(), a.sum((2, 100)));
    }
}
