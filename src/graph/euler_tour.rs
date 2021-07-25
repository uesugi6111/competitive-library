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
use std::collections::VecDeque;

pub fn euler_tour(e: &[Vec<usize>], root: usize) -> Vec<usize> {
    let mut stack = VecDeque::new();
    stack.push_back(Vertex::Out(root));
    stack.push_back(Vertex::In(root));
    let mut v = vec![];
    while let Some(vertex) = stack.pop_back() {
        if let Vertex::In(v) = vertex {
            for &to in e[v].iter() {
                stack.push_back(Vertex::Out(to));
                stack.push_back(Vertex::In(to));
            }
        }
        v.push(vertex.get_value());
    }

    v
}

#[cfg(test)]
mod tests {
    use crate::graph::euler_tour::euler_tour;

    #[test]
    fn test_eiler_tour() {
        let e = vec![vec![5, 1], vec![4, 2], vec![3], vec![], vec![], vec![]];
        let ans = euler_tour(&e, 0);
        dbg!(&ans);
    }
}
