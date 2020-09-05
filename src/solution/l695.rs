pub struct Solution {}
/*
 * @lc app=leetcode.cn id=695 lang=rust
 *
 * [695] 岛屿的最大面积
 *
 * https://leetcode-cn.com/problems/max-area-of-island/description/
 *
 * algorithms
 * Medium (63.95%)
 * Likes:    345
 * Dislikes: 0
 * Total Accepted:    59.3K
 * Total Submissions: 92.7K
 * Testcase Example:  '[[1,1,0,0,0],[1,1,0,0,0],[0,0,0,1,1],[0,0,0,1,1]]'
 *
 * 给定一个包含了一些 0 和 1 的非空二维数组 grid 。
 *
 * 一个 岛屿 是由一些相邻的 1 (代表土地) 构成的组合，这里的「相邻」要求两个 1 必须在水平或者竖直方向上相邻。你可以假设 grid 的四个边缘都被
 * 0（代表水）包围着。
 *
 * 找到给定的二维数组中最大的岛屿面积。(如果没有岛屿，则返回面积为 0 。)
 *
 *
 *
 * 示例 1:
 *
 * [[0,0,1,0,0,0,0,1,0,0,0,0,0],
 * ⁠[0,0,0,0,0,0,0,1,1,1,0,0,0],
 * ⁠[0,1,1,0,1,0,0,0,0,0,0,0,0],
 * ⁠[0,1,0,0,1,1,0,0,1,0,1,0,0],
 * ⁠[0,1,0,0,1,1,0,0,1,1,1,0,0],
 * ⁠[0,0,0,0,0,0,0,0,0,0,1,0,0],
 * ⁠[0,0,0,0,0,0,0,1,1,1,0,0,0],
 * ⁠[0,0,0,0,0,0,0,1,1,0,0,0,0]]
 *
 *
 * 对于上面这个给定矩阵应返回 6。注意答案不应该是 11 ，因为岛屿只能包含水平或垂直的四个方向的 1 。
 *
 * 示例 2:
 *
 * [[0,0,0,0,0,0,0,0]]
 *
 * 对于上面这个给定的矩阵, 返回 0。
 *
 *
 *
 * 注意: 给定的矩阵grid 的长度和宽度都不超过 50。
 *
 */

// @lc code=start

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 {
            return 0;
        }
        let mut max = 0;
        let mut used = vec![vec![false; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if used[i][j] || grid[i][j] == 0 {
                    continue;
                }
                let mut sum = 0;
                Self::dfs(&grid, &mut used, i, j, &mut sum);
                max = std::cmp::max(max, sum);
            }
        }
        return max;
    }

    pub fn dfs(grid: &Vec<Vec<i32>>, used: &mut Vec<Vec<bool>>, i: usize, j: usize, sum: &mut i32) {
        // 上下左右四个方向
        used[i][j] = true;
        *sum += 1;
        let dirs = vec![(0, 1), (0, -1i32), (1, 0), (-1i32, 0)];
        for dir in dirs {
            let next_x = i as i32 + dir.0;
            let next_y = j as i32 + dir.1;
            if next_x < 0
                || next_x >= grid.len() as i32
                || next_y < 0
                || next_y >= grid[0].len() as i32
                || used[next_x as usize][next_y as usize]
                || grid[next_x as usize][next_y as usize] == 0
            {
                continue;
            }
            Self::dfs(grid, used, next_x as usize, next_y as usize, sum);
        }
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l695() {}
}
