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
        let mut i = self.nodes.as_mut().unwrap();
        for j in (0..32).rev() {
            i.count += 1;
            let f = (x >> j & 1) as usize;
            if i.children[f].is_none() {
                i.children[f] = Some(Node::new(0));
            }
            i = i.children[f].as_mut().unwrap();
        }
        i.count += 1;
    }

    pub fn erase(&mut self, x: u32) -> Option<()> {
        let mut i = &self.nodes;

        for j in (0..32).rev() {
            let f = (x >> j & 1) as usize;
            if i.as_ref()?.count == 1 {
                return None;
            }
            i = &i.as_ref()?.children[f];
        }

        let mut i = &mut self.nodes;
        for j in (0..32).rev() {
            let f = (x >> j & 1) as usize;
            if i.as_ref()?.count == 1 {
                *i = None;
                return Some(());
            } else {
                i.as_mut()?.count -= 1;
            }

            i = &mut i.as_mut()?.children[f];
        }

        Some(())
    }

    pub fn xor_min(&self, x: u32) -> Option<u32> {
        let mut ans = 0;
        let mut i = self.nodes.as_ref()?;
        for j in (0..32).rev() {
            let mut f = (x >> j & 1) as usize;
            if i.children[f].is_none() {
                f ^= 1;
            }
            ans ^= (f as u32) << j;
            i = i.children[f].as_ref().unwrap();
        }
        Some(ans ^ x)
    }

    pub fn min(&self) -> Option<u32> {
        let mut ans = 0;
        let mut i = self.nodes.as_ref()?;
        for j in (0..32).rev() {
            let mut f = 0;
            if i.children[f].is_none() {
                f ^= 1;
            }
            ans ^= (f as u32) << j;
            i = i.children[f].as_ref().unwrap();
        }
        Some(ans)
    }
    pub fn max(&self) -> Option<u32> {
        let mut ans = 0;
        let mut i = self.nodes.as_ref()?;
        for j in (0..32).rev() {
            let mut f = 1;
            if i.children[f].is_none() {
                f ^= 1;
            }
            ans ^= (f as u32) << j;
            i = i.children[f].as_ref().unwrap();
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
        b.insert(1);

        dbg!(&b);
        let a = b.clone();
        b.insert(1);
        b.erase(1);
        assert_eq!(a.nodes, b.nodes);
    }
    #[test]
    fn btt() {
        let mut b = BinaryTrie::new();
        b.erase(10);
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
            _ => ans.push(b.xor_min(x).unwrap_or_else(|| panic!(x.to_string()))),
        });

        assert_eq!(vec![2, 1], ans);
    }
}
