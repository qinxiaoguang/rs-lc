use super::{ListNode, TreeNode};
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=120 lang=rust
 *
 * [120] 三角形最小路径和
 *
 * https://leetcode-cn.com/problems/triangle/description/
 *
 * algorithms
 * Medium (63.87%)
 * Likes:    437
 * Dislikes: 0
 * Total Accepted:    66.4K
 * Total Submissions: 102K
 * Testcase Example:  '[[2],[3,4],[6,5,7],[4,1,8,3]]'
 *
 * 给定一个三角形，找出自顶向下的最小路径和。每一步只能移动到下一行中相邻的结点上。
 *
 * 相邻的结点 在这里指的是 下标 与 上一层结点下标 相同或者等于 上一层结点下标 + 1 的两个结点。
 *
 *
 *
 * 例如，给定三角形：
 *
 * [
 * ⁠    [2],
 * ⁠   [3,4],
 * ⁠  [6,5,7],
 * ⁠ [4,1,8,3]
 * ]
 *
 *
 * 自顶向下的最小路径和为 11（即，2 + 3 + 5 + 1 = 11）。
 *
 *
 *
 * 说明：
 *
 * 如果你可以只使用 O(n) 的额外空间（n 为三角形的总行数）来解决这个问题，那么你的算法会很加分。
 *
 */

// @lc code=start
impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        // dp 问题，同样是复用数组
        use std::i32;
        let max_len = triangle[triangle.len() - 1].len();
        let mut dp = vec![i32::MAX; max_len];
        for (x, item) in triangle.iter().enumerate() {
            for (y, &val) in item.iter().enumerate().rev() {
                dp[y] = if x == 0 {
                    val
                } else {
                    val + std::cmp::min(dp[y], if y != 0 { dp[y - 1] } else { i32::MAX })
                }
            }
        }
        dp.into_iter().min().unwrap_or(0)
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l120() {
        assert_eq!(Solution::minimum_total(vec![vec![1]]), 1);
        assert_eq!(
            Solution::minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }
}
