//! Union find
use std::collections::{HashMap, HashSet};
#[derive(Copy, Clone, Debug)]
enum Node {
    Root(usize),
    Child(usize),
}
///UnionFind
#[derive(Clone, Debug)]
pub struct DisjointSetUnion {
    nodes: Vec<Node>,
}

impl DisjointSetUnion {
    pub fn new(n: usize) -> DisjointSetUnion {
        DisjointSetUnion {
            nodes: vec![Node::Root(1); n],
        }
    }

    pub fn find_root(&mut self, target: usize) -> usize {
        match unsafe { *self.nodes.get_unchecked(target) } {
            Node::Root(_) => target,
            Node::Child(parent) => {
                let parent_index = self.find_root(parent);
                self.nodes[target] = Node::Child(parent_index);
                parent_index
            }
        }
    }
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let rx = self.find_root(x);
        let ry = self.find_root(y);
        if rx == ry {
            return false;
        }
        let size_x = self.size(x);
        let size_y = self.size(y);

        let (i, j) = if size_x > size_y { (rx, ry) } else { (ry, rx) };
        self.nodes[i] = Node::Root(size_x + size_y);
        self.nodes[j] = Node::Child(i);

        true
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find_root(x) == self.find_root(y)
    }
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.find_root(x);
        match self.nodes[root] {
            Node::Root(size) => size,
            Node::Child(_) => 0,
        }
    }
    pub fn get_same_group(&mut self, x: usize) -> HashSet<usize> {
        let root = self.find_root(x);
        let mut g = HashSet::new();
        for i in 0..self.nodes.len() {
            if root == self.find_root(i) {
                g.insert(i);
            }
        }
        g
    }
    pub fn get_all_groups(&mut self) -> HashMap<usize, HashSet<usize>> {
        let mut map = HashMap::new();
        for i in 0..self.nodes.len() {
            map.entry(self.find_root(i))
                .or_insert_with(HashSet::new)
                .insert(i);
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dsu() {
        let mut d = DisjointSetUnion::new(4);
        d.unite(0, 1);
        assert!(d.is_same(0, 1));
        d.unite(1, 2);
        assert!(d.is_same(0, 2));
        assert_eq!(d.size(0), 3);
        assert!(!d.is_same(0, 3));

        // assert_eq!(d.get_all_groups(), vec![vec![0, 1, 2], vec![3]]);
    }
}
