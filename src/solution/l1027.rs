pub struct Solution {}
/*
 * @lc app=leetcode.cn id=1027 lang=rust
 *
 * [1027] 最长等差数列
 *
 * https://leetcode-cn.com/problems/longest-arithmetic-subsequence/description/
 *
 * algorithms
 * Medium (45.25%)
 * Likes:    72
 * Dislikes: 0
 * Total Accepted:    7.2K
 * Total Submissions: 15.8K
 * Testcase Example:  '[3,6,9,12]'
 *
 * 给定一个整数数组 A，返回 A 中最长等差子序列的长度。
 *
 * 回想一下，A 的子序列是列表 A[i_1], A[i_2], ..., A[i_k] 其中 0 <= i_1 < i_2 < ... < i_k <=
 * A.length - 1。并且如果 B[i+1] - B[i]( 0 <= i < B.length - 1) 的值都相同，那么序列 B
 * 是等差的。
 *
 *
 *
 * 示例 1：
 *
 * 输入：[3,6,9,12]
 * 输出：4
 * 解释：
 * 整个数组是公差为 3 的等差数列。
 *
 *
 * 示例 2：
 *
 * 输入：[9,4,7,2,10]
 * 输出：3
 * 解释：
 * 最长的等差子序列是 [4,7,10]。
 *
 *
 * 示例 3：
 *
 * 输入：[20,1,15,3,10,5,8]
 * 输出：4
 * 解释：
 * 最长的等差子序列是 [20,15,10,5]。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 2 <= A.length <= 2000
 * 0 <= A[i] <= 10000
 *
 *
 */

// @lc code=start
impl Solution {
    // l673是计算最长递增子序列的个数
    // 如果化简一下，求最长递增子序列的长度，就比较好求了
    // 使用一层dp[]，两层循环可解，dp[i]就是以i为结尾的最长递增子序列的长度
    // 那么对于dp[j],则需要遍历dp[0~j]来更新j
    // 而求最长递增子序列的个数，则是在求dp的同时，维护一个count数组，count[i]表示与lenght[i]相同的子序列个数
    // 其中length[i]就是以i为结尾的最长递增子序列的长度
    //
    // 而该题则是求最长等差序列
    // 则定义二维数组dp[i][diff]表示以i为结尾的，差值为diff的最长子序列的长度
    // 同样是二层循环来求dp[i][diff]
    // 因为diff有负值，而负值最大为10000，所以可以将其转为正值
    pub fn longest_arith_seq_length(a: Vec<i32>) -> i32 {
        if a.len() <= 1 {
            return a.len() as i32;
        }

        // 初始值都为1， 任何数本身就是等差序列，可以以任何差值来等差
        let mut dp = vec![vec![1; 20001]; a.len()];
        let mut res = 0;
        for i in 0..a.len() {
            for j in 0..i as usize {
                // 从0..i中来计算i的值
                let dif = (a[i] - a[j] + 10000) as usize; // 差值
                dp[i][dif] = std::cmp::max(dp[i][dif], dp[j][dif] + 1);
                res = std::cmp::max(res, dp[i][dif]);
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
    fn test_l1027() {
        assert_eq!(
            4,
            Solution::longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8])
        );

        assert_eq!(3, Solution::longest_arith_seq_length(vec![9, 4, 7, 2, 10]));
    }
}
