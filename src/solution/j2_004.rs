use std::ops::Index;

use super::Solution;

impl Solution {
    // 只有一个数出现一次，其他数都出现3次，找到该数
    // 如果是其他数出现一次， 使用异或就可解， 但此题是出现3次
    // 最简单的方法就是计数，当然这种方法不是最优的。
    // 第二方法就是排序
    // 第三个方法就是按位计算，每位的总数和最后再取3的余数， 就是结果。
    // 最优的方法是使用自动机,这里使用第三个方法，自动机可以上网参考
    pub fn single_number_v3(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..32 {
            let mut sum = 0;
            for num in nums.iter() {
                sum += (*num >> i) & 1;
            }
            if sum % 3 > 0 {
                res |= 1 << i;
            }
        }
        return res;
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_004() {
        assert_eq!(Solution::single_number_v3(vec![2, 2, 3, 2]), 3);
        assert_eq!(Solution::single_number_v3(vec![0, 1, 0, 1, 0, 1, 100]), 100);
    }
}
