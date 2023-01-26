#[derive(Debug)]
pub struct Node {
    priority: u64,
    children: [Option<Box<Node>>; 2],

    key: i64,
    value: i64,
}
impl Node {
    pub fn new(key: i64, value: i64, priority: u64) -> Self {
        Self {
            priority,
            children: [None, None],
            key,
            value,
        }
    }
    pub fn rotate(&mut self, child: usize) {
        let a = std::mem::replace(&mut self.children[child], None);
        if let Some(mut node) = a {
            std::mem::swap(self, &mut node);
            std::mem::swap(&mut self.children[child ^ 1], &mut node.children[child]);
            self.children[child ^ 1] = Some(node);
        }
    }
}

use crate::other::xorshift::XorShift;
#[derive(Debug)]
pub struct Treap {
    nodes: Option<Box<Node>>,
    xorshift: XorShift,
}
impl Treap {
    pub fn new() -> Self {
        Self {
            nodes: None,
            xorshift: XorShift::new(),
        }
    }

    pub fn insert(&mut self, key: i64, value: i64) {
        let new_node = Node::new(key, value, self.xorshift.next().unwrap());

        Treap::insert_inner(&mut self.nodes, new_node);
    }

    fn insert_inner(node: &mut Option<Box<Node>>, new_node: Node) {
        if node.is_none() {
            *node = Some(Box::new(new_node));
            return;
        }

        if let Some(x) = node {
            let index = if new_node.key < x.key { 0 } else { 1 };

            Treap::insert_inner(&mut x.children[index], new_node);

            if x.priority < x.children[index].as_ref().unwrap().priority {
                x.rotate(index);
            }
        }
    }
    pub fn get(&self, key: i64) -> Option<i64> {
        let mut node = &self.nodes;

        while let Some(x) = node {
            if key == x.key {
                return Some(x.value);
            }
            node = &x.children[if key < x.key { 0 } else { 1 }];
        }
        None
    }

    pub fn erase(&self, _key: i64) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a() {
        let mut a = Treap::new();
        a.insert(1, 2);
        a.insert(2, 2);
        a.insert(3, 2);
        a.insert(4, 2);
        a.insert(5, 2);
        dbg!(a);
    }

    #[test]
    fn c() {
        let mut a = Treap::new();

        for i in 0..1000 {
            a.insert(i, i + 10000);
        }

        assert_eq!(a.get(0), Some(10000));
        assert_eq!(a.get(5), Some(10005));
        assert_eq!(a.get(10), Some(10010));
        assert_eq!(a.get(100), Some(10100));
        assert_eq!(a.get(999), Some(10999));
    }
}
