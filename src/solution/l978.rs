pub struct Solution {}
/*
 * @lc app=leetcode.cn id=978 lang=rust
 *
 * [978] 最长湍流子数组
 *
 * https://leetcode-cn.com/problems/longest-turbulent-subarray/description/
 *
 * algorithms
 * Medium (41.87%)
 * Likes:    52
 * Dislikes: 0
 * Total Accepted:    7.2K
 * Total Submissions: 17.1K
 * Testcase Example:  '[9,4,2,10,7,8,8,1,9]'
 *
 * 当 A 的子数组 A[i], A[i+1], ..., A[j] 满足下列条件时，我们称其为湍流子数组：
 *
 *
 * 若 i <= k < j，当 k 为奇数时， A[k] > A[k+1]，且当 k 为偶数时，A[k] < A[k+1]；
 * 或 若 i <= k < j，当 k 为偶数时，A[k] > A[k+1] ，且当 k 为奇数时， A[k] < A[k+1]。
 *
 *
 * 也就是说，如果比较符号在子数组中的每个相邻元素对之间翻转，则该子数组是湍流子数组。
 *
 * 返回 A 的最大湍流子数组的长度。
 *
 *
 *
 * 示例 1：
 *
 * 输入：[9,4,2,10,7,8,8,1,9]
 * 输出：5
 * 解释：(A[1] > A[2] < A[3] > A[4] < A[5])
 *
 *
 * 示例 2：
 *
 * 输入：[4,8,12,16]
 * 输出：2
 *
 *
 * 示例 3：
 *
 * 输入：[100]
 * 输出：1
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= A.length <= 40000
 * 0 <= A[i] <= 10^9
 *
 *
 */

// @lc code=start
impl Solution {
    // 就是找一个最长的序列，其中的数满足上下喘息的节奏
    // 遍历两次找到结果，第一次是从上下上下的喘息中找结果
    // 第二次是从下上下上的喘息中找结果
    // 官方题解更给力，是将a数组变为[1,0,-1]组合成的数， 其中1,0,-1分别表示 ('>','=','<')
    // 那么 (1,2,3,2)就可以表示为(-1,-1,1),找到(-1,1)的最长的序列即可。
    pub fn max_turbulence_size(a: Vec<i32>) -> i32 {
        if a.len() == 1 || (a.len() == 2 && a[0] == a[1]) {
            return 1;
        }
        if a.len() == 2 {
            return 2;
        }
        let mut res = 0;
        let (mut first_res, mut second_res) = (1, 1);
        let (mut first_up, mut second_up) = (true, false);
        for i in 0..a.len() - 1 {
            //println!("i:{},first_res:{}", i, first_res);
            if first_up {
                if a[i] < a[i + 1] {
                    first_res += 1;
                } else {
                    res = std::cmp::max(res, first_res);
                    first_res = 1;
                }
            } else {
                if a[i] > a[i + 1] {
                    first_res += 1;
                } else {
                    res = std::cmp::max(res, first_res);
                    first_res = 1;
                }
            }

            if second_up {
                if a[i] < a[i + 1] {
                    second_res += 1;
                } else {
                    res = std::cmp::max(res, second_res);
                    second_res = 1;
                }
            } else {
                if a[i] > a[i + 1] {
                    second_res += 1;
                } else {
                    res = std::cmp::max(res, second_res);
                    second_res = 1;
                }
            }

            first_up = !first_up;
            second_up = !second_up;
        }
        res = std::cmp::max(res, first_res);
        std::cmp::max(res, second_res)
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l978() {
        /*assert_eq!(2, Solution::max_turbulence_size(vec![4, 8, 12, 16]));*/
        assert_eq!(
            8,
            Solution::max_turbulence_size(vec![0, 8, 45, 88, 48, 68, 28, 55, 17, 24])
        );
        /*assert_eq!(
            5,
            Solution::max_turbulence_size(vec![9, 4, 2, 10, 7, 8, 8, 1, 9])
        );*/
    }
}
