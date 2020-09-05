pub struct Solution {}
/*
 * @lc app=leetcode.cn id=714 lang=rust
 *
 * [714] 买卖股票的最佳时机含手续费
 *
 * https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/description/
 *
 * algorithms
 * Medium (68.24%)
 * Likes:    239
 * Dislikes: 0
 * Total Accepted:    29.3K
 * Total Submissions: 43K
 * Testcase Example:  '[1,3,2,8,4,9]\n2'
 *
 * 给定一个整数数组 prices，其中第 i 个元素代表了第 i 天的股票价格 ；非负整数 fee 代表了交易股票的手续费用。
 *
 * 你可以无限次地完成交易，但是你每笔交易都需要付手续费。如果你已经购买了一个股票，在卖出它之前你就不能再继续购买股票了。
 *
 * 返回获得利润的最大值。
 *
 * 注意：这里的一笔交易指买入持有并卖出股票的整个过程，每笔交易你只需要为支付一次手续费。
 *
 * 示例 1:
 *
 * 输入: prices = [1, 3, 2, 8, 4, 9], fee = 2
 * 输出: 8
 * 解释: 能够达到的最大利润:
 * 在此处买入 prices[0] = 1
 * 在此处卖出 prices[3] = 8
 * 在此处买入 prices[4] = 4
 * 在此处卖出 prices[5] = 9
 * 总利润: ((8 - 1) - 2) + ((9 - 4) - 2) = 8.
 *
 * 注意:
 *
 *
 * 0 < prices.length <= 50000.
 * 0 < prices[i] < 50000.
 * 0 <= fee < 50000.
 *
 *
 */

// @lc code=start
use std::cmp::max;
impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        // 每一天都只有3个状态,买入，卖出及不动
        // 而每一天前的状态也只有两个状态，当前持有股票/当前未持有股票
        // 所以可以使用数组来表示当前状态
        // dp[i][0][1]表示第i天当前没持有股票，但今天买入了的收益最大值
        // dp[i][0][0]表示当前没股票，且没买入
        // dp[i][1][0]表示当前持有股票，但今天卖出了
        // dp[i][1][1]表示当前持有股票，今天没动
        // 那么此题可以通过更新dp状态，来达到最优值
        // dp[i][0][1] = max(dp[i-1][1][0] -fee - prices[i], dp[i-1][0][0])
        // dp[i][0][0] = max(dp[i-1][0][0],dp[i-1][1][0])
        // dp[i][1][0] = max(dp[i-1][0][1] + price[i], dp[i-1][1][1] + price[i])
        // dp[i][1][1] = max(dp[i][1][1] , dp[i][0][1])
        if prices.len() == 0 {
            return 0;
        }

        let mut dp = vec![vec![vec![0; 2]; 2]; prices.len()];
        dp[0][0][1] = -prices[0] - fee;
        dp[0][0][0] = 0;
        dp[0][1][0] = 0;
        dp[0][1][1] = -prices[0] - fee;
        for i in 1..prices.len() {
            dp[i][0][1] = max(dp[i - 1][1][0], dp[i - 1][0][0]) - fee - prices[i];
            dp[i][0][0] = max(dp[i - 1][0][0], dp[i - 1][1][0]);
            dp[i][1][0] = max(dp[i - 1][0][1], dp[i - 1][1][1]) + prices[i];
            dp[i][1][1] = max(dp[i - 1][1][1], dp[i - 1][0][1]);
        }
        //println!("dp is :{:?}", dp);
        max(dp[prices.len() - 1][1][0], dp[prices.len() - 1][0][0])
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l714() {
        assert_eq!(8, Solution::max_profit(vec![1, 3, 2, 8, 4, 9], 2));
        assert_eq!(0, Solution::max_profit(vec![], 2));
        assert_eq!(1, Solution::max_profit(vec![1, 4], 2));
        assert_eq!(6, Solution::max_profit(vec![1, 3, 7, 5, 10, 3], 3));
    }
}
