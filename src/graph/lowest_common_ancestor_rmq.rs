use crate::graph::euler_tour;
use crate::structure::sparse_table;

struct MinDepth {}
impl sparse_table::Band for MinDepth {
    type T = (i32, i32);

    fn operate(a: &Self::T, b: &Self::T) -> Self::T {
        if a.1 < b.1 {
            *a
        } else {
            *b
        }
    }
}

pub struct LowestCommonAncestor {
    st: sparse_table::SparseTable<MinDepth>,
    first_look: Vec<usize>,
}

impl LowestCommonAncestor {
    pub fn new(e: &[Vec<usize>], root: usize) -> Self {
        let (tour, first_look, depths) = euler_tour::euler_tour(e, root);
        let v: Vec<(i32, i32)> = tour.iter().map(|&x| (x as i32, depths[x] as i32)).collect();
        let st = sparse_table::SparseTable::<MinDepth>::new(&v);

        LowestCommonAncestor { st, first_look }
    }

    pub fn get_lca(&self, u: usize, v: usize) -> usize {
        let range = if self.first_look[u] < self.first_look[v] {
            self.first_look[u]..self.first_look[v]
        } else {
            self.first_look[v]..self.first_look[u]
        };
        self.st.fold(range).0 as usize
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_lca() {
        let n = 5;
        let mut e = vec![vec![]; n];
        for (i, &v) in [0, 0, 2, 2].iter().enumerate() {
            e[v].push(i + 1);
        }

        let lca = LowestCommonAncestor::new(&e, 0);
        for &(u, v, ans) in [(0, 1, 0), (0, 4, 0), (1, 2, 0), (2, 3, 2), (3, 4, 2)].iter() {
            assert_eq!(lca.get_lca(u, v), ans);
        }
    }
}
