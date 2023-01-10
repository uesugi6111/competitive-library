use core::hash::Hash;
use std::collections::HashMap;

#[derive(Debug)]
struct Node<T>
where
    T: Eq + Hash + Copy,
{
    to: HashMap<T, usize>,
    keyword: Option<Vec<T>>,
    failure: usize,
}
impl<T> Node<T>
where
    T: Eq + Hash + Copy,
{
    pub fn new() -> Self {
        Self {
            to: HashMap::new(),
            failure: 0,
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
}
impl<T> AhoCorasick<T>
where
    T: Eq + Hash + Copy,
{
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::new()],
            prepared: false,
        }
    }
    pub fn add(&mut self, keyword: &[T]) {
        assert!(!self.prepared);

        let mut index = 0;

        for value in keyword {
            let buff = index;
            index = match self.nodes[buff].to.get(value) {
                Some(&index) => index,
                None => {
                    self.nodes.push(Node::new());
                    let index = self.nodes.len() - 1;

                    self.nodes[buff].to.insert(*value, index);
                    index
                }
            };
        }

        self.nodes[index].keyword = Some(keyword.to_vec());
    }

    fn make_failure_link(&mut self) {
        let mut queue = std::collections::VecDeque::new();

        queue.push_back(0);

        while let Some(i) = queue.pop_front() {
            for (value, &index) in self.nodes[i].to.clone().iter() {
                if i != 0 {
                    self.nodes[index].failure = self.inner_next(self.nodes[i].failure, value);
                }
                queue.push_back(index);
            }
        }
    }
    pub fn create_matcher<'a, 'b>(&'a mut self, target: &'b [T]) -> Matcher<'a, 'b, T> {
        if !self.prepared {
            self.make_failure_link();
            self.prepared = true;
        }
        Matcher::new(self, target)
    }

    pub(crate) fn get_to(&self, index: usize) -> &std::collections::HashMap<T, usize> {
        &self.nodes[index].to
    }
    pub(crate) fn get_failure(&self, index: usize) -> usize {
        self.nodes[index].failure
    }
    pub(crate) fn get_keyword(&self, index: usize) -> Option<&Vec<T>> {
        self.nodes[index].keyword.as_ref()
    }

    pub(crate) fn inner_next(&self, aho_index: usize, value: &T) -> usize {
        let mut i = aho_index;
        loop {
            if let Some(&next_index) = self.get_to(i).get(value) {
                return next_index;
            }
            i = self.get_failure(i);
            if i == 0 {
                break;
            }
        }
        0
    }
}

impl<T> Iterator for AhoCorasick<T>
where
    T: Eq + Hash + Copy,
{
    type Item = (Vec<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        None
    }
}

pub struct Matcher<'a, 'b, T>
where
    T: Eq + Hash + Copy,
{
    aho: &'a AhoCorasick<T>,
    target: &'b [T],
    target_index: usize,
    aho_index: usize,
}
impl<'a, 'b, T> Matcher<'a, 'b, T>
where
    T: Eq + Hash + Copy,
{
    pub fn new(aho: &'a AhoCorasick<T>, target: &'b [T]) -> Self {
        Self {
            aho,
            target,
            target_index: 0,
            aho_index: 0,
        }
    }
}

impl<'a, 'b, T> Iterator for Matcher<'a, 'b, T>
where
    T: Eq + Hash + Copy,
{
    type Item = (Vec<T>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        while self.target_index < self.target.len() {
            self.aho_index = match self
                .aho
                .get_to(self.aho_index)
                .get(&self.target[self.target_index])
            {
                Some(&index) => {
                    self.target_index += 1;
                    index
                }
                None => self.aho.get_failure(self.aho_index),
            };

            if let Some(keyword) = self.aho.get_keyword(self.aho_index) {
                return Some((keyword.clone(), self.target_index - keyword.len()));
            }
        }
        while self.aho_index != 0 {
            self.aho_index = self.aho.get_failure(self.aho_index);
            if let Some(keyword) = self.aho.get_keyword(self.aho_index) {
                return Some((keyword.clone(), self.target_index - keyword.len()));
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t() {
        let mut aho = AhoCorasick::new();

        aho.add(&"aho".to_string().chars().collect::<Vec<char>>());
        aho.add(&"corasick".to_string().chars().collect::<Vec<char>>());
        aho.add(&"aho-corasick".to_string().chars().collect::<Vec<char>>());

        let s = "aho-corasick".to_string().chars().collect::<Vec<char>>();
        let mut m = aho.create_matcher(&s);

        assert_eq!(m.next().unwrap(), (vec!['a', 'h', 'o'], 0));

        assert_eq!(
            m.next().unwrap(),
            (
                vec!['a', 'h', 'o', '-', 'c', 'o', 'r', 'a', 's', 'i', 'c', 'k'],
                0
            )
        );
        assert_eq!(
            m.next().unwrap(),
            (vec!['c', 'o', 'r', 'a', 's', 'i', 'c', 'k'], 4)
        );
        assert_eq!(m.next(), None);
        drop(m);
        dbg!(aho);
    }
}
