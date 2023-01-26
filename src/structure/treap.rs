#[derive(Debug)]
pub struct Node<K, V>
where
    K: Ord,
    V: Clone + Copy,
{
    priority: u64,
    children: [Option<Box<Node<K, V>>>; 2],

    key: K,
    value: V,
}
impl<K, V> Node<K, V>
where
    K: Ord,
    V: Clone + Copy,
{
    pub fn new(key: K, value: V, priority: u64) -> Self {
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
pub struct Treap<K, V>
where
    K: Ord,
    V: Clone + Copy,
{
    nodes: Option<Box<Node<K, V>>>,
    xorshift: XorShift,
}
impl<K, V> Treap<K, V>
where
    K: Ord,
    V: Clone + Copy,
{
    pub fn new() -> Self {
        Self {
            nodes: None,
            xorshift: XorShift::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let new_node = Node::new(key, value, self.xorshift.next().unwrap());
        Treap::insert_inner(&mut self.nodes, new_node)
    }

    fn insert_inner(node: &mut Option<Box<Node<K, V>>>, new_node: Node<K, V>) -> Option<V> {
        if let Some(x) = node {
            let index = match new_node.key.cmp(&x.key) {
                std::cmp::Ordering::Equal => {
                    let ret = std::mem::replace(&mut x.value, new_node.value);
                    return Some(ret);
                }
                std::cmp::Ordering::Less => 0,
                std::cmp::Ordering::Greater => 1,
            };

            let value = Treap::insert_inner(&mut x.children[index], new_node);

            if x.priority < x.children[index].as_ref().unwrap().priority {
                x.rotate(index);
            }
            return value;
        } else {
            *node = Some(Box::new(new_node));
            return None;
        }
    }
    pub fn get(&self, key: K) -> Option<V> {
        let mut node = &self.nodes;

        while let Some(x) = node {
            node = match key.cmp(&x.key) {
                std::cmp::Ordering::Equal => return Some(x.value),
                std::cmp::Ordering::Less => &x.children[0],
                std::cmp::Ordering::Greater => &x.children[1],
            }
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
        assert_eq!(a.insert(999, 2), Some(10999));
        assert_eq!(a.get(999), Some(2));
    }
}
