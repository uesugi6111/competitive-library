//! ダイクストラ

use std::collections::VecDeque;
#[derive(Debug, Clone)]
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

pub fn bfs(edge: &[Vec<(usize, i64)>], start: usize, end: usize) -> Option<i64> {
    assert_ne!(start, end);
    let mut costs = vec![None; edge.len()];
    let mut nodes = VecDeque::new();
    nodes.push_back(Node::new(start, 0));

    while let Some(Node { position, cost }) = nodes.pop_front() {
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
                if c == 0 {
                    nodes.push_front(Node::new(to, cost));
                } else {
                    nodes.push_back(Node::new(to, cost + 1));
                }
            });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dijkstra() {
        let graph = vec![
            vec![(2, 1), (1, 0)],
            vec![(3, 1)],
            vec![(1, 1), (3, 0), (4, 1)],
            vec![(0, 1), (4, 0)],
            vec![],
        ];

        assert_eq!(bfs(&graph, 0, 1), Some(0));
        assert_eq!(bfs(&graph, 0, 3), Some(1));
        assert_eq!(bfs(&graph, 3, 0), Some(1));
        assert_eq!(bfs(&graph, 0, 4), Some(1));
        assert_eq!(bfs(&graph, 4, 0), None);
    }
    #[test]
    #[should_panic]
    fn test_panic() {
        bfs(&[], 0, 0);
    }
}
