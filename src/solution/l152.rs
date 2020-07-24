pub struct Solution {}

/*
 * @lc app=leetcode.cn id=152 lang=rust
 *
 * [152] 乘积最大子序列
 *
 * https://leetcode-cn.com/problems/maximum-product-subarray/description/
 *
 * algorithms
 * Medium (37.45%)
 * Likes:    660
 * Dislikes: 0
 * Total Accepted:    79.5K
 * Total Submissions: 199.2K
 * Testcase Example:  '[2,3,-2,4]'
 *
 * 给你一个整数数组 nums ，请你找出数组中乘积最大的连续子数组（该子数组中至少包含一个数字），并返回该子数组所对应的乘积。
 *
 *
 *
 * 示例 1:
 *
 * 输入: [2,3,-2,4]
 * 输出: 6
 * 解释: 子数组 [2,3] 有最大乘积 6。
 *
 *
 * 示例 2:
 *
 * 输入: [-2,0,-1]
 * 输出: 0
 * 解释: 结果不能为 2, 因为 [-2,-1] 不是子数组。
 *
 */

// @lc code=start
impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        use std::cmp::{max, min};
        // dp 问题，min_dp[i]表示以i为底的连续数组的乘积的最小值
        // max_dp[i] 则表示以i为底的连续数组的成绩的最大值
        // 因为乘积是和负数有关联的，所以需要上述两个值
        // 那么转移方程就是 min_dp[i] = min(nums[i] * min_dp[i],nums[i]*max_dp[i]， nums[i]);
        // 同样 max_dp[i] = max(nums[i]*max_dp[i], nums[i]*max_dp[i],nums[i]);
        let (mut min_dp, mut max_dp) = (vec![0i64; nums.len()], vec![0i64; nums.len()]);
        for i in 0..nums.len() {
            if i == 0 {
                min_dp[i] = nums[i] as i64;
                max_dp[i] = nums[i] as i64;
                continue;
            }
            // 其实min_dp和max_dp都可以优化成O(1),看下面的转移公式就知道了
            min_dp[i] = min(
                max_dp[i - 1] * nums[i] as i64,
                min(nums[i] as i64, min_dp[i - 1] * nums[i] as i64),
            );
            max_dp[i] = max(
                max_dp[i - 1] * nums[i] as i64,
                max(nums[i] as i64, min_dp[i - 1] * nums[i] as i64),
            );
        }
        max_dp.into_iter().max().unwrap() as i32
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l152() {
        assert_eq!(Solution::max_product(vec![2, 3, -2, 4]), 6);
        assert_eq!(Solution::max_product(vec![-2, 0, -1]), 0);
    }
}
