use std::cmp::min;

pub trait Monoid {
    type T: Clone;
    fn identity_element() -> Self::T;
    fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T;
}
pub struct Min {}
impl Monoid for Min {
    type T = i64;
    fn identity_element() -> Self::T {
        std::i32::MAX as i64
    }
    fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T {
        min(*a, *b)
    }
}

pub struct SegmentTree<M>
where
    M: Monoid,
{
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
        let mut data = vec![M::identity_element(); 2 * size];
        data[size..(size + n)].clone_from_slice(&v);
        let mut ret = SegmentTree { size, log, data };
        for i in (1..size).rev() {
            ret.update(i);
        }
        ret
    }
}
impl<M: Monoid> SegmentTree<M> {
    pub fn query(&self, mut l: usize, mut r: usize) -> M::T {
        let mut sml = M::identity_element();
        let mut smr = M::identity_element();
        l += self.size;
        r += self.size;

        while l < r {
            if l & 1 != 0 {
                sml = M::binary_operation(&sml, &self.data[l]);
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                smr = M::binary_operation(&self.data[r], &smr);
            }
            l >>= 1;
            r >>= 1;
        }

        M::binary_operation(&sml, &smr)
    }
    fn update(&mut self, k: usize) {
        self.data[k] = M::binary_operation(&self.data[2 * k], &self.data[2 * k + 1]);
    }
    pub fn set(&mut self, mut p: usize, x: M::T) {
        p += self.size;
        self.data[p] = x;
        for i in 1..=self.log {
            self.update(p >> i);
        }
    }
}
