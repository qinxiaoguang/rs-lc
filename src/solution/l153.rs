pub struct Solution {}

/*
 * @lc app=leetcode.cn id=153 lang=rust
 *
 * [153] 寻找旋转排序数组中的最小值
 *
 * https://leetcode-cn.com/problems/find-minimum-in-rotated-sorted-array/description/
 *
 * algorithms
 * Medium (50.18%)
 * Likes:    210
 * Dislikes: 0
 * Total Accepted:    59.7K
 * Total Submissions: 116.8K
 * Testcase Example:  '[3,4,5,1,2]'
 *
 * 假设按照升序排序的数组在预先未知的某个点上进行了旋转。
 *
 * ( 例如，数组 [0,1,2,4,5,6,7]  可能变为 [4,5,6,7,0,1,2] )。
 *
 * 请找出其中最小的元素。
 *
 * 你可以假设数组中不存在重复元素。
 *
 * 示例 1:
 *
 * 输入: [3,4,5,1,2]
 * 输出: 1
 *
 * 示例 2:
 *
 * 输入: [4,5,6,7,0,1,2]
 * 输出: 0
 *
 */

// @lc code=start
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        // 二分法
        let (mut start, mut end) = (0usize, nums.len() - 1);
        while start < end {
            let mid_idx = (start + end) / 2;
            let mid_v = nums[mid_idx];
            // 分以下几种情况
            // 1. right <= left && mid >= left 结果在右侧
            // 2. right <= left && mid < left 结果在左侧
            // 3. rigth > left 结果就是nums[left]
            if nums[end] > nums[start] {
                return nums[start];
            } else {
                if mid_v >= nums[start] {
                    // 结果在右侧
                    start = mid_idx + 1;
                } else {
                    end = mid_idx;
                }
            }
        }
        nums[start]
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l153() {
        assert_eq!(Solution::find_min(vec![3, 4, 5, 1, 2]), 1);
        assert_eq!(Solution::find_min(vec![4, 5, 6, 7, 0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![0, 1, 2]), 0);
        assert_eq!(Solution::find_min(vec![2, 0, 1]), 0);
        assert_eq!(Solution::find_min(vec![1, 2, 0]), 0);
        assert_eq!(Solution::find_min(vec![1, 0]), 0);
        assert_eq!(Solution::find_min(vec![0, 1]), 0);
    }
}
