use super::Solution;

impl Solution {
    // 很明显的dp题
    // dp[i][j]表示走到该点能拿到的最大值
    // 更新规则则是dp[i][j] = min(dp[i-1][j],dp[i][j-1]) + cost[i][j]
    pub fn max_value_of_j047(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }
        let mut dp = vec![vec![0; grid[0].len()]; grid.len()];
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                dp[i][j] = if i == 0 && j == 0 {
                    0
                } else if j == 0 {
                    dp[i - 1][j]
                } else if i == 0 {
                    dp[i][j - 1]
                } else {
                    dp[i - 1][j].max(dp[i][j - 1])
                } + grid[i][j];
            }
        }
        dp[grid.len() - 1][grid[0].len() - 1]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j047() {
        assert_eq!(
            Solution::max_value_of_j047(matrix![[1, 3, 1], [1, 5, 1], [4, 2, 1]]),
            12
        );
    }
}
