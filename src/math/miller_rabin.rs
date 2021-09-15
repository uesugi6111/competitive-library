use crate::math::mod_pow::modpow;
use crate::other::xorshift::XorShift;

pub fn is_prime(n: i64) -> bool {
    if n == 2 {
        return true;
    } else if n == 1 || (n > 2 && n & 1 == 0) {
        return false;
    }

    let (mut s, mut t) = (0, n - 1);

    while t & 1 == 0 {
        s += 1;
        t >>= 1;
    }

    let a = XorShift::new().next().unwrap() as i64 % n;

    if modpow(a, t, n as i64) == 1 {
        return true;
    }

    for i in 0..s {
        if modpow(a, 2_i64.pow(i) * t, n as i64) == n - 1 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_miller_rabin() {
        let v = vec![
            (1, false),
            (2, true),
            (3, true),
            (4, false),
            (5, true),
            (6, false),
            (7, true),
            (8, false),
            (9, false),
            (10, false),
            (11, true),
            (1_000_000_007, true),
        ];
        for (i, ans) in v {
            assert_eq!(is_prime(i), ans, "testing {}", i);
        }
    }
}
