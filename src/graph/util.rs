fn to_adjacency_matrix(g: &[Vec<i64>]) -> Vec<Vec<Option<i64>>> {
    let mut v = vec![vec![None; g.len()]; g.len()];
    g.iter()
        .enumerate()
        .for_each(|(from, vec)| vec.iter().for_each(|to| v[from][*to as usize] = Some(1)));
    v
}

fn adjacency_list(matrix: &[Vec<i64>]) -> Vec<Vec<i64>> {
    matrix
        .iter()
        .map(|v| {
            v.iter()
                .enumerate()
                .filter(|(_, &v)| v > 0)
                .map(|(j, _)| j as i64)
                .collect()
        })
        .collect()
}

#[test]
fn test_to_adjacency_matrix() {
    let m = to_adjacency_matrix(&[
        vec![1, 2, 4],
        vec![0, 2, 3],
        vec![0, 1, 3, 4],
        vec![1, 2, 4],
        vec![0, 2, 3],
    ]);

    let ans = vec![
        vec![0, 1, 1, 0, 1],
        vec![1, 0, 1, 1, 0],
        vec![1, 1, 0, 1, 1],
        vec![0, 1, 1, 0, 1],
        vec![1, 0, 1, 1, 0],
    ];
    dbg!(&m);
    for i in 0..ans.len() {
        for j in 0..ans.len() {
            let v = m[i][j].unwrap_or(0);
            assert_eq!(ans[i][j], v);
        }
    }
}
#[test]
fn test_adjacency_list() {
    let al = adjacency_list(&[
        vec![0, 1, 1, 0, 1],
        vec![1, 0, 1, 1, 0],
        vec![1, 1, 0, 1, 1],
        vec![0, 1, 1, 0, 1],
        vec![1, 0, 1, 1, 0],
    ]);

    assert_eq!(
        &vec![
            vec![1, 2, 4],
            vec![0, 2, 3],
            vec![0, 1, 3, 4],
            vec![1, 2, 4],
            vec![0, 2, 3],
        ],
        &al
    );
}
