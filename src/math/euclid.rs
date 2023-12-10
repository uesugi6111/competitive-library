//! ユークリッドさんありがとう
use std::mem::swap;
use std::{cmp::min, num::NonZeroU64};
pub fn gcd(mut m: u64, mut n: u64) -> u64 {
    if m == 0 || n == 0 {
        return n;
    }
    let (i, j) = unsafe {
        (
            NonZeroU64::new_unchecked(m).trailing_zeros(),
            NonZeroU64::new_unchecked(n).trailing_zeros(),
        )
    };
    m >>= i;
    n >>= j;

    loop {
        if m > n {
            swap(&mut m, &mut n);
        }
        n -= m;
        if n == 0 {
            return m << min(i, j);
        }
        n >>= unsafe { NonZeroU64::new_unchecked(n) }.trailing_zeros();
    }
}
pub fn lcm(m: u64, n: u64) -> u64 {
    m * n / gcd(m, n)
}
pub fn ngcd(m: u64, n: u64) -> u64 {
    if m == 0 {
        n
    } else {
        ngcd(n % m, m)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn one() {
        assert_eq!(gcd(1, 2), 1);
        assert_eq!(gcd(2, 3), 1);
        assert_eq!(gcd(3, 5), 1);
        assert_eq!(gcd(5, 7), 1);
        assert_eq!(gcd(7, 9), 1);
        assert_eq!(gcd(9, 11), 1);
        assert_eq!(gcd(11, 13), 1);
    }
    #[test]
    fn t() {
        assert_eq!(gcd(2, 2), 2);
        assert_eq!(gcd(2, 4), 2);
        assert_eq!(gcd(10, 15), 5);
        assert_eq!(gcd(6, 4), 2);
        assert_eq!(gcd(100, 30), 10);
        assert_eq!(gcd(1_000_000_008, 1_000_000_007), 1);
    }
}
