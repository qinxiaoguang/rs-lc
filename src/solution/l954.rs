pub struct Solution {}
/*
 * @lc app=leetcode.cn id=954 lang=rust
 *
 * [954] 二倍数对数组
 *
 * https://leetcode-cn.com/problems/array-of-doubled-pairs/description/
 *
 * algorithms
 * Medium (27.97%)
 * Likes:    31
 * Dislikes: 0
 * Total Accepted:    3.9K
 * Total Submissions: 13.8K
 * Testcase Example:  '[3,1,3,6]'
 *
 * 给定一个长度为偶数的整数数组 A，只有对 A 进行重组后可以满足 “对于每个 0 <= i < len(A) / 2，都有 A[2 * i + 1] =
 * 2 * A[2 * i]” 时，返回 true；否则，返回 false。
 *
 *
 *
 * 示例 1：
 *
 * 输入：[3,1,3,6]
 * 输出：false
 *
 *
 * 示例 2：
 *
 * 输入：[2,1,2,6]
 * 输出：false
 *
 *
 * 示例 3：
 *
 * 输入：[4,-2,2,-4]
 * 输出：true
 * 解释：我们可以用 [-2,-4] 和 [2,4] 这两组组成 [-2,-4,2,4] 或是 [2,4,-2,-4]
 *
 * 示例 4：
 *
 * 输入：[1,2,4,16,8,4]
 * 输出：false
 *
 *
 *
 *
 * 提示：
 *
 *
 * 0 <= A.length <= 30000
 * A.length 为偶数
 * -100000 <= A[i] <= 100000
 *
 *
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    // 重组后，是否能满足后一个偶数位的数是前一个奇数位的2倍
    // 首先负数与正数肯定是一组的
    // 所以二分为负数与正数
    // 如果两组数的个数是奇数个，则返回false
    // 否则，则判断两组数是否能组成 xxx
    // 判断的方法就是，先排序，从最小的开始判断，如果最小的二倍数存在，则将这两个数排除
    // 继续下一步
    pub fn can_reorder_doubled(a: Vec<i32>) -> bool {
        if a.len() == 0 {
            return true;
        }
        let (mut pos, mut neg): (Vec<i32>, Vec<i32>) = a.into_iter().partition(|&a| a > 0);
        if neg.len() & 1 == 1 || pos.len() & 1 == 1 {
            return false;
        }
        if neg.len() != 0 && pos.len() != 0 {
            return Self::can_reorder_doubled(neg) && Self::can_reorder_doubled(pos);
        }
        let target = if neg.len() != 0 {
            neg.sort();
            neg.reverse();
            neg
        } else {
            pos.sort();
            pos
        };

        let mut map = HashMap::new();
        for &num in &target {
            *map.entry(num).or_insert(0i32) += 1;
        }

        for &num in &target {
            let cnt = map.entry(num).or_insert(0i32);
            if *cnt == 0 {
                continue;
            }
            *cnt -= 1;
            let double = num * 2;
            let double_cnt = map.entry(double).or_insert(0i32);
            if *double_cnt == 0 {
                return false;
            }
            *double_cnt -= 1;
        }

        return true;
    }
}
// @lc code=end
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l954() {
        assert_eq!(false, Solution::can_reorder_doubled(vec![3, 1, 3, 6]));
        assert_eq!(false, Solution::can_reorder_doubled(vec![2, 1, 2, 6]));
        assert_eq!(true, Solution::can_reorder_doubled(vec![2, 4, -2, -4]));
        assert_eq!(
            false,
            Solution::can_reorder_doubled(vec![1, 2, 4, 16, 8, 4])
        );
    }
}
