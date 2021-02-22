pub fn largest_rectangle(arg: &[i64]) -> i64 {
    let mut histogram: Vec<_> = arg.iter().collect();
    histogram.push(&0);

    let mut stack = std::collections::VecDeque::<(i64, i64)>::new();
    let mut ans = 0;

    for (right, h) in histogram.iter().enumerate() {
        if let Some((_, value)) = stack.back() {
            if value <= *h {
                stack.push_back((right as i64, **h));
                continue;
            }
        }
        let mut most_left = right as i64;
        while !stack.is_empty() && stack[stack.len() - 1].1 > **h {
            let (left, value) = stack.pop_back().unwrap();
            most_left = left;
            ans = std::cmp::max(ans, value * (right as i64 - most_left));
        }
        stack.push_back((most_left, **h));
    }

    ans
}

#[test]
fn test_largest_rectangle() {
    assert_eq!(largest_rectangle(&[0, 2, 5, 6, 4, 2, 3, 1]), 12);
    assert_eq!(largest_rectangle(&[2, 4, 4, 9, 4, 9]), 20);
    assert_eq!(largest_rectangle(&[200, 4, 4, 9, 4, 9]), 200);
}
