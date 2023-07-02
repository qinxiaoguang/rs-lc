use std::ops::Index;

use super::Solution;

impl Solution {
    // 给定非负整数n，输出0-n之间每个数字的二进制表示中1的个数
    // dp题，前边可以推后边的
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut dp = vec![0; n as usize + 1];
        dp[0] = 0;
        for i in 1..=n {
            dp[i as usize] = dp[i as usize >> 1] + i % 2;
        }
        dp
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_003() {
        assert_eq!(Solution::count_bits(2), vec![0, 1, 1]);
        assert_eq!(Solution::count_bits(5), vec![0, 1, 1, 2, 1, 2]);
    }
}
