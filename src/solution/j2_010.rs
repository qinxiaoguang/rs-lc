use super::Solution;

impl Solution {
    // 找到整数数组中和为k的连续子数组的个数
    // 数组中的值的范围是[-1000,1000]
    // 最简单的方法是记录l[i]为i左边所有数的和
    // 利用该数可以计算区间值
    // 然后就可以得到结果，时间复杂度是O(n^2)
    // 再换个思路，如果得到了l[i]数组
    // 那么问题就变为， 找到数组中两数相减为k的个数
    // 利用hash表， 在遍历到i的时候，同时记录i前边所有数出现次数
    // 这样的话就可以立即找到以l[i]为后者，符合条件的前者的数的个数
    // 即遍历过程 利用前缀和及map来计算结果
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum = 0;
        let mut m = std::collections::HashMap::<i32, i32>::new();
        // 需要考虑0的情况，即前n连续数字
        m.insert(0, 1);
        let mut res = 0;
        nums.iter().for_each(|&x| {
            sum += x;
            res += *m.entry(sum - k).or_insert(0);
            *m.entry(sum).or_insert(0) += 1;
        });

        res
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_j2_010() {
        assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
    }
}
