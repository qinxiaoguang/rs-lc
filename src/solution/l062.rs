pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 机器人从左上角往右下角走，每次只能向下或向右走，问走到最后点，有多少种走法
    // 使用dp，dp[i][j] = dp[i-1][j] + dp[i][j-1]
    // 优化方法就是使用一维数组
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        if m == 0 || n == 0 {
            return 1;
        }
        let mut dp = vec![0; n as usize];
        dp[0] = 1;
        for _ in 0..m {
            for j in 0..n {
                if j != 0 {
                    dp[j as usize] = dp[j as usize - 1] + dp[j as usize];
                }
            }
        }
        dp[n as usize - 1]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l062() {
        for i in 0..100 {
            for j in 0..100 {
                println!("{}", Solution::unique_paths(i, j));
            }
        }
    }
}
