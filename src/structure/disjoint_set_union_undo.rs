//! Union find undo
use std::collections::{HashMap, HashSet, VecDeque};
#[derive(Debug, Clone)]
enum Node {
    Root(usize, usize),
    Child(usize),
}
/// UnionFind
/// 経路圧縮を行わないことで undo を可能にする
#[derive(Clone, Debug)]
pub struct DisjointSetUnionRollback {
    uf: Vec<Node>,
    history: VecDeque<(usize, Node)>,
    restore_point: Option<usize>,
}

impl DisjointSetUnionRollback {
    /// 要素数 n の dsu を構築する
    #[inline]
    pub fn new(n: usize) -> DisjointSetUnionRollback {
        DisjointSetUnionRollback {
            uf: vec![Node::Root(1, 1); n],
            history: VecDeque::new(),
            restore_point: None,
        }
    }

    /// 根を取得
    /// 経路圧縮を行わない
    #[inline]
    pub fn root(&self, target: usize) -> usize {
        match self.uf[target] {
            Node::Root(_, _) => target,
            Node::Child(par) => self.root(par),
        }
    }

    /// 対象の木をマージ
    /// 経路圧縮を行わないため変更されるノード数は高々2
    /// 変更箇所をスタックで保存
    #[inline]
    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let rx = self.root(x);
        let ry = self.root(y);
        if rx == ry {
            return false;
        }
        self.history.push_back((rx, self.uf[rx].clone()));
        self.history.push_back((ry, self.uf[ry].clone()));
        let size_x = self.size(rx);
        let size_y = self.size(ry);
        let rank_x = self.rank(rx);
        let rank_y = self.rank(ry);
        let (i, j) = if rank_x > rank_y { (rx, ry) } else { (ry, rx) };
        self.uf[i] = Node::Root(
            size_x + size_y,
            (rank_x.min(rank_y) + 1).max(rank_x.max(rank_y)),
        );
        self.uf[j] = Node::Child(i);

        true
    }

    /// 同じ木に存在するか
    #[inline]
    pub fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    /// 所属する木のサイズ
    pub fn size(&mut self, x: usize) -> usize {
        let root = self.root(x);
        match self.uf[root] {
            Node::Root(size, _) => size,
            Node::Child(_) => 1,
        }
    }
    /// 所属する木のランク
    #[inline]
    pub fn rank(&mut self, x: usize) -> usize {
        let root = self.root(x);
        match self.uf[root] {
            Node::Root(_, rank) => rank,
            Node::Child(_) => 1,
        }
    }

    /// unite 操作の undo
    #[inline]
    pub fn undo(&mut self) {
        for _ in 0..2 {
            let (index, node) = self.history.pop_back().unwrap();
            self.uf[index] = node;
        }
    }

    /// 現時点の状態を保存
    /// 復元には rollback_snapshot
    #[inline]
    pub fn snapshot(&mut self) {
        self.restore_point = Some(self.history.len() >> 1);
    }

    /// 現時点での保存されている操作回数を返す
    #[inline]
    pub fn get_history_length(&self) -> usize {
        self.history.len() >> 1
    }

    /// rollback_snapshot で保存された状態へ復元
    #[inline]
    pub fn rollback_snapshot(&mut self) {
        self.rollback(self.restore_point.unwrap());
    }

    /// 復元
    /// 任意のタイミングで get_history_length を実行し取得した 値を使用する
    #[inline]
    pub fn rollback(&mut self, n: usize) {
        assert!(self.history.len() >= n << 1);

        while self.history.len() > n << 1 {
            self.undo();
        }
    }

    /// 同じ木に含まれるノードを返す
    #[inline]
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

    /// 全ノードを返却
    #[inline]
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
    fn test_dsu_rollback() {
        let mut dsu = DisjointSetUnionRollback::new(6);

        dsu.unite(0, 1);
        assert!(dsu.is_same(0, 1));
        dsu.unite(1, 2);
        assert!(dsu.is_same(0, 2));
        assert_eq!(dsu.size(0), 3);
        assert!(!dsu.is_same(0, 3));
        dsu.snapshot();
        dsu.unite(0, 3);
        dsu.unite(3, 4);
        dsu.unite(4, 5);
        assert_eq!(dsu.size(5), 6);
        assert!(dsu.is_same(0, 5));
        dsu.undo();
        assert!(!dsu.is_same(0, 5));
        dsu.rollback_snapshot();
        assert!(dsu.is_same(0, 2));
        assert_eq!(dsu.size(0), 3);
        assert!(!dsu.is_same(0, 3));
        dsu.rollback(0);
        assert!(!dsu.is_same(0, 1));
        assert_eq!(dsu.get_history_length(), 0);
    }
}
