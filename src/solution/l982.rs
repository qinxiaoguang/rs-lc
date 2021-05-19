pub struct Solution {}
/*
 * @lc app=leetcode.cn id=982 lang=rust
 *
 * [982] 按位与为零的三元组
 *
 * https://leetcode-cn.com/problems/triples-with-bitwise-and-equal-to-zero/description/
 *
 * algorithms
 * Hard (49.98%)
 * Likes:    28
 * Dislikes: 0
 * Total Accepted:    1.5K
 * Total Submissions: 3K
 * Testcase Example:  '[2,1,3]'
 *
 * 给定一个整数数组 A，找出索引为 (i, j, k) 的三元组，使得：
 *
 *
 * 0 <= i < A.length
 * 0 <= j < A.length
 * 0 <= k < A.length
 * A[i] & A[j] & A[k] == 0，其中 & 表示按位与（AND）操作符。
 *
 *
 *
 *
 * 示例：
 *
 * 输入：[2,1,3]
 * 输出：12
 * 解释：我们可以选出如下 i, j, k 三元组：
 * (i=0, j=0, k=1) : 2 & 2 & 1
 * (i=0, j=1, k=0) : 2 & 1 & 2
 * (i=0, j=1, k=1) : 2 & 1 & 1
 * (i=0, j=1, k=2) : 2 & 1 & 3
 * (i=0, j=2, k=1) : 2 & 3 & 1
 * (i=1, j=0, k=0) : 1 & 2 & 2
 * (i=1, j=0, k=1) : 1 & 2 & 1
 * (i=1, j=0, k=2) : 1 & 2 & 3
 * (i=1, j=1, k=0) : 1 & 1 & 2
 * (i=1, j=2, k=0) : 1 & 3 & 2
 * (i=2, j=0, k=1) : 3 & 2 & 1
 * (i=2, j=1, k=0) : 3 & 1 & 2
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= A.length <= 1000
 * 0 <= A[i] < 2^16
 *
 *
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    // and操作的特点是 只要有一个0，&操作的结果必然会是0
    // 即保证选出的数中至少有一个是偶数
    // 那么该问题取反，则是选出的数中全是奇数的选法有几个
    // 刚开始考虑成了组合问题，写了半天代码，这里记录一下
    // 如果是组合问题，则组合C(n,k)表示从n个里面选k的选法个数
    // 计算C(n,k)则利用公式C(n,k) = C(n-1,k-1) + C(n,k-1)来计算，其中C(i,0)=1
    // 但此题不是组合，而是三次选法都是独立的，所以如果a的长度是n，则三次选法共有n^3种
    // 那么如果奇数个数是odd_len,则全是奇数的选法有odd_len^3
    // 那么答案就是 n^3 - odd_len^3
    // ==========================
    // 再看下题目，发现上面的解析还是错的，题目是保证最终的结果是0，而不是结果的末位是0
    // = =, 重新分析吧。
    // 看了题解，表示个人把此题想复杂了。
    // 首先暴力可解，3层循环枚举
    // 优化方式则将前两个枚举的值用map存储起来，最后再对数组及map遍历一遍来计数
    // 再优化的方法就是先找到map[n]，其中map的key是num而值则是与num进行and操作后结果是0的个数
    // 计算map的方式也比较奇特，假设遍历到当前值是5(101),那么就让(010)及(000)的个数都加1
    // 同样的，假设当前的值是(10010),那么就将(0xx0x)的个数都加1
    // 也就是说当前值的1的位置都要变成0，其他位置枚举
    // 这个方法较复杂，实际代码采用第二个方法
    pub fn count_triplets(a: Vec<i32>) -> i32 {
        let mut m = HashMap::new();
        for i in 0..a.len() {
            for j in 0..a.len() {
                let n = a[i] & a[j];
                *m.entry(n).or_insert(0) += 1;
            }
        }

        let mut res = 0;
        for i in 0..a.len() {
            for (key, value) in &m {
                if a[i] & *key == 0 {
                    res += value;
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
    fn test_l982() {
        assert_eq!(12, Solution::count_triplets(vec![2, 1, 3]));
    }
}
