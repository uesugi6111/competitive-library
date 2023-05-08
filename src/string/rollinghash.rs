//! ロリハ
use crate::other::xorshift::XorShift;

const MOD: u128 = (1 << 61) - 1;

///  text 内での pattern の出現位置の始点を返す
pub fn rolling_hash<T: Into<u128> + Copy>(pattern: &[T], text: &[T]) -> Vec<usize> {
    let rh = RollingHash::new(text, pattern.len());
    rh.search(pattern)
}

pub struct RollingHash {
    pub text_hash: Vec<u128>,
    base: u128,
    length: usize,
}
impl RollingHash {
    pub fn new<T: Into<u128> + Copy>(text: &[T], length: usize) -> Self {
        let base = XorShift::new()
            .map(|x: u64| x as u128 % MOD)
            .next()
            .unwrap();
        RollingHash::from_base(text, length, base)
    }
    pub fn from_base<T: Into<u128> + Copy>(text: &[T], length: usize, base: u128) -> Self {
        let pow_base = pow_mod(base, length);

        let mut hash = 0;

        for &t in text.iter().take(length) {
            hash = mul_mod(hash, base);

            hash += t.into();
        }
        let mut text_hash = vec![hash];

        for k in 0..text.len() - length {
            hash = mul_mod(hash, base);
            hash += text[length + k].into();

            let buff = mul_mod(text[k].into(), pow_base);
            if buff < hash {
                hash -= buff;
            } else {
                hash += MOD - buff;
            }

            text_hash.push(hash);
        }
        Self {
            text_hash,
            base,
            length,
        }
    }
    pub fn search<T: Into<u128> + Copy>(&self, pattern: &[T]) -> Vec<usize> {
        assert_eq!(self.length, pattern.len());

        let mut pattern_hash = 0;

        for &p in pattern {
            pattern_hash = mul_mod(pattern_hash, self.base);
            pattern_hash += p.into();
        }

        self.text_hash
            .iter()
            .enumerate()
            .filter_map(|(i, &h)| if pattern_hash == h { Some(i) } else { None })
            .collect()
    }
}

fn mul_mod(a: u128, b: u128) -> u128 {
    let mut t = a * b;
    t = (t >> 61) + (t & MOD);

    if t >= MOD {
        t - MOD
    } else {
        t
    }
}
fn pow_mod(base: u128, exp: usize) -> u128 {
    let (mut a, mut exp) = (base as u128, exp as u128);

    if exp == 0 {
        return 1;
    }

    let mut res = 1;
    a %= MOD;

    loop {
        if exp % 2 == 1 {
            res = mul_mod(res, a);
        }

        if exp == 1 {
            return res;
        }

        exp /= 2;
        a = mul_mod(a, a);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_hash() {
        let mut mached = false;
        let s = b"hhggggghhhhhgghhhhhghgggggghggggghgghhggghhggghhhhhgggghhggggghhghghghhhhhhhgggghhhgghgggghhhggghhhgghhghhhghhghhhhhghghhghghggghhhhhghhgghghghhhhhghhhhgghhhhhghghgggggghhgghgghhghhgghghghghhghhhhhggggghggggghggggghhhhgggggghghgghhhhhhggghhhggghhghghhghhhhhhghghghhhghhhhgghhghghghgggghhghhhhgghhghghgghggghgghggggghggghhggghggghghhhgghhgggghhghhhhhghgghhhhghghhggghggghggghhhghhhgggghhhhghggghggghhggggghhghhhhhhhhggghgghhghhhhhgghhhhghhgghgghghgghghgghhgghggghghgghgghghggghghhghghhhhghhhhgghhghghhhhhghghhghghghgghhhghgghhhhhgggghhhhghghghgghhhghhhhhhhhhhhhghghghgghhhggghhhgghhhgghgghghggghhhgghhgggghhggghhghhghhhhghgghhgggghhghghhgghggggghgggghhghghhgghggggggggghhggghhhhghhhhghhggggggghhhhghhgghhhggghhghhghgghhhgghhhghgghhhhhhgghhhhgggghgghhhhhghghhhhgghhggghggghhgggghghhgghghhhgghhhhghghghggghhgggghhhhgghghhhhhhhhgghhhhgggghhgghhhghggghhghghggghhhghghhghgghggghhghhghgghghghhhgggggghghggghhhhgghhhhghghhgggghghhghgghghhhgghghhhhhghghghhgghhhhghhhgghghggggghghhhgghhhhghghhgghhghhhgghgghhhghghhgghggghhghghhgghgggghghhhhghhggghhhhhhggghgghhhghggggghhgggghhghhgghhgghhhhhghhhhhhgghhghhhhhghghghghghhggggggghghgghggghhhhhgghghhhghgghgghhhghghhhhggggghhghghghhgghghgghhgggggggghhhhhghghhhghghhhhhgghhgghhghgghghggghhgghghghhghghhghhhhghhhgggghghhghhhghhhhghhgghggghghghhghghhhggghhhgggghhggghhghgggghhhghghhhghhhhhhhghhgggghhhhgghggghhgghhhhgggghgggghhhhghhhggghhhgggggghggghgghghhhghggghghgghgggghhghhhghhgghghhhhghhgghhgghhhhhggghggghhghhghhgggghghgggghgghggghhhhhhghhghgghghhhhhghghhhggghgghghghgggghghhgghhhgghhgggggghggghhggghhhghhghghggghhggghghhhhhgghhgghhhhggghhhhgggghhhgghhggghghhghghghghhgghghggghgghgghghhgghhhghgghghhggghghhghhhhgghhghghhhghghhghghghhgghhhgghghhghggggghhhhhggggggggghhhhgghhhgghhghghggghghghhgggghghghhhgghgghhghhgghggggggghggghggghghgggghghhggghgghhhhhhgghhghggghgghhhhghhghghhhggggghhghhgghhgghhgghggghggghhhhhgghgggggghghhhhhghhhghhhghghghhhhhghggghhgghhhhhgggghhghhhhhghhgghhhgghgggghghgghggghhhhhhhhggghghghhhghhghhgghhghghhhgghghgghhhhggghhhggghhgghghgghhhhhggghhghghghghhgghghghhghghghhgggggghghhhghhhhhggghgghghhgghghhhhghhhhgghhghghhggggghghghghghhghgghggghhghhggghggghhggghhgghghhghggghgggghgghhhghgghgghhhghghghhghhhggghhhhgghhhhgghhhghghhghggggggghhhhghghhhgghhggghhghgghgghhhghhhghggghhggghghhgghhhghgghhhghhgghghhhghgghggghgghhghhggghhghhhhghgghgghggggghgghhhhhhghhgggggggghggggghgggggggghghhhhghhghggghgggghghghhghgghhgghhgghghhhghhghhghhgghgghhhgghghhhhgghghgghhgggghggggggghghghhghgggghghhgggghgghghhhhhgghhghgghghhggghghhghhghgghhggghhhhgghhhhhgggggghhggghhghhggghgghhghhhghhhhhgghhghgggggghgghggghhghgghghhgggggghgghhhghhhhghgghhhhhhghghhgghgghhgghhgghhhhhggggghhghhgghhghgghghghhhghgghggghghghgggghgghhggghghgghghhhhhhgggggghghghhhghhghhhgghghghgghhhghhggghggggggggghhhgghghhhhhhghgghgghgghghgghhhhhhgggghghhhhgggghgggggghhhghghghgghhghghhhhghgghhhhhgggggggggghghghggggggghgghhgghghhghhhhggghhghgggghghhghgghgghggghhhgghhhghgghhhhhghggghhhhghhghhhhghhggggghhhhhhhhghhhghghhggghhhhgggggghghhhgggghghhhgghhggghhghhghhgghggggggghhhgghgghghhhghgghhhghhhhhgghgghhhhgghhhhhhghhggghhhgghggggghghghghgghghgghhhhhhhhhhgghhhgghgghghhghhghgghgghggghghggghhhgghgghhhghghgghghghhhgghhggghhggggggghgghghhghghghhhhghhgghhhgghghhhghhghhhhhghhhgghgghhhghhhhghhhghghgghhghhgggggghgghghghhhghgghhhhhhhhhghghhhhhggggghgggghhhghgghhhghhhghhgghghhghggghggghhghgghhhghghhhhhggghhhghghhhgghhhhgghgggghhhghgghhggghhhggggghghhhgggghghgghhggghgg";

        for i in 0..s.len() {
            if i + 21 * 2 > s.len() {
                break;
            }
            if !rolling_hash(&s[i..i + 21], s).is_empty() {
                mached = true;
                break;
            }
        }
        assert!(mached);
    }

    #[test]
    fn abc() {
        let s = b"abcdefghijklmnopqrstuvwxyz";
        assert_eq!(rolling_hash(s, s), vec![0]);

        for i in 1..26 {
            for j in 0..26 - i {
                assert_eq!(rolling_hash(&s[j..j + i], s), vec![j], "{} {}", i, j);
            }
        }
    }

    #[test]
    fn test_mul_mod() {
        for i in XorShift::new().take(1000).map(|x: u64| x as u128 % MOD) {
            for j in XorShift::new().take(1000).map(|x: u64| x as u128 % MOD) {
                assert_eq!(mul_mod(i as u128, j as u128), (i as u128 * j as u128) % MOD);
            }
        }
    }
    #[test]
    fn test_pow_mod() {
        use crate::math::mod_pow::modpow;
        for i in XorShift::new().take(1000).map(|x: u64| x as u128 % MOD) {
            for j in XorShift::new().take(1000).map(|x: u64| x as u128 % MOD) {
                assert_eq!(
                    pow_mod(i, j as usize),
                    modpow(i as i64, j as i64, MOD as i64) as u128
                );
            }
        }
    }
}

mod monoid {

    use super::*;
    use crate::structure::segment_tree::Monoid;
    struct RollingHashMonoid {}
    impl Monoid for RollingHashMonoid {
        type T = (u128, u128);

        fn identity_element() -> Self::T {
            (0, 1)
        }

        fn binary_operation(a: &Self::T, b: &Self::T) -> Self::T {
            (
                {
                    let buff = mul_mod(a.0, b.1) + b.0;
                    if buff >= MOD {
                        buff - MOD
                    } else {
                        buff
                    }
                },
                mul_mod(a.1, b.1),
            )
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::structure::segment_tree::SegmentTree;

        use super::*;
        #[test]
        fn m() {
            let mut sg = SegmentTree::<RollingHashMonoid>::new(29);
            let base = XorShift::new().next().unwrap() as u128 % MOD;

            for (i, &value) in b"abcdefghijklmnopqrstuvwxyzabc".iter().enumerate() {
                sg.set(i, (value as u128, base));
            }

            assert_eq!(sg.query(0, 3), sg.query(26, 29));
            sg.set(3, (b'a' as u128, base));
            sg.set(4, (b'b' as u128, base));
            sg.set(5, (b'c' as u128, base));
            assert_eq!(sg.query(0, 3), sg.query(3, 6));

            let rh = RollingHash::new(b"abcdefghijklmnopqrstuvwxyzabc", 3);
            assert_eq!(sg.query(0, 3).0, rh.text_hash[0]);
        }
    }
}
