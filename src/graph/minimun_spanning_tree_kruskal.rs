use crate::structure::dsu::Dsu;
pub struct Edge(i64, i64, i64);

pub fn kruskal(n: usize, edges: &[Edge]) -> i64 {
    let mut edges = edges.iter().collect::<Vec<_>>();
    edges.sort_by_key(|e| e.2);

    let mut dsu = Dsu::new(n);

    let mut min_cost = 0;

    for e in edges.iter() {
        if dsu.is_same(e.0 as usize, e.1 as usize) {
            continue;
        }
        dsu.unite(e.0 as usize, e.1 as usize);
        min_cost += e.2;
    }

    min_cost
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_kruskal() {
        let ans = kruskal(
            5,
            &[
                Edge(0, 1, 10),
                Edge(0, 3, 5),
                Edge(1, 2, 1),
                Edge(1, 3, 1000),
                Edge(1, 4, 500),
                Edge(2, 3, 100),
                Edge(2, 4, 10000),
                Edge(3, 4, 5000),
            ],
        );

        assert_eq!(ans, 516);
    }
}
