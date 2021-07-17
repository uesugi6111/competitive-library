//! BIT

pub trait Monoid {
    type T: Clone;
    fn identity_element() -> Self::T;
    fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T;
}

pub struct Add {}
impl Monoid for Add {
    type T = i64;
    #[inline]
    fn identity_element() -> Self::T {
        0_i64
    }
    #[inline]
    fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T {
        *a + *b
    }
}

/// Binary Index Tree
#[derive(Clone, Debug)]
pub struct FenwickTree<M>
where
    M: Monoid,
{
    array: Vec<M::T>,
}

impl<M> FenwickTree<M>
where
    M: Monoid,
{
    #[inline]
    pub fn new(size: usize) -> FenwickTree<M> {
        Self {
            array: vec![M::identity_element(); size + 1],
        }
    }

    #[inline]
    pub fn operate(&mut self, index: usize, x: M::T) {
        let mut i = index + 1;
        while i < self.array.len() {
            self.array[i] = M::binary_operation(&self.array[i], &x);
            i += i & i.wrapping_neg();
        }
    }

    /// (0..end)
    #[inline]
    pub fn fold(&self, end: usize) -> M::T {
        let mut s = M::identity_element();
        let mut i = end;
        while i > 0 {
            s = M::binary_operation(&s, &self.array[i]);
            i -= i & i.wrapping_neg();
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum() {
        let mut a = FenwickTree::<Add>::new(100);

        (0..100).for_each(|i| a.operate(i, i as i64 + 1));

        (0..100).for_each(|i| assert_eq!((1..=i).sum::<i64>(), a.fold(i as usize)));
    }

    pub struct Xor {}
    impl Monoid for Xor {
        type T = u64;
        #[inline]
        fn identity_element() -> Self::T {
            0_u64
        }
        #[inline]
        fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T {
            *a ^ *b
        }
    }
    #[test]
    fn test_xor() {
        // https://atcoder.jp/contests/abc185/tasks/abc185_f
        // sample 2
        let a = vec![0, 5, 3, 4, 7, 0, 0, 0, 1, 0];
        let txy_ans = vec![
            (1, 10, 7, 0),
            (2, 8, 9, 1),
            (2, 3, 6, 0),
            (2, 1, 6, 5),
            (2, 1, 10, 3),
            (1, 9, 4, 0),
            (1, 6, 1, 0),
            (1, 6, 3, 0),
            (1, 1, 7, 0),
            (2, 3, 5, 0),
        ];

        let mut ft = FenwickTree::<Xor>::new(10);

        for (i, &v) in a.iter().enumerate() {
            ft.operate(i, v);
        }

        for (t, x, y, ans) in txy_ans {
            if t == 1 {
                ft.operate(x as usize - 1, y);
            } else {
                assert_eq!(ft.fold(y as usize) ^ ft.fold(x as usize - 1), ans);
            }
        }
    }
}
