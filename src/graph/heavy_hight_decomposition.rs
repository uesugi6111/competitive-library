//! HL 分解

#[derive(Debug, Clone)]
pub struct HeavyLightDecomposition {
    /// 根
    root: usize,
    /// 各頂点の親
    parent: Vec<usize>,
    /// e\[i]\[j] は i を親に持つ
    e: Vec<Vec<usize>>,
    /// 各頂点から見た子要素数
    child_count: Vec<usize>,
    /// 各頂点の深さ
    depths: Vec<usize>,
    pre: Vec<usize>,
    /// HLD
    hld: Vec<usize>,
    /// 列の先頭
    head: Vec<usize>,
}

impl HeavyLightDecomposition {
    #[inline]
    pub fn new(root: usize, parent: &[usize]) -> Self {
        let mut e = vec![vec![]; parent.len()];
        for (i, &v) in parent.iter().enumerate().filter(|&x| x.0 != *x.1) {
            e[v].push(i);
        }

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

    /// 分解
    #[inline]
    pub fn decompose(&mut self) -> Vec<usize> {
        let init = self.root;
        self.count_node(init);
        self.count_depth(init);
        self.decompose_inner_root(init);

        self.hld.clone()
    }

    #[inline]
    /// 根から分解
    fn decompose_inner_root(&mut self, v: usize) {
        self.decompose_inner(v, v);
    }

    /// 分解用の内部関数
    #[inline]
    fn decompose_inner(&mut self, v: usize, h: usize) {
        self.pre[v] = self.hld.len();
        self.hld.push(v);
        self.head[v] = h;

        if self.e[v].is_empty() {
            return;
        }
        let index = self.e[v]
            .iter()
            .enumerate()
            .max_by_key(|&(_, &y)| self.child_count[y])
            .unwrap()
            .0;
        self.decompose_inner(self.e[v][index], h);

        for i in (0..self.e[v].len()).filter(|&i| i != index) {
            self.decompose_inner_root(self.e[v][i]);
        }
    }

    /// 子要素のカウント
    #[inline]
    fn count_node(&mut self, index: usize) -> usize {
        if self.child_count[index] != 0 {
            return self.child_count[index];
        }
        self.child_count[index] = 1;
        for i in 0..self.e[index].len() {
            self.child_count[index] += self.count_node(self.e[index][i]);
        }
        self.child_count[index]
    }

    /// 深さのカウント
    #[inline]
    fn count_depth(&mut self, index: usize) -> usize {
        if self.depths[index] != 0 {
            return self.depths[index];
        }
        if self.parent[index] == index {
            return 0;
        }
        self.depths[index] = self.count_depth(self.parent[index]) + 1;
        self.depths[index]
    }

    /// HLD 配列の区間を返す
    #[inline]
    pub fn query(&mut self, mut u: usize, mut v: usize) -> Vec<(usize, usize)> {
        debug_assert!(!self.hld.is_empty());

        let mut ret = vec![];
        while self.head[u] != self.head[v] {
            if self.count_depth(self.head[u]) <= self.count_depth(self.head[v]) {
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

    /// LCA を求める
    #[inline]
    pub fn get_lca(&mut self, u: usize, v: usize) -> Option<usize> {
        let common_range = *self.query(u, v).last()?;
        Some(self.hld[common_range.0])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hld() {
        let v = vec![0, 0, 1, 2, 2, 1, 0, 6, 7, 7, 0, 10];

        let mut hld = HeavyLightDecomposition::new(0, &v);
        let h = hld.decompose();
        dbg!(&h);

        use std::collections::HashSet;
        let mut set = HashSet::new();
        for (f, t) in hld.query(4, 9) {
            for &i in h.iter().take(t + 1).skip(f) {
                set.insert(i);
            }
        }
        let ans_set = [4_usize, 2, 1, 0, 6, 7, 9].iter().cloned().collect();

        assert_eq!(set, ans_set);
    }
    #[test]
    fn test_lca() {
        let v = vec![0, 0, 0, 2, 2];

        let mut hld = HeavyLightDecomposition::new(0, &v);
        let h = hld.decompose();
        dbg!(&h);
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
