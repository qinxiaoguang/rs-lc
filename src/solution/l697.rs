pub struct Solution {}
/*
 * @lc app=leetcode.cn id=697 lang=rust
 *
 * [697] 数组的度
 *
 * https://leetcode-cn.com/problems/degree-of-an-array/description/
 *
 * algorithms
 * Easy (54.45%)
 * Likes:    166
 * Dislikes: 0
 * Total Accepted:    22.5K
 * Total Submissions: 41.3K
 * Testcase Example:  '[1,2,2,3,1]'
 *
 * 给定一个非空且只包含非负数的整数数组 nums, 数组的度的定义是指数组里任一元素出现频数的最大值。
 *
 * 你的任务是找到与 nums 拥有相同大小的度的最短连续子数组，返回其长度。
 *
 * 示例 1:
 *
 *
 * 输入: [1, 2, 2, 3, 1]
 * 输出: 2
 * 解释:
 * 输入数组的度是2，因为元素1和2的出现频数最大，均为2.
 * 连续子数组里面拥有相同度的有如下所示:
 * [1, 2, 2, 3, 1], [1, 2, 2, 3], [2, 2, 3, 1], [1, 2, 2], [2, 2, 3], [2, 2]
 * 最短连续子数组[2, 2]的长度为2，所以返回2.
 *
 *
 * 示例 2:
 *
 *
 * 输入: [1,2,2,3,1,4,2]
 * 输出: 6
 *
 *
 * 注意:
 *
 *
 * nums.length 在1到50,000区间范围内。
 * nums[i] 是一个在0到49,999范围内的整数。
 *
 *
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0;
        }
        let mut m = HashMap::new();
        // l=left, 表示某个数出现的左侧第一个位置
        let mut l = HashMap::new();
        // r = right, 表示某个数出现的右侧的第一个位置
        let mut r = HashMap::new();
        for num in &nums {
            *m.entry(num).or_insert(0) += 1;
        }
        for i in 0..nums.len() {
            if l.contains_key(&nums[i]) {
                continue;
            }
            l.insert(&nums[i], i as i32);
        }

        for i in (0..nums.len()).rev() {
            if r.contains_key(&nums[i]) {
                continue;
            }
            r.insert(&nums[i], i as i32);
        }

        let max = m.iter().map(|(k, v)| v).max().unwrap();
        let mut res = nums.len() as i32;
        for num in &nums {
            if m.get(num).unwrap() == max {
                let left = l.get(num).unwrap();
                let right = r.get(num).unwrap();
                res = std::cmp::min(res, right - left + 1);
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
    fn test_l697() {
        assert_eq!(2, Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1]));
        assert_eq!(1, Solution::find_shortest_sub_array(vec![1]));
        assert_eq!(0, Solution::find_shortest_sub_array(vec![]));
        assert_eq!(
            6,
            Solution::find_shortest_sub_array(vec![1, 2, 2, 3, 1, 4, 2])
        );
    }
}
