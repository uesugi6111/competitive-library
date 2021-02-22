pub fn modpow(base: i64, exp: i64, n: i64) -> i64 {
    let (mut base, mut exp, n) = (base, exp, n);

    assert!(
        exp >= 0,
        "negative exponent cannot be used in modular exponentiation"
    );

    if exp == 0 {
        return 1;
    }

    let mut res = 1;
    base %= n;

    loop {
        if exp % 2 == 1 {
            res *= &base;
            res %= &n;
        }

        if exp == 1 {
            return res;
        }

        exp /= 2;
        base *= base;
        base %= n;
    }
}

#[cfg(test)]
mod tests {
    use super::modpow;

    #[test]
    fn test_modpow() {
        assert_eq!(modpow(3, 5, 5), 3);
        assert_eq!(modpow(2, 32, 9), 4);
    }
}
