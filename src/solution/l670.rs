pub struct Solution {}
/*
 * @lc app=leetcode.cn id=670 lang=rust
 *
 * [670] 最大交换
 *
 * https://leetcode-cn.com/problems/maximum-swap/description/
 *
 * algorithms
 * Medium (41.35%)
 * Likes:    104
 * Dislikes: 0
 * Total Accepted:    8.4K
 * Total Submissions: 20.3K
 * Testcase Example:  '2736'
 *
 * 给定一个非负整数，你至多可以交换一次数字中的任意两位。返回你能得到的最大值。
 *
 * 示例 1 :
 *
 *
 * 输入: 2736
 * 输出: 7236
 * 解释: 交换数字2和数字7。
 *
 *
 * 示例 2 :
 *
 *
 * 输入: 9973
 * 输出: 9973
 * 解释: 不需要交换。
 *
 *
 * 注意:
 *
 *
 * 给定数字的范围是 [0, 10^8]
 *
 *
 */

// @lc code=start
impl Solution {
    // 将第一位和最大的数交换，如果自己就是最大的数，则对后边的数做处理
    pub fn maximum_swap(num: i32) -> i32 {
        let nums: Vec<i32> = num
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap() as i32)
            .collect();
        Self::maximum_swap_array(&nums, 0)
    }

    pub fn maximum_swap_array(nums: &[i32], mut sum: i32) -> i32 {
        if nums.len() == 0 {
            return sum;
        }
        let max = nums.iter().max().unwrap();
        // 无论第一个是不是最大值，都需要将最大值放在第一位，直接加到sum上
        let mut times_len = (nums.len() - 1) as u32;
        let times = 10i32.pow(times_len);
        sum += *max as i32 * times;
        // 第一个数是最大值
        if *max == nums[0] {
            return sum + Self::maximum_swap_array(&nums[1..], 0);
        }
        // 第一个数不是最大值, 但交换需要在最小的最大值位置上交换
        let mut flag = false;
        let mut nums = nums.clone();
        let mut times_len = 0u32;
        for num in nums.into_iter().skip(1).rev() {
            let times = 10i32.pow(times_len);
            sum += if num == max && !flag {
                flag = true;
                nums[0] * times
            } else {
                num * times
            };
            times_len += 1;
        }
        return sum;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l670() {
        assert_eq!(7236, Solution::maximum_swap(2736));
        assert_eq!(9973, Solution::maximum_swap(9973));
        assert_eq!(0, Solution::maximum_swap(0));
        assert_eq!(1, Solution::maximum_swap(1));
        assert_eq!(21, Solution::maximum_swap(12));
        assert_eq!(9999, Solution::maximum_swap(9999));
        assert_eq!(9913, Solution::maximum_swap(1993));
    }
}
