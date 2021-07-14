//! ワーシャルフロイト
pub fn floyd_warshall(matrix: &[Vec<Option<i64>>]) -> Vec<Vec<Option<i64>>> {
    let mut m: Vec<_> = matrix.to_vec();
    let n = m.len();
    (0..n).for_each(|i| {
        (0..n).for_each(|j| {
            (0..n).for_each(|k| {
                m[j][k] = if m[j][k].is_none() && m[j][i].is_none() && m[i][k].is_none() {
                    None
                } else if m[j][i].is_none() || m[i][k].is_none() {
                    m[j][k]
                } else if m[j][k].is_none() {
                    Some(m[j][i].unwrap() + m[i][k].unwrap())
                } else {
                    Some(std::cmp::min(
                        m[j][k].unwrap(),
                        m[j][i].unwrap() + m[i][k].unwrap(),
                    ))
                };
            })
        })
    });
    m
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_floyd_warshall_1() {
        let matrix = vec![
            vec![Some(0), Some(1), Some(1), None, Some(1)],
            vec![Some(1), Some(0), Some(1), Some(1), None],
            vec![Some(1), Some(1), Some(0), Some(1), Some(1)],
            vec![None, Some(1), Some(1), Some(0), Some(1)],
            vec![Some(1), None, Some(1), Some(1), Some(0)],
        ];
        let a = floyd_warshall(&matrix);

        let ans = vec![
            vec![Some(0), Some(1), Some(1), Some(2), Some(1)],
            vec![Some(1), Some(0), Some(1), Some(1), Some(2)],
            vec![Some(1), Some(1), Some(0), Some(1), Some(1)],
            vec![Some(2), Some(1), Some(1), Some(0), Some(1)],
            vec![Some(1), Some(2), Some(1), Some(1), Some(0)],
        ];

        assert_eq!(ans, a);
    }

    #[test]
    fn test_floyd_warshall_2() {
        let matrix = vec![
            vec![Some(0), Some(10), None, Some(100)],
            vec![None, Some(0), None, Some(1000)],
            vec![None, Some(1), Some(0), Some(10000)],
            vec![Some(5), None, None, Some(0)],
        ];

        let a = floyd_warshall(&matrix);
        let ans = vec![
            vec![Some(0), Some(10), None, Some(100)],
            vec![Some(1005), Some(0), None, Some(1000)],
            vec![Some(1006), Some(1), Some(0), Some(1001)],
            vec![Some(5), Some(15), None, Some(0)],
        ];
        assert_eq!(ans, a);
    }
}
