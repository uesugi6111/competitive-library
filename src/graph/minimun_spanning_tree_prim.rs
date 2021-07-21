use std::collections::BinaryHeap;

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
}
