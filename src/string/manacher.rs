pub fn manacher(s: &[char]) -> Vec<usize> {
    let (mut radius, mut i, mut j) = (vec![0; s.len()], 0, 0);
    while i < s.len() {
        while i >= j && i + j < s.len() && s[i - j] == s[i + j] {
            j += 1;
        }
        radius[i] = j;
        let mut k = 1;
        while i >= k && k + radius[i - k] < j {
            radius[i + k] = radius[i - k];
            k += 1;
        }
        i += k;
        j -= k;
    }
    radius
}

#[deprecated(since = "0.1.0", note = "use `manacher` instead")]
pub use manacher as manachar;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a() {
        assert_eq!(
            manacher(&to_v("qwertyuiop")),
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            manacher(&to_v("qwertyuiopoiuytrewq")),
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 10, 1, 1, 1, 1, 1, 1, 1, 1, 1]
        );
        assert_eq!(
            manacher(&to_v("qwqwqwqwqwqwq")),
            [1, 2, 3, 4, 5, 6, 7, 6, 5, 4, 3, 2, 1]
        );
        assert_eq!(manacher(&to_v("abaaababa")), [1, 2, 1, 4, 1, 2, 3, 2, 1]);
        assert_eq!(
            manacher(&to_v("aaaaaaaaaa")),
            [1, 2, 3, 4, 5, 5, 4, 3, 2, 1]
        );
    }

    fn to_v(s: &str) -> Vec<char> {
        String::from(s).chars().collect::<Vec<_>>()
    }
}
