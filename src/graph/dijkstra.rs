#[derive(Debug, Clone, PartialEq, Eq, Ord)]
struct Node {
    pos: usize,
    cost: i64,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&(self.cost)))
    }
}

pub fn dijkstra(edge: &[Vec<(usize, i64)>], start: usize, end: usize) -> Option<i64> {
    let mut dist = vec![std::i64::MAX; edge.len()];
    let mut pq = std::collections::BinaryHeap::<Node>::new();

    pq.push(Node {
        pos: start,
        cost: 0,
    });
    dist[start] = 0;

    let mut ret = start == end;

    while let Some(Node { pos, cost }) = pq.pop() {
        if cost > dist[pos] {
            continue;
        }
        if ret {
            ret = false;
            dist[start] = std::i64::MAX;
        } else if end == pos {
            return Some(cost);
        }
        for (t, c) in &edge[pos] {
            let total_cost = cost + *c;
            if dist[*t] <= total_cost {
                continue;
            }
            dist[*t] = total_cost;
            pq.push(Node {
                pos: *t,
                cost: total_cost,
            });
        }
    }
    None
}

#[test]
fn test_dijkstra() {
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
