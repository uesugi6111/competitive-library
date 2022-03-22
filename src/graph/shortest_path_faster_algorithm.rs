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
            if costs[to] <= cost {
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
    #[test]
    fn test_2() {
        let mx = 1000000000i64;

        let k = 500000;
        let n = 3 * k + 1;
        let mut h = vec![0; n];

        for i in 0..k {
            h[1 + i] = -(1 + i as i64);
            h[1 + k + i] = mx - 2 * i as i64;
            h[1 + 2 * k + 1] = -mx;
        }

        let mut g = vec![(0, 1)];

        for i in 1..k {
            g.push((i, i + 1));

            g.push((i, i + k));

            g.push((i + k, 2 * k + 1));
        }
        println!("{}", g.len());

        //     let mut g = vec![vec![]];
        //     g[0].push(1);
        //     g[1].push(0);

        //     for i in 1..k {
        //         g[i].push(i + 1);
        //         g[i + 1].push(i);

        //         g[i].push(i + k);
        //         g[i + k].push(i);

        //         g[i + k].push(2 * k + 1);
        //         g[2 * k + 1].push(i + k);
        //     }

        let mut e = vec![vec![]; n];
        for (u, v) in g {
            let (c1, c2) = if h[u] < h[v] {
                (-(h[v] - h[u]), 2 * (h[v] - h[u]))
            } else {
                (2 * (h[u] - h[v]), -(h[u] - h[v]))
            };
            e[u].push((v, c2));
            e[v].push((u, c1));
        }

        let ans = spfa(&e, 0).unwrap();

        println!("{}", -ans.iter().min().unwrap());
    }
}
