use super::Solution;

impl Solution {
    // 找到大于等于target的长度最小的连续子数组
    // 滑动窗口
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut sum = nums[0];
        let mut res = usize::MAX;
        let (mut left, mut right) = (0usize, 0usize);
        while left <= right && right < nums.len() {
            if sum >= target {
                res = res.min(right - left + 1);
                sum -= nums[left];
                left += 1;
                continue;
            }
            right += 1;
            sum += if right < nums.len() { nums[right] } else { 0 };
        }

        let res = res.min(if right == nums.len() && sum >= target {
            right - left + 1
        } else {
            res
        });
        (if res == usize::MAX { 0 } else { res }) as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_008() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(Solution::min_sub_array_len(4, vec![]), 0);
        assert_eq!(Solution::min_sub_array_len(4, vec![1]), 0);
        assert_eq!(Solution::min_sub_array_len(4, vec![4]), 1);
        assert_eq!(
            Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]),
            0
        );
    }
}
