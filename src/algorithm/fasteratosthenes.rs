//! エラトステネス

///エラトステネスの篩
pub struct Eratosthenes {
    flags_: Vec<u8>,
    n: usize,
}
impl Eratosthenes {
    const K_MASK: [[u8; 8]; 8] = [
        [0xfe, 0xfd, 0xfb, 0xf7, 0xef, 0xdf, 0xbf, 0x7f],
        [0xfd, 0xdf, 0xef, 0xfe, 0x7f, 0xf7, 0xfb, 0xbf],
        [0xfb, 0xef, 0xfe, 0xbf, 0xfd, 0x7f, 0xf7, 0xdf],
        [0xf7, 0xfe, 0xbf, 0xdf, 0xfb, 0xfd, 0x7f, 0xef],
        [0xef, 0x7f, 0xfd, 0xfb, 0xdf, 0xbf, 0xfe, 0xf7],
        [0xdf, 0xf7, 0x7f, 0xfd, 0xbf, 0xfe, 0xef, 0xfb],
        [0xbf, 0xfb, 0xf7, 0x7f, 0xfe, 0xef, 0xdf, 0xfd],
        [0x7f, 0xbf, 0xdf, 0xef, 0xf7, 0xfb, 0xfd, 0xfe],
    ];

    const C0: [[usize; 8]; 8] = [
        [0, 0, 0, 0, 0, 0, 0, 1],
        [1, 1, 1, 0, 1, 1, 1, 1],
        [2, 2, 0, 2, 0, 2, 2, 1],
        [3, 1, 1, 2, 1, 1, 3, 1],
        [3, 3, 1, 2, 1, 3, 3, 1],
        [4, 2, 2, 2, 2, 2, 4, 1],
        [5, 3, 1, 4, 1, 3, 5, 1],
        [6, 4, 2, 4, 2, 4, 6, 1],
    ];
    const MOD_30: [usize; 8] = [1, 7, 11, 13, 17, 19, 23, 29];
    const C1: [usize; 8] = [6, 4, 2, 4, 2, 4, 6, 2];

    ///初期化
    ///素数フラグを処理
    ///- param n:usize 探索上限
    pub fn new(n: usize) -> Self {
        if n > 10_000_000_000 {
            panic!();
        }

        let size = n / 30 + if n % 30 != 0 { 1 } else { 0 };
        let mut flags_ = vec![0xff_u8; size];
        flags_[0] = 0xfe;

        let remainder = n % 30;
        flags_[size - 1] = match remainder {
            1..=1 => 0x0,
            2..=7 => 0x1,
            8..=11 => 0x3,
            12..=13 => 0x7,
            14..=17 => 0xf,
            18..=19 => 0x1f,
            20..=23 => 0x3f,
            24..=29 => 0x7f,
            _ => panic!(),
        };

        let quart_x = ((n as f64).sqrt() + 1.0) as usize / 30 + 1;

        for i in 0..quart_x {
            let mut flags: u8 = flags_[i];

            while flags != 0 {
                let lsb = flags & flags.wrapping_neg();
                let i_bit = lsb.trailing_zeros() as usize;

                let m = Eratosthenes::MOD_30[i_bit];

                let mut k = i_bit;
                let mut j = i * (30 * i + 2 * m) + (m * m) / 30;

                while j < flags_.len() {
                    flags_[j] &= Eratosthenes::K_MASK[i_bit][k];

                    j += i * Eratosthenes::C1[k] + Eratosthenes::C0[i_bit][k];
                    k = (k + 1) & 7;
                }
                flags &= flags - 1;
            }
        }

        Eratosthenes { flags_, n }
    }

    ///素数の個数をカウント
    pub fn count(&mut self) -> usize {
        let mut ret = [2usize, 3, 5].iter().take_while(|x| self.n >= **x).count(); // count 2, 3, 5
        for f in &self.flags_ {
            ret += f.count_ones() as usize;
        }
        ret
    }

    ///フラグから素数配列を生成
    pub fn primes(&self) -> Vec<usize> {
        let mut ret = Vec::<usize>::new();

        [2usize, 3, 5]
            .iter()
            .take_while(|x| self.n >= **x)
            .for_each(|x| ret.push(*x));

        for (i, f) in self.flags_.iter().enumerate() {
            for (ii, m) in Eratosthenes::MOD_30.iter().enumerate() {
                if (*f & (1 << ii)) != 0 {
                    ret.push(30 * i + *m);
                }
            }
        }
        ret
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn aaa() {
        let mut e = Eratosthenes::new(100_000_000);

        assert_eq!(e.count(), 5_761_455);
    }
    #[test]
    fn a2() {
        let mut e = Eratosthenes::new(1);

        assert_eq!(e.count(), 0);
    }
}
