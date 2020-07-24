pub struct Solution {}

/*
 * @lc app=leetcode.cn id=189 lang=rust
 *
 * [189] 旋转数组
 *
 * https://leetcode-cn.com/problems/rotate-array/description/
 *
 * algorithms
 * Easy (40.68%)
 * Likes:    625
 * Dislikes: 0
 * Total Accepted:    141.6K
 * Total Submissions: 335.8K
 * Testcase Example:  '[1,2,3,4,5,6,7]\n3'
 *
 * 给定一个数组，将数组中的元素向右移动 k 个位置，其中 k 是非负数。
 *
 * 示例 1:
 *
 * 输入: [1,2,3,4,5,6,7] 和 k = 3
 * 输出: [5,6,7,1,2,3,4]
 * 解释:
 * 向右旋转 1 步: [7,1,2,3,4,5,6]
 * 向右旋转 2 步: [6,7,1,2,3,4,5]
 * 向右旋转 3 步: [5,6,7,1,2,3,4]
 *
 *
 * 示例 2:
 *
 * 输入: [-1,-100,3,99] 和 k = 2
 * 输出: [3,99,-1,-100]
 * 解释:
 * 向右旋转 1 步: [99,-1,-100,3]
 * 向右旋转 2 步: [3,99,-1,-100]
 *
 * 说明:
 *
 *
 * 尽可能想出更多的解决方案，至少有三种不同的方法可以解决这个问题。
 * 要求使用空间复杂度为 O(1) 的 原地 算法。
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        // 此题就是经典问题，还在那想暴力简单的方法呢
        // 记住旋转题 就用旋转方法来解决
        // 怎么旋，先整旋，再子旋
        // 因为向右旋转，其实就是将后k个数字放在头部
        // 那么就可以先将整个数组旋转后，将前k个数旋转，再将后n-k个数旋转，就得到结果了
        let k: usize = k as usize % nums.len();
        nums.reverse();
        nums[..k].reverse();
        nums[k..].reverse();
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l189() {
        let mut nums = vec![1, 2, 3, 4, 5];
        Solution::rotate(&mut nums, 2);
        assert_eq!(nums, vec![4, 5, 1, 2, 3]);
    }
}
