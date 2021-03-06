pub struct Solution {}

/*
 * @lc app=leetcode.cn id=123 lang=rust
 *
 * [123] 买卖股票的最佳时机 III
 *
 * https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-iii/description/
 *
 * algorithms
 * Hard (41.71%)
 * Likes:    434
 * Dislikes: 0
 * Total Accepted:    42.2K
 * Total Submissions: 96.8K
 * Testcase Example:  '[3,3,5,0,0,3,1,4]'
 *
 * 给定一个数组，它的第 i 个元素是一支给定的股票在第 i 天的价格。
 *
 * 设计一个算法来计算你所能获取的最大利润。你最多可以完成 两笔 交易。
 *
 * 注意: 你不能同时参与多笔交易（你必须在再次购买前出售掉之前的股票）。
 *
 * 示例 1:
 *
 * 输入: [3,3,5,0,0,3,1,4]
 * 输出: 6
 * 解释: 在第 4 天（股票价格 = 0）的时候买入，在第 6 天（股票价格 = 3）的时候卖出，这笔交易所能获得利润 = 3-0 = 3 。
 * 随后，在第 7 天（股票价格 = 1）的时候买入，在第 8 天 （股票价格 = 4）的时候卖出，这笔交易所能获得利润 = 4-1 = 3 。
 *
 * 示例 2:
 *
 * 输入: [1,2,3,4,5]
 * 输出: 4
 * 解释: 在第 1 天（股票价格 = 1）的时候买入，在第 5 天 （股票价格 = 5）的时候卖出, 这笔交易所能获得利润 = 5-1 = 4
 * 。
 * 注意你不能在第 1 天和第 2 天接连购买股票，之后再将它们卖出。
 * 因为这样属于同时参与了多笔交易，你必须在再次购买前出售掉之前的股票。
 *
 *
 * 示例 3:
 *
 * 输入: [7,6,4,3,1]
 * 输出: 0
 * 解释: 在这个情况下, 没有交易完成, 所以最大利润为 0。
 *
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // 采用暴力+剪枝的方法，依然过不去，还是超时
        // 上网看了下别人的解法，发现是dp
        // dp也考虑过，但是状态转移方程想了半天没想出来怎么做
        // 看了别人解法，原来要穷举状态
        // dp[i][k][1/0] 表示第i天交易了k次，当前持有/不持有股票的最大收益
        // 如dp[5][1][0]: 表示第5天，交易了1次，当前不持有股票的最大收益
        // 那么状态转移方程
        // dp[i][k][0] = max(dp[i-1][k][1] + price[i](把持有的卖了),dp[i-1][k][0](一直不持有))
        // dp[i][k][1] = max(dp[i-1][k-1][0] - price[i](买了今天的),dp[i-1][k][1](一直持有))
        // 该题的k = 2
        use std::cmp::max;
        let i_len = prices.len();
        if i_len == 0 {
            return 0;
        }
        let k_len = 2;
        let mut dp = vec![vec![vec![0; 2]; k_len + 1]; i_len];
        for i in 0..i_len {
            for k in 0..=k_len {
                // 处理i==0时的情况,第一天的情况比较特殊
                if i == 0 {
                    dp[i][k][0] = 0;
                    dp[i][k][1] = if k == 0 { 0 } else { -prices[i] };
                    continue;
                }
                // 处理k==0的情况,k=0时，表示没发生过交易，所以收益都是0
                if k == 0 {
                    dp[i][k][1] = 0;
                    dp[i][k][0] = 0;
                    continue;
                }
                dp[i][k][0] = max(dp[i - 1][k][1] + prices[i], dp[i - 1][k][0]);
                dp[i][k][1] = max(dp[i - 1][k - 1][0] - prices[i], dp[i - 1][k][1]);
            }
        }
        dp[i_len - 1][k_len][0]
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l122() {
        assert_eq!(Solution::max_profit(vec![3, 3, 5, 0, 0, 3, 1, 4]), 6);
        assert_eq!(Solution::max_profit(vec![1, 2, 3, 4, 5]), 4);
        assert_eq!(Solution::max_profit(vec![7, 6, 5, 4, 3, 2, 1]), 0);
        assert_eq!(Solution::max_profit(vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0]), 13);
        assert_eq!(Solution::max_profit(vec![]), 0);
    }
}
