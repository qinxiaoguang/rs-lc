pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 机器人从左上角往右下角走，每次只能向下或向右走，问走到最后点，有多少种走法
    // 使用dp，dp[i][j] = dp[i-1][j] + dp[i][j-1]
    // 但是此题有障碍物
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        if obstacle_grid.is_empty() {
            return 1;
        }
        let (m, n) = (obstacle_grid.len(), obstacle_grid[0].len());
        if n == 0 {
            return 1;
        }
        let mut dp = vec![0; n as usize];
        dp[0] = 1;
        for i in 0..m as usize {
            for j in 0..n as usize {
                if obstacle_grid[i][j] == 1 {
                    dp[j] = 0;
                    continue;
                }
                dp[j] = if j > 0 && obstacle_grid[i][j - 1] == 0 {
                    dp[j - 1]
                } else {
                    0
                } + if (i > 0 && obstacle_grid[i - 1][j] == 0) || i == 0 {
                    dp[j]
                } else {
                    0
                };
            }
        }
        dp[n as usize - 1]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l063() {
        println!(
            "{}",
            Solution::unique_paths_with_obstacles(vec![
                vec![0, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 0]
            ])
        );
    }
}
