#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Node {
    children: Vec<Option<Node>>,
    count: u32,
}
impl Node {
    fn new(n: u32) -> Self {
        Self {
            children: vec![None; 2],
            count: n,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BinaryTrie {
    nodes: Option<Node>,
}
impl BinaryTrie {
    pub fn new() -> Self {
        Self { nodes: None }
    }
    pub fn insert(&mut self, x: u32) {
        if self.nodes.is_none() {
            self.nodes = Some(Node::new(0));
        }
        let mut node = self.nodes.as_mut().unwrap();
        for i in (0..32).rev() {
            node.count += 1;
            let f = (x >> i & 1) as usize;
            if node.children[f].is_none() {
                node.children[f] = Some(Node::new(0));
            }
            node = node.children[f].as_mut().unwrap();
        }
        node.count += 1;
    }

    pub fn erase(&mut self, x: u32) -> Option<()> {
        let mut node = &self.nodes;

        for i in (0..32).rev() {
            node = &node.as_ref()?.children[(x >> i & 1) as usize];
        }
        node.as_ref()?;

        let mut node = &mut self.nodes;
        for i in (0..32).rev() {
            if node.as_ref()?.count == 1 {
                *node = None;
                return Some(());
            } else {
                node.as_mut()?.count -= 1;
            }
            node = &mut node.as_mut()?.children[(x >> i & 1) as usize];
        }
        if node.as_ref()?.count == 1 {
            *node = None;
        } else {
            node.as_mut()?.count -= 1;
        }

        Some(())
    }

    pub fn xor_min(&self, x: u32) -> Option<u32> {
        let mut ans = 0;
        let mut node = self.nodes.as_ref()?;
        for i in (0..32).rev() {
            let mut f = (x >> i & 1) as usize;
            if node.children[f].is_none() {
                f ^= 1;
            }
            ans ^= (f as u32) << i;
            node = node.children[f].as_ref().unwrap();
        }
        Some(ans ^ x)
    }

    pub fn min(&self) -> Option<u32> {
        let mut ans = 0;
        let mut node = self.nodes.as_ref()?;
        for i in (0..32).rev() {
            let mut f = 0;
            if node.children[f].is_none() {
                f ^= 1;
            }
            ans ^= (f as u32) << i;
            node = node.children[f].as_ref().unwrap();
        }
        Some(ans)
    }
    pub fn max(&self) -> Option<u32> {
        let mut ans = 0;
        let mut node = self.nodes.as_ref()?;
        for i in (0..32).rev() {
            let mut f = 1;
            if node.children[f].is_none() {
                f ^= 1;
            }
            ans ^= (f as u32) << i;
            node = node.children[f].as_ref().unwrap();
        }
        Some(ans)
    }
}

#[cfg(test)]
mod test {

    use super::*;
    #[test]
    fn bt() {
        let mut b = BinaryTrie::new();
        b.insert(6);

        let a = b.clone();
        b.insert(7);
        b.insert(7);
        b.erase(7);
        b.erase(7);
        b.erase(10);
        assert_eq!(a.nodes, b.nodes);
    }
    #[test]
    fn btt() {
        let mut b = BinaryTrie::new();
        let n = 2u32.pow(30);
        b.insert(n + 100);
        for i in 0..100 {
            b.insert(n + i);
        }
        for i in 0..99 {
            b.erase(n + i);
            assert_eq!(b.min().unwrap(), n + i + 1);
        }
    }

    #[test]
    fn library_checker() {
        let mut b = BinaryTrie::new();
        let query = vec![(0, 6), (0, 7), (2, 5), (1, 7), (1, 10), (2, 7)];
        let mut ans = vec![];
        query.iter().for_each(|&(p, x)| match p {
            0 => b.insert(x),
            1 => {
                b.erase(x);
            }
            _ => ans.push(b.xor_min(x).unwrap_or_else(|| panic!("{}", x.to_string()))),
        });

        assert_eq!(vec![2, 1], ans);
    }
}
