//! セグメントツリー
use std::cmp::max;
use std::cmp::min;

pub trait Monoid {
    type T: Clone;
    fn identity_element() -> Self::T;
    fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T;
}
pub struct Min {}
impl Monoid for Min {
    type T = i64;
    #[inline]
    fn identity_element() -> Self::T {
        std::i32::MAX as i64
    }
    #[inline]
    fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T {
        min(*a, *b)
    }
}
pub struct Max {}
impl Monoid for Max {
    type T = i64;
    #[inline]
    fn identity_element() -> Self::T {
        std::i64::MIN
    }
    #[inline]
    fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T {
        max(*a, *b)
    }
}

pub struct SegmentTree<M>
where
    M: Monoid,
{
    n: usize,
    size: usize,
    log: usize,
    data: Vec<M::T>,
}
impl<M: Monoid> SegmentTree<M> {
    pub fn new(n: usize) -> SegmentTree<M> {
        vec![M::identity_element(); n].into()
    }
}
impl<M: Monoid> From<Vec<M::T>> for SegmentTree<M> {
    fn from(v: Vec<M::T>) -> Self {
        let n = v.len();
        let log = (32 - (n as u32).saturating_sub(1).leading_zeros()) as usize;
        let size = 1 << log;
        let data = {
            let mut data = vec![M::identity_element(); 2 * size];
            data[size..(size + n)].clone_from_slice(&v);
            data
        };
        {
            let mut sg = SegmentTree { n, size, log, data };
            (1..size).rev().for_each(|i| sg.update(i));
            sg
        }
    }
}
impl<M: Monoid> SegmentTree<M> {
    pub fn query(&self, mut l: usize, mut r: usize) -> M::T {
        let (mut sml, mut smr) = (M::identity_element(), M::identity_element());
        l += self.size;
        r += self.size;

        while l < r {
            if l & 1 != 0 {
                sml = M::binary_operation(&sml, unsafe { &self.data.get_unchecked(l) });
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = M::binary_operation(unsafe { &self.data.get_unchecked(r) }, &smr);
            }
            l >>= 1;
            r >>= 1;
        }

        M::binary_operation(&sml, &smr)
    }
    fn update(&mut self, k: usize) {
        *unsafe { self.data.get_unchecked_mut(k) } =
            M::binary_operation(unsafe { &self.data.get_unchecked(2 * k) }, unsafe {
                &self.data.get_unchecked(2 * k + 1)
            });
    }
    pub fn set(&mut self, mut p: usize, x: M::T) {
        p += self.size;
        self.data[p] = x;
        (1..=self.log).for_each(|i| self.update(p >> i));
    }
    pub fn get(&self, i: usize) -> M::T {
        self.data[i].clone()
    }
    pub fn max_right<F>(&self, mut l: usize, f: F) -> usize
    where
        F: Fn(&M::T) -> bool,
    {
        assert!(l <= self.n);
        assert!(f(&M::identity_element()));
        if l == self.n {
            return self.n;
        }
        l += self.size;
        let mut sm = M::identity_element();
        while {
            // do
            while l % 2 == 0 {
                l >>= 1;
            }
            if !f(&M::binary_operation(&sm, unsafe {
                &self.data.get_unchecked(l)
            })) {
                while l < self.size {
                    l *= 2;
                    let res = M::binary_operation(&sm, unsafe { &self.data.get_unchecked(l) });
                    if f(&res) {
                        sm = res;
                        l += 1;
                    }
                }
                return l - self.size;
            }
            sm = M::binary_operation(&sm, unsafe { &self.data.get_unchecked(l) });
            l += 1;
            // while
            {
                let l = l as isize;
                (l & -l) != l
            }
        } {}
        self.n
    }

    pub fn min_left<F>(&self, mut r: usize, f: F) -> usize
    where
        F: Fn(&M::T) -> bool,
    {
        assert!(r <= self.n);
        assert!(f(&M::identity_element()));
        if r == 0 {
            return 0;
        }
        r += self.size;
        let mut sm = M::identity_element();
        while {
            // do
            r -= 1;
            while r > 1 && r % 2 == 1 {
                r >>= 1;
            }
            if !f(&M::binary_operation(
                unsafe { &self.data.get_unchecked(r) },
                &sm,
            )) {
                while r < self.size {
                    r = 2 * r + 1;
                    let res = M::binary_operation(unsafe { &self.data.get_unchecked(r) }, &sm);
                    if f(&res) {
                        sm = res;
                        r -= 1;
                    }
                }
                return r + 1 - self.size;
            }
            sm = M::binary_operation(unsafe { &self.data.get_unchecked(r) }, &sm);
            // while
            {
                let r = r as isize;
                (r & -r) != r
            }
        } {}
        0
    }
}

#[test]
fn practice2_sample() {
    let a = vec![1, 2, 3, 2, 1];
    let cxy = vec![(2, 1, 5), (3, 2, 3), (1, 3, 1), (2, 2, 4), (3, 1, 3)];

    let mut st = SegmentTree::<Max>::from(a);
    let mut ans = vec![];
    for (c, x, y) in cxy {
        if c == 1 {
            st.set(x as usize - 1, y);
        } else if c == 2 {
            ans.push(st.query(x as usize - 1, y as usize));
        } else {
            ans.push(st.max_right(x as usize - 1, |&v| y > v) as i64 + 1);
        }
    }

    assert_eq!(vec![3, 3, 2, 6], ans);
}
