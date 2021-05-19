// 并查集模板
struct DSet {
    // 并查集结点，Nodes[i]=j，则i节点的父节点为j
    nodes: Vec<usize>,
    // 每个点的集合数量
    size: Vec<i32>,
}

impl DSet {
    pub fn new(len: usize) -> Self {
        // 自己的父节点就是自己
        let mut nodes = vec![0; len]
            .into_iter()
            .enumerate()
            .map(|(i, _)| i)
            .collect();
        DSet {
            nodes: nodes,
            size: vec![1; len],
        }
    }

    // 查找该点的父节点
    pub fn find(&mut self, node: usize) -> usize {
        if self.nodes[node] != node {
            // 查找的同时将子节点的父节点都设置为顶级父节点
            self.nodes[node] = self.find(self.nodes[node])
        }
        self.nodes[node]
    }

    // 两个节点为同一个集合，合并两个结点
    pub fn union(&mut self, x: usize, y: usize) {
        let xp = self.find(x);
        let yp = self.find(y);
        if xp == yp {
            return;
        }
        self.nodes[xp] = yp;
        self.size[yp] += self.size[xp];
    }

    pub fn size(&mut self, x: usize) -> i32 {
        let xp = self.find(x);
        self.size[xp]
    }
}
