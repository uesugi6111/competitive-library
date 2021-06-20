//! ダイクストラ
#[derive(Debug, Clone, Eq)]
struct Node {
    pos: usize,
    cost: i64,
}
impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.cost.eq(&other.cost)
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<std::cmp::Ordering> {
        Some(other.cost.cmp(&(self.cost)))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&(other.cost))
    }
}

pub fn dijkstra(
    edge: &[Vec<(usize, i64)>],
    start: usize,
    end: usize,
    vertex: usize,
) -> Option<(i64, Vec<usize>)> {
    let mut dist = vec![std::i64::MAX; edge.len()];
    let mut pq = std::collections::BinaryHeap::new();
    let mut previous = vec![None; vertex];

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
            return Some((cost, restore_path(end, &previous)));
        }
        for (t, c) in &edge[pos] {
            let total_cost = cost + *c;
            if dist[*t] <= total_cost {
                continue;
            }
            previous[*t] = Some(pos);
            dist[*t] = total_cost;
            pq.push(Node {
                pos: *t,
                cost: total_cost,
            });
        }
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
