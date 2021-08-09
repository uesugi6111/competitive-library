//! Weighted Union Find
use std::collections::{HashMap, HashSet};
#[derive(Debug, Clone)]
enum Node {
    Root(usize),
    Child((usize, i64)),
}
///UnionFind
#[derive(Clone, Debug)]
pub struct WeightedDisjointSetUnion {
    uf: Vec<Node>,
}

impl WeightedDisjointSetUnion {
    pub fn new(n: usize) -> WeightedDisjointSetUnion {
        WeightedDisjointSetUnion {
            uf: vec![Node::Root(1); n],
        }
    }

    pub fn root(&mut self, target: usize) -> (usize, i64) {
        match self.uf[target] {
            Node::Root(_) => (target, 0),
            Node::Child((par, w)) => {
                let (root, root_w) = self.root(par);
                self.uf[target] = Node::Child((root, root_w + w));
                (root, root_w + w)
            }
        }
    }
    pub fn unite(&mut self, x: usize, y: usize, weight: i64) -> bool {
        let rx = self.root(x);
        let ry = self.root(y);
        if rx.0 == ry.0 {
            return false;
        }
        let size_x = self.size(x);
        let size_y = self.size(y);
        let w = weight + ry.1 - rx.1;

        let (rx, ry, w) = if size_x > size_y {
            (rx, ry, -w)
        } else {
            (ry, rx, w)
        };

        self.uf[rx.0] = Node::Root(size_x + size_y);
        self.uf[ry.0] = Node::Child((rx.0, w));

        true
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x).0 == self.root(y).0
    }
    pub fn size(&mut self, x: usize) -> usize {
        let (root, _) = self.root(x);
        match self.uf[root] {
            Node::Root(size) => size,
            Node::Child(_) => 0,
        }
    }
    pub fn get_diff(&mut self, x: usize, y: usize) -> i64 {
        let rx = self.root(x);
        let ry = self.root(y);
        assert_eq!(rx.0, ry.0);
        rx.1 - ry.1
    }
    pub fn get_same_group(&mut self, x: usize) -> HashSet<usize> {
        let root = self.root(x);
        let mut g = HashSet::new();
        for i in 0..self.uf.len() {
            if root.0 == self.root(i).0 {
                g.insert(i);
            }
        }
        g
    }
    pub fn get_all_groups(&mut self) -> HashMap<usize, HashSet<(usize, i64)>> {
        let mut map = HashMap::new();
        for i in 0..self.uf.len() {
            let (root, w) = self.root(i);

            map.entry(root).or_insert_with(HashSet::new).insert((i, w));
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dsu() {
        let mut d = WeightedDisjointSetUnion::new(5);
        d.unite(0, 1, 1);
        assert!(d.is_same(0, 1));
        d.unite(1, 2, 1);
        assert!(d.is_same(0, 2));
        assert_eq!(d.size(0), 3);
        assert!(!d.is_same(0, 3));

        d.unite(0, 3, 3);
        d.unite(1, 4, 3);

        for i in 0..5 {
            assert_eq!(d.get_diff(0, i), i as i64);
        }
    }
    #[test]
    fn test_dsu_w2() {
        let mut dsu = WeightedDisjointSetUnion::new(16);
        dsu.unite(0, 1, 2);
        dsu.unite(2, 3, 4);
        dsu.unite(4, 5, 5);
        dsu.unite(6, 7, 5);
        dsu.unite(8, 9, 8);
        dsu.unite(10, 11, 5);
        dsu.unite(12, 13, 8);
        dsu.unite(14, 15, 7);

        dsu.unite(15, 13, -16);
        dsu.unite(11, 9, -9);
        dsu.unite(7, 5, -53);
        dsu.unite(3, 1, -9);

        dsu.unite(0, 15, 973);
        dsu.unite(4, 11, 81);

        dsu.unite(3, 4, 853);

        let ans = vec![
            0, 2, 7, 11, 864, 869, 917, 922, 928, 936, 940, 945, 949, 957, 966, 973,
        ];
        for (i, &v) in ans.iter().enumerate() {
            assert_eq!(dsu.get_diff(0, i), v);
        }
    }
}
