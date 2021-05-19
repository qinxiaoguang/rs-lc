pub struct Solution {}
/*
 * @lc app=leetcode.cn id=991 lang=rust
 *
 * [991] 坏了的计算器
 *
 * https://leetcode-cn.com/problems/broken-calculator/description/
 *
 * algorithms
 * Medium (50.64%)
 * Likes:    74
 * Dislikes: 0
 * Total Accepted:    5.8K
 * Total Submissions: 11.5K
 * Testcase Example:  '2\n3'
 *
 * 在显示着数字的坏计算器上，我们可以执行以下两种操作：
 *
 *
 * 双倍（Double）：将显示屏上的数字乘 2；
 * 递减（Decrement）：将显示屏上的数字减 1 。
 *
 *
 * 最初，计算器显示数字 X。
 *
 * 返回显示数字 Y 所需的最小操作数。
 *
 *
 *
 * 示例 1：
 *
 * 输入：X = 2, Y = 3
 * 输出：2
 * 解释：先进行双倍运算，然后再进行递减运算 {2 -> 4 -> 3}.
 *
 *
 * 示例 2：
 *
 * 输入：X = 5, Y = 8
 * 输出：2
 * 解释：先递减，再双倍 {5 -> 4 -> 8}.
 *
 *
 * 示例 3：
 *
 * 输入：X = 3, Y = 10
 * 输出：3
 * 解释：先双倍，然后递减，再双倍 {3 -> 6 -> 5 -> 10}.
 *
 *
 * 示例 4：
 *
 * 输入：X = 1024, Y = 1
 * 输出：1023
 * 解释：执行递减运算 1023 次
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= X <= 10^9
 * 1 <= Y <= 10^9
 *
 *
 */

// @lc code=start
impl Solution {
    // 第一种是暴力，采用bfs
    // 但是一般来说暴力会超时
    // 那我们考虑反过来,如何将y转换为x
    // 只能是y除以2或者加1操作
    // 而y如果是奇数，则只能加1
    // 而如果y为偶数，则可以选择加2或者除以2
    // 所以只需要对偶数来判断，是加2，还是除以2即可
    // 一般来说加2后再除以2，得到的结果其实就是偶数除以2+1,明显后者步数小
    // 所以如果y比x大的时候，直接选择除以2是最优选择
    pub fn broken_calc(x: i32, y: i32) -> i32 {
        let mut cnt = 0;
        let mut y = y;
        while x != y {
            if y & 1 == 1 {
                y += 1;
            } else {
                // y是偶数
                if y < x {
                    y += 1;
                } else {
                    y /= 2;
                }
            }
            cnt += 1;
        }
        cnt
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l991() {
        assert_eq!(2, Solution::broken_calc(2, 3));
        assert_eq!(2, Solution::broken_calc(5, 8));
        assert_eq!(3, Solution::broken_calc(3, 10));
        assert_eq!(1023, Solution::broken_calc(1024, 1));
    }
}
