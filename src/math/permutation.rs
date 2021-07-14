//! 順列生成
#[test]
fn test_prm() {
    let n = 4;
    let vv = make_permutation(n);
    assert_eq!(0, vv[0][0]);
}

pub fn make_permutation(n: usize) -> Vec<Vec<usize>> {
    let mut vvec: Vec<Vec<usize>> = vec![Vec::new(); factorial(n)];
    let nums: Vec<usize> = (0..n).collect();
    let indexes: Vec<usize> = (0..factorial(n)).collect();
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
    let block_size = factorial(nums.len() - 1);
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

fn factorial(i: usize) -> usize {
    (1..=i).product()
}

#[cfg(test)]
mod tests {}
