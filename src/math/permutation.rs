//! 順列生成

pub fn make_permutation(n: usize) -> Vec<Vec<usize>> {
    let factorial = (1..=n).product();
    let mut vvec: Vec<Vec<usize>> = vec![Vec::new(); factorial];
    let nums: Vec<usize> = (0..n).collect();
    let indexes: Vec<usize> = (0..factorial).collect();
    push_recusive(nums, indexes, &mut vvec);
    vvec
}

fn push_recusive<T: Clone>(
    nums: Vec<T>,
    indexes: Vec<usize>,
    vvec: &mut Vec<Vec<T>>,
) -> &mut Vec<Vec<T>> {
    if nums.is_empty() {
        return vvec;
    }
    let block_size = (1..nums.len()).product();
    for (block_index, num) in nums.iter().enumerate() {
        for inner_index in 0..block_size {
            let index = indexes[block_size * block_index + inner_index];
            vvec[index].push(num.clone());
        }
        let new_nums = {
            let mut tmp = nums.clone();
            tmp.remove(block_index);
            tmp
        };
        let new_indexes: Vec<usize> = {
            let slice = &indexes[(block_size * block_index)..(block_size * (block_index + 1))];
            slice.to_vec()
        };
        push_recusive(new_nums, new_indexes, vvec);
    }
    vvec
}

pub struct Permutation<T>
where
    T: Clone,
{
    p: Vec<T>,
    init: bool,
}

impl<T> Permutation<T>
where
    T: Clone,
{
    pub fn new(p: &[T]) -> Self {
        Self {
            p: p.to_vec(),
            init: false,
        }
    }
}
impl<T> Iterator for Permutation<T>
where
    T: Clone + Ord,
{
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.init {
            self.p.sort();
            self.init = true;
            return Some(self.p.clone());
        }
        let Some(i) = (0..&self.p.len() - 1).rfind(|&i| self.p[i] < self.p[i + 1]) else { return None; };
        let j = self.p.iter().rposition(|x| x > &self.p[i]).unwrap();
        self.p.swap(i, j);
        self.p[i + 1..].reverse();
        Some(self.p.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_prm() {
        let vv = make_permutation(4);
        assert_eq!(0, vv[0][0]);
    }
    #[test]
    fn test_struct() {
        let expect = [
            &[0, 1, 2],
            &[0, 2, 1],
            &[1, 0, 2],
            &[1, 2, 0],
            &[2, 0, 1],
            &[2, 1, 0],
        ];
        let a = Permutation::new(&[0, 1, 2]);

        for (i, v) in a.enumerate() {
            assert_eq!(v, expect[i]);
        }
    }
}
