pub struct Solution {}
/*
 * @lc app=leetcode.cn id=924 lang=rust
 *
 * [924] 尽量减少恶意软件的传播
 *
 * https://leetcode-cn.com/problems/minimize-malware-spread/description/
 *
 * algorithms
 * Hard (36.40%)
 * Likes:    42
 * Dislikes: 0
 * Total Accepted:    3.1K
 * Total Submissions: 8.7K
 * Testcase Example:  '[[1,1,0],[1,1,0],[0,0,1]]\n[0,1]'
 *
 * 在节点网络中，只有当 graph[i][j] = 1 时，每个节点 i 能够直接连接到另一个节点 j。
 *
 * 一些节点 initial
 * 最初被恶意软件感染。只要两个节点直接连接，且其中至少一个节点受到恶意软件的感染，那么两个节点都将被恶意软件感染。这种恶意软件的传播将继续，直到没有更多的节点可以被这种方式感染。
 *
 * 假设 M(initial) 是在恶意软件停止传播之后，整个网络中感染恶意软件的最终节点数。
 *
 * 我们可以从初始列表中删除一个节点。如果移除这一节点将最小化 M(initial)， 则返回该节点。如果有多个节点满足条件，就返回索引最小的节点。
 *
 * 请注意，如果某个节点已从受感染节点的列表 initial 中删除，它以后可能仍然因恶意软件传播而受到感染。
 *
 *
 *
 *
 *
 *
 * 示例 1：
 *
 * 输入：graph = [[1,1,0],[1,1,0],[0,0,1]], initial = [0,1]
 * 输出：0
 *
 *
 * 示例 2：
 *
 * 输入：graph = [[1,0,0],[0,1,0],[0,0,1]], initial = [0,2]
 * 输出：0
 *
 *
 * 示例 3：
 *
 * 输入：graph = [[1,1,1],[1,1,1],[1,1,1]], initial = [1,2]
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 < graph.length = graph[0].length <= 300
 * 0 <= graph[i][j] == graph[j][i] <= 1
 * graph[i][i] == 1
 * 1 <= initial.length < graph.length
 * 0 <= initial[i] < graph.length
 *
 *
 */

// @lc code=start
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
impl Solution {
    // 只要一个集合里有一个感染，则所有节点都感染
    // 所以此题是一个并查集问题,但是需要用一个map来记录整个并查集的个数
    // 枚举initial的每个点，需要直接给出属于该集合的个数
    // 输出集合个数最大的那个点
    // 但是如果被黑的有两个点在同一个集合，那么这两个点取谁都一样，没有效果
    // 所以要找整个被黑的节点中独立的节点
    pub fn min_malware_spread(graph: Vec<Vec<i32>>, initial: Vec<i32>) -> i32 {
        let node_num = graph.len();
        //let mut used = vec![false; node_num];
        let mut dset = DSet::new(node_num);
        for i in 0..node_num - 1 {
            for j in i + 1..node_num {
                if graph[i][j] == 1 {
                    dset.union(i, j);
                }
            }
        }
        let mut res = -1;
        let mut max_size = -1;
        let mut count = vec![0; node_num];
        // 如果有两个结点在同一个集合，那么他们的同一个父节点的count将会不是1
        for node in &initial {
            count[dset.find(*node as usize)] += 1;
        }
        for node in &initial {
            let size = dset.size(*node as usize);
            let root = dset.find(*node as usize);
            if count[root] == 1 {
                if size > max_size {
                    max_size = size;
                    res = *node;
                    continue;
                }
                if size == max_size && res > *node {
                    res = *node;
                }
            }
        }
        if res == -1 {
            // 所有值都可以
            res = std::i32::MAX;
            for node in initial {
                res = std::cmp::min(res, node);
            }
        }
        res
    }
}
// @lc code=end
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l924() {
        assert_eq!(
            0,
            Solution::min_malware_spread(
                vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]],
                vec![0, 2]
            )
        );

        assert_eq!(
            0,
            Solution::min_malware_spread(
                vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]],
                vec![0, 2]
            )
        );

        assert_eq!(
            2,
            Solution::min_malware_spread(
                vec![vec![1, 1, 0], vec![1, 1, 0], vec![0, 0, 1]],
                vec![0, 1, 2]
            )
        );

        assert_eq!(
            3,
            Solution::min_malware_spread(
                vec![
                    vec![1, 0, 0, 0],
                    vec![0, 1, 0, 0],
                    vec![0, 0, 1, 1],
                    vec![0, 0, 1, 1]
                ],
                vec![3, 1]
            )
        );
    }
}
