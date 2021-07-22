//! SparseTable
//! 冪等半群列にたいして区間(l,r] の結果を戻す
//! 構築 O(NlogN) クエリO(1)
//! min, max, gcd, lcm 等

use std::ops::Range;

/// 冪等半群
pub trait Band {
    type T: Clone;
    fn operate(a: &Self::T, b: &Self::T) -> Self::T;
}

/// 最小値
struct Min {}
impl Band for Min {
    type T = i64;

    fn operate(a: &Self::T, b: &Self::T) -> Self::T {
        *a.min(b)
    }
}

/// SparseTable
pub struct SparseTable<B: Band> {
    table: Vec<Vec<B::T>>,
}

impl<B: Band> SparseTable<B> {
    /// O(NlogN)
    pub fn new(v: &[B::T]) -> Self {
        let mut table = vec![v.to_vec()];

        for i in 1..64 - v.len().leading_zeros() as usize {
            let mut tmp = vec![];
            for j in 0..=v.len() - (1 << i) {
                tmp.push(B::operate(
                    &table[i - 1][j],
                    &table[i - 1][j + (1 << (i - 1))],
                ));
            }
            table.push(tmp);
        }

        SparseTable { table }
    }

    /// [l,r)
    /// O(1)
    pub fn fold(&self, range: Range<usize>) -> B::T {
        let i = 64 - (range.end - range.start).leading_zeros() as usize - 1;
        B::operate(
            &self.table[i][range.start],
            &self.table[i][range.end - (1 << i)],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sparse_table() {
        let a = SparseTable::<Min>::new(&[2, 10, 1, 100]);
        for (l, r, ans) in [
            (0, 1, 2),
            (0, 2, 2),
            (0, 3, 1),
            (0, 4, 1),
            (1, 2, 10),
            (1, 3, 1),
            (1, 4, 1),
            (2, 3, 1),
            (2, 4, 1),
            (3, 4, 100),
        ]
        .iter()
        {
            assert_eq!(a.fold(*l..*r), *ans);
        }
    }
}
