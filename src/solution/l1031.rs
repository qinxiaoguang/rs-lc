pub struct Solution {}
/*
 * @lc app=leetcode.cn id=1031 lang=rust
 *
 * [1031] 两个非重叠子数组的最大和
 *
 * https://leetcode-cn.com/problems/maximum-sum-of-two-non-overlapping-subarrays/description/
 *
 * algorithms
 * Medium (53.49%)
 * Likes:    66
 * Dislikes: 0
 * Total Accepted:    2.8K
 * Total Submissions: 5.2K
 * Testcase Example:  '[0,6,5,2,2,5,1,9,4]\n1\n2'
 *
 * 给出非负整数数组 A ，返回两个非重叠（连续）子数组中元素的最大和，子数组的长度分别为 L 和 M。（这里需要澄清的是，长为 L 的子数组可以出现在长为
 * M 的子数组之前或之后。）
 *
 * 从形式上看，返回最大的 V，而 V = (A[i] + A[i+1] + ... + A[i+L-1]) + (A[j] + A[j+1] + ...
 * + A[j+M-1]) 并满足下列条件之一：
 *
 *
 *
 *
 * 0 <= i < i + L - 1 < j < j + M - 1 < A.length, 或
 * 0 <= j < j + M - 1 < i < i + L - 1 < A.length.
 *
 *
 *
 *
 * 示例 1：
 *
 * 输入：A = [0,6,5,2,2,5,1,9,4], L = 1, M = 2
 * 输出：20
 * 解释：子数组的一种选择中，[9] 长度为 1，[6,5] 长度为 2。
 *
 *
 * 示例 2：
 *
 * 输入：A = [3,8,1,3,2,1,8,9,0], L = 3, M = 2
 * 输出：29
 * 解释：子数组的一种选择中，[3,8,1] 长度为 3，[8,9] 长度为 2。
 *
 *
 * 示例 3：
 *
 * 输入：A = [2,1,5,6,0,9,5,0,3,8], L = 4, M = 3
 * 输出：31
 * 解释：子数组的一种选择中，[5,6,0,9] 长度为 4，[0,3,8] 长度为 3。
 *
 *
 *
 * 提示：
 *
 *
 * L >= 1
 * M >= 1
 * L + M <= A.length <= 1000
 * 0 <= A[i] <= 1000
 *
 *
 */

// @lc code=start
impl Solution {
    // 暴力法
    pub fn max_sum_two_no_overlap(a: Vec<i32>, l: i32, m: i32) -> i32 {
        let mut ll = vec![0; a.len()]; // 以i为结尾的长度为l的子序列的和
        let mut mm = vec![0; a.len()]; // 同上

        let (mut start, mut lend, mut mend) = (0, l as usize - 1, m as usize - 1);
        let mut lsum = a[start..lend].iter().sum::<i32>();
        let mut msum = a[start..mend].iter().sum::<i32>();
        while lend < a.len() || mend < a.len() {
            if lend < a.len() {
                lsum += a[lend];
                ll[lend] = lsum;
            }
            if mend < a.len() {
                msum += a[mend];
                mm[mend] = msum;
            }
            lsum -= a[start];
            msum -= a[start];
            start += 1;
            lend += 1;
            mend += 1;
        }
        let mut res = 0;
        //println!("l:{:?}", ll);
        //println!("m:{:?}", mm);
        for i in 0..a.len() {
            for j in 0..a.len() {
                if (i > j && i - j + 1 > l as usize) || (i < j && j - i + 1 > m as usize) {
                    res = std::cmp::max(res, ll[i] + mm[j]);
                }
            }
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l1031() {
        assert_eq!(
            Solution::max_sum_two_no_overlap(vec![2, 1, 5, 6, 0, 9, 5, 0, 3, 8], 4, 3),
            31
        );
        assert_eq!(
            Solution::max_sum_two_no_overlap(vec![3, 8, 1, 3, 2, 1, 8, 9, 0], 3, 2),
            29
        );

        assert_eq!(
            Solution::max_sum_two_no_overlap(vec![0, 6, 5, 2, 2, 5, 1, 9, 4], 1, 2),
            20
        );

        assert_eq!(
            Solution::max_sum_two_no_overlap(vec![0, 1, 2, 3, 4], 2, 1),
            9
        );

        assert_eq!(
            Solution::max_sum_two_no_overlap(vec![8, 20, 6, 2, 20, 17, 6, 3, 20, 8, 12], 5, 4),
            108
        );
    }
}
