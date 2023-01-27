pub trait Edge {
    type T: Clone;
    fn get_edge(a: &Self::T) -> usize;
    fn get_cost(a: &Self::T) -> i64;
}

pub struct UnWeightedEdge {}
impl Edge for UnWeightedEdge {
    type T = usize;
    #[inline]
    fn get_edge(a: &Self::T) -> usize {
        *a
    }
    #[inline]
    fn get_cost(_: &Self::T) -> i64 {
        1
    }
}
pub struct WeightedEdge {}
impl Edge for WeightedEdge {
    type T = (usize, i64);

    #[inline]
    fn get_edge(a: &Self::T) -> usize {
        a.0
    }
    #[inline]
    fn get_cost(a: &Self::T) -> i64 {
        a.1
    }
}

pub fn tree_diamiter<E: Edge>(e: &[Vec<E::T>]) -> (i64, Vec<usize>) {
    let (_, path_1) = bfs::<E>(e, 0);
    bfs::<E>(e, path_1[path_1.len() - 1])
}

#[inline]
fn bfs<E: Edge>(e: &[Vec<E::T>], start: usize) -> (i64, Vec<usize>) {
    let mut que = std::collections::VecDeque::new();
    let mut max_cost_index = (start, 0);
    let mut previous = vec![None; e.len()];
    previous[start] = Some(start);

    que.push_back((start, 0));

    while let Some((v, cost)) = que.pop_front() {
        for edge in e[v].iter() {
            let to = E::get_edge(edge);
            if previous[to].is_some() {
                continue;
            }
            previous[to] = Some(v);
            que.push_back((E::get_edge(edge), cost + E::get_cost(edge)));
        }

        if max_cost_index.1 < cost {
            max_cost_index = (v, cost);
        }
    }
    (max_cost_index.1, restore_path(max_cost_index.0, &previous))
}

#[inline]
fn restore_path(end: usize, previous: &[Option<usize>]) -> Vec<usize> {
    let mut buff = end;
    let mut v = vec![buff];

    while let Some(i) = previous[buff] {
        if buff == i {
            break;
        }
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
    fn test_tree_diamiter() {
        let n = 8;

        let input = vec![
            (0, 1, 5),
            (1, 2, 3),
            (2, 3, 1),
            (1, 4, 2),
            (4, 7, 4),
            (1, 5, 7),
            (2, 6, 5),
        ];
        let mut e = vec![vec![]; n];
        for (a, b, c) in input {
            e[a].push((b, c));
            e[b].push((a, c));
        }

        let a = tree_diamiter::<WeightedEdge>(&e);
        assert_eq!(&a.1, &[6, 2, 1, 5]);
        assert_eq!(a.0, 15);
    }
}
