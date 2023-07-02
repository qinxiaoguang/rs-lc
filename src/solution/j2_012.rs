use super::Solution;

impl Solution {
    // 计算数组的中心下标
    // 中心下标左侧所有的和等于右侧所有的和
    // 遍历中心的时候， 顺便求出left和right
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = nums.iter().sum();
        if let Some(v) = nums.iter().enumerate().find(|(i, &x)| {
            right -= x;
            left += x;
            left - x == right
        }) {
            return v.0 as i32;
        }
        -1
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_012() {
        assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
        assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
        assert_eq!(Solution::pivot_index(vec![2, 1, -1]), 0);
    }
}
