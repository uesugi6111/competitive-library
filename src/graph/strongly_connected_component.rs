use std::collections::VecDeque;

#[derive(Debug)]
pub enum Vertex {
    In(usize),
    Out(usize),
}

pub fn decompose(e: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut seen = vec![false; e.len()];

    let mut stack = VecDeque::new();
    let mut nodes = VecDeque::new();
    for i in 0..e.len() {
        if seen[i] {
            continue;
        }
        stack.push_back(Vertex::Out(i));
        stack.push_back(Vertex::In(i));
        while let Some(vertex) = stack.pop_back() {
            if let Vertex::In(v) = vertex {
                for &to in e[v].iter().filter(|&&to| !seen[to]) {
                    stack.push_back(Vertex::Out(to));
                    stack.push_back(Vertex::In(to));
                }
                seen[v] = true;
            } else if let Vertex::Out(v) = vertex {
                nodes.push_back(v);
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
    let mut back_stack = VecDeque::new();
    let mut back_seen = vec![false; e.len()];
    while let Some(v) = nodes.pop_back() {
        if back_seen[v] {
            continue;
        }
        let mut scc = vec![];
        back_stack.push_back(Vertex::Out(v));
        back_stack.push_back(Vertex::In(v));
        back_seen[v] = true;

        while let Some(vertex) = back_stack.pop_back() {
            if let Vertex::In(v) = vertex {
                for &to in reverse_edge[v].iter().filter(|&&to| !back_seen[to]) {
                    back_stack.push_back(Vertex::Out(to));
                    back_stack.push_back(Vertex::In(to));
                }
                back_seen[v] = true;
            } else if let Vertex::Out(v) = vertex {
                scc.push(v);
            }
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
}
