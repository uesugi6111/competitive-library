#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Node {
    lr: Vec<Option<u32>>,
    count: u32,
}
impl Node {
    fn new(n: u32) -> Self {
        Self {
            lr: vec![None; 2],
            count: n,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct BinaryTrie {
    v: Vec<Node>,
}
impl BinaryTrie {
    pub fn new() -> Self {
        Self {
            v: vec![Node::new(0); 1],
        }
    }
    pub fn add(&mut self, x: u32) {
        let mut i = 0;
        for j in (0..32).rev() {
            let f = (x >> j & 1) as usize;
            if self.v[i].lr[f].is_none() {
                self.v.push(Node::new(1));
                self.v[i].lr[f] = Some(self.v.len() as u32 - 1);
            } else {
                self.v[i].count += 1;
            }
            i = self.v[i].lr[f].unwrap() as usize;
        }
    }

}