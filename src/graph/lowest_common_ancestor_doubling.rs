//! ダブリングを使用してLCA を求める
use std::collections::VecDeque;

struct Node {
    pub parent: Option<usize>,
    pub number: usize,
    pub depth: i64,
}

impl Node {
    #[inline]
    pub fn new(parent: Option<usize>, number: usize, depth: i64) -> Self {
        Node {
            parent,
            number,
            depth,
        }
    }
}

pub struct LowestCommonAncestor {
    max_log_v: usize,
    root: usize,
    depths: Vec<i64>,
    ancestors: Vec<Vec<Option<usize>>>,
}

impl LowestCommonAncestor {
    // 隣接リストで受け取る
    #[inline]
    pub fn new(edges: &[Vec<i64>], root: usize) -> Self {
        let max_v = edges.len();
        let max_log_v = ((max_v as f64).ln() / 2.0_f64.ln()) as usize + 1;
        let mut ancestors = vec![vec![None; max_v]; max_log_v + 1];
        let mut depths = vec![0; max_v];

        let mut q = VecDeque::new();
        q.push_back(Node::new(None, root, 0));
        while let Some(node) = q.pop_front() {
            ancestors[0][node.number] = node.parent;

            depths[node.number] = node.depth;

            edges[node.number]
                .iter()
                .filter(|&&v| node.parent.filter(|&x| x == v as usize).is_none())
                .for_each(|&v| {
                    q.push_back(Node::new(Some(node.number), v as usize, node.depth + 1))
                });
        }

        (0..max_log_v).for_each(|i| {
            (0..max_v).for_each(|j| {
                if let Some(ancetor) = ancestors[i][j] {
                    ancestors[i + 1][j] = ancestors[i][ancetor];
                }
            })
        });

        LowestCommonAncestor {
            max_log_v,
            root,
            depths,
            ancestors,
        }
    }
    #[inline]
    pub fn get_lca(&self, u: usize, v: usize) -> Option<usize> {
        let (mut u, mut v) = if self.depths[u] > self.depths[v] {
            (v, u)
        } else {
            (u, v)
        };

        for k in 0..self.max_log_v {
            if (((self.depths[v] - self.depths[u]) >> k) & 1) == 1 {
                v = self.ancestors[k][v].unwrap();
            }
        }

        if u == v {
            return Some(u);
        }

        for k in (0..self.max_log_v).rev() {
            if self.ancestors[k][u].is_none()
                || self.ancestors[k][v].is_none()
                || self.ancestors[k][u] == self.ancestors[k][v]
            {
                continue;
            }

            u = self.ancestors[k][u].unwrap();
            v = self.ancestors[k][v].unwrap();
        }
        self.ancestors[0][u]
    }
    #[inline]
    pub fn get_distance(&self, u: usize, v: usize) -> i64 {
        let lca = self.get_lca(u, v).unwrap_or(self.root);
        self.depths[u] + self.depths[v] - self.depths[lca] * 2
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
            e[v].push(i as i64 + 1);
        }

        let lca = LowestCommonAncestor::new(&e, 0);
        for &(u, v, ans) in [(0, 1, 0), (0, 4, 0), (1, 2, 0), (2, 3, 2), (3, 4, 2)].iter() {
            assert_eq!(lca.get_lca(u, v).unwrap_or(0), ans);
        }
    }
}
