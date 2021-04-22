//! ランレングス
pub fn compress(s: &str) -> Vec<(char, usize)> {
    let vs: Vec<char> = s.chars().collect();

    let mut buff = vs[0];
    let mut count = 1;
    let mut v = vec![];
    for c in vs.iter().skip(1) {
        if *c != buff {
            v.push((buff, count));
            count = 1;
            buff = *c;
        } else {
            count += 1;
        }
    }
    v.push((buff, count));
    v
}

#[test]
fn test_run_length() {
    let v = vec![('a', 5usize), ('b', 3), ('c', 1)];

    assert_eq!(compress(&"aaaaabbbc"), v);
}
