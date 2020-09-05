pub struct Solution {}
/*
 * @lc app=leetcode.cn id=724 lang=rust
 *
 * [724] 寻找数组的中心索引
 *
 * https://leetcode-cn.com/problems/find-pivot-index/description/
 *
 * algorithms
 * Easy (38.26%)
 * Likes:    214
 * Dislikes: 0
 * Total Accepted:    56.9K
 * Total Submissions: 148.7K
 * Testcase Example:  '[1,7,3,6,5,6]'
 *
 * 给定一个整数类型的数组 nums，请编写一个能够返回数组 “中心索引” 的方法。
 *
 * 我们是这样定义数组 中心索引 的：数组中心索引的左侧所有元素相加的和等于右侧所有元素相加的和。
 *
 * 如果数组不存在中心索引，那么我们应该返回 -1。如果数组有多个中心索引，那么我们应该返回最靠近左边的那一个。
 *
 *
 *
 * 示例 1：
 *
 * 输入：
 * nums = [1, 7, 3, 6, 5, 6]
 * 输出：3
 * 解释：
 * 索引 3 (nums[3] = 6) 的左侧数之和 (1 + 7 + 3 = 11)，与右侧数之和 (5 + 6 = 11) 相等。
 * 同时, 3 也是第一个符合要求的中心索引。
 *
 *
 * 示例 2：
 *
 * 输入：
 * nums = [1, 2, 3]
 * 输出：-1
 * 解释：
 * 数组中不存在满足此条件的中心索引。
 *
 *
 *
 * 说明：
 *
 *
 * nums 的长度范围为 [0, 10000]。
 * 任何一个 nums[i] 将会是一个范围在 [-1000, 1000]的整数。
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return -1;
        }
        let mut left = vec![0; nums.len()];
        let mut right = vec![0; nums.len()];
        let mut sum = 0;
        for i in 0..nums.len() - 1 {
            sum += nums[i];
            left[i + 1] = sum;
        }
        sum = 0;
        // 其实在这里可以边计算边判断，不需要后边的一个循环
        for i in (1..nums.len()).rev() {
            sum += nums[i];
            right[i - 1] = sum;
        }
        for i in 0..nums.len() {
            if left[i] == right[i] {
                return i as i32;
            }
        }
        return -1;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l724() {
        assert_eq!(3, Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]));
        assert_eq!(-1, Solution::pivot_index(vec![1, 2, 3]));
        assert_eq!(-1, Solution::pivot_index(vec![]));
        assert_eq!(0, Solution::pivot_index(vec![1]));
    }
}
