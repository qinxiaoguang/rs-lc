use super::Solution;

impl Solution {
    pub fn min_pair_sum(nums: Vec<i32>) -> i32 {
        // 排序后，最前面的和最后面的组成一组
        let mut nums = nums;
        nums.sort();

        let mut res = nums[0] + nums[nums.len() - 1];
        let mut i = 1;
        while i < nums.len() / 2 {
            res = res.max(nums[i] + nums[nums.len() - 1 - i]);
            i += 1;
        }
        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1877() {
        assert_eq!(Solution::min_pair_sum(vec![3, 5, 2, 3]), 7);
        assert_eq!(Solution::min_pair_sum(vec![3, 5, 4, 2, 4, 6]), 8);
    }
}
