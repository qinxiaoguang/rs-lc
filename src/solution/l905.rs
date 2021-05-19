pub struct Solution {}
/*
 * @lc app=leetcode.cn id=905 lang=rust
 *
 * [905] 按奇偶排序数组
 *
 * https://leetcode-cn.com/problems/sort-array-by-parity/description/
 *
 * algorithms
 * Easy (69.19%)
 * Likes:    161
 * Dislikes: 0
 * Total Accepted:    41.1K
 * Total Submissions: 59.4K
 * Testcase Example:  '[3,1,2,4]'
 *
 * 给定一个非负整数数组 A，返回一个数组，在该数组中， A 的所有偶数元素之后跟着所有奇数元素。
 *
 * 你可以返回满足此条件的任何数组作为答案。
 *
 *
 *
 * 示例：
 *
 * 输入：[3,1,2,4]
 * 输出：[2,4,3,1]
 * 输出 [4,2,3,1]，[2,4,1,3] 和 [4,2,1,3] 也会被接受。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= A.length <= 5000
 * 0 <= A[i] <= 5000
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn sort_array_by_parity(a: Vec<i32>) -> Vec<i32> {
        if a.len() == 0 {
            return a;
        }
        let mut a = a;
        // 双指针
        let (mut start, mut end): (usize, usize) = (0, a.len() - 1);
        while start < end {
            if a[start] & 1 == 0 {
                start += 1;
                continue;
            }

            if a[end] & 1 == 1 {
                end -= 1;
                continue;
            }

            a.swap(start, end);
            start += 1;
            end -= 1;
        }
        a
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l905() {
        println!("{:?}", Solution::sort_array_by_parity(vec![1, 2, 3, 4]));
        println!("{:?}", Solution::sort_array_by_parity(vec![1, 2]));
        println!("{:?}", Solution::sort_array_by_parity(vec![2, 4]));
        println!("{:?}", Solution::sort_array_by_parity(vec![]));
    }
}
