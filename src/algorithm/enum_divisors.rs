//!  約数列挙
pub fn enum_divisors(n: i64) -> Vec<i64> {
    let mut res = vec![];
    for i in 1..=(n as f64).sqrt() as i64 {
        if n % i != 0 {
            continue;
        }
        res.push(i);
        if i.pow(2) != n {
            res.push(n / i);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_enum_divisors() {
        let map = {
            let mut ret = HashMap::new();
            ret.insert(0, vec![]);
            ret.insert(1, vec![1]);
            ret.insert(2, vec![1, 2]);
            ret.insert(3, vec![1, 3]);
            ret.insert(4, vec![1, 2, 4]);
            ret.insert(6, vec![1, 2, 3, 6]);
            ret.insert(20, vec![1, 2, 4, 5, 10, 20]);
            ret.insert(25, vec![1, 5, 25]);
            ret.insert(30, vec![1, 2, 3, 5, 6, 10, 15, 30]);
            ret.insert(
                2520,
                vec![
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 14, 15, 18, 20, 21, 24, 28, 30, 35, 36, 40,
                    42, 45, 56, 60, 63, 70, 72, 84, 90, 105, 120, 126, 140, 168, 180, 210, 252,
                    280, 315, 360, 420, 504, 630, 840, 1260, 2520,
                ],
            );
            ret.insert(1_000_000_007, vec![1, 1_000_000_007]);
            ret
        };

        for (k, v) in map {
            let mut a = enum_divisors(k);
            a.sort_unstable();
            assert_eq!(a, v);
        }
    }
}
