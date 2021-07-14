//! 転倒数
pub fn inversion_number(array: &[i64]) -> i64 {
    count_merge(
        &mut array.iter().copied().collect::<Vec<_>>(),
        0..array.len(),
    )
}
fn count_merge(array: &mut Vec<i64>, range: std::ops::Range<usize>) -> i64 {
    let length = range.len() as i64;
    if length <= 1 {
        return 0;
    }

    let mut count = 0;
    let mid = (range.start + range.end) / 2;
    count += count_merge(array, range.start..mid);
    count += count_merge(array, mid..range.end);

    let b = array
        .iter()
        .skip(range.start)
        .take(mid - range.start)
        .copied()
        .collect::<Vec<_>>();
    let c = array
        .iter()
        .skip(mid)
        .take(range.end - mid)
        .copied()
        .collect::<Vec<_>>();

    let (mut ai, mut bi, mut ci) = (0, 0, 0);

    while ai < length {
        if bi < b.len() && (ci == c.len() || b[bi] <= c[ci]) {
            array[range.start + ai as usize] = b[bi];
            ai += 1;
            bi += 1;
        } else {
            count += length / 2 - bi as i64;
            array[range.start + ai as usize] = c[ci];
            ai += 1;
            ci += 1;
        }
    }
    count
}

#[test]
fn test_inversion_number() {
    let array = {
        let v = vec![
            (vec![3, 1, 5, 4, 2], 5),
            (vec![3, 5, 2, 1, 4], 6),
            (vec![3, 1, 2], 2),
            (vec![6, 1, 5, 8, 2, 3, 4, 7], 12),
            (vec![7, 6, 1, 5, 8, 2, 3, 10, 4, 9], 19),
            (
                vec![
                    63, 16, 24, 7, 29, 57, 65, 26, 36, 32, 50, 5, 34, 1, 18, 15, 49, 9, 47, 53, 10,
                    35, 76, 79,
                ],
                122,
            ),
            (
                vec![
                    0, 18, 35, 2, 31, 33, 32, 6, 11, 15, 36, 19, 42, 23, 9, 20, 24, 3, 10, 47, 8,
                    38, 5, 37, 46,
                ],
                125,
            ),
        ];
        v
    };

    for (input, ans) in array {
        assert_eq!(inversion_number(&input), ans);
    }
}
