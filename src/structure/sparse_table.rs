//! SparseTable
pub struct SparseTable {
    v: Vec<i64>,
    log_table: Vec<usize>,
    table: Vec<Vec<usize>>,
}

impl SparseTable {
    pub fn new(v: &[i64]) -> Self {
        let mut log_table = vec![0; v.len() + 1];
        for i in 2..=v.len() {
            log_table[i] = log_table[i >> 1] + 1;
        }
        let mut table: Vec<Vec<usize>> = (0..v.len())
            .map(|i| vec![i; log_table[v.len() - i] + 1])
            .collect();
        for k in 1..=log_table[v.len()] {
            for i in 0..=v.len() - (1 << k) {
                let index_1 = table[i][k - 1];
                let index_2 = table[i + (1 << (k - 1))][k - 1];
                table[i][k] = if v[index_1] < v[index_2] {
                    index_1
                } else {
                    index_2
                };
            }
        }
        SparseTable {
            v: v.to_vec(),
            log_table,
            table,
        }
    }
    pub fn query(&self, l: usize, r: usize) -> i64 {
        let i = self.log_table[r - l];
        std::cmp::min(
            self.v[self.table[l][i]],
            self.v[self.table[r - (1 << i)][i]],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sparse_table() {
        let a = SparseTable::new(&[2, 10, 1, 100]);
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
            assert_eq!(a.query(*l, *r), *ans);
        }
    }
}
