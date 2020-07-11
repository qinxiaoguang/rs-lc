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
    pub fn max_profit_v2(prices: &[i32]) -> i32 {
        // l[i]表示第i个结点左边的最小值，r[i]表示i结点右边的最大值
        // 最后找到r[i] - l[i] 的最大值
        let mut res = 0;
        for i in 0..prices.len() {
            res = std::cmp::max(r[i as usize] - l[i as usize], res);
        }
        res
    }
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // 与121题类似，将当前价格以i为临界点分为两部分 l[i],r[i]
        // 分别计算两个数组的最大收益，最后遍历一遍得到最终结果

        let mut res = 0;
        let mut prices = prices;
        prices.push(-1);

        let (mut l, mut r) = (vec![0; prices.len()], vec![0; prices.len()]);
        // 先从左往右遍历，更新l
        let mut min = std::i32::MAX;
        for (x, &val) in prices.iter().enumerate() {
            min = std::cmp::min(min, val);
            l[x] = min;
        }
        // 更新r
        let mut max = std::i32::MIN;
        for (x, &val) in prices.iter().enumerate().rev() {
            max = std::cmp::max(max, val);
            r[x] = max;
        }

        for i in 0..prices.len() {
            let i = i as usize;
            res = std::cmp::max(
                Self::max_profit_v2(&prices[0..i]) + Self::max_profit_v2(&prices[i..]),
                res,
            );
        }
        res
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
    }
}
