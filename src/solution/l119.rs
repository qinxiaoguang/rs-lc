use super::{ListNode, TreeNode};
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=119 lang=rust
 *
 * [119] 杨辉三角 II
 *
 * https://leetcode-cn.com/problems/pascals-triangle-ii/description/
 *
 * algorithms
 * Easy (60.32%)
 * Likes:    160
 * Dislikes: 0
 * Total Accepted:    58.6K
 * Total Submissions: 95.5K
 * Testcase Example:  '3'
 *
 * 给定一个非负索引 k，其中 k ≤ 33，返回杨辉三角的第 k 行。
 *
 *
 *
 * 在杨辉三角中，每个数是它左上方和右上方的数的和。
 *
 * 示例:
 *
 * 输入: 3
 * 输出: [1,3,3,1]
 *
 *
 * 进阶：
 *
 * 你可以优化你的算法到 O(k) 空间复杂度吗？
 *
 */

// @lc code=start
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        // O(k)空间复杂度，意思就是复用数组
        let row_index = row_index + 1;
        if row_index <= 0 {
            return vec![];
        }

        let mut res = vec![0; row_index as usize];
        res[0] = 1;

        for row in 1..row_index {
            // 因为要复用，所以需要从后往前遍历
            for x in (0..row + 1).rev() {
                let last_x = x - 1;
                res[x as usize] =
                    if last_x >= 0 { res[x as usize - 1] } else { 0 } + res[x as usize]
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
    fn test_l119() {
        assert_eq!(vec![1], Solution::get_row(0));
        assert_eq!(vec![1, 1], Solution::get_row(1));
        assert_eq!(vec![1, 2, 1], Solution::get_row(2));
        assert_eq!(vec![1, 3, 3, 1], Solution::get_row(3));
        assert_eq!(vec![1, 4, 6, 4, 1], Solution::get_row(4));
    }
}
