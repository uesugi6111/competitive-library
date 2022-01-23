//! SPFA

use std::collections::VecDeque;

pub fn spfa(edge: &[Vec<(usize, i64)>], start: usize) -> Option<Vec<i64>> {
    let mut pending = vec![false; edge.len()];
    let mut times = vec![0; edge.len()];
    let mut costs = vec![std::i64::MAX; edge.len()];
    let mut q = VecDeque::new();
    q.push_back(start);
    times[start] = 1;
    costs[start] = 0;
    pending[start] = true;

    while let Some(p) = q.pop_front() {
        pending[p] = false;

        for &(to, c) in &edge[p] {
            let cost = costs[p] + c;
            if costs[to] < cost {
                continue;
            }
            costs[to] = cost;
            if !pending[to] {
                times[to] += 1;
                if times[to] >= edge.len() {
                    return None;
                }
                pending[to] = true;
                q.push_back(to);
            }
        }
    }

    Some(costs)
}

#[cfg(test)]
mod tests {
    use super::spfa;

    #[test]
    fn test_spfa() {
        let graph = vec![
            vec![(2, 10), (1, 1)],
            vec![(3, 2)],
            vec![(1, 1), (3, 3), (4, 1)],
            vec![(0, 7), (4, 2)],
            vec![],
        ];
        let ans = spfa(&graph, 0);

        assert_eq!(ans.unwrap(), vec![0, 1, 10, 3, 5]);
    }
}
