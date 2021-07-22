/// O(|N|^2)
pub fn prim(g: &[Vec<i64>]) -> i64 {
    let n = g.len();
    let mut min_cost = vec![std::i64::MAX; n];
    let mut used = vec![false; n];
    let mut sum = 0;
    min_cost[0] = 0;
    loop {
        let mut v = None;
        for u in 0..n {
            if used[u] || v.filter(|&x| min_cost[u] > min_cost[x]).is_some() {
                continue;
            }
            v = Some(u);
        }
        if v.is_none() {
            break;
        }
        let v = v.unwrap();
        used[v] = true;
        sum += min_cost[v];
        (0..n).filter(|&u| g[v][u] != -1).for_each(|u| {
            min_cost[u] = min_cost[u].min(g[v][u]);
        });
    }
    sum
}

use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Debug, Clone, Eq)]
struct Vertex {
    v: usize,
    cost: i64,
}
impl Vertex {
    #[inline]
    pub fn new(v: usize, cost: i64) -> Self {
        Vertex { v, cost }
    }
}
impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.cost.eq(&other.cost)
    }
}
impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Vertex) -> Option<Ordering> {
        Some(other.cost.cmp(&(self.cost)))
    }
}
impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&(other.cost))
    }
}

/// O(|E|log|V|)
pub fn prim_heap(g: &[Vec<i64>]) -> Option<i64> {
    let mut min_cost = vec![None; g.len()];
    let mut heap = BinaryHeap::new();
    heap.push(Vertex::new(0, 0));

    let mut v_count = 0;

    let mut total_cost = 0;
    while let Some(Vertex { v, cost }) = heap.pop() {
        if min_cost[v].is_some() {
            continue;
        }
        total_cost += cost;
        min_cost[v] = Some(total_cost);
        v_count += 1;
        if v_count == g.len() {
            return Some(total_cost);
        }

        (0..g.len()).filter(|&i| g[v][i] != -1).for_each(|i| {
            heap.push(Vertex::new(i, g[v][i]));
        });
    }
    None
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_prim() {
        let g = vec![
            vec![-1, 2, 3, 1, -1],
            vec![2, -1, -1, 4, -1],
            vec![3, -1, -1, 1, 1],
            vec![1, 4, 1, -1, 3],
            vec![-1, -1, 1, 3, -1],
        ];

        assert_eq!(prim(&g), 5);
    }
    #[test]
    fn test_prim_heap() {
        let g = vec![
            vec![-1, 2, 3, 1, -1],
            vec![2, -1, -1, 4, -1],
            vec![3, -1, -1, 1, 1],
            vec![1, 4, 1, -1, 3],
            vec![-1, -1, 1, 3, -1],
        ];

        assert_eq!(prim_heap(&g).unwrap(), 5);
    }
}
