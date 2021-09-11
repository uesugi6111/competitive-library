use std::cmp::Ordering;

use crate::structure::disjoint_set_union_undo::DisjointSetUnionRollback
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
    let mut uf = DisjointSetUnionRollback::new(e.len());

    let mut from = vec![0; e.len()];
    let mut costs = vec![0; e.len()];
    let mut used = vec![0; e.len()];
    let mut heap = vec![SkewHeap::new(); e.len()];
    let mut cycles = vec![];
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
        let mut current = start;
        let mut processing = vec![];
        while used[current] != 2 {
            used[current] = 1;
            processing.push(current);

            if heap[current].is_empty() {
                return None;
            }
            if let Some((c, e)) = heap[current].pop() {
                from[current] = (uf.root(e.from),e);
                costs[current] = c;
            }
            if from[current].0 == current {
                continue;
            }
            ans += costs[current];

            if used[from[current].0] == 1 {
                let mut p = current;
                let time = dsu.get_history_length();

                while {
                    if !heap[p].is_empty() {
                        heap[p].add(-costs[p])
                    }

                    uf.unite(p, current);
                    let buff = heap[p].node.take();
                    SkewHeap::merge(&mut heap[current].node, buff);

                    p = uf.root(from[p].0);
                    p != current
                } {}
                cycles.push((p,time));
            } else {
                current = from[current].0;
            }
        }
        for v in processing {
            used[v] = 2;
        }
    }
    for it in cycles.iter().ewv(){
        let  vrepr = dsu.root(from[it.0].1.to);
        dsu.rollback(it.1);
        let  vinc = dsu.find(from[edge[vrepr]].to);
        edge[vinc] = exchange(edge[vrepr], it->first);
    }



    Some((ans, from))
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
