pub struct Solution {}
/*
 * @lc app=leetcode.cn id=870 lang=rust
 *
 * [870] 优势洗牌
 *
 * https://leetcode-cn.com/problems/advantage-shuffle/description/
 *
 * algorithms
 * Medium (38.66%)
 * Likes:    75
 * Dislikes: 0
 * Total Accepted:    8.1K
 * Total Submissions: 20.9K
 * Testcase Example:  '[2,7,11,15]\n[1,10,4,11]'
 *
 * 给定两个大小相等的数组 A 和 B，A 相对于 B 的优势可以用满足 A[i] > B[i] 的索引 i 的数目来描述。
 *
 * 返回 A 的任意排列，使其相对于 B 的优势最大化。
 *
 *
 *
 * 示例 1：
 *
 * 输入：A = [2,7,11,15], B = [1,10,4,11]
 * 输出：[2,11,7,15]
 *
 *
 * 示例 2：
 *
 * 输入：A = [12,24,8,32], B = [13,25,32,11]
 * 输出：[24,32,8,12]
 *
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= A.length = B.length <= 10000
 * 0 <= A[i] <= 10^9
 * 0 <= B[i] <= 10^9
 *
 *
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    // 首先肯定是需要对两个数组排序的，排序完后，计算一下排序后的分数，根据排序后的分数来做调整
    // 如果只有三个数，那可以使用田忌赛马，那如果多个数呢，还能使用吗?
    // 暴力思想就是排序后一次前移来计算结果,这样的话需要O(n^2)的复杂度，有可能超时
    // 而最好的思路是一句话就能总结的，那就是我们要用a中最小的牌去打败b中最小的牌
    // 如果不能打败，就等待重分配，并寻找下一个a中能打败b中最小的牌的牌。
    pub fn advantage_count(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut a = a;
        let src_b = b.clone();
        let mut b = b;
        a.sort();
        b.sort();
        let mut m = HashMap::new();
        let mut idx = 0usize;
        let mut remaining = vec![];
        for num in a {
            if num > b[idx] {
                (*m.entry(b[idx]).or_insert(vec![])).push(num);
                idx += 1;
            } else {
                remaining.push(num);
            }
        }
        let mut res = vec![0; b.len()];
        for idx in 0..src_b.len() {
            if m.contains_key(&src_b[idx]) {
                if let Some(n) = m.get_mut(&src_b[idx]).unwrap().pop() {
                    res[idx] = n;
                    continue;
                }
            }
            res[idx] = remaining.pop().unwrap();
        }
        res
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l870() {
        assert_eq!(
            vec![24, 32, 8, 12],
            Solution::advantage_count(vec![12, 24, 8, 32], vec![13, 25, 32, 11])
        );
    }
}
