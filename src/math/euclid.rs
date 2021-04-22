//! ユークリッドさんありがとう
pub fn gcd(m: i64, n: i64) -> i64 {
    if m == 0 {
        n.abs()
    } else {
        gcd(n % m, m)
    }
}
pub fn lcm(m: i64, n: i64) -> i64 {
    m * n / gcd(m, n)
}
