pub struct Solution {}
/*
 * @lc app=leetcode.cn id=1020 lang=rust
 *
 * [1020] 飞地的数量
 *
 * https://leetcode-cn.com/problems/number-of-enclaves/description/
 *
 * algorithms
 * Medium (51.96%)
 * Likes:    35
 * Dislikes: 0
 * Total Accepted:    5.5K
 * Total Submissions: 10.4K
 * Testcase Example:  '[[0,0,0,0],[1,0,1,0],[0,1,1,0],[0,0,0,0]]'
 *
 * 给出一个二维数组 A，每个单元格为 0（代表海）或 1（代表陆地）。
 *
 * 移动是指在陆地上从一个地方走到另一个地方（朝四个方向之一）或离开网格的边界。
 *
 * 返回网格中无法在任意次数的移动中离开网格边界的陆地单元格的数量。
 *
 *
 *
 * 示例 1：
 *
 * 输入：[[0,0,0,0],[1,0,1,0],[0,1,1,0],[0,0,0,0]]
 * 输出：3
 * 解释：
 * 有三个 1 被 0 包围。一个 1 没有被包围，因为它在边界上。
 *
 * 示例 2：
 *
 * 输入：[[0,1,1,0],[0,0,1,0],[0,0,1,0],[0,0,0,0]]
 * 输出：0
 * 解释：
 * 所有 1 都在边界上或可以到达边界。
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= A.length <= 500
 * 1 <= A[i].length <= 500
 * 0 <= A[i][j] <= 1
 * 所有行的大小都相同
 *
 *
 */

// @lc code=start

// 并查集模板
struct DSet {
    // 并查集结点，Nodes[i]=j，则i节点的父节点为j
    nodes: Vec<usize>,
    // 某个点能否到达边界
    size: Vec<bool>,
}

impl DSet {
    pub fn new(len: usize) -> Self {
        // 自己的父节点就是自己
        let mut nodes = (0..=len).collect();
        DSet {
            nodes: nodes,
            size: vec![false; len],
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
        if self.size[yp] {
            self.size[xp] = true;
        }
    }

    pub fn size(&mut self, x: usize) -> bool {
        let xp = self.find(x);
        self.size[xp]
    }

    pub fn reach(&mut self, x: usize) {
        let xp = self.find(x);
        self.size[xp] = true;
    }
}

impl Solution {
    // bfs加并查集
    pub fn num_enclaves(a: Vec<Vec<i32>>) -> i32 {
        if a.len() == 0 {
            return 0;
        }
        let (row, col) = (a.len(), a[0].len());
        let mut dset = DSet::new(row * col as usize + 1);
        let mut used = vec![vec![false; col]; row];
        for i in 0..row {
            for j in 0..col {
                if a[i][j] == 1 && !used[i][j] {
                    used[i][j] = true;
                    Self::dfs(&a, &mut used, &mut dset, (i, j));
                }
            }
        }
        let mut res = 0;
        for i in 0..row {
            for j in 0..col {
                /*print!(
                    "{},{} ",
                    dset.find(Self::get_seq((i, j), col)),
                    dset.size(Self::get_seq((i, j), col))
                );*/
                if a[i][j] == 1 && !dset.size(Self::get_seq((i, j), col)) {
                    res += 1;
                }
            }
        }
        res
    }

    fn dfs(a: &Vec<Vec<i32>>, used: &mut Vec<Vec<bool>>, dset: &mut DSet, (x, y): (usize, usize)) {
        let dirs = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];

        for dir in dirs {
            let (next_x, next_y) = (x as i32 + dir.0, y as i32 + dir.1);
            if Self::check((next_x, next_y), (a.len(), a[0].len())) {
                // 越界，当前点可以到达陆地
                dset.reach(Self::get_seq((x as usize, y as usize), a[0].len()));
                continue;
            }
            if !used[next_x as usize][next_y as usize] && a[next_x as usize][next_y as usize] == 1 {
                // 下一个是陆地且没被用过
                used[next_x as usize][next_y as usize] = true;
                dset.union(
                    Self::get_seq((next_x as usize, next_y as usize), a[0].len()),
                    Self::get_seq((x, y), a[0].len()),
                );
                Self::dfs(a, used, dset, (next_x as usize, next_y as usize));
            }
        }
    }

    // 将二维坐标转换为一维
    fn get_seq((x, y): (usize, usize), len: usize) -> usize {
        x * len + y
    }

    // 检查是否越界,越界返回true
    fn check((x, y): (i32, i32), (row, col): (usize, usize)) -> bool {
        x < 0 || y < 0 || x >= row as i32 || y >= col as i32
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1020() {
        assert_eq!(
            0,
            Solution::num_enclaves(vec![
                vec![0, 1, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 1, 0],
                vec![0, 0, 0, 0]
            ])
        );

        assert_eq!(
            3,
            Solution::num_enclaves(vec![
                vec![0, 0, 0, 0],
                vec![1, 0, 1, 0],
                vec![0, 1, 1, 0],
                vec![0, 0, 0, 0]
            ])
        );

        assert_eq!(
            8,
            Solution::num_enclaves(vec![
                vec![0, 1, 1, 0, 0, 1, 1, 1, 0, 0, 0, 0, 1],
                vec![0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1],
                vec![1, 0, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 0],
                vec![0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0],
                vec![1, 1, 0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 1],
                vec![0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 1],
                vec![1, 0, 1, 1, 0, 1, 1, 1, 1, 1, 1, 0, 0],
                vec![0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 1, 0],
                vec![1, 0, 1, 1, 1, 0, 1, 1, 1, 0, 0, 1, 0],
                vec![1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 1, 0, 1]
            ])
        );
    }
}
