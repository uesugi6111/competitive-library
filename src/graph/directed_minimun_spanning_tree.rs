use std::cmp::Ordering;

use crate::structure::disjoint_set_union::Dsu;
use crate::structure::skew_heap_lazy::SkewHeap;

use crate::structure::disjoint_set_union_undo::DisjointSetUnionRollback;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd)]
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
pub fn directed_mst(e: &[Vec<(usize, i64)>], root: usize) {
    let mut heap = vec![SkewHeap::new(); e.len()];

    for i in 0..e.len() {
        for j in 0..e[i].len() {
            heap[e[i][j].0].push(
                e[i][j].1,
                Edge {
                    from: i,
                    to: e[i][j].0,
                    cost: e[i][j].1,
                },
            );
        }
    }
}
