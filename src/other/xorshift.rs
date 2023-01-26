//! Xorshift random number generator
use std::{
    fmt::{Debug, Display, Formatter},
    time::SystemTime,
};

#[derive(Clone)]
pub struct XorShift<T>
where
    T: std::fmt::Debug + Sized + Copy + Display,
{
    seed: T,
    f: fn(&mut T) -> (),
}

impl XorShift<u64> {
    pub fn new() -> XorShift<u64> {
        XorShift::<u64>::from_seed(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
    }
    pub fn from_seed(seed: u64) -> XorShift<u64> {
        XorShift {
            seed,
            f: |state: &mut u64| {
                *state ^= *state << 13;
                *state ^= *state >> 7;
                *state ^= *state << 17;
            },
        }
    }
}
impl XorShift<u32> {
    pub fn new() -> XorShift<u32> {
        XorShift::<u32> {
            seed: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs() as u32,
            f: |state: &mut u32| {
                *state ^= *state << 13;
                *state ^= *state >> 17;
                *state ^= *state << 5;
            },
        }
    }
}

impl<T> Iterator for XorShift<T>
where
    T: std::fmt::Debug + Sized + Copy + Display,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        (self.f)(&mut self.seed);
        Some(self.seed)
    }
}

impl<T> Debug for XorShift<T>
where
    T: std::fmt::Debug + Sized + Copy + Display,
{
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        writeln!(f, "seed : {}:", self.seed)?;
        Ok(())
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
