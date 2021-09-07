//! Xorshift random number generator
use std::time::SystemTime;

#[derive(Debug, Default)]
pub struct XorShift {
    seed: u64,
}

impl XorShift {
    pub fn new() -> Self {
        Self {
            seed: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    pub fn from_seed(seed: u64) -> Self {
        Self { seed }
    }
}

impl Iterator for XorShift {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.seed ^= self.seed << 13;
        self.seed ^= self.seed >> 7;
        self.seed ^= self.seed << 17;
        Some(self.seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn test_xorshift() {
        let mut set = HashSet::new();
        let xorshift = XorShift::new();

        for v in xorshift.take(100_000) {
            assert!(!set.contains(&v));
            set.insert(v);
        }
    }
}
