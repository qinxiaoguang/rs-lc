pub struct Solution {}
/*
 * @lc app=leetcode.cn id=861 lang=rust
 *
 * [861] 翻转矩阵后的得分
 *
 * https://leetcode-cn.com/problems/score-after-flipping-matrix/description/
 *
 * algorithms
 * Medium (74.02%)
 * Likes:    78
 * Dislikes: 0
 * Total Accepted:    5.6K
 * Total Submissions: 7.6K
 * Testcase Example:  '[[0,0,1,1],[1,0,1,0],[1,1,0,0]]'
 *
 * 有一个二维矩阵 A 其中每个元素的值为 0 或 1 。
 *
 * 移动是指选择任一行或列，并转换该行或列中的每一个值：将所有 0 都更改为 1，将所有 1 都更改为 0。
 *
 * 在做出任意次数的移动后，将该矩阵的每一行都按照二进制数来解释，矩阵的得分就是这些数字的总和。
 *
 * 返回尽可能高的分数。
 *
 *
 *
 *
 *
 *
 * 示例：
 *
 * 输入：[[0,0,1,1],[1,0,1,0],[1,1,0,0]]
 * 输出：39
 * 解释：
 * 转换为 [[1,1,1,1],[1,0,0,1],[1,1,1,1]]
 * 0b1111 + 0b1001 + 0b1111 = 15 + 9 + 15 = 39
 *
 *
 *
 * 提示：
 *
 *
 * 1 <= A.length <= 20
 * 1 <= A[0].length <= 20
 * A[i][j] 是 0 或 1
 *
 *
 */

// @lc code=start
impl Solution {
    // 其实不需要将二进制转换为10进制再做加和
    // 直接对二进制进行加和即可。
    // 其实最大的值就是争取将第一列中所有的数都变为1
    // 但是第一列所有数都变为1有很多种方法，怎么准确的找到最优秀的那一种
    // 我们可以得到这么一个规律，对于任何一列的数据，其反转后，如果1的个数最多，则反转后将变得最多。
    // 所以此题可以这么解，首先对于任何行的行头都是0的进行反转，将第一列的数都变为1
    // 接着将每一列的0多的数进行反转，即得到结果
    // 其实可以做个实验，不管你先对列做操作还是对行，最终的结果都一样。
    // 虽然不知道为啥，但是隐约觉得与高等代数中的矩阵的某个性质有关系
    pub fn matrix_score(a: Vec<Vec<i32>>) -> i32 {
        let mut a = a;
        if a.len() == 0 {
            return 0;
        }
        let (row, col) = (a.len(), a[0].len());
        // 记录i列的0的个数
        let mut zero_num = vec![0; col];
        for i in 0..row {
            let mut reverse = false;
            if a[i][0] == 0 {
                reverse = true;
            }
            for j in 0..col {
                if reverse {
                    a[i][j] = if a[i][j] == 1 { 0 } else { 1 }
                }
                if a[i][j] == 0 {
                    zero_num[j] += 1;
                }
            }
        }
        for j in 1..col {
            let reverse = zero_num[j] > row / 2;
            for i in 0..row {
                if reverse {
                    a[i][j] = if a[i][j] == 1 { 0 } else { 1 };
                }
            }
        }

        let mut res = 0;
        for v in a {
            // 将v转换为字符串
            let str = v
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("");
            let value = i32::from_str_radix(&str, 2).unwrap();
            res += value;
        }
        res
    }
}
// @lc code=end
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_l861() {
        assert_eq!(
            39,
            Solution::matrix_score(vec![vec![0, 0, 1, 1], vec![1, 0, 1, 0], vec![1, 1, 0, 0]])
        );
    }
}
