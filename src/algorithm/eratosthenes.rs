//! エラトステネス

use std::collections::BTreeMap;

pub struct Sieve {
    n: usize,
    pub table: Vec<i64>,
    pub primes: Vec<usize>,
}
impl Sieve {
    pub fn new(n: usize) -> Self {
        let mut primes = vec![];
        let mut table = vec![0_i64; n + 1];
        for i in 2..n + 1 {
            if table[i] == 0 {
                primes.push(i);
                table[i] = i as i64;
            }
            for &p in &primes {
                if p * i > n {
                    break;
                }
                table[p * i] = p as i64;
            }
        }
        Sieve { n, table, primes }
    }

    pub fn factorize(&self, n: i64) -> BTreeMap<i64, i64> {
        assert!(self.n as i64 >= n);

        let mut map = BTreeMap::new();
        let mut target = n;

        while target > 1 {
            let p = self.table[target as usize];
            let mut count = 0;

            while self.table[target as usize] == p {
                target /= p;
                count += 1;
            }
            map.insert(p, count);
        }
        map
    }

    // 約数列挙
    pub fn divisors(&self, n: i64) -> Vec<i64> {
        assert_ne!(n, 0);

        let mut ret = vec![1];
        let factor = self.factorize(n);

        for (k, exp) in factor {
            for i in 0..ret.len() {
                let mut v = 1;
                for _ in 0..exp {
                    v *= k;
                    ret.push(ret[i] * v);
                }
            }
        }
        ret
    }

    pub fn is_prime(&self, n: i64) -> bool {
        assert!(self.n as i64 >= n);
        self.table[n as usize] == n
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_prime() {
        assert_eq!(Sieve::new(1_000_000).primes.len(), 78_498);
    }

    #[test]
    fn test_factorize() {
        let ls = Sieve::new(1_000_000);
        let case = vec![
            31, 4657, 3, 65732, 7836, 1278, 8615, 798_179, 425, 38715, 3272, 149, 7, 48, 97, 5823,
            517, 8231, 6986, 751, 8651, 671, 7, 23,
        ];

        for v in case {
            let f = ls.factorize(v);

            let mut n = 1;
            for (k, value) in f {
                n *= k.pow(value as u32);
            }
            assert_eq!(v, n);
        }
    }

    use std::collections::HashMap;
    #[test]
    fn test_divide() {
        let ls = Sieve::new(1_000_000);
        let map = {
            let mut ret = HashMap::new();
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
            ret
        };

        for (k, v) in map {
            let mut a = ls.divisors(k);
            a.sort_unstable();
            assert_eq!(a, v);
        }
    }
}
