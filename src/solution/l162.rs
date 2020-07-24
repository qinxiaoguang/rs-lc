pub struct Solution {}

/*
 * @lc app=leetcode.cn id=162 lang=rust
 *
 * [162] 寻找峰值
 *
 * https://leetcode-cn.com/problems/find-peak-element/description/
 *
 * algorithms
 * Medium (45.09%)
 * Likes:    248
 * Dislikes: 0
 * Total Accepted:    48.7K
 * Total Submissions: 103.8K
 * Testcase Example:  '[1,2,3,1]'
 *
 * 峰值元素是指其值大于左右相邻值的元素。
 *
 * 给定一个输入数组 nums，其中 nums[i] ≠ nums[i+1]，找到峰值元素并返回其索引。
 *
 * 数组可能包含多个峰值，在这种情况下，返回任何一个峰值所在位置即可。
 *
 * 你可以假设 nums[-1] = nums[n] = -∞。
 *
 * 示例 1:
 *
 * 输入: nums = [1,2,3,1]
 * 输出: 2
 * 解释: 3 是峰值元素，你的函数应该返回其索引 2。
 *
 * 示例 2:
 *
 * 输入: nums = [1,2,1,3,5,6,4]
 * 输出: 1 或 5
 * 解释: 你的函数可以返回索引 1，其峰值元素为 2；
 * 或者返回索引 5， 其峰值元素为 6。
 *
 *
 * 说明:
 *
 * 你的解法应该是 O(logN) 时间复杂度的。
 *
 */

// @lc code=start
impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        // 因为是logN的时间复杂度，所以需要二分法来解
        // 问题是怎么个二分
        // 我tm裂开，看了官方解答，发现是如此简单
        // 只需要迭代，每次选择两个数中的大的那一方,最终一定能找到答案
        // 因为大的那一方的另一边一定会大于边界值，所以必然会有峰值
        // 假设数组中 An > An-1 那么可以假设小的值即An-1为负无穷，那么结果一定在A[0-n]中
        // 也可以用数学归纳法找到其中的规律
        let (mut l, mut r) = (0, nums.len() - 1);
        while l < r {
            let mid = (l + r) / 2;
            if nums[mid] > nums[mid + 1] {
                r = mid;
            } else {
                l = mid + 1;
            }
        }
        return l as i32;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l162() {
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3, 1]), 2);
        assert_eq!(Solution::find_peak_element(vec![1]), 0);
        assert_eq!(Solution::find_peak_element(vec![1, 2]), 1);
        assert_eq!(Solution::find_peak_element(vec![2, 1]), 0);
        assert_eq!(Solution::find_peak_element(vec![1, 2, 3]), 2);
        assert_eq!(Solution::find_peak_element(vec![1, 3, 2]), 1);
    }
}
