pub fn ternary_search(mut low: f64, mut high: f64, f: Box<dyn Fn(f64) -> f64>) -> f64 {
    let mut cnt = 1000;
    while cnt > 0 {
        let c1 = (low * 2.0 + high) / 3.0;
        let c2 = (low + high * 2.0) / 3.0;

        if f(c1) > f(c2) {
            low = c1;
        } else {
            high = c2;
        }
        cnt -= 1;
    }
    high
}

#[cfg(test)]
mod tests {
    use super::ternary_search;
    // https://atcoder.jp/contests/abc279/tasks/abc279_d
    #[test]
    fn a() {
        let (a, b) = (10.0, 1.0);
        let f = move |x| (x * b) + a / ((x + 1.0) as f64).sqrt();
        let h = ternary_search(0.0, 1_000_000_000_000_000_000.0, Box::new(f));

        assert!((f((h + 0.5) as i64 as f64) - 7.773_502_691_9).abs() < 0.000_001);
    }
    #[test]
    fn b() {
        let (a, b) = (5.0, 10.0);
        let f = move |x| (x * b) + a / ((x + 1.0) as f64).sqrt();
        let h = ternary_search(0.0, 1_000_000_000_000_000_000.0, Box::new(f));

        assert!(dbg!((f((h + 0.5) as i64 as f64) - 5.000_000_000_0).abs()) < 0.000_001);
    }

    #[test]
    fn c() {
        let (a, b) = (1_000_000_000_000_000_000.0, 100.0);
        let f = move |x| (x * b) + a / ((x + 1.0) as f64).sqrt();
        let h = ternary_search(0.0, 1_000_000_000_000_000_000.0, Box::new(f));

        assert!(dbg!((f((h + 0.4) as i64 as f64) - 8_772_053_214_538.598_f64).abs()) < 0.01);
    }
}
