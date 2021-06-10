use super::Solution;

impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        // dp题，dp[i][j]表示到当前点的最小路径和
        let n = matrix.len();
        let mut dp = vec![vec![i32::MAX; n]; n];
        for i in 0..n {
            dp[0][i] = matrix[0][i];
        }

        for i in 1..n {
            for j in 0..n {
                dp[i][j] = dp[i][j].min(dp[i - 1][j] + matrix[i][j]);
                if j != 0 {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j - 1] + matrix[i][j]);
                }
                if j != n - 1 {
                    dp[i][j] = dp[i][j].min(dp[i - 1][j + 1] + matrix[i][j]);
                }
            }
        }
        *dp[n - 1].iter().min().unwrap()
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l931() {
        assert_eq!(
            Solution::min_falling_path_sum(matrix![[2, 1, 3], [6, 5, 4], [7, 8, 9]]),
            13
        );
        assert_eq!(
            Solution::min_falling_path_sum(matrix![[-19, 57], [-40, -5]]),
            -59
        );
        assert_eq!(Solution::min_falling_path_sum(matrix![[-48]]), -48);
    }
}
