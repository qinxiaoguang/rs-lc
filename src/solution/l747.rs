pub struct Solution {}
/*
 * @lc app=leetcode.cn id=747 lang=rust
 *
 * [747] 至少是其他数字两倍的最大数
 *
 * https://leetcode-cn.com/problems/largest-number-at-least-twice-of-others/description/
 *
 * algorithms
 * Easy (39.73%)
 * Likes:    63
 * Dislikes: 0
 * Total Accepted:    34.5K
 * Total Submissions: 86.8K
 * Testcase Example:  '[0,0,0,1]'
 *
 * 在一个给定的数组nums中，总是存在一个最大元素 。
 *
 * 查找数组中的最大元素是否至少是数组中每个其他数字的两倍。
 *
 * 如果是，则返回最大元素的索引，否则返回-1。
 *
 * 示例 1:
 *
 * 输入: nums = [3, 6, 1, 0]
 * 输出: 1
 * 解释: 6是最大的整数, 对于数组中的其他整数,
 * 6大于数组中其他元素的两倍。6的索引是1, 所以我们返回1.
 *
 *
 *
 *
 * 示例 2:
 *
 * 输入: nums = [1, 2, 3, 4]
 * 输出: -1
 * 解释: 4没有超过3的两倍大, 所以我们返回 -1.
 *
 *
 *
 *
 * 提示:
 *
 *
 * nums 的长度范围在[1, 50].
 * 每个 nums[i] 的整数范围在 [0, 100].
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn dominant_index(nums: Vec<i32>) -> i32 {
        let mut max_idx = -1;
        let mut max_num = 0i32;

        for i in 0..nums.len() {
            if nums[i] >= max_num {
                max_idx = if nums[i] as i64 >= max_num as i64 * 2 {
                    i as i32
                } else {
                    -1
                };
                max_num = nums[i];
            } else {
                if nums[i] * 2 > max_num {
                    max_idx = -1;
                }
            }
        }
        max_idx
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l747() {
        assert_eq!(-1, Solution::dominant_index(vec![1, 2, 3, 4]));
        assert_eq!(1, Solution::dominant_index(vec![3, 6, 1, 0]));
        assert_eq!(-1, Solution::dominant_index(vec![]));
        assert_eq!(0, Solution::dominant_index(vec![1]));
        assert_eq!(0, Solution::dominant_index(vec![0]));
        assert_eq!(1, Solution::dominant_index(vec![0, 0]));
        assert_eq!(-1, Solution::dominant_index(vec![6, 6]));
        assert_eq!(-1, Solution::dominant_index(vec![0, 0, 3, 2]));
    }
}