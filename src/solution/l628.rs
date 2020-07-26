pub struct Solution {}
/*
 * @lc app=leetcode.cn id=628 lang=rust
 *
 * [628] 三个数的最大乘积
 *
 * https://leetcode-cn.com/problems/maximum-product-of-three-numbers/description/
 *
 * algorithms
 * Easy (49.35%)
 * Likes:    144
 * Dislikes: 0
 * Total Accepted:    22.4K
 * Total Submissions: 44.6K
 * Testcase Example:  '[1,2,3]'
 *
 * 给定一个整型数组，在数组中找出由三个数组成的最大乘积，并输出这个乘积。
 *
 * 示例 1:
 *
 *
 * 输入: [1,2,3]
 * 输出: 6
 *
 *
 * 示例 2:
 *
 *
 * 输入: [1,2,3,4]
 * 输出: 24
 *
 *
 * 注意:
 *
 *
 * 给定的整型数组长度范围是[3,10^4]，数组中所有的元素范围是[-1000, 1000]。
 * 输入的数组中任意三个数的乘积不会超出32位有符号整数的范围。
 *
 *
 */

// @lc code=start
impl Solution {
    // 其实就是找到3个最大数和2个最小数，组合比较即可
    // 原因就是乘积有负负得正这么一说
    pub fn maximum_product(nums: Vec<i32>) -> i32 {
        if nums.len() < 3 {
            return 0;
        } else if nums.len() == 3 {
            return nums.iter().fold(1, |t, n| t * n);
        }

        let mut nums = nums;
        nums.sort();
        let big = nums.len() as usize - 1;
        let (mid, small) = (big - 1, big - 2);
        return std::cmp::max(
            nums[big] * nums[mid] * nums[small],
            nums[big] * nums[0] * nums[1],
        );
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l628() {
        assert_eq!(Solution::maximum_product(vec![1, 2, 3]), 6);
        assert_eq!(Solution::maximum_product(vec![1, 2, 3, 4]), 24);
    }
}
