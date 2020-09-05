pub struct Solution {}
/*
 * @lc app=leetcode.cn id=713 lang=rust
 *
 * [713] 乘积小于K的子数组
 *
 * https://leetcode-cn.com/problems/subarray-product-less-than-k/description/
 *
 * algorithms
 * Medium (36.48%)
 * Likes:    162
 * Dislikes: 0
 * Total Accepted:    8.8K
 * Total Submissions: 24.2K
 * Testcase Example:  '[10,5,2,6]\n100'
 *
 * 给定一个正整数数组 nums。
 *
 * 找出该数组内乘积小于 k 的连续的子数组的个数。
 *
 * 示例 1:
 *
 *
 * 输入: nums = [10,5,2,6], k = 100
 * 输出: 8
 * 解释: 8个乘积小于100的子数组分别为: [10], [5], [2], [6], [10,5], [5,2], [2,6], [5,2,6]。
 * 需要注意的是 [10,5,2] 并不是乘积小于100的子数组。
 *
 *
 * 说明:
 *
 *
 * 0 < nums.length <= 50000
 * 0 < nums[i] < 1000
 * 0 <= k < 10^6
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        // 采用尺取法，或者双指针法，每次取出最长的连续子序列，并做计算
        if k <= 1 {
            return 0;
        }
        let mut times = 1;
        let mut res = 0;
        let mut left = 0;

        for (right, val) in nums.iter().enumerate() {
            times *= val;
            while times >= k {
                times /= nums[left];
                left += 1;
            }
            // 这一步很重要，需要细品，可以将示例举例来品
            // 在right右移的时候，就将单个的，多个都加上了
            // 但是这一步的逻辑很难想出来，不知道别人是怎么想到的
            res += right - left + 1;
        }
        res as i32
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l713() {
        assert_eq!(
            8,
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100)
        );
        assert_eq!(
            3,
            Solution::num_subarray_product_less_than_k(vec![1, 1], 100)
        );
        assert_eq!(
            6,
            Solution::num_subarray_product_less_than_k(vec![1, 1, 1], 100)
        );
    }
}
