use super::Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        // 前缀和，left[i] - left[j]即为i-j的子数组的和
        // 而(left[i] - left[j]) %k == 0,可得left[i]%k == left[j]%k
        // 所以只要统计left[i]%k的个数n,并计算C(n,2),最后sum即可求得解
        // 所以遇到连续子串，要想到前缀和，取模则想到 i%k== j%k这个公式
        let mut m = vec![0; k as usize];
        let (mut sum, mut res) = (0, 0);
        m[0] = 1;
        nums.iter().for_each(|&v| {
            sum += v;
            m[((sum % k + k) % k) as usize] += 1
        });
        m.iter().for_each(|&v| res += (v * (v - 1)) / 2);
        res
    }

    // bad bad ,考虑成了所有子数组被k整除的个数,所以才用了dp
    pub fn bad(nums: Vec<i32>, k: i32) -> i32 {
        // 首先数组中每个数都要取模，得到由0-k组成的数组
        // 假设dp[i] 表示当前前i个数中，有多少个子串能被k整除
        // 那么dp[i+1]的递推式为，当nums[i+1]可以被k整除时，dp[i+1] = 2*dp[i] +1
        // 当nums[i+1]不能被k整除时， dp[i+1] = dp[i] + map[k-nums[i+1]]
        // 每次要拿nums[i+1]去遍历map并更新map
        if nums.len() == 0 {
            return 0;
        }
        let mut nums: Vec<i32> = nums.iter().map(|c| (*c + k) % k).collect();
        let mut dp = vec![0; nums.len()];
        dp[0] = if nums[0] == 0 { 1 } else { 0 };
        let mut map = std::collections::HashMap::new();
        *map.entry(nums[0]).or_insert(0) += 1;
        for i in 1..nums.len() {
            if nums[i] == 0 {
                dp[i] = 2 * dp[i - 1] + 1;
            } else {
                dp[i] = dp[i - 1] + *map.entry(k - nums[i]).or_insert(0)
            }
            for (_, v) in map.clone().iter() {
                *map.entry((*v + nums[i] + k) % k).or_insert(0) += 1;
            }
            *map.entry(nums[i]).or_insert(0) += 1;
        }
        dp[nums.len() - 1]
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l974() {
        assert_eq!(Solution::subarrays_div_by_k(vec![4, 5, 0, -2, -3, 1], 5), 7);
        assert_eq!(Solution::subarrays_div_by_k(vec![0], 5), 1);
        assert_eq!(Solution::subarrays_div_by_k(vec![], 5), 0);
        assert_eq!(Solution::subarrays_div_by_k(vec![0, 0], 5), 3);
        assert_eq!(Solution::subarrays_div_by_k(vec![0, 5], 5), 3);
        assert_eq!(Solution::subarrays_div_by_k(vec![5], 9), 0);
        assert_eq!(Solution::subarrays_div_by_k(vec![1, -10, 5], 9), 1);
    }
}
