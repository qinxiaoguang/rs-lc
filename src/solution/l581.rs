pub struct Solution {}

/*
 * @lc app=leetcode.cn id=581 lang=rust
 *
 * [581] 最短无序连续子数组
 *
 * https://leetcode-cn.com/problems/shortest-unsorted-continuous-subarray/description/
 *
 * algorithms
 * Easy (34.09%)
 * Likes:    348
 * Dislikes: 0
 * Total Accepted:    32.3K
 * Total Submissions: 92.8K
 * Testcase Example:  '[2,6,4,8,10,9,15]'
 *
 * 给定一个整数数组，你需要寻找一个连续的子数组，如果对这个子数组进行升序排序，那么整个数组都会变为升序排序。
 *
 * 你找到的子数组应是最短的，请输出它的长度。
 *
 * 示例 1:
 *
 *
 * 输入: [2, 6, 4, 8, 10, 9, 15]
 * 输出: 5
 * 解释: 你只需要对 [6, 4, 8, 10, 9] 进行升序排序，那么整个表都会变为升序排序。
 *
 *
 * 说明 :
 *
 *
 * 输入的数组长度范围在 [1, 10,000]。
 * 输入的数组可能包含重复元素 ，所以升序的意思是<=。
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn find_unsorted_subarray(nums: Vec<i32>) -> i32 {
        // 从左侧找到第一个未升序的点
        // 从右侧找到第一个未降序的点
        // 找到两者点的最小值及最大值
        // 若左侧点的值大于最小值，则从左侧找到比最小值小的位置
        // 同理找到右侧点比最大值大的位置

        let mut nums = nums;
        nums.push(std::i32::MAX);
        nums.insert(0, std::i32::MIN);
        let (mut lidx, mut ridx) = (0, 0);
        for i in 1..nums.len() {
            if nums[i] < nums[i - 1] {
                lidx = i - 1;
                break;
            }
        }
        for i in (0..nums.len() - 1).rev() {
            if nums[i] > nums[i + 1] {
                ridx = i + 1;
                break;
            }
        }
        let (mut min, mut max) = (std::i32::MAX, std::i32::MIN);
        for i in lidx..=ridx {
            min = std::cmp::min(min, nums[i]);
            max = std::cmp::max(max, nums[i]);
        }
        for i in (ridx + 1..nums.len()).rev() {
            if nums[i] < max {
                ridx = i;
                break;
            }
        }
        for i in 0..lidx {
            if nums[i] > min {
                lidx = i;
                break;
            }
        }
        let res = ridx as i32 - lidx as i32 + 1;
        if res <= 1 {
            0
        } else {
            res
        }
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l581() {
        assert_eq!(
            5,
            Solution::find_unsorted_subarray(vec![2, 6, 4, 8, 10, 9, 15])
        );
        assert_eq!(0, Solution::find_unsorted_subarray(vec![1, 2, 3, 4, 5]));
        assert_eq!(0, Solution::find_unsorted_subarray(vec![1, 2]));
        assert_eq!(2, Solution::find_unsorted_subarray(vec![1, 2, 4, 3, 5]));
    }
}
