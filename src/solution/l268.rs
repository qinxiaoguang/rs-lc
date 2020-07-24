pub struct Solution {}

/*
 * @lc app=leetcode.cn id=268 lang=rust
 *
 * [268] 缺失数字
 *
 * https://leetcode-cn.com/problems/missing-number/description/
 *
 * algorithms
 * Easy (54.59%)
 * Likes:    286
 * Dislikes: 0
 * Total Accepted:    74.7K
 * Total Submissions: 132.9K
 * Testcase Example:  '[3,0,1]'
 *
 * 给定一个包含 0, 1, 2, ..., n 中 n 个数的序列，找出 0 .. n 中没有出现在序列中的那个数。
 *
 *
 *
 * 示例 1:
 *
 * 输入: [3,0,1]
 * 输出: 2
 *
 *
 * 示例 2:
 *
 * 输入: [9,6,4,2,3,5,7,0,1]
 * 输出: 8
 *
 *
 *
 *
 * 说明:
 * 你的算法应具有线性时间复杂度。你能否仅使用额外常数空间来实现?
 *
 */

// @lc code=start
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        // 将正确的数放到正确的位置
        let mut nums = nums;
        let len = nums.len();
        let mut idx = 0usize;
        loop {
            if idx == nums.len() {
                break;
            }
            let next_idx = nums[idx] as usize;
            if next_idx != idx && next_idx < len {
                nums.swap(next_idx, idx);
                continue;
            }
            idx += 1;
        }
        nums.into_iter()
            .enumerate()
            .find(|(idx, num)| *idx as i32 != *num)
            .map(|(idx, _)| idx)
            .unwrap_or(len) as i32
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l268() {
        assert_eq!(2, Solution::missing_number(vec![3, 0, 1]));
        assert_eq!(8, Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]));
    }
}
