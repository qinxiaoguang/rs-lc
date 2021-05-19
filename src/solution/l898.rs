pub struct Solution {}
/*
 * @lc app=leetcode.cn id=898 lang=rust
 *
 * [898] 子数组按位或操作
 *
 * https://leetcode-cn.com/problems/bitwise-ors-of-subarrays/description/
 *
 * algorithms
 * Medium (29.32%)
 * Likes:    66
 * Dislikes: 0
 * Total Accepted:    3.4K
 * Total Submissions: 11.5K
 * Testcase Example:  '[0]'
 *
 * 我们有一个非负整数数组 A。
 *
 * 对于每个（连续的）子数组 B = [A[i], A[i+1], ..., A[j]] （ i <= j），我们对 B
 * 中的每个元素进行按位或操作，获得结果 A[i] | A[i+1] | ... | A[j]。
 *
 * 返回可能结果的数量。 （多次出现的结果在最终答案中仅计算一次。）
 *
 *
 *
 * 示例 1：
 *
 * 输入：[0]
 * 输出：1
 * 解释：
 * 只有一个可能的结果 0 。
 *
 *
 * 示例 2：
 *
 * 输入：[1,1,2]
 * 输出：3
 * 解释：
 * 可能的子数组为 [1]，[1]，[2]，[1, 1]，[1, 2]，[1, 1, 2]。
 * 产生的结果为 1，1，2，1，3，3 。
 * 有三个唯一值，所以答案是 3 。
 *
 *
 * 示例 3：
 *
 * 输入：[1,2,4]
 * 输出：6
 * 解释：
 * 可能的结果是 1，2，3，4，6，以及 7 。
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= A.length <= 50000
 * 0 <= A[i] <= 10^9
 *
 *
 */

// @lc code=start
use std::collections::HashSet;
impl Solution {
    // 暴力解法是枚举i~j的值
    // 而i,j可以通过i,j-1 |j 得出，所以可以通过O(n^2)的时间内求出
    // 这个复杂度可能会超时，所以需要对其进行优化
    // 假设当前已经求出了所有的以j结尾的集合，即set[j]是{A[0~j],A[1~j]..A[j~j]}的集合
    // 其中A[0~j]是A[0]|A[1]..|A[j]
    // 那么set[j]中的结果不会超过32个。
    // 因为或有一更性质是，1与任何数或都是1，而只有0与0或才是0
    // 所以a与b或时，a中的1不可能变为0，而a中的0则可能变为1
    // 所以set[j]中，假设A[j~j]为32个0,那么A[j-1~j] = A[j-1] | A[j~j],则A[j-1~j]中的0的个数一定不会超过32
    // 且当他们俩0个数相等时，这两个数一定相等
    // zero[i]表示A[i~j]中0的个数
    // 那么一定有zero[i]<=zero[j]其中i<j
    pub fn subarray_bitwise_o_rs(a: Vec<i32>) -> i32 {
        if a.len() == 0 {
            return 0;
        }
        // 以j为结尾的所有set值，当j+1时 ，将set[j]中所有的可能值与a[j+1]进行或操作，得到新的set[j+1]的值
        // 注意set[j+1]还要加上a[j+1]本身
        let mut cur = HashSet::new();

        // 结果值
        let mut ans = HashSet::new();

        cur.insert(0);
        for num in a {
            let mut tmp = HashSet::new();
            for c in &cur {
                let res = c | num;
                ans.insert(res);
                tmp.insert(res);
            }
            tmp.insert(num);
            ans.insert(num);
            cur = tmp;
        }
        ans.len() as i32
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l898() {
        assert_eq!(6, Solution::subarray_bitwise_o_rs(vec![1, 2, 4]));
    }
}
