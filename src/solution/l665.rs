pub struct Solution {}
/*
 * @lc app=leetcode.cn id=665 lang=rust
 *
 * [665] 非递减数列
 *
 * https://leetcode-cn.com/problems/non-decreasing-array/description/
 *
 * algorithms
 * Easy (21.98%)
 * Likes:    301
 * Dislikes: 0
 * Total Accepted:    22.5K
 * Total Submissions: 100.5K
 * Testcase Example:  '[4,2,3]'
 *
 * 给你一个长度为 n 的整数数组，请你判断在 最多 改变 1 个元素的情况下，该数组能否变成一个非递减数列。
 *
 * 我们是这样定义一个非递减数列的： 对于数组中所有的 i (0 <= i <= n-2)，总满足 nums[i] <= nums[i + 1]。
 *
 *
 *
 * 示例 1:
 *
 * 输入: nums = [4,2,3]
 * 输出: true
 * 解释: 你可以通过把第一个4变成1来使得它成为一个非递减数列。
 *
 *
 * 示例 2:
 *
 * 输入: nums = [4,2,1]
 * 输出: false
 * 解释: 你不能在只改变一个元素的情况下将其变为非递减数列。
 *
 *
 *
 *
 * 说明：
 *
 *
 * 1 <= n <= 10 ^ 4
 * - 10 ^ 5 <= nums[i] <= 10 ^ 5
 *
 *
 */

// @lc code=start
impl Solution {
    // 只需要求整个数列中，两个数中非递减的数量，如果大于1个，就无法完成
    // 但如果只有一个，也可能会导致结果不对，所以需要一步校验
    pub fn check_possibility(nums: Vec<i32>) -> bool {
        let mut down_cnt = 0;
        let mut s_idx = 0;
        if nums.len() <= 1 {
            return true;
        }
        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                down_cnt += 1;
                s_idx = i;
            }
        }
        if down_cnt == 0 {
            return true;
        }
        if down_cnt == 1 {
            let mut nums = nums;
            let (s, l) = (nums[s_idx], nums[s_idx + 1]);
            nums[s_idx] = l;
            if Self::check(&nums) {
                return true;
            }
            nums[s_idx + 1] = s;
            nums[s_idx] = s;
            if Self::check(&nums) {
                return true;
            }
        }
        return false;
    }

    pub fn check(nums: &Vec<i32>) -> bool {
        for i in 0..nums.len() - 1 {
            if nums[i] > nums[i + 1] {
                return false;
            }
        }
        return true;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l665() {
        assert_eq!(true, Solution::check_possibility(vec![4, 2, 3]));
        assert_eq!(false, Solution::check_possibility(vec![4, 2, 1]));
        assert_eq!(false, Solution::check_possibility(vec![3, 4, 2, 3]));
    }
}
