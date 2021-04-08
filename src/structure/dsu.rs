use std::collections::{HashMap, HashSet};
#[derive(Debug, Clone)]
enum Node {
    Root(usize),
    Child(usize),
}
///UnionFind
#[derive(Clone, Debug)]
pub struct Dsu {
    uf: Vec<Node>,
}

impl Dsu {
    pub fn new(n: usize) -> Dsu {
        Dsu {
            uf: vec![Node::Root(1); n],
        }
    }

    pub fn root(&mut self, target: usize) -> usize {
        match self.uf[target] {
            Node::Root(_) => target,
            Node::Child(par) => {
                let root = self.root(par);
                self.uf[target] = Node::Child(self.root(par));
                root
            }
        }
    }
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let rx = self.root(x);
        let ry = self.root(y);
        if rx == ry {
            return false;
        }
        let size_x = self.size(x);
        let size_y = self.size(y);

        let (i, j) = if size_x > size_y { (rx, ry) } else { (ry, rx) };
        self.uf[i] = Node::Root(size_x + size_y);
        self.uf[j] = Node::Child(i);

        true
    }
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        match self.uf[root] {
            Node::Root(size) => size,
            Node::Child(_) => 0,
        }
    }
    pub fn get_same_group(&mut self, x: usize) -> HashSet<usize> {
        let root = self.root(x);
        let mut g = HashSet::new();
        for i in 0..self.uf.len() {
            if root == self.root(i) {
                g.insert(i);
            }
        }
        g
    }
    pub fn get_all_groups(&mut self) -> HashMap<usize, HashSet<usize>> {
        let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();
        for i in 0..self.uf.len() {
            let root = self.root(i);

            map.entry(root).or_insert_with(HashSet::new).insert(i);
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsu_works() {
        let mut d = Dsu::new(4);
        d.unite(0, 1);
        assert_eq!(d.is_same(0, 1), true);
        d.unite(1, 2);
        assert_eq!(d.is_same(0, 2), true);
        assert_eq!(d.size(0), 3);
        assert_eq!(d.is_same(0, 3), false);
        // assert_eq!(d.groups(), vec![vec![0, 1, 2], vec![3]]);
    }
}
