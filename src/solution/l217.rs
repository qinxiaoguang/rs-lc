pub struct Solution {}

/*
 * @lc app=leetcode.cn id=217 lang=rust
 *
 * [217] 存在重复元素
 *
 * https://leetcode-cn.com/problems/contains-duplicate/description/
 *
 * algorithms
 * Easy (51.70%)
 * Likes:    268
 * Dislikes: 0
 * Total Accepted:    150K
 * Total Submissions: 284.7K
 * Testcase Example:  '[1,2,3,1]'
 *
 * 给定一个整数数组，判断是否存在重复元素。
 *
 * 如果任意一值在数组中出现至少两次，函数返回 true 。如果数组中每个元素都不相同，则返回 false 。
 *
 *
 *
 * 示例 1:
 *
 * 输入: [1,2,3,1]
 * 输出: true
 *
 * 示例 2:
 *
 * 输入: [1,2,3,4]
 * 输出: false
 *
 * 示例 3:
 *
 * 输入: [1,1,1,3,3,4,3,2,4,2]
 * 输出: true
 *
 */

// @lc code=start
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        // 注意不要和经典问题 ：数组中每个元素出现两次，找出出现一次的数 搞混
        // 异或是 ： 0和任何数异或都是任何数，任何数和他自己异或都是0
        // 所以此题不能用异或，而是用排序，或者map
        if nums.len() <= 1 {
            return false;
        }
        let mut nums = nums;
        nums.sort();
        for (idx, &num) in nums.iter().enumerate() {
            if idx == 0 {
                continue;
            }
            if num == nums[idx - 1] {
                return true;
            }
        }
        return false;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l217() {
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::contains_duplicate(vec![1, 1, 2, 3, 4]), true);
        assert_eq!(Solution::contains_duplicate(vec![1, 1, 1, 2, 3, 4]), true);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
        assert_eq!(Solution::contains_duplicate(vec![1, 1]), true);
    }
}
