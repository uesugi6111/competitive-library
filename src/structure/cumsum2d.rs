#[derive(Clone, Debug)]
pub struct CumSum2D {
    v: Vec<Vec<i64>>,
}

impl CumSum2D {
    pub fn new(source: &[Vec<i64>]) -> Self {
        let h = source.len();
        let w = source[0].len();
        let mut v = vec![vec![0i64; w + 1]; h + 1];

        for i in 0..h {
            for j in 0..w {
                v[i + 1][j + 1] = source[i][j] + v[i][j + 1] + v[i + 1][j] - v[i][j];
            }
        }
        CumSum2D { v }
    }

    pub fn query(&self, top: usize, bottom: usize, left: usize, right: usize) -> i64 {
        self.v[bottom + 1][right + 1] - self.v[bottom + 1][left] - self.v[top][right + 1]
            + self.v[top][left]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let a = CumSum2D::new(&[
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
        ]);
        assert_eq!(
            a.v,
            vec![
                vec![0, 0, 0, 0, 0],
                vec![0, 1, 3, 6, 10],
                vec![0, 2, 6, 12, 20],
                vec![0, 3, 9, 18, 30],
                vec![0, 4, 12, 24, 40]
            ]
        );
        assert_eq!(a.query(0, 0, 0, 0), 1);
        assert_eq!(a.query(0, 1, 0, 1), 6);
        assert_eq!(a.query(1, 2, 2, 3), 14);
        assert_eq!(a.query(0, 0, 0, 3), 10);
        assert_eq!(a.query(0, 3, 0, 0), 4);
        assert_eq!(a.query(3, 3, 3, 3), 4);

        assert_eq!(a.query(0, 3, 0, 3), 40);
    }
}
