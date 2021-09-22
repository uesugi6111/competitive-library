pub struct BinarySearch<T> {
    target: T,
    min: i64,
    max: i64,
}
impl<T> BinarySearch<T> {
    pub fn new(target: T, min: i64, max: i64) -> Self {
        Self { target, min, max }
    }
    /// f が true を帰す最小値を探す
    pub fn search<F>(&self, f: F) -> i64
    where
        F: Fn(&T, i64) -> bool,
    {
        let mut left = self.min;
        let mut right = self.max;

        while right - left > 1 {
            let mid = left + (right - left) / 2;

            if f(&self.target, mid) {
                left = mid;
            } else {
                right = mid
            };
        }
        right
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_binary_search() {
        let v = (0_i64..100).filter(|x| x % 2 == 0).collect::<Vec<_>>();
        let bs = BinarySearch::new(&v, -1, v.len() as i64);
        for &i in v.iter() {
            let ans = bs.search(|x, j| x[j as usize] < i);
            assert_eq!(v[ans as usize], i)
        }
    }
}
