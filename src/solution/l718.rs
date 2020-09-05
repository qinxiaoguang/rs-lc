pub struct Solution {}
/*
 * @lc app=leetcode.cn id=718 lang=rust
 *
 * [718] 最长重复子数组
 *
 * https://leetcode-cn.com/problems/maximum-length-of-repeated-subarray/description/
 *
 * algorithms
 * Medium (54.13%)
 * Likes:    308
 * Dislikes: 0
 * Total Accepted:    42.7K
 * Total Submissions: 78.9K
 * Testcase Example:  '[1,2,3,2,1]\n[3,2,1,4,7]'
 *
 * 给两个整数数组 A 和 B ，返回两个数组中公共的、长度最长的子数组的长度。
 *
 *
 *
 * 示例：
 *
 * 输入：
 * A: [1,2,3,2,1]
 * B: [3,2,1,4,7]
 * 输出：3
 * 解释：
 * 长度最长的公共子数组是 [3, 2, 1] 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= len(A), len(B) <= 1000
 * 0 <= A[i], B[i] < 100
 *
 *
 */

// @lc code=start
impl Solution {
    // dp[i][j]表示以a[i]b[j]结尾的公共长最大的重复数组
    pub fn find_length(a: Vec<i32>, b: Vec<i32>) -> i32 {
        if a.len() == 0 || b.len() == 0 {
            return 0;
        }
        let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
        for i in 0..=a.len() {
            dp[i][0] = 0;
        }
        for j in 0..=b.len() {
            dp[0][j] = 0;
        }
        let mut max = 0;
        for i in 1..=a.len() {
            for j in 1..=b.len() {
                // 若a[i]==b[j], dp[i-1][j-1] +1
                // 若a[i]!=b[j], 则dp[i][j] = 0
                // 你看看这个公式，跟i还有什么关系?
                // 所以其实可以优化成一维dp。 自己动手吧
                dp[i][j] = if a[i - 1] == b[j - 1] {
                    dp[i - 1][j - 1] + 1
                } else {
                    0
                };
                max = std::cmp::max(dp[i][j], max);
            }
        }
        max
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l718() {
        assert_eq!(
            3,
            Solution::find_length(vec![1, 2, 3, 2, 1], vec![3, 2, 1, 4, 7])
        );
    }
}
