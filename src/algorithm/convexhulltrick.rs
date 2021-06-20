// 線形で処理できるもの

// 追加する直線の傾きが単調減少(増加)
// 計算する最小値(最大値)の座標xが単調増加(減少)

use std::collections::VecDeque;

pub struct LinearFunction(i64, i64);

pub struct ConvexHullTrick {
    d: std::collections::VecDeque<LinearFunction>,
    f: fn(&LinearFunction, i64) -> i64,
}
fn f(f: &LinearFunction, x: i64) -> i64 {
    f.0 * x + f.1
}

impl ConvexHullTrick {
    pub fn new() -> Self {
        ConvexHullTrick {
            d: VecDeque::new(),
            f,
        }
    }
    pub fn from(f: fn(&LinearFunction, i64) -> i64) -> Self {
        ConvexHullTrick {
            d: VecDeque::new(),
            f,
        }
    }
    fn check(f1: &LinearFunction, f2: &LinearFunction, f3: &LinearFunction) -> bool {
        (f2.0 - f1.0) * (f3.1 - f2.1) >= (f2.1 - f1.1) * (f3.0 - f2.0)
    }

    // a_{prev} >= a

    pub fn add_line(&mut self, a: i64, b: i64) {
        let f = LinearFunction(a, b);

        while self.d.len() >= 2
            && ConvexHullTrick::check(&self.d[self.d.len() - 2], &self.d[self.d.len() - 1], &f)
        {
            self.d.pop_back();
        }
        self.d.push_back(f);
    }

    // x_{prev} <= x
    pub fn query(&mut self, x: i64) -> i64 {
        while self.d.len() >= 2 && (self.f)(&(self.d)[0], x) >= (self.f)(&(self.d)[1], x) {
            self.d.pop_front();
        }
        (self.f)(&(self.d)[0], x)
    }
}
impl Default for ConvexHullTrick {
    fn default() -> Self {
        ConvexHullTrick::new()
    }
}

#[test]
fn test_cht() {
    let mut cht = ConvexHullTrick::new();
    for (a, b) in &[(2, 0), (1, 1), (0, -1), (-1, 0)] {
        cht.add_line(*a, *b);
    }
    let ans: Vec<_> = (0..10).map(|i| cht.query(-5 + i)).collect();
    assert_eq!(ans, [-10, -8, -6, -4, -2, -1, -1, -2, -3, -4]);
}
