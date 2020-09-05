pub struct Solution {}
/*
 * @lc app=leetcode.cn id=674 lang=rust
 *
 * [674] 最长连续递增序列
 *
 * https://leetcode-cn.com/problems/longest-continuous-increasing-subsequence/description/
 *
 * algorithms
 * Easy (45.13%)
 * Likes:    107
 * Dislikes: 0
 * Total Accepted:    37.5K
 * Total Submissions: 83.2K
 * Testcase Example:  '[1,3,5,4,7]'
 *
 * 给定一个未经排序的整数数组，找到最长且连续的的递增序列，并返回该序列的长度。
 *
 *
 *
 * 示例 1:
 *
 * 输入: [1,3,5,4,7]
 * 输出: 3
 * 解释: 最长连续递增序列是 [1,3,5], 长度为3。
 * 尽管 [1,3,5,7] 也是升序的子序列, 但它不是连续的，因为5和7在原数组里被4隔开。
 *
 *
 * 示例 2:
 *
 * 输入: [2,2,2,2,2]
 * 输出: 1
 * 解释: 最长连续递增序列是 [2], 长度为1。
 *
 *
 *
 *
 * 注意：数组长度不会超过10000。
 *
 */

// @lc code=start
impl Solution {
    pub fn find_length_of_lcis(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut res = 1;
        let mut max = 1;
        let mut last = nums[0];
        for num in nums.into_iter().skip(1) {
            if num > last {
                max += 1;
            } else {
                res = std::cmp::max(res, max);
                max = 1;
            }
            last = num;
        }
        return std::cmp::max(res, max);
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l674() {
        assert_eq!(3, Solution::find_length_of_lcis(vec![1, 3, 5, 4, 7]));
        assert_eq!(4, Solution::find_length_of_lcis(vec![1, 3, 5, 7]));
        assert_eq!(1, Solution::find_length_of_lcis(vec![2, 2, 2, 2]));
    }
}
