pub struct Solution {}
/*
 * @lc app=leetcode.cn id=962 lang=rust
 *
 * [962] 最大宽度坡
 *
 * https://leetcode-cn.com/problems/maximum-width-ramp/description/
 *
 * algorithms
 * Medium (39.45%)
 * Likes:    73
 * Dislikes: 0
 * Total Accepted:    6.5K
 * Total Submissions: 16.4K
 * Testcase Example:  '[6,0,8,2,1,5]'
 *
 * 给定一个整数数组 A，坡是元组 (i, j)，其中  i < j 且 A[i] <= A[j]。这样的坡的宽度为 j - i。
 *
 * 找出 A 中的坡的最大宽度，如果不存在，返回 0 。
 *
 *
 *
 * 示例 1：
 *
 * 输入：[6,0,8,2,1,5]
 * 输出：4
 * 解释：
 * 最大宽度的坡为 (i, j) = (1, 5): A[1] = 0 且 A[5] = 5.
 *
 *
 * 示例 2：
 *
 * 输入：[9,8,1,0,1,9,4,0,4,1]
 * 输出：7
 * 解释：
 * 最大宽度的坡为 (i, j) = (2, 9): A[2] = 1 且 A[9] = 1.
 *
 *
 *
 *
 * 提示：
 *
 *
 * 2 <= A.length <= 50000
 * 0 <= A[i] <= 50000
 *
 *
 *
 *
 */

// @lc code=start
impl Solution {
    // 找的是最大宽度,最暴力的算法就是先使用最长长度扫描
    // 比如使用n的长度扫描，再使用n-1的长度扫描，直到扫描到结果
    // 非暴力算法，将数组中的数与其索引联系起来组成一个元组(x,idx)，其中x是他的数，idx是他的下标
    // 之后借助x来进行排序，得到idx的序列，那么该题就变成了该idx序列中后边的数减前边的数最大差的值
    // 因为宽度的前提是i<j, a[i]<=a[j],所以只能大的数的下标减去小的数的下标才能得到正确的宽度
    pub fn max_width_ramp(a: Vec<i32>) -> i32 {
        let mut a: Vec<(i32, usize)> = a.into_iter().enumerate().map(|(idx, x)| (x, idx)).collect();
        a.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let a: Vec<usize> = a.into_iter().map(|(x, idx)| idx).collect();

        let mut min = std::usize::MAX; // 保存当前值前的最小值
        let mut res = 0;
        for n in a {
            if n > min {
                res = std::cmp::max(res, n - min);
            }
            min = std::cmp::min(n, min);
        }
        res as i32
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l962() {
        assert_eq!(
            7,
            Solution::max_width_ramp(vec![9, 8, 1, 0, 1, 9, 4, 0, 4, 1])
        );
        assert_eq!(4, Solution::max_width_ramp(vec![6, 0, 8, 2, 1, 5]));
        assert_eq!(0, Solution::max_width_ramp(vec![]));
        assert_eq!(0, Solution::max_width_ramp(vec![1]));
    }
}
