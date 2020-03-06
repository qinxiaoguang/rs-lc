pub struct Solution {}

impl Solution {
    // 数组中每个数表示 最多可跳跃的步数，求跳到最后一个位置，最小步数是多少
    #![allow(dead_code)]
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut dp = vec![std::i32::MAX - 1; nums.len()];
        if nums.is_empty() {
            return 0;
        }
        dp[0] = 0;
        for i in 0..nums.len() {
            if nums[i] == 0 {
                continue;
            }
            for j in 1..=nums[i] {
                let index = i + j as usize;
                if index >= nums.len() {
                    break;
                }
                dp[index] = std::cmp::min(dp[index], dp[i] + 1);
            }
        }
        if dp[nums.len() - 1] == std::i32::MAX - 1 {
            return 0;
        }
        dp[nums.len() - 1]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l045() {
        assert_eq!(2, Solution::jump(vec![2, 3, 1, 1, 4]));
        assert_eq!(0, Solution::jump(vec![0, 3, 1, 1, 4]));
        assert_eq!(0, Solution::jump(vec![0]));
    }
}
