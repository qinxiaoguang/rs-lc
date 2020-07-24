pub struct Solution {}

/*
 * @lc app=leetcode.cn id=228 lang=rust
 *
 * [228] 汇总区间
 *
 * https://leetcode-cn.com/problems/summary-ranges/description/
 *
 * algorithms
 * Medium (51.51%)
 * Likes:    53
 * Dislikes: 0
 * Total Accepted:    11.2K
 * Total Submissions: 21.1K
 * Testcase Example:  '[0,1,2,4,5,7]'
 *
 * 给定一个无重复元素的有序整数数组，返回数组区间范围的汇总。
 *
 * 示例 1:
 *
 * 输入: [0,1,2,4,5,7]
 * 输出: ["0->2","4->5","7"]
 * 解释: 0,1,2 可组成一个连续的区间; 4,5 可组成一个连续的区间。
 *
 * 示例 2:
 *
 * 输入: [0,2,3,4,6,8,9]
 * 输出: ["0","2->4","6","8->9"]
 * 解释: 2,3,4 可组成一个连续的区间; 8,9 可组成一个连续的区间。
 *
 */

// @lc code=start
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut res = vec![];
        if nums.len() == 0 {
            return res;
        }
        let (mut l, mut r) = (nums[0], nums[0]);
        let mut nums = nums;
        nums.push(nums[nums.len() - 1]);
        for i in 1..nums.len() {
            if nums[i] == r + 1 {
                r = nums[i];
            } else {
                // 将结果写入
                if l == r {
                    res.push(format!("{}", l));
                } else {
                    res.push(format!("{}->{}", l, r));
                }

                l = nums[i];
                r = nums[i];
            }
        }
        return res;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l228() {
        assert_eq!(
            Solution::summary_ranges(vec![0, 2, 3, 4, 6, 8, 9]),
            vec![
                String::from("0"),
                String::from("2->4"),
                String::from("6"),
                String::from("8->9")
            ]
        );

        assert_eq!(
            Solution::summary_ranges(vec![0, 1, 2, 4, 5, 7]),
            vec![
                String::from("0->2"),
                String::from("4->5"),
                String::from("7"),
            ]
        );

        assert_eq!(Solution::summary_ranges(vec![1]), vec![String::from("1")]);
    }
}
