use super::{ListNode, TreeNode};
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=118 lang=rust
 *
 * [118] 杨辉三角
 *
 * https://leetcode-cn.com/problems/pascals-triangle/description/
 *
 * algorithms
 * Easy (65.94%)
 * Likes:    327
 * Dislikes: 0
 * Total Accepted:    88.4K
 * Total Submissions: 132.6K
 * Testcase Example:  '5'
 *
 * 给定一个非负整数 numRows，生成杨辉三角的前 numRows 行。
 *
 *
 *
 * 在杨辉三角中，每个数是它左上方和右上方的数的和。
 *
 * 示例:
 *
 * 输入: 5
 * 输出:
 * [
 * ⁠    [1],
 * ⁠   [1,1],
 * ⁠  [1,2,1],
 * ⁠ [1,3,3,1],
 * ⁠[1,4,6,4,1]
 * ]
 *
 */

// @lc code=start
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res = vec![vec![1]];
        if num_rows == 0 {
            return vec![];
        }

        for row in 1..num_rows {
            let mut row_vec = vec![0; row as usize + 1];
            for x in 0..row + 1 {
                let last_x = x - 1;
                row_vec[x as usize] = if last_x >= 0 {
                    res[row as usize - 1][x as usize - 1]
                } else {
                    0
                } + if x == row {
                    0
                } else {
                    res[row as usize - 1][x as usize]
                };
            }
            res.push(row_vec);
        }

        res
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l118() {
        println!("{:?}", Solution::generate(0));
    }
}
