//! 文字列に関係しそうなもの
pub mod aho_corasick;
pub mod manacher;
pub mod rolling_hash;
pub mod z_algorithm;

#[deprecated(since = "0.1.0", note = "use `manacher` instead")]
pub mod manachar {
    pub use super::manacher::*;
}

#[deprecated(since = "0.1.0", note = "use `rolling_hash` instead")]
pub mod rollinghash {
    pub use super::rolling_hash::*;
}
