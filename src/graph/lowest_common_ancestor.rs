struct Node {
    pub parent: Option<usize>,
    pub me: usize,
    pub depth: i64,
}

impl Node {
    pub fn new(parent: Option<usize>, me: usize, depth: i64) -> Self {
        Node { parent, me, depth }
    }
}

pub struct LowestCommonAncestor {
    max_log_v: usize,
    depth: Vec<i64>,
    ancestors: Vec<Vec<Option<usize>>>,
}

impl LowestCommonAncestor {
    pub fn new(edges: &[Vec<i64>], root: usize) -> Self {
        let max_v = edges.len();
        let max_log_v = ((max_v as f64).ln() / 2.0_f64.ln()) as usize + 1;
        let mut ancestors = vec![vec![None; max_v]; max_log_v + 1];
        let mut depth = vec![0; max_v];

        let mut q = std::collections::VecDeque::new();
        q.push_back(Node::new(None, root, 0));
        while let Some(u) = q.pop_front() {
            ancestors[0][u.me] = u.parent;

            depth[u.me] = u.depth;
            for i in 0..edges[u.me].len() {
                if u.parent.is_none() || u.parent.unwrap() as i64 != edges[u.me][i] {
                    q.push_back(Node::new(Some(u.me), edges[u.me][i] as usize, u.depth + 1));
                }
            }
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
            depth,
            ancestors,
        }
    }

    pub fn get_lca(&self, u: usize, v: usize) -> Option<usize> {
        let (mut u, mut v) = if self.depth[u] > self.depth[v] {
            (v, u)
        } else {
            (u, v)
        };

        for k in 0..self.max_log_v {
            if (((self.depth[v] - self.depth[u]) >> k) & 1) == 1 {
                v = self.ancestors[k][v].unwrap();
            }
        }

        if u == v {
            return Some(u);
        }

        for k in (0..self.max_log_v).rev() {
            if self.ancestors[k][u].is_some()
                && self.ancestors[k][v].is_some()
                && self.ancestors[k][u] != self.ancestors[k][v]
            {
                u = self.ancestors[k][u].unwrap();
                v = self.ancestors[k][v].unwrap();
            }
        }
        self.ancestors[0][u]
    }

    pub fn get_distance() -> i64 {
        0
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
