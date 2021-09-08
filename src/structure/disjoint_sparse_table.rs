use std::ops::Range;

pub trait SemiGroup {
    type T: Clone;
    fn operate(a: &Self::T, b: &Self::T) -> Self::T;
}

pub struct Add {}
impl SemiGroup for Add {
    type T = i64;

    fn operate(a: &Self::T, b: &Self::T) -> Self::T {
        *a + *b
    }
}

#[derive(Debug)]
pub struct DisjointSparseTable<S: SemiGroup> {
    pub table: Vec<Vec<S::T>>,
}

impl<S: SemiGroup> DisjointSparseTable<S> {
    #[inline]
    pub fn new(v: &[S::T]) -> Self {
        let size = (32 - (v.len() as u32).saturating_sub(1).leading_zeros()) as usize;
        let mut table = vec![v.to_vec()];

        (1..size).for_each(|i| {
            let mut tmp = v.to_vec();

            let span = 2i64.pow(i as u32) as usize;

            (0..(v.len() + (span * 2) - 1) / (span * 2)).for_each(|j| {
                let start = span * 2 * j + span;

                (0..span - 1)
                    .map(|k| start - 2 - k)
                    .filter(|&k| k + 1 < v.len())
                    .for_each(|k| {
                        tmp[k] = S::operate(&tmp[k], &tmp[k + 1]);
                    });

                (0..span - 1)
                    .map(|k| k + start + 1)
                    .filter(|&k| k < v.len())
                    .for_each(|k| {
                        tmp[k] = S::operate(&tmp[k], &tmp[k - 1]);
                    });
            });
            table.push(tmp);
        });

        DisjointSparseTable { table }
    }

    #[inline]
    pub fn fold(&self, range: Range<usize>) -> S::T {
        if range.len() == 1 {
            return self.table[0][range.start].clone();
        }
        let h = (32 - ((range.start ^ (range.end - 1)) as u32).leading_zeros()) as usize - 1;
        S::operate(&self.table[h][range.start], &self.table[h][range.end - 1])
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_disjoint_sparse_table() {
        let a = DisjointSparseTable::<Add>::new(&[2, 10, 1, 100]);
        for &(l, r, ans) in [
            (0, 1, 2),
            (0, 2, 12),
            (0, 3, 13),
            (0, 4, 113),
            (1, 2, 10),
            (1, 3, 11),
            (1, 4, 111),
            (2, 3, 1),
            (2, 4, 101),
            (3, 4, 100),
        ]
        .iter()
        {
            assert_eq!(a.fold(l..r), ans);
        }
    }

    #[test]
    fn test_library_checker_sample() {
        let a = DisjointSparseTable::<Add>::new(&[1, 10, 100, 1000, 10000]);
        for &(l, r, ans) in [
            (2, 3, 100),
            (0, 3, 111),
            (2, 5, 11100),
            (3, 4, 1000),
            (0, 5, 11111),
        ]
        .iter()
        {
            assert_eq!(a.fold(l..r), ans);
        }
    }
}
