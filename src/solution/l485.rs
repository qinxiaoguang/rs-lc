pub struct Solution {}

/*
 * @lc app=leetcode.cn id=485 lang=rust
 *
 * [485] 最大连续1的个数
 *
 * https://leetcode-cn.com/problems/max-consecutive-ones/description/
 *
 * algorithms
 * Easy (55.88%)
 * Likes:    108
 * Dislikes: 0
 * Total Accepted:    44.5K
 * Total Submissions: 78.7K
 * Testcase Example:  '[1,0,1,1,0,1]'
 *
 * 给定一个二进制数组， 计算其中最大连续1的个数。
 *
 * 示例 1:
 *
 *
 * 输入: [1,1,0,1,1,1]
 * 输出: 3
 * 解释: 开头的两位和最后的三位都是连续1，所以最大连续1的个数是 3.
 *
 *
 * 注意：
 *
 *
 * 输入的数组只包含 0 和1。
 * 输入数组的长度是正整数，且不超过 10,000。
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let (mut max_cnt, mut tmp_cnt) = (0, 0);
        let mut nums = nums;
        nums.push(0);
        for num in nums {
            if num != 1 {
                max_cnt = std::cmp::max(max_cnt, tmp_cnt);
                tmp_cnt = 0;
            } else {
                tmp_cnt += 1;
            }
        }
        max_cnt
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l485() {
        assert_eq!(
            3,
            Solution::find_max_consecutive_ones(vec![1, 1, 0, 1, 1, 1])
        );
        assert_eq!(0, Solution::find_max_consecutive_ones(vec![]));
        assert_eq!(0, Solution::find_max_consecutive_ones(vec![0]));
        assert_eq!(1, Solution::find_max_consecutive_ones(vec![1]));
    }
}
