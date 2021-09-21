//! HL 分解

pub struct HeavyLightDecomposition {
    child_count: Vec<usize>,
    depths: Vec<usize>,
    e: Vec<Vec<usize>>,
    pre: Vec<usize>,
    parent: Vec<usize>,
    hld: Vec<usize>,
    head: Vec<usize>,
}

impl HeavyLightDecomposition {
    pub fn new(parent: &[usize]) -> Self {
        let e = {
            let mut e = vec![vec![]];
            for (i, &v) in parent.iter().enumerate() {
                e[v].push(i);
            }
            e
        };

        Self {
            child_count: vec![0; parent.len()],
            e,
            depths: vec![0; parent.len()],
            parent: parent.to_vec(),
            pre: vec![0; parent.len()],
            hld: vec![],
            head: (0..parent.len()).collect(),
        }
    }
    pub fn decompose(&mut self) {
        for i in 0..self.e.len() {
            if self.parent[i] == i {
                continue;
            }
            self.decompose_inner(i, i);
        }
    }
    fn decompose_inner(&mut self, v: usize, a: usize) {
        self.pre[v] = self.hld.len();
        self.hld.push(v);
        self.head[v] = a;

        if self.e[v].is_empty() {
            return;
        }
        let mut m = 0;
        let mut index = -1;
        for i in 0..self.e[v].len() {
            if self.count_node(self.e[v][i]) > m {
                m = self.count_node(self.e[v][i]);
                index = i as i32;
            }
        }
        self.decompose_inner(self.e[v][index as usize], a);

        for i in 0..self.e[v].len() {
            if i != index as usize {
                self.decompose_inner(self.e[v][i], self.e[v][i]);
            }
        }
    }
    pub fn count_node(&mut self, value: usize) -> usize {
        if self.child_count[value] != 0 {
            return self.child_count[value];
        }
        self.child_count[value] = 1;
        for i in 0..self.e[value].len() {
            self.child_count[value] += self.count_node(self.e[value][i]);
        }
        self.child_count[value]
    }

    pub fn depth(&mut self, v: usize) -> usize {
        if self.depths[v] != 0 {
            return self.depths[v];
        }
        if self.parent[v] == v {
            return 0;
        }
        self.depths[v] = self.depth(self.parent[v]) + 1;
        self.depths[v]
    }
}
