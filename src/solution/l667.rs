pub struct Solution {}
/*
 * @lc app=leetcode.cn id=667 lang=rust
 *
 * [667] 优美的排列 II
 *
 * https://leetcode-cn.com/problems/beautiful-arrangement-ii/description/
 *
 * algorithms
 * Medium (58.82%)
 * Likes:    62
 * Dislikes: 0
 * Total Accepted:    4.5K
 * Total Submissions: 7.4K
 * Testcase Example:  '3\n2'
 *
 * 给定两个整数 n 和 k，你需要实现一个数组，这个数组包含从 1 到 n 的 n 个不同整数，同时满足以下条件：
 *
 * ① 如果这个数组是 [a1, a2, a3, ... , an] ，那么数组 [|a1 - a2|, |a2 - a3|, |a3 - a4|, ...
 * , |an-1 - an|] 中应该有且仅有 k 个不同整数；.
 *
 * ② 如果存在多种答案，你只需实现并返回其中任意一种.
 *
 * 示例 1:
 *
 *
 * 输入: n = 3, k = 1
 * 输出: [1, 2, 3]
 * 解释: [1, 2, 3] 包含 3 个范围在 1-3 的不同整数， 并且 [1, 1] 中有且仅有 1 个不同整数 : 1
 *
 *
 *
 *
 * 示例 2:
 *
 *
 * 输入: n = 3, k = 2
 * 输出: [1, 3, 2]
 * 解释: [1, 3, 2] 包含 3 个范围在 1-3 的不同整数， 并且 [2, 1] 中有且仅有 2 个不同整数: 1 和 2
 *
 *
 *
 *
 * 提示:
 *
 *
 * n 和 k 满足条件 1 <= k < n <= 10^4.
 *
 *
 *
 *
 */

// @lc code=start
impl Solution {
    // 枚举4-k,及5-k的数，可以发现规律
    // 对于n-k而言，固定前 (n-k)个数为[1~n-k],后边的数按照n-k~1的递减序列来插入值
    // 如n=5,k=3, 固定前5-3=2个数为[1~2],即结果的前两个数为[1,2],后边的数按3~1的序列插值，
    // 那么第3个数为2+3或2-3，显然2+3符合，所以结果的前几个数为[1,2,5]，接着下一个要插入的值为5+2或5-2
    // 显然5-2=3符合，所以结果为[1,2,5,3]，下一个序列是1，那么为3-1或3+1，显然3+1=4符合
    // 所以答案是[1,2,5,3,4], 其实就是(1,2,2+3, 2+3-2, 2+3-2+1)
    // 可以总结为一个规律，对于任意的n,k
    // 固定前n-k个数为1到n-k,
    // 后边的数按照 An = An-1 + (-1)^(n) * k-n来补充(n为下标，从0开始)
    // 按此规律得到的数组必然是其中一个答案
    pub fn construct_array(n: i32, k: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in 1..=n - k {
            res.push(i)
        }
        let mut idx = 0i32;
        loop {
            let last = res[res.len() - 1];
            let tmp = last + (-1i32).pow(idx as u32) * (k - idx);
            res.push(tmp);
            idx += 1;
            if res.len() == n as usize {
                return res;
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l667() {
        assert_eq!(vec![1, 2, 3], Solution::construct_array(3, 1));
        assert_eq!(vec![1, 3, 2], Solution::construct_array(3, 2));
    }
}
