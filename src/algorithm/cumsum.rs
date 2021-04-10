pub fn cumsum(v: &[i64]) -> Vec<i64> {
    (0..1)
        .chain(v.iter().scan(0, |c, &x| {
            *c += x;
            Some(*c)
        }))
        .collect::<Vec<_>>()
}

#[test]
fn a() {
    let v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let cumsum = cumsum(&v);
    assert_eq!(&cumsum, &[0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55]);
}
