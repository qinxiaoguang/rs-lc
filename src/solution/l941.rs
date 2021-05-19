pub struct Solution {}
/*
 * @lc app=leetcode.cn id=941 lang=rust
 *
 * [941] 有效的山脉数组
 *
 * https://leetcode-cn.com/problems/valid-mountain-array/description/
 *
 * algorithms
 * Easy (35.85%)
 * Likes:    53
 * Dislikes: 0
 * Total Accepted:    12.6K
 * Total Submissions: 35K
 * Testcase Example:  '[2,1]'
 *
 * 给定一个整数数组 A，如果它是有效的山脉数组就返回 true，否则返回 false。
 *
 * 让我们回顾一下，如果 A 满足下述条件，那么它是一个山脉数组：
 *
 *
 * A.length >= 3
 * 在 0 < i < A.length - 1 条件下，存在 i 使得：
 *
 * A[0] < A[1] < ... A[i-1] < A[i]
 * A[i] > A[i+1] > ... > A[A.length - 1]
 *
 *
 *
 *
 *
 *
 *
 *
 *
 *
 * 示例 1：
 *
 * 输入：[2,1]
 * 输出：false
 *
 *
 * 示例 2：
 *
 * 输入：[3,5,5]
 * 输出：false
 *
 *
 * 示例 3：
 *
 * 输入：[0,3,2,1]
 * 输出：true
 *
 *
 *
 * 提示：
 *
 *
 * 0 <= A.length <= 10000
 * 0 <= A[i] <= 10000
 *
 *
 *
 *
 *
 *
 */

// @lc code=start
impl Solution {
    // 经典题
    // 必须遍历一遍得出结果
    // 首先开头必须升序
    pub fn valid_mountain_array(a: Vec<i32>) -> bool {
        if a.len() < 3 {
            return false;
        }
        if a[0] >= a[1] {
            return false;
        }
        let mut down = false;
        for i in 1..a.len() - 1 {
            // 比较a[i]和a[i-1]
            if a[i] == a[i + 1] {
                return false;
            }
            if a[i] < a[i + 1] {
                if down {
                    return false;
                }
                continue;
            }
            down = true;
        }
        return down;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l941() {
        assert_eq!(true, Solution::valid_mountain_array(vec![0, 3, 2, 1]));
        assert_eq!(false, Solution::valid_mountain_array(vec![3, 5, 5]));
        assert_eq!(false, Solution::valid_mountain_array(vec![2, 1]));
    }
}
