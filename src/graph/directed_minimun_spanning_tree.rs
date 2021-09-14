use std::cmp::Ordering;

use crate::structure::disjoint_set_union::Dsu;
use crate::structure::disjoint_set_union_undo::DisjointSetUnionRollback;
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
    let mut uf_undo = DisjointSetUnionRollback::new(e.len());
    let mut uf = Dsu::new(e.len());
    let mut from_v = vec![(0, None); e.len()];
    let mut from_cost = vec![0; e.len()];
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

            if let Some((c, e)) = heap[current].pop() {
                from_v[current] = (uf_undo.root(e.from), Some(e));
                from_cost[current] = c;
            } else {
                return None;
            }
            if from_v[current].0 == current {
                continue;
            }
            ans += from_cost[current];

            if used[from_v[current].0] != 1 {
                current = from_v[current].0;
                continue;
            }
            let mut p = current;
            let time = uf_undo.get_history_length();
            while {
                if !heap[p].is_empty() {
                    heap[p].add(-from_cost[p]);
                }
                uf_undo.unite(p, current);
                let buff = heap[p].node.take();
                SkewHeap::merge(&mut heap[current].node, buff);

                p = uf_undo.root(from_v[p].0);
                p != current
            } {}
            cycles.push((from_v[p].clone(), time));
        }
        for v in processing {
            used[v] = 2;
        }
    }
    for (f, time) in cycles
        .iter()
        .rev()
        .take(0.max(cycles.len() as i64 - 1) as usize)
    {
        let vrepr = uf_undo.root(f.1.as_ref().unwrap().to);
        uf_undo.rollback(*time);
        let vinc = uf_undo.root(from_v[vrepr].1.as_ref().unwrap().to);

        from_v[vinc] = from_v[vrepr].clone();
        from_v[vrepr] = f.to_owned();
    }
    let mut edges = vec![11111; e.len()];
    for i in 0..e.len() {
        edges[i] = if i == root {
            root
        } else {
            from_v[i].1.as_ref().unwrap().from
        };
    }

    Some((ans, edges))
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
