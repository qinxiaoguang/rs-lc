pub struct Solution {}

/*
 * @lc app=leetcode.cn id=611 lang=rust
 *
 * [611] 有效三角形的个数
 *
 * https://leetcode-cn.com/problems/valid-triangle-number/description/
 *
 * algorithms
 * Medium (48.57%)
 * Likes:    107
 * Dislikes: 0
 * Total Accepted:    7.5K
 * Total Submissions: 15.3K
 * Testcase Example:  '[2,2,3,4]'
 *
 * 给定一个包含非负整数的数组，你的任务是统计其中可以组成三角形三条边的三元组个数。
 *
 * 示例 1:
 *
 *
 * 输入: [2,2,3,4]
 * 输出: 3
 * 解释:
 * 有效的组合是:
 * 2,3,4 (使用第一个 2)
 * 2,3,4 (使用第二个 2)
 * 2,2,3
 *
 *
 * 注意:
 *
 *
 * 数组长度不超过1000。
 * 数组里整数的范围为 [0, 1000]。
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        // 先排序，再迭代
        if nums.len() <= 2 {
            return 0;
        }
        let mut nums = nums;
        nums.sort();
        let mut res = 0;
        for i in 0..nums.len() - 2 {
            for j in i + 1..nums.len() - 1 {
                for k in j + 1..nums.len() {
                    if nums[i] + nums[j] > nums[k] && nums[j] - nums[i] < nums[k] {
                        res += 1;
                    } else {
                        break;
                    }
                }
            }
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l611() {
        assert_eq!(3, Solution::triangle_number(vec![2, 2, 3, 4]));
    }
}
