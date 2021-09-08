//! 試割

use std::collections::HashMap;
pub fn trial_division(mut n: i64) -> HashMap<i64, i64> {
    let mut primes = HashMap::new();
    let mut i = 2;

    while i * i <= n {
        while n % i == 0 {
            n /= i;
            primes.entry(i).and_modify(|e| *e += 1).or_insert(1);
        }
        i += 1;
    }
    if n > 1 {
        primes.entry(n).and_modify(|e| *e += 1).or_insert(1);
    }
    primes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trial_division() {
        assert!(trial_division(25).contains_key(&5));
        assert!(trial_division(25).get(&5).unwrap() == &2);
    }
}
