//! HL 分解

pub struct HeavyLightDecomposition {
    root: usize,
    parent: Vec<usize>,
    e: Vec<Vec<usize>>,
    child_count: Vec<usize>,
    depths: Vec<usize>,

    pre: Vec<usize>,

    hld: Vec<usize>,
    head: Vec<usize>,
}

impl HeavyLightDecomposition {
    pub fn new(root: usize, parent: &[usize]) -> Self {
        let e = {
            let mut e = vec![vec![]; parent.len()];
            for (i, &v) in parent.iter().enumerate() {
                if i == v {
                    continue;
                }
                e[v].push(i);
            }
            e
        };

        Self {
            root,
            parent: parent.to_vec(),
            e,
            child_count: vec![0; parent.len()],

            depths: vec![0; parent.len()],

            pre: vec![0; parent.len()],
            hld: vec![],
            head: (0..parent.len()).collect(),
        }
    }
    pub fn decompose(&mut self) -> Vec<usize> {
        let init = self.root;
        self.decompose_inner(init, init);

        self.hld.clone()
    }
    fn decompose_inner(&mut self, v: usize, a: usize) {
        self.pre[v] = self.hld.len();
        self.hld.push(v);
        self.head[v] = a;

        if self.e[v].is_empty() {
            return;
        }
        let mut m = 0;
        let mut index = 0;
        for i in 0..self.e[v].len() {
            if self.count_node(self.e[v][i]) > m {
                m = self.count_node(self.e[v][i]);
                index = i;
            }
        }
        self.decompose_inner(self.e[v][index], a);

        for i in 0..self.e[v].len() {
            if i != index {
                self.decompose_inner(self.e[v][i], self.e[v][i]);
            }
        }
    }
    pub fn count_node(&mut self, value: usize) -> usize {
        if self.child_count[value] != 0 {
            return self.child_count[value];
        }
        self.child_count[value] = 1;
        for i in 0..self.e[value].len() {
            self.child_count[value] += self.count_node(self.e[value][i]);
        }
        self.child_count[value]
    }

    pub fn depth(&mut self, v: usize) -> usize {
        if self.depths[v] != 0 {
            return self.depths[v];
        }
        if self.parent[v] == v {
            return 0;
        }
        self.depths[v] = self.depth(self.parent[v]) + 1;
        self.depths[v]
    }

    pub fn query(&mut self, u: usize, v: usize) -> Vec<(usize, usize)> {
        let (mut u, mut v) = (u, v);

        let mut ret = vec![];
        while self.head[u] != self.head[v] {
            if self.depth(self.head[u]) <= self.depth(self.head[v]) {
                ret.push((self.pre[self.head[v]], self.pre[v]));
                v = self.parent[self.head[v]];
            } else {
                ret.push((self.pre[self.head[u]], self.pre[u]));
                u = self.parent[self.head[u]];
            }
        }
        ret.push(if self.pre[u] < self.pre[v] {
            (self.pre[u], self.pre[v])
        } else {
            (self.pre[v], self.pre[u])
        });
        ret
    }
    pub fn get_lca(&mut self, u: usize, v: usize) -> Option<usize> {
        if u == v {
            return Some(u);
        }
        let common_range = *self.query(u, v).last()?;
        Some(if self.depth(common_range.0) < self.depth(common_range.1) {
            common_range.0
        } else {
            common_range.1
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hld() {
        let v = vec![0, 0, 1, 2, 2, 1, 0, 6, 7, 7, 0, 10];

        let mut hld = HeavyLightDecomposition::new(0, &v);
        let _h = hld.decompose();

        use std::collections::HashSet;

        assert_eq!(
            hld.query(4, 9).iter().collect::<HashSet<_>>(),
            vec![(9, 9), (4, 4), (6, 7), (0, 2)]
                .iter()
                .collect::<HashSet<_>>()
        );
    }
    #[test]
    fn test_lca() {
        let v = vec![0, 0, 0, 2, 2];

        let mut hld = HeavyLightDecomposition::new(0, &v);
        let _h = hld.decompose();
        dbg!(_h);
        for &(u, v, ans) in [
            (0, 0, 0),
            (0, 1, 0),
            (0, 2, 0),
            (0, 3, 0),
            (0, 4, 0),
            (1, 1, 1),
            (1, 2, 0),
            (1, 3, 0),
            (1, 4, 0),
            (2, 2, 2),
            (2, 3, 2),
            (2, 4, 2),
            (3, 3, 3),
            (3, 4, 2),
            (4, 4, 4),
        ]
        .iter()
        {
            assert_eq!(hld.get_lca(u, v).unwrap(), ans, "{} {}", u, v);
        }
    }
}
