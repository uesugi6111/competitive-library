//! Zobrist Hash
use super::xorshift::XorShift;
use std::collections::HashSet;

#[derive(Default, Clone)]
pub struct ZobristHash<T: Eq + std::hash::Hash + Copy> {
    map: std::collections::HashMap<T, u64>,
    rand: XorShift,
}

impl<T: Eq + std::hash::Hash + Copy> ZobristHash<T> {
    pub fn new() -> Self {
        Self {
            map: std::collections::HashMap::<T, u64>::new(),
            rand: XorShift::new(),
        }
    }
    #[allow(clippy::or_fun_call)]
    pub fn hash(&mut self, hash: u64, add: T) -> u64 {
        hash ^ *self.map.entry(add).or_insert(self.rand.next().unwrap())
    }
    pub fn hash_vec_from_vec(&mut self, v: &[T]) -> Vec<u64> {
        let mut set = std::collections::HashSet::new();
        let mut ret = vec![];

        let mut hash = 0;
        for &value in v.iter() {
            if !set.contains(&value) {
                set.insert(value);
                hash = self.hash(hash, value);
            }
            ret.push(hash);
        }
        ret
    }
    pub fn hash_from_set(&mut self, set: &HashSet<T>) -> u64 {
        set.iter()
            .scan(0, |s, &x| {
                *s = self.hash(*s, x);
                Some(*s)
            })
            .last()
            .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn test() {
        let mut zh = ZobristHash::new();
        let v = vec![1, 2, 3, 4, 5];
        let hash_vec = zh.hash_vec_from_vec(&v);

        assert_eq!(hash_vec.len(), 5);
        assert_eq!(hash_vec[0], zh.hash_from_set(&HashSet::from([1])));
        assert_eq!(hash_vec[1], zh.hash_from_set(&HashSet::from([1, 2])));
        assert_eq!(hash_vec[2], zh.hash_from_set(&HashSet::from([1, 2, 3])));
        assert_eq!(hash_vec[3], zh.hash_from_set(&HashSet::from([1, 2, 3, 4])));
        assert_eq!(
            hash_vec[4],
            zh.hash_from_set(&HashSet::from([1, 2, 3, 4, 5]))
        );
    }
    #[test]
    fn rand() {
        let mut rand = XorShift::new();

        let v = (0..10000).map(|_| rand.next().unwrap()).collect::<Vec<_>>();
        let mut vv = v.clone();
        vv.sort_unstable();
        let mut zh = ZobristHash::new();
        assert_eq!(
            zh.hash_from_set(&v.into_iter().collect::<HashSet::<_>>()),
            zh.hash_from_set(&vv.into_iter().collect::<HashSet::<_>>())
        );
    }
}
