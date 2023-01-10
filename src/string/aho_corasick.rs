use core::hash::Hash;
use std::{num::NonZeroUsize, ops::Range};

use std::collections::HashMap;

#[derive(Debug)]
struct Node<T>
where
    T: Eq + Hash + Copy,
{
    to: HashMap<T, NonZeroUsize>,
    keyword: Option<Vec<T>>,
    failure: Option<NonZeroUsize>,
}
impl<T> Node<T>
where
    T: Eq + Hash + Copy,
{
    pub fn new() -> Self {
        Self {
            to: HashMap::new(),
            failure: None,
            keyword: None,
        }
    }
}

#[derive(Default, Debug)]
pub struct AhoCorasick<T>
where
    T: Eq + Hash + Copy,
{
    nodes: Vec<Node<T>>,
    prepared: bool,
    state: usize,
}
impl<T> AhoCorasick<T>
where
    T: Eq + Hash + Copy,
{
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::new()],
            prepared: false,
            state: 0,
        }
    }
    pub fn add(&mut self, keyword: &[T]) {
        assert!(!self.prepared);

        let mut index = None;

        for value in keyword {
            let buff = index.map(usize::from).unwrap_or(0);
            index = match self.nodes[buff].to.get(value) {
                Some(&index) => Some(index),
                None => {
                    self.nodes.push(Node::new());
                    let index = NonZeroUsize::new(self.nodes.len() - 1).unwrap();

                    self.nodes[buff].to.insert(*value, index);
                    Some(index)
                }
            };
        }

        self.nodes[to_usize(index)].keyword = Some(keyword.to_vec());
    }

    pub fn prepare(&mut self) {
        assert!(!self.prepared);
    }
    pub fn next_search(&self, target: &[T]) {
        assert!(self.prepared);
    }
}

fn to_usize(n: Option<NonZeroUsize>) -> usize {
    n.map(usize::from).unwrap_or(0)
}

impl<T> Iterator for AhoCorasick<T>
where
    T: Eq + Hash + Copy,
{
    type Item = (Vec<T>, Range<usize>);
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a() {
        let mut aho = AhoCorasick::new();

        aho.add(&"aho-corasick".to_string().chars().collect::<Vec<char>>());
        dbg!(&aho);
    }
}
