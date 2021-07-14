//! Z algorithm
pub fn z_algorithm(s: &[char]) -> Vec<usize> {
    let length = s.len();
    let mut z_array = vec![0_usize; length];

    z_array[0] = length;
    let (mut i, mut j) = (1, 0);

    while i < length {
        while i + j < length && s[j] == s[i + j] {
            j += 1;
        }

        z_array[i] = j;

        if j == 0 {
            i += 1;
            continue;
        }
        let mut k = 1;
        while k < j && k + z_array[k] < j {
            z_array[i + k] = z_array[k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    z_array
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let case = vec![
            ("abcbcba", vec![7, 0, 0, 0, 0, 0, 1]),
            ("mississippi", vec![11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
            ("ababacaca", vec![9, 0, 3, 0, 1, 0, 1, 0, 1]),
            ("aaaaa", vec![5, 4, 3, 2, 1]),
        ];

        for (s, ans) in case {
            let z = z_algorithm(&s.to_string().chars().collect::<Vec<_>>());
            assert_eq!(z, ans);
        }
    }
}
