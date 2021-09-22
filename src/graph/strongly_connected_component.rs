#[derive(Debug)]
pub enum Vertex {
    In(usize),
    Out(usize),
}

pub fn decompose(e: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut seen = vec![false; e.len()];

    let mut stack = vec![];
    let mut nodes = Vec::with_capacity(e.len());
    for i in 0..e.len() {
        if seen[i] {
            continue;
        }
        stack.push(Vertex::Out(i));
        stack.push(Vertex::In(i));
        seen[i] = true;
        while let Some(vertex) = stack.pop() {
            if let Vertex::In(v) = vertex {
                for &to in e[v].iter() {
                    if seen[to] {
                        continue;
                    }
                    stack.push(Vertex::Out(to));
                    stack.push(Vertex::In(to));
                    seen[to] = true;
                }
            } else if let Vertex::Out(v) = vertex {
                nodes.push(v);
            }
        }
    }
    let mut reverse_edge = vec![vec![]; e.len()];
    for i in 0..e.len() {
        for j in 0..e[i].len() {
            reverse_edge[e[i][j]].push(i);
        }
    }

    let mut components = vec![];
    let mut back_stack = vec![];
    let mut back_seen = vec![false; e.len()];
    while let Some(v) = nodes.pop() {
        if back_seen[v] {
            continue;
        }
        let mut scc = vec![];
        back_stack.push(v);
        back_seen[v] = true;

        while let Some(v) = back_stack.pop() {
            for &to in reverse_edge[v].iter() {
                if back_seen[to] {
                    continue;
                }
                back_stack.push(to);
                back_seen[to] = true;
            }

            scc.push(v);
        }
        components.push(scc);
    }
    components
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_scc() {
        let n = 6;
        let v = vec![(1, 4), (5, 2), (3, 0), (5, 5), (4, 1), (0, 3), (4, 2)];
        let mut e = vec![vec![]; n];
        for &(v, u) in v.iter() {
            e[v].push(u);
        }
        let a = decompose(&e);
        assert_eq!(a, vec![vec![5], vec![1, 4], vec![2], vec![0, 3]]);
    }
    #[test]
    fn test_scc2() {
        let n = 7;
        let v = vec![
            (0, 2),
            (1, 2),
            (2, 3),
            (3, 2),
            (3, 4),
            (4, 5),
            (5, 6),
            (6, 4),
        ];
        let mut e = vec![vec![]; n];
        for &(v, u) in v.iter() {
            e[v].push(u);
        }
        let a = decompose(&e);
        dbg!(&a);
        assert_eq!(a, vec![vec![0], vec![1], vec![3, 2], vec![5, 6, 4]]);
    }

    #[test]
    fn test_scc3() {
        let n = 11;
        let v = vec![
            (0, 1),
            (1, 2),
            (1, 10),
            (2, 0),
            (2, 3),
            (3, 4),
            (4, 5),
            (4, 10),
            (5, 6),
            (6, 3),
            (7, 8),
            (7, 9),
            (8, 10),
            (9, 7),
            (9, 7),
            (9, 7),
            (9, 7),
            (10, 7),
        ];
        let mut e = vec![vec![]; n];
        for &(v, u) in v.iter() {
            e[v].push(u);
        }
        let a = decompose(&e);
        dbg!(&a);
        //assert_eq!(a, vec![vec![0], vec![1], vec![3, 2], vec![5, 6, 4]]);
    }
    #[test]
    fn test_scc4() {
        let n = 5;
        let v = vec![(0, 1), (0, 2), (2, 3), (3, 4)];
        let mut e = vec![vec![]; n];
        for &(v, u) in v.iter() {
            e[v].push(u);
        }
        let a = decompose(&e);
        dbg!(&a);
        //assert_eq!(a, vec![vec![0], vec![1], vec![3, 2], vec![5, 6, 4]]);
    }
    #[test]
    fn test_scc5() {
        let n = 5;
        let v = vec![(0, 1), (1, 2), (2, 3), (3, 4), (4, 0)];
        let mut e = vec![vec![]; n];
        for &(v, u) in v.iter() {
            e[v].push(u);
        }
        let a = decompose(&e);
        dbg!(&a);
        //assert_eq!(a, vec![vec![0], vec![1], vec![3, 2], vec![5, 6, 4]]);
    }
    #[test]
    fn test_scc6() {
        let n = 6;
        let v = vec![(0, 1), (1, 2), (2, 0), (0, 3), (3, 4), (4, 0)];
        let mut e = vec![vec![]; n];
        for &(v, u) in v.iter() {
            e[v].push(u);
        }
        let a = decompose(&e);
        dbg!(&a);
        //assert_eq!(a, vec![vec![0], vec![1], vec![3, 2], vec![5, 6, 4]]);
    }
}
