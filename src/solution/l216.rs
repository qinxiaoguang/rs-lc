pub struct Solution {}

/*
 * @lc app=leetcode.cn id=216 lang=rust
 *
 * [216] 组合总和 III
 *
 * https://leetcode-cn.com/problems/combination-sum-iii/description/
 *
 * algorithms
 * Medium (70.45%)
 * Likes:    137
 * Dislikes: 0
 * Total Accepted:    24.5K
 * Total Submissions: 34.2K
 * Testcase Example:  '3\n7'
 *
 * 找出所有相加之和为 n 的 k 个数的组合。组合中只允许含有 1 - 9 的正整数，并且每种组合中不存在重复的数字。
 *
 * 说明：
 *
 *
 * 所有数字都是正整数。
 * 解集不能包含重复的组合。
 *
 *
 * 示例 1:
 *
 * 输入: k = 3, n = 7
 * 输出: [[1,2,4]]
 *
 *
 * 示例 2:
 *
 * 输入: k = 3, n = 9
 * 输出: [[1,2,6], [1,3,5], [2,3,4]]
 *
 *
 */

// @lc code=start
impl Solution {
    pub fn combination_sum3(k: i32, n: i32) -> Vec<Vec<i32>> {
        // 不允许用重复数字
        // 那么结果就是 递归，先固定第一个数，并从剩余的数中找到所有的k-1个组合为n-i的数
        let mut res = vec![];
        Self::dfs(k, n, 1, &mut res, &mut vec![]);
        res
    }

    fn dfs(k: i32, n: i32, now_cnt: i32, res: &mut Vec<Vec<i32>>, tmp: &mut Vec<i32>) {
        if k == 0 && n == 0 && tmp.len() != 0 {
            res.push(tmp.clone());
            return;
        }
        if k < 0 || n < 0 || now_cnt >= 10 {
            return;
        }

        for i in now_cnt..=9 {
            tmp.push(i as i32);
            Self::dfs(k - 1, n - i as i32, i + 1, res, tmp);
            tmp.pop();
        }
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l216() {
        assert_eq!(Solution::combination_sum3(3, 7), vec![vec![1, 2, 4]]);
        assert_eq!(Solution::combination_sum3(0, 0), vec![vec![]; 0]);
    }
}
