pub struct Solution {}

/*
 * @lc app=leetcode.cn id=121 lang=rust
 *
 * [121] 买卖股票的最佳时机
 *
 * https://leetcode-cn.com/problems/best-time-to-buy-and-sell-stock/description/
 *
 * algorithms
 * Easy (53.60%)
 * Likes:    1062
 * Dislikes: 0
 * Total Accepted:    234.9K
 * Total Submissions: 431.7K
 * Testcase Example:  '[7,1,5,3,6,4]'
 *
 * 给定一个数组，它的第 i 个元素是一支给定股票第 i 天的价格。
 *
 * 如果你最多只允许完成一笔交易（即买入和卖出一支股票一次），设计一个算法来计算你所能获取的最大利润。
 *
 * 注意：你不能在买入股票前卖出股票。
 *
 *
 *
 * 示例 1:
 *
 * 输入: [7,1,5,3,6,4]
 * 输出: 5
 * 解释: 在第 2 天（股票价格 = 1）的时候买入，在第 5 天（股票价格 = 6）的时候卖出，最大利润 = 6-1 = 5 。
 * ⁠    注意利润不能是 7-1 = 6, 因为卖出价格需要大于买入价格；同时，你不能在买入前卖出股票。
 *
 *
 * 示例 2:
 *
 * 输入: [7,6,4,3,1]
 * 输出: 0
 * 解释: 在这种情况下, 没有交易完成, 所以最大利润为 0。
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        // l[i]表示第i个结点左边的最小值，r[i]表示i结点右边的最大值
        // 最后找到r[i] - l[i] 的最大值
        let (mut l, mut r) = (vec![0; prices.len()], vec![0; prices.len()]);
        // 先从左往右遍历，更新l
        let mut min = std::i32::MAX;
        for (x, &val) in prices.iter().enumerate() {
            min = std::cmp::min(min, val);
            l[x] = min;
        }
        let mut max = std::i32::MIN;
        for (x, &val) in prices.iter().enumerate().rev() {
            max = std::cmp::max(max, val);
            r[x] = max;
        }

        let mut res = 0;
        for i in 0..prices.len() {
            res = std::cmp::max(r[i as usize] - l[i as usize], res);
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l121() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
        assert_eq!(Solution::max_profit(vec![-1]), 0);
    }
}
