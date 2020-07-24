pub struct Solution {}

/*
 * @lc app=leetcode.cn id=565 lang=rust
 *
 * [565] 数组嵌套
 *
 * https://leetcode-cn.com/problems/array-nesting/description/
 *
 * algorithms
 * Medium (56.13%)
 * Likes:    99
 * Dislikes: 0
 * Total Accepted:    6.9K
 * Total Submissions: 11.7K
 * Testcase Example:  '[5,4,0,3,1,6,2]'
 *
 * 索引从0开始长度为N的数组A，包含0到N - 1的所有整数。找到最大的集合S并返回其大小，其中 S[i] = {A[i], A[A[i]],
 * A[A[A[i]]], ... }且遵守以下的规则。
 *
 * 假设选择索引为i的元素A[i]为S的第一个元素，S的下一个元素应该是A[A[i]]，之后是A[A[A[i]]]...
 * 以此类推，不断添加直到S出现重复的元素。
 *
 *
 *
 * 示例 1:
 *
 * 输入: A = [5,4,0,3,1,6,2]
 * 输出: 4
 * 解释:
 * A[0] = 5, A[1] = 4, A[2] = 0, A[3] = 3, A[4] = 1, A[5] = 6, A[6] = 2.
 *
 * 其中一种最长的 S[K]:
 * S[0] = {A[0], A[5], A[6], A[2]} = {5, 6, 2, 0}
 *
 *
 *
 *
 * 提示：
 *
 *
 * N是[1, 20,000]之间的整数。
 * A中不含有重复的元素。
 * A中的元素大小在[0, N-1]之间。
 *
 *
 */

// @lc code=start
impl Solution {
    // 求的就是并查集,任何一个数都会组成一个环，可证，但我不会证
    // 想成并查集又有点复杂，不如直接迭代,已经迭代过的直接赋值为-1
    pub fn array_nesting(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        let mut res = 0;
        for i in 0..nums.len() {
            if nums[i] == -1 {
                continue;
            }

            let mut cnt = 1;
            let mut next_idx = nums[i] as usize;
            nums[i] = -1;
            // 从i开始找数
            while nums[next_idx] != -1 {
                let tmp_next_idx = nums[next_idx] as usize;
                cnt += 1;
                nums[next_idx] = -1;
                next_idx = tmp_next_idx;
            }
            res = std::cmp::max(res, cnt);
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l565() {
        assert_eq!(4, Solution::array_nesting(vec![5, 4, 0, 3, 1, 6, 2]))
    }
}
