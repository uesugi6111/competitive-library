use std::cmp::Ordering;

use crate::structure::skew_heap::SkewHeap;

#[derive(Debug, PartialEq, Eq, PartialOrd)]
struct Edge {
    from: usize,
    to: usize,
    cost: i64,
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&(other.cost))
    }
}
pub fn directed_mst() {
    
}
