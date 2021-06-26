//! エラトステネス

use std::collections::BTreeMap;

pub struct LinearSieve {
    n: usize,
    pub table: Vec<i64>,
    pub primes: Vec<usize>,
}
impl LinearSieve {
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
        LinearSieve { n, table, primes }
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
        assert_eq!(LinearSieve::new(1_000_000).primes.len(), 78_498);
    }

    #[test]
    fn test_factorize() {
        let ls = LinearSieve::new(1_000_000);
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
}
