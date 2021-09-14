use std::cmp::Ordering;

use crate::structure::disjoint_set_union::Dsu;
use crate::structure::skew_heap_lazy::SkewHeap;

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
pub fn directed_mst(e: &[Vec<(usize, i64)>], root: usize) -> Option<(i64, Vec<usize>)> {
    let mut uf = Dsu::new(e.len());

    let mut from_v = vec![0; e.len()];
    let mut from_cost = vec![0; e.len()];
    let mut used = vec![0; e.len()];
    let mut heap = vec![SkewHeap::new(); e.len()];
    used[root] = 2;

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

    let mut ans = 0;
    for start in 0..e.len() {
        if used[start] != 0 {
            continue;
        }
        let mut cur = start;
        let mut processing = vec![];
        while used[cur] != 2 {
            used[cur] = 1;
            processing.push(cur);

            if heap[cur].is_empty() {
                return None;
            }
            if let Some((c, e)) = heap[cur].pop() {
                from_v[cur] = uf.root(e.from);
                from_cost[cur] = c;
            }
            if from_v[cur] == cur {
                continue;
            }
            ans += from_cost[cur];

            if used[from_v[cur]] == 1 {
                let mut p = cur;
                if !heap[p].is_empty() {
                    heap[p].add(-from_cost[p])
                };
                if p != cur {
                    uf.unite(p, cur);
                    let buff = heap[p].node.take();
                    SkewHeap::merge(&mut heap[cur].node, buff);
                }
                p = uf.root(from_v[p]);
                while p != cur {
                    if !heap[p].is_empty() {
                        heap[p].add(-from_cost[p])
                    };
                    if p != cur {
                        uf.unite(p, cur);
                        let buff = heap[p].node.take();
                        SkewHeap::merge(&mut heap[cur].node, buff);
                    }
                    p = uf.root(from_v[p]);
                }
            } else {
                cur = from_v[cur];
            }
        }
        for v in processing {
            used[v] = 2;
        }
    }
    Some((ans, from_v))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_directed_mst() {
        let input = vec![(0, 1, 10), (0, 2, 10), (0, 3, 3), (3, 2, 4)];
        let mut e = vec![vec![]; 4];

        for (from, to, cost) in input {
            e[from].push((to, cost));
        }
        let ans = directed_mst(&e, 0).unwrap();
        assert_eq!(ans.0, 17);
        assert_eq!(ans.1, &[0, 0, 3, 0]);
    }
    #[test]
    fn test_directed_mst2() {
        let input = vec![
            (3, 1, 10),
            (1, 2, 1),
            (2, 0, 1),
            (0, 1, 1),
            (2, 6, 10),
            (6, 4, 1),
            (4, 5, 1),
            (5, 6, 1),
        ];

        let mut e = vec![vec![]; 7];

        for (from, to, cost) in input {
            e[from].push((to, cost));
        }
        let ans = directed_mst(&e, 3).unwrap();
        assert_eq!(ans.0, 24);
        assert_eq!(ans.1, &[2, 3, 1, 3, 6, 4, 2]);
    }
}
