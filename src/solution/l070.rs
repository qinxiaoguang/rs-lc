pub struct Solution {}

impl Solution {
    #[allow(dead_code)]
    // 经典题，爬楼梯，一次只能爬1或2
    // 其实就是斐波那qi数列
    pub fn climb_stairs(n: i32) -> i32 {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 1;
        dp[1] = 1;
        if n <= 1 {
            return dp[n as usize];
        }
        for i in 2..n as usize + 1 {
            dp[i] = dp[i - 1] + dp[i - 2];
        }
        dp[n as usize]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l070() {
        println!("{}", Solution::climb_stairs(3));
    }
}
