pub struct Solution {}

/*
 * @lc app=leetcode.cn id=560 lang=rust
 *
 * [560] 和为K的子数组
 *
 * https://leetcode-cn.com/problems/subarray-sum-equals-k/description/
 *
 * algorithms
 * Medium (44.18%)
 * Likes:    521
 * Dislikes: 0
 * Total Accepted:    60.4K
 * Total Submissions: 135.3K
 * Testcase Example:  '[1,1,1]\n2'
 *
 * 给定一个整数数组和一个整数 k，你需要找到该数组中和为 k 的连续的子数组的个数。
 *
 * 示例 1 :
 *
 *
 * 输入:nums = [1,1,1], k = 2
 * 输出: 2 , [1,1] 与 [1,1] 为两种不同的情况。
 *
 *
 * 说明 :
 *
 *
 * 数组的长度为 [1, 20,000]。
 * 数组中元素的范围是 [-1000, 1000] ，且整数 k 的范围是 [-1e7, 1e7]。
 *
 *
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        Self::subarray_sum_v2(nums, k)
    }

    // l[j+1] - l[i] = k
    // 那么 l[j+1] = l[i] + k
    // 假设 l[i] +k 为sum,那么只需要保存 sum的个数,在遍历的时候，查看sum的个数为多少个，进行加和就得到了结果
    pub fn subarray_sum_v2(nums: Vec<i32>, k: i32) -> i32 {
        let mut l = vec![0; nums.len() + 1];
        let mut sum = 0;
        let mut res = 0;
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            l[i] = sum;
            sum += nums[i];
            res += *map.entry(l[i]).or_insert(0);
            *map.entry(l[i] + k).or_insert(0) += 1;
        }
        l[nums.len()] = sum;
        res += *map.entry(l[nums.len()]).or_insert(0);
        *map.entry(sum + k).or_insert(0) += 1;
        res
    }

    pub fn subarray_sum_v1(nums: Vec<i32>, k: i32) -> i32 {
        // 连续子数组和为k的个数
        // 如果数组内的数都是正数，就可以使用 尺取法
        // 但是数组内还有负数，所以暴力可以通过O(n^2)来解
        // 优化则通过线段树
        let mut l = vec![0; nums.len() + 1];
        let mut sum = 0;
        let mut res = 0;
        for i in 0..nums.len() {
            l[i] = sum;
            sum += nums[i];
        }
        l[nums.len()] = sum;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if l[j + 1] - l[i] == k {
                    //println!("l:{:?}, j+1:{},i:{}", l, j + 1, i);
                    res += 1;
                }
            }
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l560() {
        assert_eq!(2, Solution::subarray_sum(vec![1, 1, 1], 2));
        assert_eq!(1, Solution::subarray_sum(vec![-1, -1, 1], 0));
    }
}
