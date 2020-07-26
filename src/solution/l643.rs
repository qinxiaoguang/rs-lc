pub struct Solution {}
/*
 * @lc app=leetcode.cn id=643 lang=rust
 *
 * [643] 子数组最大平均数 I
 *
 * https://leetcode-cn.com/problems/maximum-average-subarray-i/description/
 *
 * algorithms
 * Easy (38.00%)
 * Likes:    101
 * Dislikes: 0
 * Total Accepted:    16.6K
 * Total Submissions: 42.7K
 * Testcase Example:  '[1,12,-5,-6,50,3]\n4'
 *
 * 给定 n 个整数，找出平均数最大且长度为 k 的连续子数组，并输出该最大平均数。
 *
 * 示例 1:
 *
 * 输入: [1,12,-5,-6,50,3], k = 4
 * 输出: 12.75
 * 解释: 最大平均数 (12-5-6+50)/4 = 51/4 = 12.75
 *
 *
 *
 *
 * 注意:
 *
 *
 * 1 <= k <= n <= 30,000。
 * 所给数据范围 [-10,000，10,000]。
 *
 *
 */

// @lc code=start
impl Solution {
    // 限定了长度为k
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;
        if nums.len() < k {
            return 0f64;
        }
        let mut max: i32 = nums[..k].iter().sum();
        let mut sum = max;
        for i in k..nums.len() {
            sum = sum + nums[i] - nums[i - k];
            //println!("i:{},sum:{}", i, sum);
            max = std::cmp::max(max, sum);
        }
        max as f64 / k as f64
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l643() {
        assert_eq!(
            12.75,
            Solution::find_max_average(vec![1, 12, -5, -6, 50, 3], 4)
        );
        assert_eq!(4f64, Solution::find_max_average(vec![0, 4, 0, 3, 2], 1));
        assert_eq!(3f64, Solution::find_max_average(vec![4, 2, 1, 3, 3], 2));
    }
}
