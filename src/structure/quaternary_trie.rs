//! QuaternaryTrie

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Node {
    children: [Option<usize>; 4],
    count: u64,
}
impl Node {
    #[inline]
    fn new() -> Self {
        Self {
            children: [None; 4],
            count: 0,
        }
    }
    #[inline]
    fn get_child(&self, index: usize) -> &Option<usize> {
        unsafe { self.children.get_unchecked(index) }
    }
    #[inline]
    fn get_child_mut(&mut self, index: usize) -> &mut Option<usize> {
        unsafe { self.children.get_unchecked_mut(index) }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct QuaternaryTrie {
    nodes: Vec<Node>,
    bit_length: u32,
}
impl QuaternaryTrie {
    /// 構築
    #[inline]
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::new()],
            bit_length: 30,
        }
    }

    #[inline]
    fn get_node_mut(&mut self, index: usize) -> &mut Node {
        unsafe { self.nodes.get_unchecked_mut(index) }
    }
    #[inline]
    fn get_node(&self, index: usize) -> &Node {
        unsafe { self.nodes.get_unchecked(index) }
    }

    /// 値の挿入
    #[inline]
    pub fn insert(&mut self, x: u32) -> u64 {
        self.insert_n(x, 1)
    }
    #[inline]
    pub fn insert_n(&mut self, x: u32, n: u64) -> u64 {
        if n == 0 {
            return 0;
        }
        let mut node_index = 0;
        for i in (0..self.bit_length / 2).rev() {
            self.get_node_mut(node_index).count += n;

            node_index = match self
                .get_node(node_index)
                .get_child((x >> (i * 2) & 3) as usize)
            {
                Some(i) => *i,
                None => {
                    self.nodes.push(Node::new());
                    *self
                        .get_node_mut(node_index)
                        .get_child_mut((x >> (i * 2) & 3) as usize) = Some(self.nodes.len() - 1);
                    self.nodes.len() - 1
                }
            };
        }
        self.get_node_mut(node_index).count += n;
        self.get_node(node_index).count
    }

    /// xのカウント
    #[inline]
    pub fn count(&self, x: u32) -> u64 {
        let mut node_index = Some(0);

        for i in (0..self.bit_length / 2).rev() {
            if node_index.is_none() {
                return 0;
            }
            node_index = *self
                .get_node(node_index.unwrap())
                .get_child((x >> (i * 2) & 3) as usize);
        }
        if node_index.is_none() {
            return 0;
        }
        self.get_node(node_index.unwrap()).count
    }

    /// 値の削除
    #[inline]
    pub fn erase(&mut self, x: u32) -> Option<()> {
        if 1 > self.count(x) {
            return None;
        }
        self.inner_erase(x, 1)
    }

    /// 値をすべて削除
    #[inline]
    pub fn erase_all(&mut self, x: u32) -> Option<()> {
        let erase_count = self.count(x);
        if erase_count == 0 {
            return None;
        }
        self.inner_erase(x, erase_count)
    }

    /// 値を削除
    /// 内部関数
    #[inline]
    fn inner_erase(&mut self, x: u32, erase_count: u64) -> Option<()> {
        let mut node_index = Some(0);
        for i in (0..self.bit_length / 2).rev() {
            self.get_node_mut(node_index?).count -= erase_count;
            node_index = *self
                .get_node(node_index?)
                .get_child((x >> (i * 2) & 3) as usize);
        }
        self.get_node_mut(node_index?).count -= erase_count;

        Some(())
    }

    /// xor 後の最小値を求める
    #[inline]
    pub fn xor_min(&self, x: u32) -> Option<u32> {
        let mut ans = 0;

        let mut node_index = Some(0);
        for i in (0..self.bit_length / 2).rev() {
            let bit = {
                let mut buff = (x >> (i * 2) & 3) as usize;
                let a = self.get_node(node_index.unwrap());

                for j in 0..4 {
                    if a.get_child(buff ^ j)
                        .filter(|&index| self.get_node(index).count > 0)
                        .is_some()
                    {
                        buff ^= j;
                        break;
                    }
                }
                buff
            };
            ans ^= (bit as u32) << (i * 2);
            node_index = *self.get_node(node_index.unwrap()).get_child(bit);
        }
        Some(ans ^ x)
    }

    #[inline]
    pub fn size(&self) -> u64 {
        self.get_node(0).count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn library_checker() {
        let mut b = QuaternaryTrie::new();
        let query = vec![(0, 6), (0, 7), (2, 5), (1, 7), (1, 10), (2, 7)];
        let mut ans = vec![];
        query.iter().for_each(|&(p, x)| match p {
            0 => {
                b.insert(x);
            }
            1 => {
                b.erase_all(x);
            }
            _ => ans.push(b.xor_min(x).unwrap_or_else(|| panic!("{}", x.to_string()))),
        });

        assert_eq!(vec![2, 1], ans);
        assert_eq!(b.count(6), 1);
        assert_eq!(b.count(7), 0);
    }
}
