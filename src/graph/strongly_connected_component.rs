use std::collections::VecDeque;

#[derive(Debug)]
pub enum Vertex {
    In(usize),
    Out(usize),
}
impl Vertex {
    pub fn get_value(&self) -> usize {
        match self {
            Vertex::In(value) => *value,
            Vertex::Out(value) => *value,
        }
    }
}

pub fn decompose(e: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let mut seen = vec![false; e.len()];

    let mut stack = VecDeque::new();
    let mut nodes = VecDeque::new();
    for i in 0..e.len() {
        stack.push_back(Vertex::In(i));

        while let Some(vertex) = stack.pop_back() {
            if let Vertex::In(v) = vertex {
                for &to in e[v].iter().filter(|&&to| !seen[to]) {
                    stack.push_back(Vertex::Out(v));
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
    let mut bask_stack = VecDeque::new();
    let mut back_seen = vec![false; e.len()];
    while let Some(v) = nodes.pop_back() {
        if back_seen[v] {
            continue;
        }
        let mut scc = vec![];

        while let Some(vertex) = bask_stack.pop_back() {
            if let Vertex::In(v) = vertex {
                for &to in reverse_edge[v].iter().filter(|&&to| !back_seen[to]) {
                    bask_stack.push_back(Vertex::Out(v));
                    bask_stack.push_back(Vertex::In(to));
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
