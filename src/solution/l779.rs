pub struct Solution {}
/*
 * @lc app=leetcode.cn id=779 lang=rust
 *
 * [779] 第K个语法符号
 *
 * https://leetcode-cn.com/problems/k-th-symbol-in-grammar/description/
 *
 * algorithms
 * Medium (42.72%)
 * Likes:    88
 * Dislikes: 0
 * Total Accepted:    12.4K
 * Total Submissions: 29K
 * Testcase Example:  '1\n1'
 *
 * 在第一行我们写上一个 0。接下来的每一行，将前一行中的0替换为01，1替换为10。
 *
 * 给定行数 N 和序数 K，返回第 N 行中第 K个字符。（K从1开始）
 *
 *
 * 例子:
 *
 * 输入: N = 1, K = 1
 * 输出: 0
 *
 * 输入: N = 2, K = 1
 * 输出: 0
 *
 * 输入: N = 2, K = 2
 * 输出: 1
 *
 * 输入: N = 4, K = 5
 * 输出: 1
 *
 * 解释:
 * 第一行: 0
 * 第二行: 01
 * 第三行: 0110
 * 第四行: 01101001
 *
 *
 *
 * 注意：
 *
 *
 * N 的范围 [1, 30].
 * K 的范围 [1, 2^(N-1)].
 *
 *
 */

// @lc code=start
impl Solution {
    // 研究其规律，可以发现以下几个规律
    // 1. n+1行的数是包含n行的数
    // 2. 若n为奇数，则该行的数为中心对称的
    // 3. 若n为偶数，则该行的数为中心反方向对称的，比如 01101001, 对称左边的数为0110,对称右边的数为1001,对左右两边任意一边取反，就能得到对应边的值。
    // 所以按照这个规律，可以将n/k的值化简到n=1,k=1上。但是需要计算取反的次数
    // 官方题解是用递归来解的，所以代码很精简
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        let (mut n, mut k) = (n, k);
        let mut op_cnt = 0;
        while n > 1 {
            let mid = 2i32.pow((n - 2) as u32);
            if k > mid {
                k = 2 * mid - k + 1;
                if n % 2 == 0 {
                    op_cnt += 1;
                }
            }
            n -= 1;
        }

        op_cnt % 2
    }
}
// @lc code=end

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l779() {
        assert_eq!(0, Solution::kth_grammar(1, 1));
        assert_eq!(0, Solution::kth_grammar(2, 1));
        assert_eq!(1, Solution::kth_grammar(2, 2));
        assert_eq!(0, Solution::kth_grammar(3, 1));
        assert_eq!(1, Solution::kth_grammar(3, 2));
        assert_eq!(1, Solution::kth_grammar(3, 3));
        assert_eq!(0, Solution::kth_grammar(3, 4));
        assert_eq!(0, Solution::kth_grammar(4, 1));
        assert_eq!(1, Solution::kth_grammar(4, 5));
    }
}
