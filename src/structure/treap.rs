#[derive(Debug)]
pub struct Node<K, V>
where
    K: Ord,
    V: Clone + Copy,
{
    priority: u32,
    children: [Option<Box<Node<K, V>>>; 2],

    key: K,
    value: V,
}
impl<K, V> Node<K, V>
where
    K: Ord,
    V: Clone + Copy,
{
    pub fn new(key: K, value: V, priority: u32) -> Self {
        Self {
            priority,
            children: [None, None],
            key,
            value,
        }
    }
    pub fn rotate(&mut self, child: usize) -> bool {
        let a = std::mem::replace(&mut self.children[child], None);
        if let Some(mut node) = a {
            std::mem::swap(self, &mut node);
            std::mem::swap(&mut self.children[child ^ 1], &mut node.children[child]);
            self.children[child ^ 1] = Some(node);
            true
        } else {
            false
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
    xorshift: XorShift<u32>,
}
impl<K, V> Treap<K, V>
where
    K: Ord,
    V: Clone + Copy,
{
    pub fn new() -> Self {
        Self {
            nodes: None,
            xorshift: XorShift::<u32>::new(),
        }
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        let new_node = Box::new(Node::new(key, value, self.xorshift.next().unwrap()));
        Treap::insert_inner(&mut self.nodes, new_node)
    }

    fn insert_inner(node: &mut Option<Box<Node<K, V>>>, new_node: Box<Node<K, V>>) -> Option<V> {
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
            *node = Some(new_node);
            return None;
        }
    }
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut node = &self.nodes;

        while let Some(x) = node {
            node = match key.cmp(&x.key) {
                std::cmp::Ordering::Equal => return Some(&x.value),
                std::cmp::Ordering::Less => &x.children[0],
                std::cmp::Ordering::Greater => &x.children[1],
            }
        }
        None
    }

    pub fn erase(&mut self, key: &K) -> Option<V> {
        let mut node = &mut self.nodes;

        while node.is_some() {
            node = match key.cmp(&node.as_ref().unwrap().key) {
                std::cmp::Ordering::Equal => {
                    let mut y = if node.as_mut().unwrap().rotate(0) {
                        &mut node.as_mut().unwrap().children[1]
                    } else if node.as_mut().unwrap().rotate(1) {
                        &mut node.as_mut().unwrap().children[0]
                    } else {
                        return Some(std::mem::replace(node, None).unwrap().value);
                    };

                    loop {
                        y = if y.as_mut().unwrap().rotate(0) {
                            &mut y.as_mut().unwrap().children[1]
                        } else if y.as_mut().unwrap().rotate(1) {
                            &mut y.as_mut().unwrap().children[0]
                        } else {
                            return Some(std::mem::replace(y, None).unwrap().value);
                        };
                    }
                }
                std::cmp::Ordering::Less => &mut node.as_mut().unwrap().children[0],
                std::cmp::Ordering::Greater => &mut node.as_mut().unwrap().children[1],
            }
        }

        None
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

        assert_eq!(a.get(&0), Some(&10000));
        assert_eq!(a.get(&5), Some(&10005));
        assert_eq!(a.get(&10), Some(&10010));
        assert_eq!(a.get(&100), Some(&10100));
        assert_eq!(a.get(&999), Some(&10999));
        assert_eq!(a.insert(999, 2), Some(10999));
        assert_eq!(a.get(&999), Some(&2));
    }

    #[test]
    fn heiko() {
        let mut treap = Treap::new();

        let x = XorShift::<u64>::new();
        for i in x.take(1 << 17) {
            treap.insert(i, ());
        }

        assert!(dbg!(f(&treap.nodes, 0)) < 100);
    }

    fn f(node: &Option<Box<Node<u64, ()>>>, count: u64) -> u64 {
        if let Some(x) = node {
            f(&x.children[0], count + 1).max(f(&x.children[1], count + 1))
        } else {
            count
        }
    }

    #[test]
    fn erase() {
        let mut treap = Treap::new();
        for i in 0..10000 {
            treap.insert(i, i);
        }

        for i in 0..10000 {
            if i % 2 == 0 {
                assert_eq!(treap.erase(&i), Some(i));
            }
        }
        for i in 0..10000 {
            if i % 2 != 0 {
                assert_eq!(treap.get(&i), Some(&i));
            }
        }
    }
}
