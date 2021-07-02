#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Node {
    children: Vec<Option<u32>>,
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
    nodes: Vec<Node>,
}
impl BinaryTrie {
    pub fn new() -> Self {
        Self {
            nodes: vec![Node::new(0); 1],
        }
    }
    pub fn add(&mut self, x: u32) {
        let mut i = 0;
        for j in (0..32).rev() {
            let f = (x >> j & 1) as usize;
            if self.nodes[i].children[f].is_none() {
                self.nodes.push(Node::new(1));
                self.nodes[i].children[f] = Some(self.nodes.len() as u32 - 1);
            } else {
                self.nodes[i].count += 1;
            }
            i = self.nodes[i].children[f].unwrap() as usize;
        }
    }

    pub fn remove(&mut self, x: u32) -> bool {
        let mut path = vec![0; 32];
        let mut i = 0;
        for j in (0..32).rev() {
            path[j] = i;
            let f = (x >> j & 1) as usize;
            if self.nodes[i].children[f].is_none() {
                return false;
            }
            i = self.nodes[i].children[f].unwrap() as usize;
        }
        path.iter().for_each(|&i| self.nodes[i].count -= 1);
        true
    }

    pub fn xor_min(&self, n: u32) -> u32 {
        let mut x = n;
        let mut i = 0;
        for j in (0..32).rev() {
            let mut f = (x >> j & 1) as usize;
            if self.nodes[i].children[f].is_none() {
                f ^= 1;
            }
            x ^= (f as u32) << j;
            i = self.nodes[i].children[f].unwrap() as usize;
        }
        x
    }
}

#[test]
fn a() {
    let mut b = BinaryTrie::new();
    b.add(1);
    let a = b.clone();
    dbg!(&b);
    b.add(1);
    dbg!(&b);
    b.remove(1);
    dbg!(&b);
    assert_eq!(a.nodes, b.nodes);
    println!("{}", !0usize);
    println!("{}", 8 * std::mem::size_of::<u32>());
}
