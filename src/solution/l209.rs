pub struct Solution {}

/*
 * @lc app=leetcode.cn id=209 lang=rust
 *
 * [209] 长度最小的子数组
 *
 * https://leetcode-cn.com/problems/minimum-size-subarray-sum/description/
 *
 * algorithms
 * Medium (41.57%)
 * Likes:    380
 * Dislikes: 0
 * Total Accepted:    75.1K
 * Total Submissions: 169.2K
 * Testcase Example:  '7\n[2,3,1,2,4,3]'
 *
 * 给定一个含有 n 个正整数的数组和一个正整数 s ，找出该数组中满足其和 ≥ s 的长度最小的 连续
 * 子数组，并返回其长度。如果不存在符合条件的子数组，返回 0。
 *
 *
 *
 * 示例：
 *
 * 输入：s = 7, nums = [2,3,1,2,4,3]
 * 输出：2
 * 解释：子数组 [4,3] 是该条件下的长度最小的子数组。
 *
 *
 *
 *
 * 进阶：
 *
 *
 * 如果你已经完成了 O(n) 时间复杂度的解法, 请尝试 O(n log n) 时间复杂度的解法。
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn min_sub_array_len(s: i32, nums: Vec<i32>) -> i32 {
        // 尺取法
        let mut min_res = std::i32::MAX;
        if nums.len() == 0 {
            return 0;
        }
        let mut v = nums[0];
        let (mut l, mut r) = (0, 0);
        loop {
            //println!("l:{},r:{}, min_res:{},v:{}", l, r, min_res, v);
            if v >= s {
                min_res = std::cmp::min(min_res, r - l + 1);
                v -= nums[l as usize];
                l += 1;
                if l >= nums.len() as i32 {
                    break;
                }
            } else {
                r += 1;
                if r >= nums.len() as i32 {
                    break;
                }
                v += nums[r as usize];
            }
        }
        if min_res == std::i32::MAX {
            0
        } else {
            min_res
        }
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l209() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }
}
