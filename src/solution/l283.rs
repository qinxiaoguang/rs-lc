pub struct Solution {}

/*
 * @lc app=leetcode.cn id=283 lang=rust
 *
 * [283] 移动零
 *
 * https://leetcode-cn.com/problems/move-zeroes/description/
 *
 * algorithms
 * Easy (59.83%)
 * Likes:    656
 * Dislikes: 0
 * Total Accepted:    177K
 * Total Submissions: 286.8K
 * Testcase Example:  '[0,1,0,3,12]'
 *
 * 给定一个数组 nums，编写一个函数将所有 0 移动到数组的末尾，同时保持非零元素的相对顺序。
 *
 * 示例:
 *
 * 输入: [0,1,0,3,12]
 * 输出: [1,3,12,0,0]
 *
 * 说明:
 *
 *
 * 必须在原数组上操作，不能拷贝额外的数组。
 * 尽量减少操作次数。
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        // 注意要点是保持非0元素的相对顺序
        // 暴力解大家都会
        // 所以应该是找一些经典解
        // 经典解就是使用两个指针，第一个指针(0指针)去找0的点，第二个指针(非0指针)去找非0
        // 当找到非0时，就交换两个指针的值，并同时更新0指针,继续上述过程
        let (mut zero, mut not_zero) = (0, 0);
        while zero < nums.len() && not_zero < nums.len() {
            if zero < not_zero {
                nums.swap(zero, not_zero);
                zero += 1;
            }
            not_zero += 1;
            while zero < nums.len() && nums[zero] != 0 {
                zero += 1;
            }
            while not_zero < nums.len() && nums[not_zero] == 0 {
                not_zero += 1;
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l283() {
        let mut nums = vec![0, 1, 0, 3, 12];
        Solution::move_zeroes(&mut nums);
        assert_eq!(vec![1, 3, 12, 0, 0], nums);
    }
}
