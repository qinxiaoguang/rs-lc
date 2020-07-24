pub struct Solution {}

/*
 * @lc app=leetcode.cn id=128 lang=rust
 *
 * [128] 最长连续序列
 *
 * https://leetcode-cn.com/problems/longest-consecutive-sequence/description/
 *
 * algorithms
 * Hard (48.19%)
 * Likes:    461
 * Dislikes: 0
 * Total Accepted:    66.2K
 * Total Submissions: 129K
 * Testcase Example:  '[100,4,200,1,3,2]'
 *
 * 给定一个未排序的整数数组，找出最长连续序列的长度。
 *
 * 要求算法的时间复杂度为 O(n)。
 *
 * 示例:
 *
 * 输入: [100, 4, 200, 1, 3, 2]
 * 输出: 4
 * 解释: 最长连续序列是 [1, 2, 3, 4]。它的长度为 4。
 *
 */

// @lc code=start
use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        // set+map
        // set保存当前所有的数字，map保存当前数字的最大连续子序列
        // 每次针对一个num数字，进行自增1操作，在set中查看是否有自增后的数字，如果有
        // 则到map查看是否有保存的已经得到的结果，如果有直接使用，且不用往后遍历
        // 若没有，则继续往后遍历，并保存当前遍历的长度，直到有结果
        // 其实就是利用了类似剪枝的思路，用空间换时间
        let set: HashSet<i32> = nums.clone().into_iter().collect();

        let mut used: HashMap<i32, i32> = HashMap::new();
        let mut max = 0;

        for (idx, &num) in nums.iter().enumerate() {
            let mut tmp_cnt = 1;

            max = std::cmp::max(Self::get_longest(&set, &mut used, num), max);
        }
        max
    }

    pub fn get_longest(set: &HashSet<i32>, used: &mut HashMap<i32, i32>, num: i32) -> i32 {
        let mut tmp_max = 1;
        let mut tmp_cnt = 0;

        let mut max_bound = num;

        for i in num.. {
            if used.contains_key(&i) {
                let max = used.get(&i).unwrap();
                tmp_max = max + tmp_cnt;
                break;
            }
            if !set.contains(&i) {
                tmp_max = tmp_cnt;
                break;
            }
            tmp_cnt += 1;
        }

        for i in num..=max_bound {
            used.insert(i, tmp_max);
        }
        tmp_max
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l128() {
        assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
        assert_eq!(Solution::longest_consecutive(vec![1]), 1);
        assert_eq!(
            Solution::longest_consecutive(vec![100, 2, 3, 4, 5, 7, 9, 10, 11, 12, 13, 14]),
            6
        );
        assert_eq!(Solution::longest_consecutive(vec![]), 0);
    }
}
