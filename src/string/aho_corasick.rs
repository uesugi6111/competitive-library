use core::hash::Hash;
use std::{
    collections::{HashMap, VecDeque},
    rc::Rc,
};

#[derive(Debug)]
struct Node<T>
where
    T: Eq + Hash + Copy,
{
    to: HashMap<T, usize>,
    keyword: Option<Rc<Vec<T>>>,
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

        self.nodes[index].keyword = Some(Rc::new(keyword.to_vec()));
    }

    pub fn make_failure_link(&mut self) {
        self.prepared = true;
        let mut queue = VecDeque::new();

        queue.push_back(0);

        while let Some(i) = queue.pop_front() {
            for (value, &index) in self.nodes[i].to.clone().iter() {
                if i != 0 {
                    let mut buff = self.nodes[i].failure;
                    self.nodes[index].failure = loop {
                        if let Some(&next_index) = self.get_to(buff).get(value) {
                            break next_index;
                        } else {
                            if buff == 0 {
                                break 0;
                            }
                            buff = self.nodes[buff].failure
                        };
                    }
                }
                queue.push_back(index);
            }
        }
    }
    pub fn create_matcher<'a, 'b>(&'a self, target: &'b [T]) -> Matcher<'a, 'b, T> {
        assert!(self.prepared);

        Matcher::new(self, target)
    }

    pub(crate) fn get_to(&self, index: usize) -> &HashMap<T, usize> {
        &self.nodes[index].to
    }
    pub(crate) fn get_failure(&self, index: usize) -> usize {
        self.nodes[index].failure
    }
    pub(crate) fn get_keyword(&self, index: usize) -> Option<Rc<Vec<T>>> {
        self.nodes[index].keyword.clone()
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
    failure_index: usize,
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
            failure_index: 0,
        }
    }
}

impl<'a, 'b, T> Iterator for Matcher<'a, 'b, T>
where
    T: Eq + Hash + Copy,
{
    type Item = (Rc<Vec<T>>, usize);
    fn next(&mut self) -> Option<Self::Item> {
        while self.target_index < self.target.len() {
            while self.failure_index != 0 {
                if let Some(keyword) = self.aho.get_keyword(self.failure_index) {
                    self.failure_index = self.aho.get_failure(self.failure_index);
                    return Some((keyword.clone(), self.target_index - keyword.len()));
                }
                self.failure_index = self.aho.get_failure(self.failure_index);
            }

            match self
                .aho
                .get_to(self.aho_index)
                .get(&self.target[self.target_index])
            {
                Some(&index) => {
                    self.target_index += 1;
                    self.failure_index = self.aho.get_failure(index);
                    self.aho_index = index;
                }
                None => {
                    let buff = self.aho.get_failure(self.aho_index);
                    if buff == self.aho_index {
                        self.target_index += 1;
                        continue;
                    }
                    self.aho_index = buff;
                    continue;
                }
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
    use std::ops::Deref;

    use super::*;

    #[test]
    fn t() {
        let mut aho = AhoCorasick::new();

        aho.add(&"aho".to_string().chars().collect::<Vec<char>>());
        aho.add(&"corasick".to_string().chars().collect::<Vec<char>>());
        aho.add(&"aho-corasick".to_string().chars().collect::<Vec<char>>());

        let s = "aho-corasick".to_string().chars().collect::<Vec<char>>();
        aho.make_failure_link();
        let mut m = aho.create_matcher(&s);
        dbg!(&aho);
        assert_eq!(m.next().unwrap().0.deref(), &['a', 'h', 'o']);

        assert_eq!(
            m.next().unwrap().0.deref(),
            &['a', 'h', 'o', '-', 'c', 'o', 'r', 'a', 's', 'i', 'c', 'k']
        );
        assert_eq!(
            m.next().unwrap().0.deref(),
            &['c', 'o', 'r', 'a', 's', 'i', 'c', 'k']
        );
        assert_eq!(m.next(), None);
    }

    #[test]
    fn aaa() {
        let mut aho = AhoCorasick::new();

        aho.add(&"a".to_string().chars().collect::<Vec<char>>());
        aho.make_failure_link();
        let s = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
            .to_string()
            .chars()
            .collect::<Vec<char>>();
        for (i, (keyword, index)) in aho.create_matcher(&s).enumerate() {
            assert_eq!(keyword.deref(), &['a']);
            assert_eq!(index, i);
        }
    }
    #[test]
    fn abc() {
        let mut aho = AhoCorasick::new();

        let s = "abcdefghijklmnopqrstuvwxyz"
            .to_string()
            .chars()
            .collect::<Vec<char>>();

        for i in 0..s.len() {
            aho.add(&s.iter().skip(i).copied().collect::<Vec<char>>());
        }

        aho.make_failure_link();
        for (i, (keyword, index)) in aho.create_matcher(&s).enumerate() {
            assert_eq!(index, i);
            assert_eq!(
                keyword.deref(),
                &s.iter().skip(i).copied().collect::<Vec<char>>()
            )
        }
    }
    #[test]
    fn add() {
        let mut aho = AhoCorasick::new();

        let s = "zabcdefghijklmn".to_string().chars().collect::<Vec<char>>();

        aho.add(&"abcd".to_string().chars().collect::<Vec<char>>());
        aho.add(&"ijk".to_string().chars().collect::<Vec<char>>());
        aho.add(&"ghi".to_string().chars().collect::<Vec<char>>());
        aho.make_failure_link();
        let mut m = aho.create_matcher(&s);

        assert_eq!(m.next().unwrap().0.deref(), &['a', 'b', 'c', 'd']);
        assert_eq!(m.next().unwrap().0.deref(), &['g', 'h', 'i']);
        assert_eq!(m.next().unwrap().0.deref(), &['i', 'j', 'k']);
        assert_eq!(m.next(), None);
    }
    #[test]
    fn xbabcdex() {
        let mut aho = AhoCorasick::new();

        let s = "xbabcdex".to_string().chars().collect::<Vec<char>>();

        aho.add(&"ab".to_string().chars().collect::<Vec<char>>());
        aho.add(&"bc".to_string().chars().collect::<Vec<char>>());
        aho.add(&"bab".to_string().chars().collect::<Vec<char>>());
        aho.add(&"d".to_string().chars().collect::<Vec<char>>());
        aho.add(&"abcde".to_string().chars().collect::<Vec<char>>());
        aho.make_failure_link();
        let mut m = aho.create_matcher(&s);

        assert_eq!(m.next().unwrap().0.deref(), &['b', 'a', 'b']);
        assert_eq!(m.next().unwrap().0.deref(), &['a', 'b']);

        assert_eq!(m.next().unwrap().0.deref(), &['b', 'c']);
        assert_eq!(m.next().unwrap().0.deref(), &['d']);
        assert_eq!(m.next().unwrap().0.deref(), &['a', 'b', 'c', 'd', 'e']);
        assert_eq!(m.next(), None);
    }
}
