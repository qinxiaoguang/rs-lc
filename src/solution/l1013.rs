pub struct Solution {}
/*
 * @lc app=leetcode.cn id=1013 lang=rust
 *
 * [1013] 将数组分成和相等的三个部分
 *
 * https://leetcode-cn.com/problems/partition-array-into-three-parts-with-equal-sum/description/
 *
 * algorithms
 * Easy (40.35%)
 * Likes:    120
 * Dislikes: 0
 * Total Accepted:    37.6K
 * Total Submissions: 93.3K
 * Testcase Example:  '[0,2,1,-6,6,-7,9,1,2,0,1]'
 *
 * 给你一个整数数组 A，只有可以将其划分为三个和相等的非空部分时才返回 true，否则返回 false。
 *
 * 形式上，如果可以找出索引 i+1 < j 且满足 A[0] + A[1] + ... + A[i] == A[i+1] + A[i+2] + ... +
 * A[j-1] == A[j] + A[j-1] + ... + A[A.length - 1] 就可以将数组三等分。
 *
 *
 *
 * 示例 1：
 *
 * 输入：[0,2,1,-6,6,-7,9,1,2,0,1]
 * 输出：true
 * 解释：0 + 2 + 1 = -6 + 6 - 7 + 9 + 1 = 2 + 0 + 1
 *
 *
 * 示例 2：
 *
 * 输入：[0,2,1,-6,6,7,9,-1,2,0,1]
 * 输出：false
 *
 *
 * 示例 3：
 *
 * 输入：[3,3,6,5,-2,2,5,1,-9,4]
 * 输出：true
 * 解释：3 + 3 = 6 = 5 - 2 + 2 + 5 + 1 - 9 + 4
 *
 *
 *
 *
 * 提示：
 *
 *
 * 3 <= A.length <= 50000
 * -10^4 <= A[i] <= 10^4
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn can_three_parts_equal_sum(a: Vec<i32>) -> bool {
        let sum = a.iter().sum::<i32>();
        if sum % 3 != 0 {
            return false;
        }
        let avg = sum / 3;
        let mut part = 0;
        let mut tmp_sum = 0;
        for i in 0..a.len() {
            tmp_sum += a[i];
            if tmp_sum == avg {
                tmp_sum = 0;
                part += 1;
                if part == 2 && i + 1 < a.len() {
                    return a[i + 1..].iter().sum::<i32>() == avg;
                }
            }
        }
        return false;
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1013() {
        assert_eq!(
            true,
            Solution::can_three_parts_equal_sum(vec![3, 3, 6, 5, -2, 2, 5, 1, -9, 4])
        );

        assert_eq!(
            true,
            Solution::can_three_parts_equal_sum(vec![10, -10, 10, -10, 10, -10, 10, -10])
        )
    }
}
