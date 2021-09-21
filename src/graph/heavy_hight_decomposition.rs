//! HL 分解

pub struct HeavyLightDecomposition {
    s: Vec<usize>,
    d: Vec<usize>,
    c: Vec<Vec<usize>>,

    parent: Vec<usize>,
}

impl HeavyLightDecomposition {
    pub fn new(parent: &[usize]) -> Self {
        Self {
            s: vec![0; parent.len()],
            c: vec![],
            d: vec![0; parent.len()],
            parent: parent.to_vec(),
        }
    }
    pub fn count_node(&mut self, v: usize) -> usize {
        if self.s[v] != 0 {
            return self.s[v];
        }
        self.s[v] = 1;
        for i in 0..self.c[v].len() {
            self.s[v] += self.count_node(self.c[v][i]);
        }
        self.s[v]
    }

    pub fn depth(&mut self, v: usize) -> usize {
        if self.d[v] != 0 {
            return self.d[v];
        }
        if self.parent[v] == v {
            return 0;
        }
        self.d[v] = self.depth(self.parent[v]) + 1;
        self.d[v]
    }
}
