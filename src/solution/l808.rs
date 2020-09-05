pub struct Solution {}
/*
 * @lc app=leetcode.cn id=808 lang=rust
 *
 * [808] 分汤
 *
 * https://leetcode-cn.com/problems/soup-servings/description/
 *
 * algorithms
 * Medium (45.33%)
 * Likes:    40
 * Dislikes: 0
 * Total Accepted:    2.8K
 * Total Submissions: 6.1K
 * Testcase Example:  '50'
 *
 * 有 A 和 B 两种类型的汤。一开始每种类型的汤有 N 毫升。有四种分配操作：
 *
 *
 * 提供 100ml 的汤A 和 0ml 的汤B。
 * 提供 75ml 的汤A 和 25ml 的汤B。
 * 提供 50ml 的汤A 和 50ml 的汤B。
 * 提供 25ml 的汤A 和 75ml 的汤B。
 *
 *
 *
 * 当我们把汤分配给某人之后，汤就没有了。每个回合，我们将从四种概率同为0.25的操作中进行分配选择。如果汤的剩余量不足以完成某次操作，我们将尽可能分配。当两种类型的汤都分配完时，停止操作。
 *
 * 注意不存在先分配100 ml汤B的操作。
 *
 * 需要返回的值： 汤A先分配完的概率 + 汤A和汤B同时分配完的概率 / 2。
 *
 *
 * 示例:
 * 输入: N = 50
 * 输出: 0.625
 * 解释:
 * 如果我们选择前两个操作，A将首先变为空。对于第三个操作，A和B会同时变为空。对于第四个操作，B将首先变为空。
 * 所以A变为空的总概率加上A和B同时变为空的概率的一半是 0.25 *(1 + 1 + 0.5 + 0)= 0.625。
 *
 *
 * 注释:
 *
 *
 * 0 <= N <= 10^9。
 *
 * 返回值在 10^-6 的范围将被认为是正确的。
 *
 *
 *
 */

// @lc code=start
impl Solution {
    // 一开始想的是这个题应该可以用dp来解
    // 但是看到n的取值范围就慌了，太大了，不太好取。
    // 为什么别人就知道题目怎么化简呢。
    // 因为对于N以及ab的取法都是25的倍数，所以可以将所有值都除以25，不足25的补1
    // 那么此题就化简了许多。
    // 那么上述四种ab两种取汤就变成了(4,0)(3,1)(2,2)(1,3)
    // 设dp[i][j]为a剩余i,b剩j的概率
    // dp[N][N] = 1
    // 则dp[i][j] = 0.25*(dp[i+4][j]+dp[i+3][j+1]+dp[i+2][j+2] +dp[i+1][j+3])
    // 注意i,j 取值范围，若不在0~N，则dp[i][j]=0
    // 但是注意的一点是，当N>500＊25的时候，直接返回1，否则会超时，至于为什么
    // 我就纳闷谁tm能想到n>500＊25的时候直接返回1啊。。真的是
    pub fn soup_servings(n: i32) -> f64 {
        1f64
    }

    fn get_sum_rate(dp: &Vec<Vec<f64>>, i: usize, j: usize) -> f64 {
        0.25f64
            * (Self::get_rate(dp, i + 4, j)
                + Self::get_rate(dp, i + 3, j + 1)
                + Self::get_rate(dp, i + 2, j + 2)
                + Self::get_rate(dp, i + 1, j + 3)) as f64
    }

    fn get_rate(dp: &Vec<Vec<f64>>, i: usize, j: usize) -> f64 {
        if i >= dp.len() || j >= dp.len() {
            return 0f64;
        }
        return dp[i][j];
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l808() {
        assert_eq!(0.625, Solution::soup_servings(50));
    }
}
