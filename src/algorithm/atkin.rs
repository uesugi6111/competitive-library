pub fn sieve_of_atkin(n: u64) -> Vec<bool> {
    let mut is_prime = vec![false; n as usize + 1];
    let sqrt_n = (n as f64).sqrt() as u64 + 1;

    for i in 1..sqrt_n {
        for j in 1..sqrt_n {
            let ii = i.pow(2);
            let jj = j.pow(2);

            let mut buff = 4 * ii + jj;
            let buff_mod12 = buff % 12;
            if buff <= n && (buff_mod12 == 1 || buff_mod12 == 5) {
                is_prime[buff as usize] ^= true;
            }

            buff = 3 * ii + jj;
            if buff <= n && buff % 12 == 7 {
                is_prime[buff as usize] ^= true;
            }

            if i <= j {
                continue;
            }

            buff = 3 * ii - jj;
            if i > j && buff <= n && buff % 12 == 11 {
                is_prime[buff as usize] ^= true;
            }
        }
    }
    for i in 5..sqrt_n {
        if !is_prime[i as usize] {
            continue;
        }
        let k = i * i;
        for j in (k..n).step_by(k as usize) {
            is_prime[j as usize] = false;
        }
    }
    is_prime[2] = true;
    is_prime[3] = true;
    is_prime
}

#[test]
fn aaa() {
    let prime = sieve_of_atkin(1000000);

    let count: Vec<_> = prime
        .iter()
        .enumerate()
        .filter(|x| *x.1)
        .map(|x| x.0)
        .collect();
    assert_eq!(count.len(), 78498);
}
