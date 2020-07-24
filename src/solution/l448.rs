pub struct Solution {}

/*
 * @lc app=leetcode.cn id=448 lang=rust
 *
 * [448] 找到所有数组中消失的数字
 *
 * https://leetcode-cn.com/problems/find-all-numbers-disappeared-in-an-array/description/
 *
 * algorithms
 * Easy (56.94%)
 * Likes:    402
 * Dislikes: 0
 * Total Accepted:    44.9K
 * Total Submissions: 75.9K
 * Testcase Example:  '[4,3,2,7,8,2,3,1]'
 *
 * 给定一个范围在  1 ≤ a[i] ≤ n ( n = 数组大小 ) 的 整型数组，数组中的元素一些出现了两次，另一些只出现一次。
 *
 * 找到所有在 [1, n] 范围之间没有出现在数组中的数字。
 *
 * 您能在不使用额外空间且时间复杂度为O(n)的情况下完成这个任务吗? 你可以假定返回的数组不算在额外空间内。
 *
 * 示例:
 *
 *
 * 输入:
 * [4,3,2,7,8,2,3,1]
 *
 * 输出:
 * [5,6]
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn find_disappeared_numbers(nums: Vec<i32>) -> Vec<i32> {
        //与442题一样，只需要把值放在对应的下标下，最后遍历一遍即可
        let mut nums = nums;
        let mut idx = 0;
        let mut res = vec![];
        //vec![4, 3, 2, 7, 8, 2, 3, 1]
        while idx < nums.len() {
            //println!("idx:{},nums:{:?}", idx, nums);
            let next_idx = nums[idx] as usize - 1;
            if next_idx == idx {
                idx += 1;
                continue;
            }
            if nums[next_idx] == nums[idx] {
                idx += 1;
                continue;
            }
            nums.swap(next_idx, idx);
        }
        // 再遍历一遍，不在自己位置的数，就是重复的数
        for idx in 0..nums.len() {
            if idx as i32 == nums[idx] - 1 {
                continue;
            }
            res.push(idx as i32 + 1);
        }
        return res;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l448() {
        assert_eq!(
            Solution::find_disappeared_numbers(vec![4, 3, 2, 7, 8, 2, 3, 1]),
            vec![5, 6]
        )
    }
}
