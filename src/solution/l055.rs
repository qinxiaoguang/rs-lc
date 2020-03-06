pub struct Solution {}

impl Solution {
    // 数组中每个数表示 最多可跳跃的步数
    // 可以利用之前的最小跳跃步数来求解
    // 但是最简单的方法是从后往前遍历，直到遇到一个0，记该位置为j，那么就找前边的i位置，使得j-i<nums[i]即可，
    // 即i位置可以跳过j位置,并继续往前遍历。
    // 原理是，所有位置不为0时，必定可以跳到最终位置，如果有位置为，只要能跳过该位置，即可跳到最终位置
    #![allow(dead_code)]
    pub fn can_jump(nums: Vec<i32>) -> bool {
        if nums.len() == 1 && nums[0] >= 0 {
            return true;
        }
        if Self::jump(nums) <= 0 {
            false
        } else {
            true
        }
    }
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
    fn test_l055() {
        assert_eq!(true, Solution::can_jump(vec![2, 3, 1, 1, 4]));
        assert_eq!(false, Solution::can_jump(vec![3, 2, 1, 0, 4]));
        assert_eq!(false, Solution::can_jump(vec![0]));
    }
}
