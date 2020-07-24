pub struct Solution {}

/*
 * @lc app=leetcode.cn id=154 lang=rust
 *
 * [154] 寻找旋转排序数组中的最小值 II
 *
 * https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array-ii/description/
 *
 * algorithms
 * Hard (46.84%)
 * Likes:    128
 * Dislikes: 0
 * Total Accepted:    26.9K
 * Total Submissions: 55.3K
 * Testcase Example:  '[1,3,5]'
 *
 * 假设按照升序排序的数组在预先未知的某个点上进行了旋转。
 *
 * ( 例如，数组 [0,1,2,4,5,6,7]  可能变为 [4,5,6,7,0,1,2] )。
 *
 * 请找出其中最小的元素。
 *
 * 注意数组中可能存在重复的元素。
 *
 * 示例 1：
 *
 * 输入: [1,3,5]
 * 输出: 1
 *
 * 示例 2：
 *
 * 输入: [2,2,2,0,1]
 * 输出: 0
 *
 * 说明：
 *
 *
 * 这道题是 寻找旋转排序数组中的最小值 的延伸题目。
 * 允许重复会影响算法的时间复杂度吗？会如何影响，为什么？
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        Self::find_min_v2(&nums)
    }

    pub fn find_min_v2(nums: &[i32]) -> i32 {
        // 二分法
        let (mut start, mut end) = (0usize, nums.len() - 1);
        if nums.len() == 1 {
            return nums[0];
        }
        if nums.len() == 2 {
            return std::cmp::min(nums[0], nums[1]);
        }
        let mid_idx = (start + end) / 2;
        let mid_v = nums[mid_idx];
        // 分以下几种情况
        // 1. right <= left && mid >= left 结果在右侧
        // 2. right <= left && mid < left 结果在左侧
        // 3. rigth > left 结果就是nums[left]

        if nums[end] > nums[start] {
            return nums[start];
        } else {
            if mid_v > nums[start] {
                // 结果在右侧
                return Self::find_min_v2(&nums[mid_idx + 1..]);
            } else {
                if mid_v == nums[start] && mid_v == nums[end] && mid_idx < nums.len() {
                    // 与上题唯一不同的是
                    // 这种情况特殊处理，无法知道结果在右边还是左边
                    // 直接递归调用自己，找到最小值返回即可
                    return std::cmp::min(
                        Self::find_min_v2(&nums[start..mid_idx + 1]),
                        Self::find_min_v2(&nums[mid_idx + 1..]),
                    );
                }
                if mid_v >= nums[start] {
                    // 结果在右侧
                    return Self::find_min_v2(&nums[mid_idx + 1..]);
                } else {
                    return Self::find_min_v2(&nums[..mid_idx + 1]);
                }
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l154() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![2, 0, 1]), 0);
        assert_eq!(Solution::find_min(vec![1, 2, 0]), 0);
        assert_eq!(Solution::find_min(vec![1, 0]), 0);
        assert_eq!(Solution::find_min(vec![0, 1]), 0);
        assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
        assert_eq!(Solution::find_min(vec![2, 2, 2, 2, 1]), 1);
        assert_eq!(Solution::find_min(vec![2, 2, 2]), 2);
        assert_eq!(Solution::find_min(vec![2, 2, 1, 2, 2]), 1);
        assert_eq!(Solution::find_min(vec![10, 1, 10, 10, 10]), 1);
    }
}
