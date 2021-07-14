//! ダイクストラ

use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Debug, Clone, Eq)]
pub struct Node {
    position: usize,
    cost: i64,
}
impl Node {
    #[inline]
    pub fn new(position: usize, cost: i64) -> Self {
        Node { position, cost }
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.cost.eq(&other.cost)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(other.cost.cmp(&(self.cost)))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&(other.cost))
    }
}

pub fn dijkstra(edge: &[Vec<(usize, i64)>], start: usize, end: usize) -> Option<i64> {
    assert_ne!(start, end);
    let mut costs = vec![None; edge.len()];
    let mut nodes = BinaryHeap::new();
    nodes.push(Node::new(start, 0));

    while let Some(Node { position, cost }) = nodes.pop() {
        if costs[position].is_some() {
            continue;
        }
        if position == end {
            return Some(cost);
        }
        costs[position] = Some(cost);

        edge[position]
            .iter()
            .filter(|(to, c)| costs[*to].filter(|&d| d <= cost + c).is_none())
            .for_each(|&(to, c)| {
                nodes.push(Node::new(to, cost + c));
            });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test() {
            let graph = vec![
                vec![(2, 10), (1, 1)],
                vec![(3, 2)],
                vec![(1, 1), (3, 3), (4, 1)],
                vec![(0, 7), (4, 2)],
                vec![],
            ];

            assert_eq!(dijkstra(&graph, 0, 1), Some(1));
            assert_eq!(dijkstra(&graph, 0, 3), Some(3));
            assert_eq!(dijkstra(&graph, 3, 0), Some(7));
            assert_eq!(dijkstra(&graph, 0, 4), Some(5));
            assert_eq!(dijkstra(&graph, 4, 0), None);
        }
        #[test]
        #[should_panic]
        fn test_panic() {
            dijkstra(&[], 0, 0);
        }
    }
}
