//! ダイクストラ
use std::{cmp::Ordering, collections::BinaryHeap};

#[derive(Debug, Clone, Eq)]
pub struct Node {
    position: usize,
    cost: i64,
    from: Option<usize>,
}
impl Node {
    #[inline]
    pub fn new(position: usize, cost: i64, from: Option<usize>) -> Self {
        Node {
            position,
            cost,
            from,
        }
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

pub fn dijkstra(
    edge: &[Vec<(usize, i64)>],
    start: usize,
    end: usize,
    vertex: usize,
) -> Option<(i64, Vec<usize>)> {
    let mut costs = vec![None; edge.len()];
    let mut nodes = BinaryHeap::new();
    let mut previous = vec![None; vertex];
    nodes.push(Node::new(start, 0, None));

    while let Some(Node {
        position,
        cost,
        from,
    }) = nodes.pop()
    {
        if costs[position].is_some() {
            continue;
        }

        previous[position] = from;
        costs[position] = Some(cost);
        if position == end {
            return Some((cost, restore_path(end, &previous)));
        }

        edge[position]
            .iter()
            .filter(|(to, c)| costs[*to].filter(|&d| d <= cost + c).is_none())
            .for_each(|&(to, c)| {
                nodes.push(Node::new(to, cost + c, Some(position)));
            });
    }
    None
}

fn restore_path(end: usize, previous: &[Option<usize>]) -> Vec<usize> {
    let mut buff = end;
    let mut v = vec![buff];

    while let Some(i) = previous[buff] {
        buff = i;
        v.push(buff);
    }
    v.reverse();
    v
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_dijkstra() {
        let graph = vec![
            vec![(2, 10), (1, 1)],
            vec![(3, 2)],
            vec![(1, 1), (3, 3), (4, 1)],
            vec![(0, 7), (4, 2)],
            vec![],
        ];
        let l = graph.len();
        for (start, end, ans) in &[
            (0, 1, Some((1, vec![0, 1]))),
            (0, 3, Some((3, vec![0, 1, 3]))),
            (3, 0, Some((7, vec![3, 0]))),
            (0, 4, Some((5, vec![0, 1, 3, 4]))),
            (4, 0, None),
        ] {
            match dijkstra(&graph, *start, *end, l) {
                Some((a, b)) => {
                    assert_eq!(a, ans.as_ref().unwrap().0);
                    assert_eq!(b, ans.as_ref().unwrap().1);
                }
                None => assert!(ans.is_none()),
            }
        }
    }
}
