use super::{ListNode, TreeNode};
pub struct Solution {}

/*
 * @lc app=leetcode.cn id=238 lang=rust
 *
 * [238] 除自身以外数组的乘积
 *
 * https://leetcode-cn.com/problems/product-of-array-except-self/description/
 *
 * algorithms
 * Medium (66.87%)
 * Likes:    533
 * Dislikes: 0
 * Total Accepted:    70.2K
 * Total Submissions: 99.7K
 * Testcase Example:  '[1,2,3,4]'
 *
 * 给你一个长度为 n 的整数数组 nums，其中 n > 1，返回输出数组 output ，其中 output[i] 等于 nums 中除 nums[i]
 * 之外其余各元素的乘积。
 *
 *
 *
 * 示例:
 *
 * 输入: [1,2,3,4]
 * 输出: [24,12,8,6]
 *
 *
 *
 * 提示：题目数据保证数组之中任意元素的全部前缀元素和后缀（甚至是整个数组）的乘积都在 32 位整数范围内。
 *
 * 说明: 请不要使用除法，且在 O(n) 时间复杂度内完成此题。
 *
 * 进阶：
 * 你可以在常数空间复杂度内完成这个题目吗？（ 出于对空间复杂度分析的目的，输出数组不被视为额外空间。）
 *
 */

// @lc code=start
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        // 如果是常数空间，那么就不能再额外创建与nums等长的数组了。
        Self::product_except_self_v2(nums)
    }

    // 常数空间的解法
    pub fn product_except_self_v2(nums: Vec<i32>) -> Vec<i32> {
        // 因为输出数组不被视为空间，所以可以创建一个长度为n的数组来辅助得到结果
        let mut res = vec![0; nums.len()];
        // 那么将res这数组当做r数组，先计算出r数组，之后，再通过一个临时变量从左侧开始计算乘积来计算l，并算出l*r得出结果
        for idx in (0..nums.len()).rev() {
            if idx == nums.len() - 1 {
                res[idx] = 1;
                continue;
            }
            res[idx] = res[idx + 1] * nums[idx + 1];
        }
        let mut l = 1;
        for idx in 0..nums.len() {
            res[idx] = l * res[idx];
            l *= nums[idx];
        }
        res
    }

    // 使用O(n)的空间的方法, l[i]为i点左侧所有值的积，r[i]为i点右侧所有值的积
    // 最后求两者的积即可
    pub fn product_except_self_v1(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 0 {
            return vec![];
        }
        if nums.len() == 1 {
            return nums;
        }
        if nums.len() == 2 {
            let mut nums = nums;
            nums.reverse();
            return nums;
        }
        let (mut l, mut r) = (vec![0; nums.len()], vec![0; nums.len()]);

        for idx in 0..nums.len() {
            if idx == 0 {
                l[idx] = 1;
                continue;
            }
            l[idx] = l[idx - 1] * nums[idx - 1]
        }
        for idx in (0..nums.len()).rev() {
            if idx == nums.len() - 1 {
                r[idx] = 1;
                continue;
            }
            r[idx] = r[idx + 1] * nums[idx + 1];
        }
        let mut res = vec![0; nums.len()];
        l.into_iter()
            .zip(r.into_iter())
            .map(|(x, y)| x * y)
            .collect()
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l238() {
        assert_eq!(
            vec![24, 12, 8, 6],
            Solution::product_except_self(vec![1, 2, 3, 4])
        );

        let zero: Vec<i32> = vec![];
        assert_eq!(zero, Solution::product_except_self(vec![]));
        assert_eq!(vec![1], Solution::product_except_self(vec![1]));
        assert_eq!(vec![2, 1], Solution::product_except_self(vec![1, 2]));
    }
}
