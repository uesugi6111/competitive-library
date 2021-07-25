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

pub fn euler_tour(e: &[Vec<usize>], root: usize) -> (Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut stack = VecDeque::new();
    stack.push_back(Vertex::In(root));
    let mut tour = vec![];
    let mut first_look = vec![None; e.len()];
    let mut depth = 0;
    let mut depths = vec![0; e.len()];
    while let Some(vertex) = stack.pop_back() {
        if let Vertex::In(v) = vertex {
            for &to in e[v].iter() {
                if first_look[to].is_some() {
                    continue;
                }
                stack.push_back(Vertex::Out(v));
                stack.push_back(Vertex::In(to));
            }
            first_look[v] = Some(tour.len());
            depths[v] = depth;
            depth += 1;
        } else {
            depth -= 1;
        }
        tour.push(vertex.get_value());
    }

    (
        tour,
        first_look.iter().map(|x| x.unwrap()).collect(),
        depths,
    )
}

#[cfg(test)]
mod tests {
    use crate::graph::euler_tour::euler_tour;

    #[test]
    fn test_eiler_tour() {
        let e = vec![vec![5, 1], vec![4, 2], vec![3], vec![], vec![], vec![]];
        let (ans, _, _) = euler_tour(&e, 0);
        assert_eq!(&ans, &[0, 1, 2, 3, 2, 1, 4, 1, 0, 5, 0]);
    }
}
