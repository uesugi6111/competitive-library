use crate::math::mod_pow::modpow;
use crate::other::xorshift::XorShift;

pub fn is_prime(n: i64) -> bool {
    if n == 2 || n == 3 {
        return true;
    } else if n == 1 || (n > 2 && n & 1 == 0) {
        return false;
    }

    let (mut s, mut t) = (0, n - 1);

    while t & 1 == 0 {
        s += 1;
        t >>= 1;
    }

    let r = {
        let mut r = XorShift::new().next().unwrap() as i64 % (n - 3);
        r += 3;
        r
    };

    if modpow(r, t, n as i64) == 1 {
        return true;
    }

    for i in 0..s {
        if modpow(r, 2_i64.pow(i) * t, n as i64) == n - 1 {
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
            (8191, true),
            (131_071, true),
            (524_287, true),
            (6_700_417, true),
            (2_147_483_647, true),
            (67_280_421_310_721, true),
            (67_280_421_310_722, false),
            (100_000_000_000_000, false),
            (68_718_297_093, false), // 131071 * 524287
        ];
        for (i, ans) in v {
            assert_eq!(is_prime(i), ans, "testing {}", i);
        }
    }
    #[test]
    fn test_miller_rabin_loop() {
        for _ in 0..10 {
            test_miller_rabin();
        }
    }
}
