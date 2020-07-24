pub struct Solution {}

/*
 * @lc app=leetcode.cn id=169 lang=rust
 *
 * [169] 多数元素
 *
 * https://leetcode-cn.com/problems/majority-element/description/
 *
 * algorithms
 * Easy (61.79%)
 * Likes:    665
 * Dislikes: 0
 * Total Accepted:    186.8K
 * Total Submissions: 292.6K
 * Testcase Example:  '[3,2,3]'
 *
 * 给定一个大小为 n 的数组，找到其中的多数元素。多数元素是指在数组中出现次数大于 ⌊ n/2 ⌋ 的元素。
 *
 * 你可以假设数组是非空的，并且给定的数组总是存在多数元素。
 *
 *
 *
 * 示例 1:
 *
 * 输入: [3,2,3]
 * 输出: 3
 *
 * 示例 2:
 *
 * 输入: [2,2,1,1,1,2,2]
 * 输出: 2
 *
 *
 */
// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        // 最简单的方法就是排序,但我觉得有更好的方法
        // 空间换时间，map保存个数
        Self::majority_element_v2(nums)
    }

    pub fn majority_element_v1(nums: Vec<i32>) -> i32 {
        // 最简单的方法就是排序
        let mut nums = nums;
        nums.sort();
        nums[(0 + nums.len()) / 2]
    }

    pub fn majority_element_v2(nums: Vec<i32>) -> i32 {
        // 空间换时间，map保存个数
        let mut map = HashMap::new();
        let len = nums.len();
        for num in nums {
            let mut cnt = map.entry(num).or_insert(0);
            *cnt += 1;
            if *cnt > len / 2 {
                return num;
            }
        }
        return 0;
    }

    pub fn majority_element_v3(nums: Vec<i32>) -> i32 {
        // 一个更巧妙的方法，投票法， 细品
        let mut count = 0;
        let mut value = 0;

        for num in nums {
            if count == 0 {
                value = num;
                count += 1;
            } else if value == num {
                count += 1;
            } else {
                count -= 1;
            }
        }
        return count;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l169() {}
}
