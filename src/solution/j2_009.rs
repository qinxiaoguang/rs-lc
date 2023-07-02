use super::Solution;

impl Solution {
    // 乘积小于k的连续子数组的个数
    // 滑动窗口[i,j]，如果j+1刚好大于k，此时以j为底的子数组个数则为j-i+1
    // 以此种方法依次枚举j,即可得出结果
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut res = 0;
        let (mut left, mut right) = (0usize, 0usize);

        let mut prod = 1;
        while left <= right && right < nums.len() {
            prod *= nums[right];
            while left < right && prod >= k {
                prod /= nums[left];
                left += 1;
            }
            if prod < k {
                res += right - left + 1;
            }
            right += 1;
        }
        res as i32
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_009() {
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![10, 5, 2, 6], 100),
            8
        );
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![1, 2, 3], 0),
            0
        );
        assert_eq!(
            Solution::num_subarray_product_less_than_k(vec![100, 100, 100], 10),
            0
        );
    }
}
