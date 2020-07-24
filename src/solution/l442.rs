pub struct Solution {}

/*
 * @lc app=leetcode.cn id=442 lang=rust
 *
 * [442] 数组中重复的数据
 *
 * https://leetcode-cn.com/problems/find-all-duplicates-in-an-array/description/
 *
 * algorithms
 * Medium (64.19%)
 * Likes:    232
 * Dislikes: 0
 * Total Accepted:    19.8K
 * Total Submissions: 29.9K
 * Testcase Example:  '[4,3,2,7,8,2,3,1]'
 *
 * 给定一个整数数组 a，其中1 ≤ a[i] ≤ n （n为数组长度）, 其中有些元素出现两次而其他元素出现一次。
 *
 * 找到所有出现两次的元素。
 *
 * 你可以不用到任何额外空间并在O(n)时间复杂度内解决这个问题吗？
 *
 * 示例：
 *
 *
 * 输入:
 * [4,3,2,7,8,2,3,1]
 *
 * 输出:
 * [2,3]
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        // 不用额外空间，且只在O(n)时间复杂度内解决,但没说能不能改原数组，那说明是可以更改
        // 1<=a[i]<=n 这就是知识点
        // 可以将数字放到自己(对应的值-1)的下标中，并判断是否和该下标下的值相等
        // 若相等，则目标元素存在

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
            res.push(nums[idx]);
        }
        return res;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l442() {
        assert_eq!(
            vec![3, 2],
            Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1])
        );

        assert_eq!(vec![2], Solution::find_duplicates(vec![1, 3, 4, 2, 2]));
        assert_eq!(vec![3], Solution::find_duplicates(vec![3, 1, 3, 4, 2]));
    }
}
