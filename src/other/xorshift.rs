//! Xorshift random number generator
use std::{
    fmt::{Debug, Display},
    time::SystemTime,
};

#[derive(Clone, Default, Copy, Debug)]
pub struct XorShift<T>
where
    T: std::fmt::Debug + Sized + Copy + Display + Shift,
{
    seed: T,
}

impl<T> XorShift<T>
where
    T: std::fmt::Debug + Sized + Copy + Display + Shift,
{
    pub fn new() -> Self {
        XorShift::from_seed(T::seed())
    }
    pub fn from_seed(seed: T) -> XorShift<T> {
        XorShift { seed }
    }
}

impl<T> Iterator for XorShift<T>
where
    T: std::fmt::Debug + Sized + Copy + Display + Shift,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        T::shift(&mut self.seed);
        Some(self.seed)
    }
}

pub trait Shift {
    fn seed() -> Self;
    fn shift(n: &mut Self);
}

impl Shift for u64 {
    fn seed() -> Self {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    fn shift(state: &mut u64) {
        *state ^= *state << 13;
        *state ^= *state >> 7;
        *state ^= *state << 17;
    }
}
impl Shift for u32 {
    fn seed() -> Self {
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32
    }

    fn shift(state: &mut u32) {
        *state ^= *state << 13;
        *state ^= *state >> 17;
        *state ^= *state << 5;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn test_xorshift() {
        let mut set = HashSet::new();
        let xorshift = XorShift::<u64>::new();

        for v in xorshift.take(100_000) {
            assert!(!set.contains(&v));
            set.insert(v);
        }
    }
}
