pub struct Solution {}
/*
 * @lc app=leetcode.cn id=790 lang=rust
 *
 * [790] 多米诺和托米诺平铺
 *
 * https://leetcode-cn.com/problems/domino-and-tromino-tiling/description/
 *
 * algorithms
 * Medium (42.30%)
 * Likes:    61
 * Dislikes: 0
 * Total Accepted:    2.1K
 * Total Submissions: 5K
 * Testcase Example:  '3'
 *
 * 有两种形状的瓷砖：一种是 2x1 的多米诺形，另一种是形如 "L" 的托米诺形。两种形状都可以旋转。
 *
 *
 * XX  <- 多米诺
 *
 * XX  <- "L" 托米诺
 * X
 *
 *
 * 给定 N 的值，有多少种方法可以平铺 2 x N 的面板？返回值 mod 10^9 + 7。
 *
 * （平铺指的是每个正方形都必须有瓷砖覆盖。两个平铺不同，当且仅当面板上有四个方向上的相邻单元中的两个，使得恰好有一个平铺有一个瓷砖占据两个正方形。）
 *
 *
 * 示例:
 * 输入: 3
 * 输出: 5
 * 解释:
 * 下面列出了五种不同的方法，不同字母代表不同瓷砖：
 * XYZ XXZ XYY XXY XYY
 * XYZ YYZ XZZ XYY XXY
 *
 * 提示：
 *
 *
 * N  的范围是 [1, 1000]
 *
 *
 *
 *
 */

// @lc code=start
impl Solution {
    // 一般来说这种题是dp题，但是咋dp呢= =
    // 可以列举状态
    // 当前尾部铺满 dp[n][0]
    // 当前尾部上部未铺满 dp[n][1]
    // 当前尾部下部未铺满 dp[n][2]
    // 那么状态转移为
    // dp[n][0] = dp[n-1][0] + dp[n-2][0] + dp[n-1][1]+dp[n-1][2]
    // dp[n][1] = dp[n-2][0] + dp[n-1][2]
    // dp[n][2] = dp[n-2][0] + dp[n-1][1]
    pub fn num_tilings(n: i32) -> i32 {
        if n == 1 {
            return 1;
        }
        if n == 2 {
            return 2;
        }
        let mut dp = vec![vec![0i64; 3]; n as usize + 1];
        dp[0][0] = 0;
        dp[0][1] = 0;
        dp[0][2] = 0;
        dp[1][0] = 1;
        dp[1][1] = 0;
        dp[1][2] = 0;
        dp[2][0] = 2;
        dp[2][1] = 1;
        dp[2][2] = 1;
        for i in 3..=n as usize {
            dp[i][0] = Self::mod_v(vec![dp[i - 1][0], dp[i - 2][0], dp[i - 1][1], dp[i - 1][2]]);
            dp[i][1] = Self::mod_v(vec![dp[i - 2][0], dp[i - 1][2]]);
            dp[i][2] = Self::mod_v(vec![dp[i - 2][0], dp[i - 1][1]]);
        }
        return dp[n as usize][0] as i32;
    }

    fn mod_v(vs: Vec<i64>) -> i64 {
        let m = 10i64.pow(9) + 7;
        vs.into_iter().map(|v| v).sum::<i64>() % m
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l790() {
        assert_eq!(5, Solution::num_tilings(3));
        assert_eq!(312342182, Solution::num_tilings(30));
    }
}
