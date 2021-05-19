pub struct Solution {}
/*
 * @lc app=leetcode.cn id=879 lang=rust
 *
 * [879] 盈利计划
 *
 * https://leetcode-cn.com/problems/profitable-schemes/description/
 *
 * algorithms
 * Hard (36.13%)
 * Likes:    34
 * Dislikes: 0
 * Total Accepted:    1.3K
 * Total Submissions: 3.7K
 * Testcase Example:  '5\n3\n[2,2]\n[2,3]'
 *
 * 帮派里有 G 名成员，他们可能犯下各种各样的罪行。
 *
 * 第 i 种犯罪会产生 profit[i] 的利润，它要求 group[i] 名成员共同参与。
 *
 * 让我们把这些犯罪的任何子集称为盈利计划，该计划至少产生 P 的利润。
 *
 * 有多少种方案可以选择？因为答案很大，所以返回它模 10^9 + 7 的值。
 *
 *
 *
 * 示例 1：
 *
 * 输入：G = 5, P = 3, group = [2,2], profit = [2,3]
 * 输出：2
 * 解释：
 * 至少产生 3 的利润，该帮派可以犯下罪 0 和罪 1 ，或仅犯下罪 1 。
 * 总的来说，有两种方案。
 *
 *
 * 示例 2:
 *
 * 输入：G = 10, P = 5, group = [2,3,5], profit = [6,7,8]
 * 输出：7
 * 解释：
 * 至少产生 5 的利润，只要他们犯其中一种罪就行，所以该帮派可以犯下任何罪行 。
 * 有 7 种可能的计划：(0)，(1)，(2)，(0,1)，(0,2)，(1,2)，以及 (0,1,2) 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= G <= 100
 * 0 <= P <= 100
 * 1 <= group[i] <= 100
 * 0 <= profit[i] <= 100
 * 1 <= group.length = profit.length <= 100
 *
 *
 *
 *
 */

// @lc code=start
impl Solution {
    // 暴力法可解
    // 动态规划可解
    // 但是为什么动态规划可解，刚开始也考虑dp来解问题，但是又觉得子问题又无法递推出后面问题的解
    // dp[i][j][k]表示对于前i个profit,人数有j个时(注意是严格的j个)，至少获取的利润有k的个数
    // 那么dp的递推公式为 dp[i][j][k] = dp[i-1][j][k](当前组不选) + dp[i-1][j-group[i-1]][k-profit[i-1]] (当前组选)
    pub fn profitable_schemes(g: i32, p: i32, group: Vec<i32>, profit: Vec<i32>) -> i32 {
        let m = 10i32.pow(9) + 7;
        let mut dp = vec![vec![vec![0; p as usize + 1]; g as usize + 1]; profit.len() + 1];

        for i in 0..=profit.len() {
            for j in 0..=g as usize {
                dp[i][j][0] = 1;
            }
        }
        for i in 1..=profit.len() {
            let tmp_g = group[i - 1] as usize;
            let tmp_p = profit[i - 1] as usize;
            for j in 1..=g as usize {
                for k in 0..=p as usize {
                    dp[i][j][k] = dp[i - 1][j][k]
                        + if j >= tmp_g {
                            dp[i - 1][j - tmp_g][if k >= tmp_p { k - tmp_p } else { 0usize }]
                        } else {
                            0
                        };
                    dp[i][j][k] %= m;
                }
            }
        }
        dp[profit.len()][g as usize][p as usize]
    }
}
// @lc code=end
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l879() {
        assert_eq!(
            2,
            Solution::profitable_schemes(5, 3, vec![2, 2], vec![2, 3])
        );
        assert_eq!(
            7,
            Solution::profitable_schemes(10, 5, vec![2, 3, 5], vec![6, 7, 8])
        );
    }
}
