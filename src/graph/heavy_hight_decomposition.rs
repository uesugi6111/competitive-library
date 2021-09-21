//! HL 分解

pub struct HeavyLightDecomposition {
    s: Vec<usize>,
    c: Vec<Vec<usize>>,
}

impl HeavyLightDecomposition {
    pub fn new(e: &[Vec<usize>]) -> Self {
        Self {
            s: vec![0; e.len()],
            c: vec![],
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
}
