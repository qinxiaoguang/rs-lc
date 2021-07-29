use super::Solution;

impl Solution {
    // 计算所有点数相加结果的出现概率
    // 最小的点数一定是n,最大的点数一定是6n
    // 需要计算投出n枚和为k的点
    // 需要dp即可完成
    // dp[i][j]表示投i枚色子，总和为j的点
    // 那么dp[i][j] = (dp[i-1][j-1]+..dp[i-1][j-6])即最后一枚投出1~6等情况
    // 而总次数为6^n
    pub fn dices_probability(n: i32) -> Vec<f64> {
        let mut dp = vec![vec![0; 6 * n as usize + 1]; n as usize + 1];
        (1..=n as usize).for_each(|i| dp[i][i] = 1);
        (1..=6).for_each(|i| dp[1][i] = 1);
        (2..=n as usize).for_each(|i| {
            // 从i开始，执行到6i
            (i..=6 * i).for_each(|j| {
                dp[i][j] =
                    (1..=6).fold(0, |sum, n| if j > n { sum + dp[i - 1][j - n] } else { sum });
            });
        });
        let all = 6f64.powf(n as f64);
        let mut res = vec![];
        (n as usize..=6 * n as usize).for_each(|i| res.push(dp[n as usize][i] as f64 / all));
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j060() {
        println!(
            "Solution::dices_probability(3) = {:?}",
            Solution::dices_probability(11)
        );
    }
}
