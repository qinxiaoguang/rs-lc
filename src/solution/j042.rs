use super::Solution;

impl Solution {
    // 求子数组和的最大值
    // 只需要加和，遇到和等于0的时候，重新进行加
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut res = i32::MIN;
        let mut sum = 0;
        nums.iter().for_each(|n| {
            sum += n;
            res = res.max(sum);
            if sum < 0 {
                sum = 0;
            }
        });

        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j042() {
        assert_eq!(
            Solution::max_sub_array(vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]),
            6
        );
        assert_eq!(Solution::max_sub_array(vec![-1]), -1);
    }
}
